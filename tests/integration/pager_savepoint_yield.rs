/// Regression test for https://github.com/tursodatabase/turso/issues/7536
///
/// Dropping a statement mid-execution (after StepResult::IO) inside an open
/// transaction must roll back the statement subtransaction cleanly. Under
/// cache spill pressure, the abandoned DELETE's spilled overflow pages leaked
/// into the committed transaction and corrupted the database.
use std::sync::Arc;

use turso_core::{Database, DatabaseOpts, MemoryYieldIO, OpenFlags, StepResult};

#[test]
fn stmt_rollback_after_abandoned_io_with_cache_spill() {
    let io = Arc::new(MemoryYieldIO::new());
    let db = Database::open_file_with_flags(
        io.clone(),
        "pager-savepoint-yield-repro.db",
        OpenFlags::Create,
        DatabaseOpts::new(),
        None,
    )
    .unwrap();
    let conn = db.connect().unwrap();

    conn.execute("PRAGMA page_size = 1024").unwrap();
    conn.execute("PRAGMA journal_mode = WAL").unwrap();
    conn.execute("PRAGMA cache_size = 12").unwrap();
    conn.execute("PRAGMA cache_spill = ON").unwrap();
    conn.execute("CREATE TABLE t(id INTEGER PRIMARY KEY, x BLOB)")
        .unwrap();

    conn.execute("BEGIN").unwrap();
    for id in 1..=32 {
        let size = 6000 + (id % 7) * 733;
        conn.execute(format!("INSERT INTO t VALUES ({id}, zeroblob({size}))"))
            .unwrap();
    }
    conn.execute("COMMIT").unwrap();

    conn.execute("BEGIN").unwrap();

    let mut stmt = conn
        .prepare("DELETE FROM t WHERE id BETWEEN 1 AND 16")
        .unwrap();
    match stmt.step().unwrap() {
        StepResult::IO => drop(stmt),
        other => panic!("expected IO, got {other:?}"),
    }

    conn.execute("COMMIT").unwrap();

    let mut check = conn.query("PRAGMA integrity_check").unwrap().unwrap();
    let rows = check.run_collect_rows().unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].len(), 1);
    let msg = rows[0][0].to_string();
    assert_eq!(msg, "ok", "integrity_check failed: {msg}");
}
