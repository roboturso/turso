//! Regression test for https://github.com/tursodatabase/turso/issues/7603
//!
//! Under `BEGIN CONCURRENT` (MVCC), when a sequence write (`nextval`) hits a
//! write-write conflict the transaction is rolled back, but a subsequent
//! `COMMIT` on that connection returned success instead of reporting that no
//! transaction is active. A client was told its transaction committed while
//! its writes (here, a DELETE) were silently discarded.
//!
//! The reproducer requires the in-memory MVCC backend (`:memory:`).

use turso_core::{Database, DatabaseOpts, OpenFlags};

#[test]
fn test_mvcc_commit_after_write_write_conflict_rollback_must_not_report_success(
) -> anyhow::Result<()> {
    let (_io, db) = Database::open_new::<&str>(
        ":memory:",
        None,
        OpenFlags::default(),
        DatabaseOpts::default(),
        None,
    )?;

    let setup = db.connect()?;
    setup.execute("PRAGMA journal_mode = 'mvcc'")?;
    setup.execute("CREATE TABLE t(a INTEGER)")?;
    setup.execute("CREATE SEQUENCE s")?;
    setup.execute("INSERT INTO t VALUES (1)")?;

    let conn1 = db.connect()?;
    let conn0 = db.connect()?;

    conn1.execute("BEGIN CONCURRENT")?;
    conn0.execute("BEGIN CONCURRENT")?;

    conn1.execute("DELETE FROM t WHERE TRUE")?;
    conn0.execute("SELECT setval('s', 100, true)")?;

    // conn1's nextval conflicts with conn0's concurrent setval on the same
    // sequence: the engine rolls conn1's transaction back.
    let nextval_res = conn1.execute("SELECT nextval('s')");
    assert!(
        nextval_res.is_err(),
        "expected write-write conflict on nextval, got {nextval_res:?}"
    );

    conn0.execute("COMMIT")?;

    // conn1 was already rolled back by the conflict, so its COMMIT must not
    // report success while the transaction's writes were discarded.
    let commit_res = conn1.execute("COMMIT");

    // Fresh connection so we observe the durably committed state.
    let checker = db.connect()?;
    let mut stmt = checker.prepare("SELECT count(*) FROM t")?;
    let mut count: Option<i64> = None;
    stmt.run_with_row_callback(|row| {
        count = Some(row.get::<i64>(0)?);
        Ok(())
    })?;
    let count = count.expect("count query returned no row");

    match commit_res {
        // Acceptable fix: COMMIT reports that the transaction is no longer
        // active, and conn1's DELETE did not take effect.
        Err(_) => {
            assert_eq!(
                count, 1,
                "COMMIT failed but conn1's rolled-back DELETE took effect"
            );
        }
        // Also acceptable: COMMIT succeeds only if it durably applied conn1's
        // writes. Pre-fix behavior (COMMIT ok, DELETE discarded) must fail.
        Ok(()) => {
            assert_eq!(
                count, 0,
                "COMMIT reported success but conn1's DELETE was silently discarded \
                 (write-write conflict already rolled the transaction back)"
            );
        }
    }

    Ok(())
}
