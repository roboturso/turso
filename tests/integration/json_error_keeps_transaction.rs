use crate::common::{limbo_exec_rows, TempDatabase};
use rusqlite::types::Value;

#[turso_macros::test]
fn json_parse_error_in_select_keeps_transaction_open(tmp_db: TempDatabase) -> anyhow::Result<()> {
    let conn = tmp_db.connect_limbo();
    conn.execute("CREATE TABLE t(id INT)")?;

    conn.execute("BEGIN")?;
    conn.execute("INSERT INTO t VALUES(9)")?;

    let mut stmt = conn.prepare("SELECT json_extract('not json','$.a')")?;
    let json_error = stmt.run_with_row_callback(|_| Ok(()));
    assert!(
        json_error.is_err(),
        "malformed JSON should make the SELECT fail"
    );
    drop(stmt);

    assert_eq!(
        limbo_exec_rows(&conn, "SELECT count(*) FROM t"),
        vec![vec![Value::Integer(1)]],
        "the INSERT must still be visible inside the transaction after the SELECT failed"
    );

    conn.execute("COMMIT")?;

    assert_eq!(
        limbo_exec_rows(&conn, "SELECT count(*) FROM t"),
        vec![vec![Value::Integer(1)]],
        "the committed row must be visible after COMMIT"
    );
    Ok(())
}
