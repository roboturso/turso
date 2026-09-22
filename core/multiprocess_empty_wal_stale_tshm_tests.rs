use super::*;
use std::process::Command;

#[cfg(all(target_os = "windows", feature = "experimental_win_iocp"))]
use crate::WindowsIOCP;

const WRITE_WAL_AND_EXIT_CHILD_TEST: &str =
    "multiprocess_empty_wal_stale_tshm_tests::multiprocess_write_large_wal_and_exit_child_process";

#[test]
fn multiprocess_open_recovers_when_wal_is_empty_but_tshm_claims_frames() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("empty-wal-stale-tshm.db");
    let db_path_str = db_path.to_str().unwrap();
    let wal_path = format!("{db_path_str}-wal");
    let tshm_path = format!("{db_path_str}-tshm");

    let current_exe = std::env::current_exe().unwrap();
    let child_output = Command::new(&current_exe)
        .arg(WRITE_WAL_AND_EXIT_CHILD_TEST)
        .arg("--exact")
        .arg("--nocapture")
        .env("TURSO_MULTIPROCESS_DB_PATH", db_path_str)
        .output()
        .unwrap();
    assert!(
        child_output.status.success(),
        "child writer failed: stdout={}; stderr={}",
        String::from_utf8_lossy(&child_output.stdout),
        String::from_utf8_lossy(&child_output.stderr)
    );

    let wal_len = std::fs::metadata(&wal_path).unwrap().len();
    assert!(
        wal_len > crate::storage::sqlite3_ondisk::WAL_HEADER_SIZE as u64,
        "child should leave committed frames in the WAL, got {wal_len} bytes"
    );
    let tshm_bytes = std::fs::read(&tshm_path).unwrap();
    assert!(
        tshm_bytes.starts_with(b"TSHMWAL"),
        "child should leave a -tshm file with the TSHMWAL magic"
    );

    std::fs::OpenOptions::new()
        .write(true)
        .open(&wal_path)
        .unwrap()
        .set_len(0)
        .unwrap();
    assert_eq!(std::fs::metadata(&wal_path).unwrap().len(), 0);

    let io: Arc<dyn IO> = multiprocess_test_io();
    let db = open_multiprocess_db(io, db_path_str)
        .expect("open must succeed when the WAL is empty even if -tshm still claims frames");
    let conn = db.connect().unwrap();

    let mut rows = Vec::new();
    conn.prepare("pragma integrity_check")
        .unwrap()
        .run_with_row_callback(|row| {
            rows.push(row.get::<String>(0).unwrap());
            Ok(())
        })
        .unwrap();
    assert_eq!(rows, vec!["ok".to_string()]);

    let mut schema_rows = 0i64;
    conn.prepare("select count(*) from sqlite_schema")
        .unwrap()
        .run_with_row_callback(|row| {
            schema_rows = row.get(0).unwrap();
            Ok(())
        })
        .unwrap();
    assert!(schema_rows >= 0);
}

#[test]
fn multiprocess_write_large_wal_and_exit_child_process() {
    let Some(db_path) = std::env::var_os("TURSO_MULTIPROCESS_DB_PATH") else {
        return;
    };

    let io: Arc<dyn IO> = multiprocess_test_io();
    let db = open_multiprocess_db(io, db_path.to_str().unwrap()).unwrap();
    let conn = db.connect().unwrap();
    conn.execute("create table t (id integer primary key, blob blob)")
        .unwrap();
    let blob_hex = "07".repeat(4096);
    for _ in 0..900 {
        conn.execute(&format!("insert into t (blob) values (x'{blob_hex}')"))
            .unwrap();
    }

    std::process::exit(0);
}

fn multiprocess_test_io() -> Arc<dyn IO> {
    #[cfg(all(target_os = "windows", feature = "experimental_win_iocp"))]
    {
        Arc::new(WindowsIOCP::new().unwrap())
    }

    #[cfg(not(all(target_os = "windows", feature = "experimental_win_iocp")))]
    {
        Arc::new(PlatformIO::new().unwrap())
    }
}

fn open_multiprocess_db(io: Arc<dyn IO>, path: &str) -> Result<Arc<Database>> {
    Database::open_file_with_flags(
        io,
        path,
        OpenFlags::default(),
        DatabaseOpts::new().with_multiprocess_wal(true),
        None,
        Arc::new(SqliteDialect),
    )
}
