//! Regression test for https://github.com/tursodatabase/turso/issues/7728
//!
//! An active SELECT that has parked at a root-page read must not resume
//! against a root page that was recycled by DROP TABLE and reused by a
//! subsequent CREATE TABLE / INSERT. Doing so panics with a TableInterior
//! page-type assertion (or worse, could return stale data).

use crate::queued_io::QueuedIo;
use std::sync::Arc;
use turso_core::{Connection, Database, DatabaseOpts, OpenFlags, StepResult};

fn open_queued_conn(io: Arc<QueuedIo>, path: &str) -> anyhow::Result<Arc<Connection>> {
    let db =
        Database::open_file_with_flags(io, path, OpenFlags::default(), DatabaseOpts::new(), None)?;
    Ok(db.connect()?)
}

fn collect_rows(stmt: &mut turso_core::Statement) -> anyhow::Result<Vec<(i64, i64)>> {
    let mut rows = Vec::new();

    loop {
        match stmt.step()? {
            StepResult::IO => stmt._io().step()?,
            StepResult::Yield => {}
            StepResult::Row => {
                let row = stmt.row().expect("row should be available after Row");
                rows.push((row.get::<i64>(0)?, row.get::<i64>(1)?));
            }
            StepResult::Done => return Ok(rows),
            StepResult::Interrupt | StepResult::Busy => {
                anyhow::bail!("unexpected non-progress result while draining statement")
            }
        }
    }
}

#[test]
fn active_table_seek_after_drop_reuse_must_not_use_recycled_root_page() -> anyhow::Result<()> {
    let io = Arc::new(QueuedIo::new());
    let dir = tempfile::TempDir::new().unwrap();
    let path = dir.path().join("table-interior-drop-reuse.db");
    let path = path.to_str().unwrap();
    let conn = open_queued_conn(io, path)?;

    conn.execute("PRAGMA page_size=512")?;
    conn.execute("PRAGMA cache_size=9")?;
    conn.execute("PRAGMA cache_spill=ON")?;
    conn.execute("PRAGMA journal_mode='wal'")?;
    conn.execute("CREATE TABLE u(id INTEGER PRIMARY KEY, b BLOB)")?;

    for id in 1..=32 {
        conn.execute(format!("INSERT INTO u VALUES({id}, zeroblob(60))"))?;
    }

    conn.execute("PRAGMA wal_checkpoint(TRUNCATE)")?;

    let mut select = conn.prepare("SELECT id, length(b) FROM u WHERE id = 16")?;
    match select.step()? {
        StepResult::IO => select._io().step()?,
        other => anyhow::bail!("SELECT did not yield at the root-page read: {other:?}"),
    }

    conn.execute("DROP TABLE u")?;
    conn.execute("CREATE TABLE reuse(id INTEGER PRIMARY KEY, b BLOB)")?;
    conn.execute("INSERT INTO reuse VALUES(16, zeroblob(5000))")?;

    // Resuming the statement must not panic and must not observe data from
    // the recycled root page. Either an empty result or a clean error (e.g.
    // a schema-changed style error) is acceptable.
    match collect_rows(&mut select) {
        Ok(rows) => assert_eq!(rows, Vec::<(i64, i64)>::new()),
        Err(err) => {
            eprintln!("statement resumption failed cleanly: {err}");
        }
    }

    Ok(())
}
