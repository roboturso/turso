//! Regression test for https://github.com/tursodatabase/turso/issues/7995
//!
//! With `synchronous=FULL`, every committed transaction is fsync-acknowledged
//! to the `-wal` file before the commit returns. If the process crashes (power
//! loss) before the *first* checkpoint, the committed data exists only in the
//! `-wal`; the main database file has never been fsync'd. On reopen, recovery
//! must replay the durable `-wal` — otherwise the whole first WAL epoch of
//! committed transactions is silently lost.
//!
//! This test models power loss faithfully: writes only become durable when the
//! file is fsync'd. After the "crash", only the fsync-acknowledged bytes of
//! each file are materialized on disk, and the database is reopened with the
//! regular platform IO. All committed rows must be recovered.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use turso_core::{
    io::{FileId, FileSyncType},
    Buffer, Clock, Completion, Database, File, MonotonicInstant, OpenFlags, PlatformIO,
    SqliteDialect, WallClockInstant, IO,
};

use crate::common::limbo_exec_rows;

/// Per-file state: `volatile` is what the OS page cache would hold, `durable`
/// is what has been fsync-acknowledged and thus survives power loss.
#[derive(Default)]
struct FileState {
    volatile: Vec<u8>,
    durable: Vec<u8>,
}

struct PowerLossFile {
    state: Mutex<FileState>,
}

impl File for PowerLossFile {
    fn lock_file(&self, _exclusive: bool) -> turso_core::Result<()> {
        Ok(())
    }

    fn unlock_file(&self) -> turso_core::Result<()> {
        Ok(())
    }

    fn pread(&self, pos: u64, c: Completion) -> turso_core::Result<Completion> {
        let state = self.state.lock().unwrap();
        let buf = c.as_read().buf();
        let dst = buf.as_mut_slice();
        let pos = pos as usize;
        let n = if pos >= state.volatile.len() {
            0
        } else {
            let n = dst.len().min(state.volatile.len() - pos);
            dst[..n].copy_from_slice(&state.volatile[pos..pos + n]);
            n
        };
        c.complete(n as i32);
        Ok(c)
    }

    fn pwrite(
        &self,
        pos: u64,
        buffer: Arc<Buffer>,
        c: Completion,
    ) -> turso_core::Result<Completion> {
        let mut state = self.state.lock().unwrap();
        let data = buffer.as_slice();
        let pos = pos as usize;
        let end = pos + data.len();
        if state.volatile.len() < end {
            state.volatile.resize(end, 0);
        }
        state.volatile[pos..end].copy_from_slice(data);
        c.complete(data.len() as i32);
        Ok(c)
    }

    fn sync(&self, c: Completion, _sync_type: FileSyncType) -> turso_core::Result<Completion> {
        let mut state = self.state.lock().unwrap();
        state.durable = state.volatile.clone();
        c.complete(0);
        Ok(c)
    }

    fn size(&self) -> turso_core::Result<u64> {
        Ok(self.state.lock().unwrap().volatile.len() as u64)
    }

    fn truncate(&self, len: u64, c: Completion) -> turso_core::Result<Completion> {
        let mut state = self.state.lock().unwrap();
        state.volatile.resize(len as usize, 0);
        c.complete(0);
        Ok(c)
    }
}

struct PowerLossIo {
    files: Mutex<HashMap<String, Arc<PowerLossFile>>>,
    clock: Arc<turso_core::MemoryIO>,
}

impl PowerLossIo {
    fn new() -> Self {
        Self {
            files: Mutex::new(HashMap::new()),
            clock: Arc::new(turso_core::MemoryIO::new()),
        }
    }

    /// Simulate power loss: return, for each file, only the bytes that were
    /// fsync-acknowledged before the crash.
    fn durable_files(&self) -> Vec<(String, Vec<u8>)> {
        self.files
            .lock()
            .unwrap()
            .iter()
            .map(|(path, file)| (path.clone(), file.state.lock().unwrap().durable.clone()))
            .collect()
    }
}

impl Clock for PowerLossIo {
    fn current_time_monotonic(&self) -> MonotonicInstant {
        self.clock.current_time_monotonic()
    }

    fn current_time_wall_clock(&self) -> WallClockInstant {
        self.clock.current_time_wall_clock()
    }
}

impl IO for PowerLossIo {
    fn open_file(
        &self,
        path: &str,
        _flags: OpenFlags,
        _direct: bool,
    ) -> turso_core::Result<Arc<dyn File>> {
        let mut files = self.files.lock().unwrap();
        let file = files
            .entry(path.to_string())
            .or_insert_with(|| {
                Arc::new(PowerLossFile {
                    state: Mutex::new(FileState::default()),
                })
            })
            .clone();
        Ok(file)
    }

    fn remove_file(&self, path: &str) -> turso_core::Result<()> {
        self.files.lock().unwrap().remove(path);
        Ok(())
    }

    fn file_id(&self, path: &str) -> turso_core::Result<FileId> {
        Ok(FileId::from_path_hash(path))
    }
}

/// Committed (fsync-acknowledged) transactions must survive power loss even
/// when the crash happens before the first checkpoint, i.e. while the data
/// only exists in the `-wal` file.
#[test]
fn test_committed_wal_survives_power_loss_before_first_checkpoint() {
    let _ = env_logger::try_init();

    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("crash.db");
    let db_path_str = db_path.to_str().unwrap().to_string();

    let io = Arc::new(PowerLossIo::new());

    // Phase 1: commit 10 transactions with synchronous=FULL, never checkpoint.
    {
        let db = Database::open_file(io.clone(), &db_path_str, Arc::new(SqliteDialect)).unwrap();
        let conn = db.connect().unwrap();
        limbo_exec_rows(&conn, "PRAGMA synchronous=FULL");
        limbo_exec_rows(&conn, "CREATE TABLE t(x)");
        for i in 0..10 {
            // Autocommit: each INSERT is its own committed transaction whose
            // WAL frames are fsync'd before the commit returns.
            limbo_exec_rows(&conn, &format!("INSERT INTO t VALUES ({i})"));
        }
        // Crash here: no clean shutdown, no checkpoint. The Database/Connection
        // are intentionally leaked so no close-time cleanup runs.
        std::mem::forget(conn);
        std::mem::forget(db);
    }

    // Phase 2: materialize only the fsync-acknowledged bytes of every file,
    // exactly what a power loss would leave on disk.
    let durable = io.durable_files();
    let wal_path = format!("{db_path_str}-wal");
    let durable_wal = durable
        .iter()
        .find(|(path, _)| *path == wal_path)
        .map(|(_, bytes)| bytes.len())
        .unwrap_or(0);
    assert!(
        durable_wal > 0,
        "synchronous=FULL commits must have fsync'd frames into the -wal"
    );
    for (path, bytes) in &durable {
        std::fs::write(path, bytes).unwrap();
    }

    // Phase 3: reopen the crashed image with the regular platform IO and
    // verify that recovery replays the durable WAL.
    let io: Arc<dyn IO> = Arc::new(PlatformIO::new().unwrap());
    let db = Database::open_file(io, &db_path_str, Arc::new(SqliteDialect)).unwrap();
    let conn = db.connect().unwrap();
    let rows = limbo_exec_rows(&conn, "SELECT count(*) FROM t");
    assert_eq!(
        rows,
        vec![vec![rusqlite::types::Value::Integer(10)]],
        "all 10 committed (fsync-acknowledged) transactions must be recovered \
         from the WAL after a crash before the first checkpoint"
    );
}
