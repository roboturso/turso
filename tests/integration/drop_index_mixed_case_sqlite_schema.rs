use std::path::PathBuf;
use std::sync::Arc;

use rusqlite::types::Value;
use tempfile::TempDir;

use crate::common::{limbo_exec_rows, sqlite_exec_rows, TempDatabase};

const INDEX_NAME: &str = "CustomerLookupMixedCase";

fn create_sqlite_database_with_mixed_case_index() -> (TempDir, PathBuf) {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("mixed_case_index.db");
    let sqlite = rusqlite::Connection::open(&path).unwrap();
    sqlite.pragma_update(None, "journal_mode", "wal").unwrap();
    sqlite
        .execute_batch(&format!(
            "CREATE TABLE entries (
                id TEXT PRIMARY KEY,
                legacy_lookup_value TEXT,
                replacement_value TEXT
            );
            CREATE INDEX {INDEX_NAME} ON entries(legacy_lookup_value, id);"
        ))
        .unwrap();
    (dir, path)
}

fn persisted_index_rows(path: &std::path::Path) -> Vec<Vec<Value>> {
    let sqlite = rusqlite::Connection::open(path).unwrap();
    sqlite_exec_rows(
        &sqlite,
        &format!("SELECT name FROM sqlite_schema WHERE type = 'index' AND name = '{INDEX_NAME}'"),
    )
}

#[test]
fn drop_index_removes_sqlite_created_mixed_case_row_from_sqlite_schema() {
    let (_dir, path) = create_sqlite_database_with_mixed_case_index();
    assert_eq!(
        persisted_index_rows(&path),
        vec![vec![Value::Text(INDEX_NAME.to_string())]]
    );

    let db = TempDatabase::new_with_existent(&path);
    let conn = db.connect_limbo();
    conn.execute(format!("DROP INDEX \"{INDEX_NAME}\""))
        .unwrap();

    assert!(
        persisted_index_rows(&path).is_empty(),
        "DROP INDEX left the SQLite-created index row in sqlite_schema"
    );

    let fresh_conn: Arc<turso_core::Connection> = db.limbo_database().connect().unwrap();
    let rows = limbo_exec_rows(
        &fresh_conn,
        &format!("SELECT name FROM sqlite_schema WHERE type = 'index' AND name = '{INDEX_NAME}'"),
    );
    assert!(rows.is_empty(), "fresh connection still sees dropped index");
}

#[test]
fn drop_index_then_drop_column_on_sqlite_created_database_keeps_schema_loadable() {
    let (_dir, path) = create_sqlite_database_with_mixed_case_index();

    let db = TempDatabase::new_with_existent(&path);
    let conn = db.connect_limbo();
    conn.execute("BEGIN TRANSACTION").unwrap();
    conn.execute(format!("DROP INDEX \"{INDEX_NAME}\""))
        .unwrap();
    conn.execute("ALTER TABLE entries DROP COLUMN legacy_lookup_value")
        .unwrap();
    conn.execute("COMMIT").unwrap();

    let fresh_db = turso_core::Database::open_file(
        db.io.clone(),
        path.to_str().unwrap(),
        Arc::new(turso_core::SqliteDialect),
    )
    .expect("reopening the database after DROP INDEX + DROP COLUMN must succeed");
    let fresh_conn = fresh_db.connect().unwrap();
    let rows = limbo_exec_rows(
        &fresh_conn,
        "SELECT name FROM sqlite_schema WHERE type = 'index' AND sql IS NOT NULL",
    );
    assert!(rows.is_empty(), "stale index row survived: {rows:?}");

    let sqlite = rusqlite::Connection::open(&path).unwrap();
    let result = sqlite_exec_rows(&sqlite, "PRAGMA integrity_check");
    assert_eq!(result, vec![vec![Value::Text("ok".to_string())]]);
}
