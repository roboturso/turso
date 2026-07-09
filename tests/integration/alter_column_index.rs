//! Regression test for issue #6876.
//!
//! `ALTER TABLE ... ALTER COLUMN` physically rewrites the table rows with a
//! new column layout/affinity but did not rebuild secondary indexes, leaving
//! the index keyed by the old row image. This left the database corrupt:
//! `PRAGMA integrity_check` reports rows missing from the index.

use crate::common::{limbo_exec_rows, TempDatabase};
use rusqlite::types::Value;

/// After ALTER COLUMN on an indexed column, the secondary index must stay
/// consistent with the table, so integrity_check should report "ok".
#[test]
fn alter_column_rebuilds_secondary_index() {
    let tmp_db = TempDatabase::new_empty();
    let conn = tmp_db.connect_limbo();

    limbo_exec_rows(&conn, "CREATE TABLE t(x NUMERIC)");
    limbo_exec_rows(&conn, "CREATE INDEX idx_x ON t(x)");
    limbo_exec_rows(&conn, "INSERT INTO t VALUES(10),(2),(30)");

    limbo_exec_rows(&conn, "ALTER TABLE t ALTER COLUMN x TO y TEXT");

    let integrity = limbo_exec_rows(&conn, "PRAGMA integrity_check");
    assert_eq!(
        integrity,
        vec![vec![Value::Text("ok".to_string())]],
        "integrity_check must pass after ALTER COLUMN on an indexed column, \
         got: {integrity:?}"
    );
}
