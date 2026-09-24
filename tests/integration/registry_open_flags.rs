use std::sync::Arc;
use turso_core::{Database, DatabaseOpts, OpenFlags, SqliteDialect};

fn open(io: &Arc<dyn turso_core::IO>, path: &str, flags: OpenFlags) -> Arc<Database> {
    Database::open_file_with_flags(
        io.clone(),
        path,
        flags,
        DatabaseOpts::new(),
        None,
        Arc::new(SqliteDialect),
    )
    .unwrap()
}

fn create_db_with_table(io: &Arc<dyn turso_core::IO>, path: &str) {
    let db = open(io, path, OpenFlags::Create);
    let conn = db.connect().unwrap();
    conn.execute("CREATE TABLE t(x)").unwrap();
    conn.execute("INSERT INTO t VALUES (1)").unwrap();
}

#[test]
fn read_write_open_after_readonly_open_can_write() {
    let tmp_dir = tempfile::TempDir::new().unwrap();
    let path = tmp_dir.path().join("ro_then_rw.db");
    let path = path.to_str().unwrap();
    let io: Arc<dyn turso_core::IO> = Arc::new(turso_core::PlatformIO::new().unwrap());
    create_db_with_table(&io, path);

    let db_ro = open(&io, path, OpenFlags::ReadOnly);
    let _conn_ro = db_ro.connect().unwrap();
    assert!(db_ro.is_readonly());

    let db_rw = open(&io, path, OpenFlags::default());
    let conn_rw = db_rw.connect().unwrap();
    assert!(
        !db_rw.is_readonly(),
        "read-write open returned the cached readonly Database"
    );
    conn_rw.execute("INSERT INTO t VALUES (2)").unwrap();
}

#[test]
fn readonly_open_after_read_write_open_cannot_write() {
    let tmp_dir = tempfile::TempDir::new().unwrap();
    let path = tmp_dir.path().join("rw_then_ro.db");
    let path = path.to_str().unwrap();
    let io: Arc<dyn turso_core::IO> = Arc::new(turso_core::PlatformIO::new().unwrap());
    create_db_with_table(&io, path);

    let db_rw = open(&io, path, OpenFlags::default());
    let _conn_rw = db_rw.connect().unwrap();

    let db_ro = open(&io, path, OpenFlags::ReadOnly);
    let conn_ro = db_ro.connect().unwrap();
    assert!(
        conn_ro.execute("INSERT INTO t VALUES (2)").is_err(),
        "readonly open returned the cached read-write Database and allowed a write"
    );
}
