//! Regression test for issue #6860.
//!
//! When an UPSERT's DO UPDATE branch hits a constraint error, SQLite resolves
//! it with ABORT, so only that statement is undone. The INSERT's OR ROLLBACK
//! clause does not apply to the DO UPDATE. The explicit transaction must stay
//! open and earlier statements in it must remain visible.

use crate::common::{limbo_exec_rows, TempDatabase};
use rusqlite::types::Value;

#[turso_macros::test]
fn upsert_or_rollback_do_update_failure_keeps_transaction(tmp_db: TempDatabase) {
    let conn = tmp_db.connect_limbo();
    conn.execute("CREATE TABLE t(id INTEGER PRIMARY KEY, u TEXT UNIQUE, a TEXT UNIQUE)")
        .unwrap();
    conn.execute("INSERT INTO t VALUES(1,'u1','a1'),(2,'u2','a2')")
        .unwrap();

    conn.execute("BEGIN").unwrap();
    conn.execute("INSERT INTO t VALUES(3,'u3','a3')").unwrap();

    let err = conn
        .execute(
            "INSERT OR ROLLBACK INTO t(u,a) VALUES('u1','a2') ON CONFLICT(u) DO UPDATE SET a=excluded.a",
        )
        .expect_err("DO UPDATE must fail with UNIQUE constraint on t.a");
    assert!(
        err.to_string().contains("UNIQUE constraint failed: t.a"),
        "unexpected error: {err}"
    );

    let rows = limbo_exec_rows(&conn, "SELECT count(*) FROM t WHERE id=3");
    assert_eq!(
        rows,
        vec![vec![Value::Integer(1)]],
        "row inserted earlier in the transaction must still be visible"
    );

    let rows = limbo_exec_rows(&conn, "SELECT id, u, a FROM t ORDER BY id");
    assert_eq!(
        rows,
        vec![
            vec![
                Value::Integer(1),
                Value::Text("u1".into()),
                Value::Text("a1".into())
            ],
            vec![
                Value::Integer(2),
                Value::Text("u2".into()),
                Value::Text("a2".into())
            ],
            vec![
                Value::Integer(3),
                Value::Text("u3".into()),
                Value::Text("a3".into())
            ],
        ]
    );

    conn.execute("COMMIT")
        .expect("transaction must still be open after the failed UPSERT");

    let rows = limbo_exec_rows(&conn, "SELECT count(*) FROM t");
    assert_eq!(rows, vec![vec![Value::Integer(3)]]);
}
