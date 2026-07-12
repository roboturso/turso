//! Regression test for issue #6859.
//!
//! `INSERT OR REPLACE ... ON CONFLICT(u) DO UPDATE` must use ABORT semantics
//! for constraint violations raised by the DO UPDATE arm, like SQLite does.
//! Turso incorrectly used REPLACE journal analysis for the DO UPDATE arm,
//! so a failed UPDATE left the secondary index out of sync with the table
//! (index corruption reported by `PRAGMA integrity_check`).

use std::sync::Arc;

use crate::common::{limbo_exec_rows, TempDatabase};
use turso_core::Value;

/// Run `PRAGMA integrity_check` and return the joined result text.
fn run_integrity_check(conn: &Arc<turso_core::Connection>) -> String {
    let rows = conn
        .pragma_query("integrity_check")
        .expect("integrity_check should succeed");
    rows.into_iter()
        .filter_map(|row| {
            row.into_iter().next().and_then(|v| {
                if let Value::Text(text) = v {
                    Some(text.as_str().to_string())
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn test_upsert_or_replace_do_update_unique_failure_no_index_corruption() {
    let tmp_db = TempDatabase::builder()
        .with_db_name("issue_6859.db")
        .build();
    let conn = tmp_db.connect_limbo();

    conn.execute("CREATE TABLE t(id INTEGER PRIMARY KEY, u TEXT UNIQUE, a TEXT UNIQUE)")
        .unwrap();
    conn.execute("INSERT INTO t VALUES(1,'u1','a1'),(2,'u2','a2')")
        .unwrap();

    conn.execute("BEGIN").unwrap();

    // The DO UPDATE arm sets t.a='a2' for row 1, conflicting with row 2's
    // unique value. SQLite aborts this statement with a UNIQUE constraint
    // failure and leaves the database intact.
    let res = conn.execute(
        "INSERT OR REPLACE INTO t(u,a) VALUES('u1','a2') \
         ON CONFLICT(u) DO UPDATE SET a=excluded.a",
    );
    let err = res.expect_err("DO UPDATE arm must fail with UNIQUE constraint violation");
    assert!(
        format!("{err}").contains("UNIQUE constraint failed: t.a"),
        "unexpected error: {err}"
    );

    // The failed statement must have been rolled back cleanly: the secondary
    // index on t.a must still be consistent with the table.
    let result = run_integrity_check(&conn);
    assert_eq!(
        result, "ok",
        "integrity_check must pass inside the transaction after the failed UPSERT"
    );

    conn.execute("COMMIT").unwrap();

    let result = run_integrity_check(&conn);
    assert_eq!(
        result, "ok",
        "integrity_check must pass after COMMIT following the failed UPSERT"
    );

    // Table contents must be unchanged by the aborted statement.
    let rows = limbo_exec_rows(&conn, "SELECT id, u, a FROM t ORDER BY id");
    assert_eq!(
        rows,
        vec![
            vec![
                rusqlite::types::Value::Integer(1),
                rusqlite::types::Value::Text("u1".to_string()),
                rusqlite::types::Value::Text("a1".to_string()),
            ],
            vec![
                rusqlite::types::Value::Integer(2),
                rusqlite::types::Value::Text("u2".to_string()),
                rusqlite::types::Value::Text("a2".to_string()),
            ],
        ]
    );
}
