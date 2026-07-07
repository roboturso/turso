//! Regression test for https://github.com/tursodatabase/turso/issues/7474
//!
//! For a table without AUTOINCREMENT, the rowid allocated for an insert must
//! be based on the largest rowid currently in the table, not a persistent
//! counter. In MVCC mode, a rolled-back insert must not permanently bump the
//! next rowid: after `BEGIN CONCURRENT; INSERT ...; ROLLBACK;` a fresh insert
//! into an empty table should get rowid 1, matching SQLite and Turso's
//! non-MVCC behavior.

use crate::common::{ExecRows, TempDatabase};

#[turso_macros::test]
fn test_mvcc_rowid_not_sticky_after_rollback(tmp_db: TempDatabase) -> anyhow::Result<()> {
    let conn = tmp_db.connect_limbo();
    conn.pragma_update("journal_mode", "'mvcc'")?;

    conn.execute("CREATE TABLE t(a)")?;

    conn.execute("BEGIN CONCURRENT")?;
    conn.execute("INSERT INTO t VALUES(NULL)")?;
    conn.execute("ROLLBACK")?;

    conn.execute("INSERT INTO t VALUES(NULL)")?;

    let rows: Vec<(i64,)> = conn.exec_rows("SELECT rowid FROM t");
    assert_eq!(
        rows,
        vec![(1,)],
        "rowid allocation without AUTOINCREMENT must restart from max(rowid)+1 \
         after a rolled-back insert; got {rows:?}"
    );

    Ok(())
}
