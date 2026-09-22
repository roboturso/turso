//! Regression tests for https://github.com/tursodatabase/turso/issues/6878
//!
//! The RETURNING clause runs after the row has already been written. When the
//! RETURNING expression fails inside an explicit transaction, the row write
//! must be undone and the transaction must still be able to COMMIT with the
//! table unchanged, matching SQLite.
//!
//! Multi-row statements already open a statement savepoint, so the issue's
//! own reproducers pass. Single-row statements skip the savepoint because
//! they are not marked as multi-write, so the written row survives the error
//! and gets committed.
use crate::common::TempDatabase;
use std::sync::Arc;

fn query_rows(conn: &Arc<turso_core::Connection>, sql: &str) -> Vec<String> {
    let mut stmt = conn.prepare(sql).unwrap();
    let mut rows = Vec::new();
    stmt.run_with_row_callback(|row| {
        let vals: Vec<String> = row.get_values().map(|v| format!("{v}")).collect();
        rows.push(vals.join("|"));
        Ok(())
    })
    .unwrap();
    rows
}

fn assert_returning_fails(conn: &Arc<turso_core::Connection>, sql: &str) {
    let result = conn.execute(sql);
    let err = result.expect_err("statement with invalid ESCAPE in RETURNING must fail");
    assert!(
        err.to_string().contains("ESCAPE"),
        "unexpected error: {err}"
    );
}

#[turso_macros::test]
fn single_row_insert_returning_error_in_tx_keeps_rows(tmp_db: TempDatabase) -> anyhow::Result<()> {
    let conn = tmp_db.connect_limbo();
    conn.execute("CREATE TABLE t(a INT)")?;
    conn.execute("INSERT INTO t VALUES(1)")?;

    conn.execute("BEGIN")?;
    assert_returning_fails(
        &conn,
        "INSERT INTO t VALUES(2) RETURNING 'x' LIKE 'x' ESCAPE 'yy'",
    );
    assert_eq!(query_rows(&conn, "SELECT a FROM t ORDER BY a"), vec!["1"]);
    conn.execute("COMMIT")?;

    assert_eq!(query_rows(&conn, "SELECT a FROM t ORDER BY a"), vec!["1"]);
    Ok(())
}

#[turso_macros::test]
fn single_row_delete_returning_error_in_tx_keeps_rows(tmp_db: TempDatabase) -> anyhow::Result<()> {
    let conn = tmp_db.connect_limbo();
    conn.execute("CREATE TABLE t(a INT)")?;
    conn.execute("INSERT INTO t VALUES(1),(2)")?;

    conn.execute("BEGIN")?;
    conn.execute("INSERT INTO t VALUES(3)")?;
    assert_returning_fails(
        &conn,
        "DELETE FROM t WHERE rowid=1 RETURNING 'x' LIKE 'x' ESCAPE 'yy'",
    );
    assert_eq!(
        query_rows(&conn, "SELECT a FROM t ORDER BY a"),
        vec!["1", "2", "3"]
    );
    conn.execute("COMMIT")?;

    assert_eq!(
        query_rows(&conn, "SELECT a FROM t ORDER BY a"),
        vec!["1", "2", "3"]
    );
    Ok(())
}

#[turso_macros::test]
fn single_row_update_returning_error_in_tx_keeps_rows(tmp_db: TempDatabase) -> anyhow::Result<()> {
    let conn = tmp_db.connect_limbo();
    conn.execute("CREATE TABLE t(a INT, b INT)")?;
    conn.execute("INSERT INTO t VALUES(1,10),(2,20)")?;

    conn.execute("BEGIN")?;
    assert_returning_fails(
        &conn,
        "UPDATE t SET b=b+1 WHERE rowid=1 RETURNING 'x' LIKE 'x' ESCAPE 'yy'",
    );
    assert_eq!(
        query_rows(&conn, "SELECT a,b FROM t ORDER BY a"),
        vec!["1|10", "2|20"]
    );
    conn.execute("COMMIT")?;

    assert_eq!(
        query_rows(&conn, "SELECT a,b FROM t ORDER BY a"),
        vec!["1|10", "2|20"]
    );
    Ok(())
}

