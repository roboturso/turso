//! Regression test for https://github.com/tursodatabase/turso/issues/7642
//!
//! When an MVCC checkpoint fails with DatabaseFull (because max_page_count is
//! exceeded), the failed checkpoint must not leak partially-applied root
//! mappings. Pre-fix, the next read of the table used the leaked root mapping
//! and failed with `short read on page 3: expected 4096 bytes, got 0`.

use crate::common::TempDatabase;

#[turso_macros::test]
fn test_mvcc_read_after_checkpoint_database_full(db: TempDatabase) -> anyhow::Result<()> {
    let conn = db.connect_limbo();
    conn.pragma_update("journal_mode", "'mvcc'")?;
    conn.execute("PRAGMA mvcc_checkpoint_threshold=-1")?;
    conn.execute("PRAGMA max_page_count=5")?;
    conn.execute("CREATE TABLE t(x TEXT)")?;
    conn.execute("INSERT INTO t SELECT printf('%.*c', 1800, 'x') FROM generate_series(1, 60)")?;

    // The checkpoint must fail: the data cannot fit within max_page_count pages.
    let err = conn
        .execute("PRAGMA wal_checkpoint(TRUNCATE)")
        .expect_err("checkpoint should fail with DatabaseFull");
    assert!(
        err.to_string().contains("full"),
        "expected DatabaseFull error, got: {err}"
    );

    // A read after the failed checkpoint must still see all rows. Pre-fix it
    // failed with `short read on page 3: expected 4096 bytes, got 0`.
    let mut stmt = conn.prepare("SELECT count(*) FROM t")?;
    let mut count = None;
    loop {
        match stmt.step()? {
            turso_core::StepResult::Row => {
                count = Some(stmt.row().unwrap().get::<i64>(0)?);
            }
            turso_core::StepResult::Done => break,
            turso_core::StepResult::IO => {
                stmt._io().step()?;
            }
            turso_core::StepResult::Yield => {}
            other => anyhow::bail!("unexpected step result: {other:?}"),
        }
    }
    assert_eq!(count, Some(60));

    Ok(())
}
