//! Regression test for https://github.com/tursodatabase/turso/issues/7994
//!
//! "no such column" errors must echo the identifier exactly as written in the
//! query text, not the casing stored in the schema. SQLite reports
//! `no such column: main.id` for the query below, even though the column is
//! declared as `Id` in the schema.

use crate::common::TempDatabase;

#[test]
fn no_such_column_error_uses_query_casing() -> anyhow::Result<()> {
    let tmp_db = TempDatabase::new_empty();
    let conn = tmp_db.connect_limbo();

    conn.execute("CREATE TABLE MAIN (Id INTEGER, Id1 INTEGER)")?;
    conn.execute("CREATE TABLE B (Id INTEGER, Id1 INTEGER)")?;
    conn.execute("CREATE VIEW v2 AS SELECT * FROM MAIN")?;

    let err = conn
        .execute(
            "INSERT INTO B SELECT * FROM main WHERE id > 10 \
             AND (SELECT count(*) FROM v2 GROUP BY main.id)",
        )
        .expect_err("query references a nonexistent column and must fail");

    let msg = err.to_string();
    assert!(
        msg.contains("no such column: main.id"),
        "error must echo the identifier as written in the query (`main.id`), got: {msg}"
    );

    Ok(())
}