#[turso_macros::test]
fn delete_returning_error_in_tx_keeps_rows(tmp_db: TempDatabase) -> anyhow::Result<()> {
    let conn = tmp_db.connect_limbo();
    conn.execute("CREATE TABLE t(a INT)")?;
    conn.execute("INSERT INTO t VALUES(1),(2)")?;

    conn.execute("BEGIN")?;
    assert_returning_fails(
        &conn,
        "DELETE FROM t WHERE a=1 RETURNING 'x' LIKE 'x' ESCAPE 'yy'",
    );
    assert_eq!(
        query_rows(&conn, "SELECT * FROM t ORDER BY a"),
        vec!["1", "2"]
    );
    conn.execute("COMMIT")?;

    assert_eq!(
        query_rows(&conn, "SELECT * FROM t ORDER BY a"),
        vec!["1", "2"]
    );
    assert_eq!(query_rows(&conn, "PRAGMA integrity_check"), vec!["ok"]);
    Ok(())
}

#[turso_macros::test]
fn delete_returning_row_dependent_error_in_tx_keeps_rows(
    tmp_db: TempDatabase,
) -> anyhow::Result<()> {
    let conn = tmp_db.connect_limbo();
    conn.execute("CREATE TABLE t(a INT, e TEXT)")?;
    conn.execute("INSERT INTO t VALUES(1,'yy'),(2,'yy')")?;

    conn.execute("BEGIN")?;
    conn.execute("INSERT INTO t VALUES(3,'y')")?;
    assert_returning_fails(&conn, "DELETE FROM t RETURNING 'x' LIKE 'x' ESCAPE e");
    assert_eq!(
        query_rows(&conn, "SELECT a FROM t ORDER BY a"),
        vec!["1", "2", "3"]
    );
    conn.execute("COMMIT")?;

    assert_eq!(
        query_rows(&conn, "SELECT a FROM t ORDER BY a"),
        vec!["1", "2", "3"]
    );
    Ok(())
}

#[turso_macros::test]
fn update_returning_error_in_tx_keeps_rows(tmp_db: TempDatabase) -> anyhow::Result<()> {
    let conn = tmp_db.connect_limbo();
    conn.execute("CREATE TABLE t(a INT, b INT, e TEXT)")?;
    conn.execute("INSERT INTO t VALUES(1,10,'yy'),(2,20,'yy')")?;

    conn.execute("BEGIN")?;
    assert_returning_fails(&conn, "UPDATE t SET b=b+1 RETURNING 'x' LIKE 'x' ESCAPE e");
    assert_eq!(
        query_rows(&conn, "SELECT a,b FROM t ORDER BY a"),
        vec!["1|10", "2|20"]
    );
    conn.execute("COMMIT")?;

    assert_eq!(
        query_rows(&conn, "SELECT a,b FROM t ORDER BY a"),
        vec!["1|10", "2|20"]
    );
    Ok(())
}

#[turso_macros::test]
fn insert_or_replace_returning_error_in_tx_keeps_old_row(
    tmp_db: TempDatabase,
) -> anyhow::Result<()> {
    let conn = tmp_db.connect_limbo();
    conn.execute("CREATE TABLE t(a INT UNIQUE, b INT)")?;
    conn.execute("INSERT INTO t VALUES(1,10)")?;

    conn.execute("BEGIN")?;
    assert_returning_fails(
        &conn,
        "INSERT OR REPLACE INTO t VALUES(1,20) RETURNING 'x' LIKE 'x' ESCAPE 'yy'",
    );
    assert_eq!(query_rows(&conn, "SELECT rowid,a,b FROM t"), vec!["1|1|10"]);
    conn.execute("COMMIT")?;

    assert_eq!(query_rows(&conn, "SELECT rowid,a,b FROM t"), vec!["1|1|10"]);
    assert_eq!(query_rows(&conn, "PRAGMA integrity_check"), vec!["ok"]);
    Ok(())
}

#[turso_macros::test]
fn multi_row_insert_returning_error_in_tx_keeps_rows(tmp_db: TempDatabase) -> anyhow::Result<()> {
    let conn = tmp_db.connect_limbo();
    conn.execute("CREATE TABLE t(a INT)")?;
    conn.execute("INSERT INTO t VALUES(1)")?;

    conn.execute("BEGIN")?;
    assert_returning_fails(
        &conn,
        "INSERT INTO t VALUES(2),(3) RETURNING 'x' LIKE 'x' ESCAPE 'yy'",
    );
    assert_eq!(query_rows(&conn, "SELECT a FROM t ORDER BY a"), vec!["1"]);
    conn.execute("COMMIT")?;

    assert_eq!(query_rows(&conn, "SELECT a FROM t ORDER BY a"), vec!["1"]);
    Ok(())
}
