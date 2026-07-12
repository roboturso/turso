//! Regression test for https://github.com/tursodatabase/turso/issues/7476
//!
//! While a `COMMIT` of a `BEGIN CONCURRENT` transaction is suspended at a
//! yield point (`CommitYieldPoint::LogRecordPrepared`), the same connection
//! must not be able to run statements that misread the transaction's own
//! prepared writes (a `SELECT` returning an empty result set even though the
//! transaction inserted a row), nor should an `INSERT` panic instead of
//! returning a proper error.

use super::tests::MvccTestDbNoConn;
use crate::mvcc::database::CommitYieldPoint;
use crate::mvcc::yield_hooks::YieldPointMarker;
use crate::mvcc::yield_points::{YieldInjector, YieldPoint};
use crate::sync::Mutex;
use crate::types::Value;
use crate::Connection;
use rustc_hash::FxHashSet as HashSet;
use std::sync::Arc;

#[derive(Debug)]
struct FixedYieldInjector {
    remaining: Mutex<HashSet<YieldPoint>>,
}

impl FixedYieldInjector {
    fn new(points: impl IntoIterator<Item = YieldPoint>) -> Arc<Self> {
        Arc::new(Self {
            remaining: Mutex::new(points.into_iter().collect()),
        })
    }

    fn is_empty(&self) -> bool {
        self.remaining.lock().is_empty()
    }
}

impl YieldInjector for FixedYieldInjector {
    fn should_yield(&self, _instance_id: u64, _selection_key: u64, point: YieldPoint) -> bool {
        self.remaining.lock().remove(&point)
    }
}

fn try_get_rows(conn: &Arc<Connection>, query: &str) -> crate::Result<Vec<Vec<Value>>> {
    let mut stmt = conn.prepare(query)?;
    let mut rows = Vec::new();
    stmt.run_with_row_callback(|row| {
        let values = row.get_values().cloned().collect::<Vec<_>>();
        rows.push(values);
        Ok(())
    })?;
    Ok(rows)
}

#[test]
fn mvcc_same_connection_statement_during_yielded_commit() {
    let db = MvccTestDbNoConn::new_with_random_db();
    let conn = db.connect();

    conn.execute("CREATE TABLE t(id INTEGER PRIMARY KEY, v TEXT)")
        .unwrap();

    conn.execute("BEGIN CONCURRENT").unwrap();
    conn.execute("INSERT INTO t VALUES (1, 'pending')").unwrap();

    let injector = FixedYieldInjector::new([CommitYieldPoint::LogRecordPrepared.point()]);
    conn.set_yield_injector(Some(injector.clone()));

    let mut commit = conn.prepare("COMMIT").unwrap();
    let first = commit.step();

    assert!(
        matches!(first, Ok(crate::StepResult::Yield)),
        "COMMIT should yield at LogRecordPrepared, got {first:?}"
    );
    assert!(injector.is_empty(), "commit injector should have fired");

    // While the COMMIT is suspended, a SELECT on the same connection must
    // either fail cleanly or still observe the transaction's own pending
    // write. It must not panic and must not silently return an empty
    // (wrong) result set.
    let select_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        try_get_rows(&conn, "SELECT id, v FROM t ORDER BY id")
    }));
    match &select_result {
        Err(_) => panic!("SELECT during yielded COMMIT panicked instead of returning an error"),
        Ok(Ok(rows)) => assert_eq!(
            rows,
            &[vec![Value::from_i64(1), Value::build_text("pending")]],
            "SELECT during yielded COMMIT misread the transaction's own pending write"
        ),
        Ok(Err(_)) => {
            // A clean error is acceptable behavior.
        }
    }

    // An INSERT on the same connection while COMMIT is suspended must not
    // panic; it may fail with a proper error or be rejected.
    let insert_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        conn.execute("INSERT INTO t VALUES (2, 'during-commit')")
    }));
    assert!(
        insert_result.is_ok(),
        "INSERT during yielded COMMIT panicked instead of returning an error"
    );

    conn.set_yield_injector(None);
    commit.run_ignore_rows().unwrap();
    drop(commit);

    let final_rows = try_get_rows(&conn, "SELECT id, v FROM t ORDER BY id").unwrap();
    assert!(
        final_rows
            .first()
            .is_some_and(|row| row == &[Value::from_i64(1), Value::build_text("pending")]),
        "committed row must be visible after COMMIT completes, got {final_rows:?}"
    );
}
