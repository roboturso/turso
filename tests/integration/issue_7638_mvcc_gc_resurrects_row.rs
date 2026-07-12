//! Regression test for issue #7638: an abandoned MVCC post-commit
//! auto-checkpoint combined with GC resurrects a deleted row and corrupts the
//! secondary index.

use std::sync::Arc;

use turso_core::{Database, DatabaseOpts, OpenFlags, PlatformIO, StepResult, Value};

fn open_conn(path: &str) -> Arc<turso_core::Connection> {
    let io = Arc::new(PlatformIO::new().unwrap());
    let db =
        Database::open_file_with_flags(io, path, OpenFlags::default(), DatabaseOpts::new(), None)
            .unwrap();
    db.connect().unwrap()
}

fn collect(conn: &Arc<turso_core::Connection>, sql: &str) -> Vec<Vec<Value>> {
    let mut stmt = conn.prepare(sql).unwrap();
    let mut rows = Vec::new();
    loop {
        match stmt.step().unwrap() {
            StepResult::Row => rows.push(stmt.row().unwrap().get_values().cloned().collect()),
            StepResult::Done => return rows,
            StepResult::IO => stmt._io().step().unwrap(),
            StepResult::Yield => {}
            other => panic!("unexpected step result for {sql}: {other:?}"),
        }
    }
}

fn abandon_after_pause(conn: &Arc<turso_core::Connection>, sql: &str, pause_target: usize) -> bool {
    let mut stmt = conn.prepare(sql).unwrap();
    let mut pauses = 0usize;
    loop {
        match stmt.step().unwrap() {
            StepResult::IO => {
                pauses += 1;
                stmt._io().step().unwrap();
                if pauses == pause_target {
                    drop(stmt);
                    return true;
                }
            }
            StepResult::Yield => {
                pauses += 1;
                if pauses == pause_target {
                    drop(stmt);
                    return true;
                }
            }
            StepResult::Row => {}
            StepResult::Done => return false,
            other => panic!("unexpected step result for {sql}: {other:?}"),
        }
    }
}

#[test]
fn abandoned_post_durable_checkpoint_gc_indexed_sibling_probe() {
    const ROWS: i64 = 1_500;
    const TARGET: i64 = 1_500;
    let pause_target = 9;

    let dir = tempfile::TempDir::new().unwrap();
    let path = dir.path().join("gc-witness-indexed.db");
    let path = path.to_str().unwrap();

    let conn = open_conn(path);
    conn.execute("PRAGMA journal_mode = 'mvcc'").unwrap();
    conn.execute("PRAGMA mvcc_checkpoint_threshold = -1")
        .unwrap();
    conn.execute("PRAGMA mvcc_gc_threshold = 1").unwrap();
    conn.execute("CREATE TABLE t(id INTEGER PRIMARY KEY, v TEXT)")
        .unwrap();
    conn.execute("CREATE INDEX t_v ON t(v)").unwrap();
    conn.execute("PRAGMA wal_checkpoint(TRUNCATE)").unwrap();
    conn.execute("PRAGMA mvcc_checkpoint_threshold = 0")
        .unwrap();

    conn.execute("BEGIN CONCURRENT").unwrap();
    for id in 1..=ROWS {
        conn.execute(format!("INSERT INTO t VALUES({id}, 'old{id}')"))
            .unwrap();
    }

    assert!(abandon_after_pause(&conn, "COMMIT", pause_target));
    assert_eq!(
        collect(&conn, "SELECT count(*) FROM t")[0][0]
            .as_int()
            .unwrap(),
        ROWS
    );

    conn.execute("PRAGMA mvcc_checkpoint_threshold = -1")
        .unwrap();
    conn.execute(format!("UPDATE t SET v = 'mid' WHERE id = {TARGET}"))
        .unwrap();
    conn.execute(format!("DELETE FROM t WHERE id = {TARGET}"))
        .unwrap();

    assert_eq!(
        collect(&conn, &format!("SELECT id, v FROM t WHERE id = {TARGET}")),
        Vec::<Vec<Value>>::new()
    );

    conn.execute("PRAGMA wal_checkpoint(TRUNCATE)").unwrap();

    let table_rows = collect(&conn, &format!("SELECT id, v FROM t WHERE id = {TARGET}"));
    let index_rows = collect(
        &conn,
        &format!("SELECT id, v FROM t INDEXED BY t_v WHERE v = 'old{TARGET}'"),
    );
    let integrity = collect(&conn, "PRAGMA integrity_check");

    assert_eq!(table_rows, Vec::<Vec<Value>>::new());
    assert_eq!(index_rows, Vec::<Vec<Value>>::new());
    assert_eq!(integrity, vec![vec![Value::build_text("ok")]]);
}
