//! Regression test for issue #7480: an MVCC checkpoint truncates the logical
//! log (to zero bytes) before it truncates the WAL. A crash in that window
//! leaves committed WAL frames on disk alongside an empty logical log, and
//! startup recovery classified that state as corruption
//! (`Corrupt("WAL has committed frames but logical log header is missing")`)
//! instead of completing the interrupted checkpoint, making the database
//! impossible to reopen after a clean crash.

use std::sync::Arc;

use crate::mvcc::database::checkpoint_state_machine::{CheckpointState, CheckpointStateMachine};
use crate::mvcc::database::tests::MvccTestDbNoConn;
use crate::state_machine::{StateTransition, TransitionResult};
use crate::{Connection, Value};

fn get_rows(conn: &Arc<Connection>, query: &str) -> Vec<Vec<Value>> {
    let mut stmt = conn.prepare(query).unwrap();
    let mut rows = Vec::new();
    stmt.run_with_row_callback(|row| {
        rows.push(row.get_values().cloned().collect::<Vec<_>>());
        Ok(())
    })
    .unwrap();
    rows
}

/// What this test checks: reopening after a crash that happens after the
/// checkpoint truncated the logical log but before it truncated the WAL must
/// succeed and preserve all committed data.
/// Why this matters: this is a normal crash window in the checkpoint sequence
/// (log truncate + fsync happen before WAL truncate), so recovery must treat
/// "committed WAL frames + empty logical log" as an interrupted checkpoint,
/// not as corruption.
#[test]
fn test_checkpoint_crash_after_log_truncate_before_wal_truncate_recovers() {
    let mut db = MvccTestDbNoConn::new_with_random_db();
    let wal_path = std::path::PathBuf::from(format!("{}-wal", db.get_db().path));

    {
        let conn = db.connect();

        conn.execute("CREATE TABLE t(id INTEGER PRIMARY KEY, v TEXT)")
            .unwrap();
        conn.execute("INSERT INTO t VALUES (1, 'a')").unwrap();
        conn.execute("INSERT INTO t VALUES (2, 'b')").unwrap();

        let mvcc_store = db.get_mvcc_store();
        let pager = conn.pager.load().clone();
        let mut checkpoint_sm = CheckpointStateMachine::new(
            pager.clone(),
            mvcc_store.clone(),
            conn.clone(),
            true,
            conn.get_sync_mode(),
            crate::MAIN_DB_ID,
        );

        let mut reached_pre_wal_truncate = false;
        for _ in 0..50_000 {
            if checkpoint_sm.state_for_test() == CheckpointState::TruncateWal {
                // Simulate a crash after the logical log was truncated and
                // fsynced but before the WAL was truncated.
                reached_pre_wal_truncate = true;
                break;
            }
            match checkpoint_sm.step(&()).unwrap() {
                TransitionResult::Io(io) => io.wait(pager.io.as_ref()).unwrap(),
                TransitionResult::Continue => {}
                TransitionResult::Done(_) => {
                    panic!("checkpoint finished before reaching the TruncateWal state")
                }
            }
        }
        assert!(
            reached_pre_wal_truncate,
            "checkpoint state machine never reached TruncateWal"
        );

        // Crash-window invariants: the logical log has already been truncated
        // to zero bytes while the WAL still holds committed frames.
        assert_eq!(
            mvcc_store.get_logical_log_file().size().unwrap(),
            0,
            "logical log should be truncated before the WAL"
        );
        let wal_len = wal_path.metadata().map(|m| m.len()).unwrap_or(0);
        assert!(
            wal_len > 0,
            "WAL file must still contain committed frames in the crash window"
        );
    }

    db.restart_result().expect(
        "reopening after a crash between logical-log truncate and WAL truncate should succeed",
    );

    let conn = db
        .get_db()
        .connect()
        .expect("connecting after the checkpoint crash window should succeed");

    let rows = get_rows(&conn, "SELECT id, v FROM t ORDER BY id");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0][0].as_int().unwrap(), 1);
    assert_eq!(rows[0][1].to_string(), "a");
    assert_eq!(rows[1][0].as_int().unwrap(), 2);
    assert_eq!(rows[1][1].to_string(), "b");

    let integrity = get_rows(&conn, "PRAGMA integrity_check");
    assert_eq!(integrity[0][0].to_string(), "ok");
}
