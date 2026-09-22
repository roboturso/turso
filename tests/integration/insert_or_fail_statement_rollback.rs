//! Regression tests for issue #8746.
//!
//! `INSERT OR FAIL` only changes how conflicts covered by `ON CONFLICT`
//! (UNIQUE, NOT NULL, CHECK, PRIMARY KEY) are handled. Any other error,
//! such as a foreign key violation or a rowid type mismatch, must undo
//! every row the statement already wrote, exactly like SQLite does.

use std::sync::Arc;

use crate::common::{limbo_exec_rows, TempDatabase};
use rusqlite::types::Value;

#[test]
fn insert_or_fail_foreign_key_violation_undoes_earlier_rows_in_autocommit() {
    let (db, conn) = setup_parent_child();

    let result = conn.execute("INSERT OR FAIL INTO u VALUES(1,1),(2,7)");
    let err = result.expect_err("second row references a missing parent");
    assert!(
        err.to_string().contains("FOREIGN KEY constraint failed"),
        "unexpected error: {err}"
    );

    assert_eq!(count(&db, &conn, "u"), 0);
}

#[test]
fn insert_or_fail_rowid_type_mismatch_undoes_earlier_rows_in_autocommit() {
    let db = TempDatabase::new_empty();
    let conn = db.connect_limbo();
    conn.execute("CREATE TABLE v(x INTEGER PRIMARY KEY)")
        .unwrap();

    let result = conn.execute("INSERT OR FAIL INTO v VALUES(1),('a')");
    let err = result.expect_err("'a' is not a valid rowid");
    assert!(
        err.to_string().contains("datatype mismatch"),
        "unexpected error: {err}"
    );

    assert_eq!(count(&db, &conn, "v"), 0);
}

#[test]
fn insert_or_fail_foreign_key_violation_undoes_earlier_rows_in_transaction() {
    let (db, conn) = setup_parent_child();

    conn.execute("BEGIN").unwrap();
    let result = conn.execute("INSERT OR FAIL INTO u VALUES(3,1),(4,7)");
    assert!(result.is_err(), "second row references a missing parent");
    conn.execute("COMMIT").unwrap();

    assert_eq!(count(&db, &conn, "u"), 0);
}

#[test]
fn insert_or_fail_still_keeps_rows_before_a_unique_conflict() {
    let db = TempDatabase::new_empty();
    let conn = db.connect_limbo();
    conn.execute("CREATE TABLE w(a INTEGER PRIMARY KEY, b TEXT UNIQUE)")
        .unwrap();

    let result = conn.execute("INSERT OR FAIL INTO w VALUES(1,'one'),(2,'one'),(3,'three')");
    let err = result.expect_err("second row conflicts on b");
    assert!(
        err.to_string().contains("UNIQUE constraint failed"),
        "unexpected error: {err}"
    );

    assert_eq!(count(&db, &conn, "w"), 1);
}

fn setup_parent_child() -> (TempDatabase, Arc<turso_core::Connection>) {
    let db = TempDatabase::new_empty();
    let conn = db.connect_limbo();
    conn.execute("PRAGMA foreign_keys=ON").unwrap();
    conn.execute("CREATE TABLE t(id INTEGER PRIMARY KEY)")
        .unwrap();
    conn.execute("CREATE TABLE u(id INTEGER PRIMARY KEY, p REFERENCES t(id))")
        .unwrap();
    conn.execute("INSERT INTO t VALUES(1)").unwrap();
    (db, conn)
}

fn count(_db: &TempDatabase, conn: &Arc<turso_core::Connection>, table: &str) -> i64 {
    let rows = limbo_exec_rows(conn, &format!("SELECT count(*) FROM {table}"));
    match rows.as_slice() {
        [row] => match row.as_slice() {
            [Value::Integer(n)] => *n,
            other => panic!("unexpected row: {other:?}"),
        },
        other => panic!("unexpected rows: {other:?}"),
    }
}
