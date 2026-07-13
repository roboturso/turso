//! Regression test for issue #6858: UPSERT DO UPDATE deletes secondary index
//! entries before proving the replacement row is valid.
//!
//! When an `INSERT ... ON CONFLICT(u) DO UPDATE` resolves the conflict but the
//! resulting UPDATE itself violates another UNIQUE constraint, the statement
//! must fail without leaving the target row's secondary index entries deleted.
//! Previously, index entries for the conflicting row were removed before the
//! replacement row was validated, corrupting all secondary indexes.

use std::sync::Arc;

use crate::common::{limbo_exec_rows, TempDatabase};
use rusqlite::types::Value;

fn run_integrity_check(conn: &Arc<turso_core::Connection>) -> String {
    let rows = conn
        .pragma_query("integrity_check")
        .expect("integrity_check should succeed");

    rows.into_iter()
        .filter_map(|row| {
            row.into_iter().next().and_then(|v| {
                if let turso_core::Value::Text(text) = v {
                    Some(text.as_str().to_string())
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[turso_macros::test]
fn test_upsert_do_update_failure_preserves_indexes(tmp_db: TempDatabase) -> anyhow::Result<()> {
    let conn = tmp_db.connect_limbo();

    conn.execute("CREATE TABLE t(id INTEGER PRIMARY KEY, u INT UNIQUE, b INT, c INT UNIQUE)")?;
    conn.execute("CREATE INDEX idx_b ON t(b)")?;
    conn.execute("INSERT INTO t VALUES(1,1,10,10)")?;
    conn.execute("INSERT INTO t VALUES(2,2,20,20)")?;

    conn.execute("BEGIN")?;

    // The INSERT conflicts on u=1 (row 1), so DO UPDATE fires; the update sets
    // c=20 which conflicts with row 2 -> UNIQUE constraint failure.
    let res = conn
        .execute("INSERT OR FAIL INTO t VALUES(3,1,30,30) ON CONFLICT(u) DO UPDATE SET b=99,c=20");
    assert!(
        res.is_err(),
        "UPSERT DO UPDATE should fail with UNIQUE constraint violation, got {res:?}"
    );

    // The failed statement must not have removed row 1's secondary index entries.
    let ic = run_integrity_check(&conn);
    assert_eq!(ic, "ok", "integrity_check inside transaction: {ic}");

    let rows = limbo_exec_rows(&conn, "SELECT id,u,b,c FROM t ORDER BY id");
    assert_eq!(
        rows,
        vec![
            vec![
                Value::Integer(1),
                Value::Integer(1),
                Value::Integer(10),
                Value::Integer(10)
            ],
            vec![
                Value::Integer(2),
                Value::Integer(2),
                Value::Integer(20),
                Value::Integer(20)
            ],
        ],
        "table contents must be unchanged after failed UPSERT"
    );

    // Row 1 must still be reachable through every index.
    let via_idx_b = limbo_exec_rows(&conn, "SELECT id FROM t INDEXED BY idx_b WHERE b=10");
    assert_eq!(via_idx_b, vec![vec![Value::Integer(1)]]);
    let via_u = limbo_exec_rows(&conn, "SELECT id FROM t WHERE u=1");
    assert_eq!(via_u, vec![vec![Value::Integer(1)]]);
    let via_c = limbo_exec_rows(&conn, "SELECT id FROM t WHERE c=10");
    assert_eq!(via_c, vec![vec![Value::Integer(1)]]);

    conn.execute("COMMIT")?;

    let ic = run_integrity_check(&conn);
    assert_eq!(ic, "ok", "integrity_check after commit: {ic}");

    Ok(())
}
