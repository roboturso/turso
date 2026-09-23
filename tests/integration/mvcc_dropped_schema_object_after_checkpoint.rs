use crate::common::{ExecRows, TempDatabase};
use std::sync::Arc;
use turso_core::{Database, SqliteDialect};

#[turso_macros::test]
fn test_mvcc_dropped_trigger_stays_dropped_after_rename_and_checkpoint(
    db: TempDatabase,
) -> anyhow::Result<()> {
    let path = db.path.clone();
    let io = db.io.clone();
    {
        let conn = db.connect_limbo();
        conn.pragma_update("journal_mode", "'mvcc'")?;
        conn.execute("CREATE TABLE t1(id INTEGER PRIMARY KEY, a)")?;
        conn.execute("CREATE TABLE log(x)")?;
        conn.execute(
            "CREATE TRIGGER tr AFTER INSERT ON t1 BEGIN INSERT INTO log VALUES(new.id); END",
        )?;
        conn.execute("PRAGMA wal_checkpoint(TRUNCATE)")?;
        conn.execute("ALTER TABLE t1 RENAME TO t2")?;
        conn.execute("ALTER TABLE t2 RENAME TO t1")?;
        conn.execute("DROP TRIGGER tr")?;
        conn.execute("PRAGMA wal_checkpoint(TRUNCATE)")?;
        let rows: Vec<(String,)> =
            conn.exec_rows("SELECT name FROM sqlite_schema WHERE type='trigger'");
        assert_eq!(rows, Vec::<(String,)>::new());
    }
    drop(db);

    let db = Database::open_file(io, path.to_str().unwrap(), Arc::new(SqliteDialect))?;
    let conn = db.connect()?;
    conn.execute("INSERT INTO t1 VALUES(1,1)")?;
    let rows: Vec<(i64,)> = conn.exec_rows("SELECT x FROM log");
    assert_eq!(rows, Vec::<(i64,)>::new());
    Ok(())
}

#[turso_macros::test]
fn test_mvcc_dropped_view_stays_dropped_after_rename_and_checkpoint(
    db: TempDatabase,
) -> anyhow::Result<()> {
    let conn = db.connect_limbo();
    conn.pragma_update("journal_mode", "'mvcc'")?;
    conn.execute("CREATE TABLE t1(id INTEGER PRIMARY KEY, a)")?;
    conn.execute("CREATE VIEW v AS SELECT a FROM t1")?;
    conn.execute("PRAGMA wal_checkpoint(TRUNCATE)")?;
    conn.execute("ALTER TABLE t1 RENAME TO t2")?;
    conn.execute("ALTER TABLE t2 RENAME TO t1")?;
    conn.execute("DROP VIEW v")?;
    conn.execute("PRAGMA wal_checkpoint(TRUNCATE)")?;
    let rows: Vec<(String,)> = conn.exec_rows("SELECT name FROM sqlite_schema WHERE type='view'");
    assert_eq!(rows, Vec::<(String,)>::new());
    Ok(())
}

#[turso_macros::test]
fn test_mvcc_dropped_trigger_stays_dropped_after_restart_and_checkpoint(
    db: TempDatabase,
) -> anyhow::Result<()> {
    let path = db.path.clone();
    let io = db.io.clone();
    {
        let conn = db.connect_limbo();
        conn.pragma_update("journal_mode", "'mvcc'")?;
        conn.execute("CREATE TABLE t(x)")?;
        conn.execute("CREATE TRIGGER tr1 AFTER INSERT ON t BEGIN SELECT 1; END")?;
        conn.execute("PRAGMA wal_checkpoint(TRUNCATE)")?;
        conn.execute("DROP TRIGGER tr1")?;
    }
    drop(db);

    let db = Database::open_file(io, path.to_str().unwrap(), Arc::new(SqliteDialect))?;
    let conn = db.connect()?;
    conn.execute("CREATE TRIGGER tr2 AFTER INSERT ON t BEGIN SELECT 2; END")?;
    conn.execute("DROP TRIGGER tr2")?;
    conn.execute("PRAGMA wal_checkpoint(TRUNCATE)")?;
    let rows: Vec<(String,)> =
        conn.exec_rows("SELECT name FROM sqlite_schema WHERE type='trigger'");
    assert_eq!(rows, Vec::<(String,)>::new());
    Ok(())
}
