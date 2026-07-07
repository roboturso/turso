//! Regression test for https://github.com/tursodatabase/turso/issues/7643
//!
//! An abandoned auto-checkpoint after CREATE TABLE in MVCC mode must not
//! leak a stale root mapping that poisons subsequent public reads with
//! short reads.
#![cfg(feature = "io_memory_yield")]

use std::sync::Arc;
use turso_core::{Database, DatabaseOpts, MemoryYieldIO, OpenFlags, StepResult, IO};

fn open_conn(path: &str, io: Arc<MemoryYieldIO>) -> Arc<turso_core::Connection> {
    let db = Database::open_file_with_flags(
        io,
        path,
        OpenFlags::default(),
        DatabaseOpts::new(),
        None,
    )
    .unwrap();
    db.connect().unwrap()
}

fn run_sql(conn: &Arc<turso_core::Connection>, io: &Arc<MemoryYieldIO>, sql: &str) {
    let mut stmt = conn.prepare(sql).unwrap();
    loop {
        match stmt.step().unwrap() {
            StepResult::IO => io.step().unwrap(),
            StepResult::Done => return,
            StepResult::Row | StepResult::Yield => {}
            other => panic!("unexpected step result for {sql}: {other:?}"),
        }
    }
}

fn try_run_sql(
    conn: &Arc<turso_core::Connection>,
    io: &Arc<MemoryYieldIO>,
    sql: &str,
) -> turso_core::Result<()> {
    let mut stmt = conn.prepare(sql)?;
    loop {
        match stmt.step()? {
            StepResult::IO => io.step().unwrap(),
            StepResult::Done => return Ok(()),
            StepResult::Row | StepResult::Yield => {}
            other => panic!("unexpected step result for {sql}: {other:?}"),
        }
    }
}

#[test]
fn abandoned_auto_checkpoint_create_table_leaks_root_mapping() {
    for abandon_after_io in 1..=16 {
        let io = Arc::new(MemoryYieldIO::new());
        let conn = open_conn(
            &format!("mvcc-create-abandon-{abandon_after_io}.db"),
            io.clone(),
        );

        run_sql(&conn, &io, "PRAGMA journal_mode='mvcc'");
        run_sql(&conn, &io, "PRAGMA mvcc_checkpoint_threshold=0");

        let mut stmt = conn
            .prepare("CREATE TABLE t(id INTEGER PRIMARY KEY, v TEXT)")
            .unwrap();

        let mut io_count = 0;
        loop {
            match stmt.step().unwrap() {
                StepResult::IO => {
                    io_count += 1;
                    io.step().unwrap();
                    if io_count == abandon_after_io {
                        drop(stmt);
                        break;
                    }
                }
                StepResult::Done => break,
                StepResult::Row | StepResult::Yield => {}
                other => panic!("unexpected CREATE TABLE step result: {other:?}"),
            }
        }

        if io_count < abandon_after_io {
            continue;
        }

        match try_run_sql(&conn, &io, "SELECT * FROM t") {
            Ok(()) => {}
            Err(err) if err.to_string().contains("no such table") => {}
            Err(err) => panic!(
                "abandon_after_io={abandon_after_io}: public read failed after abandoned auto-checkpoint: {err:?}"
            ),
        }
    }
}
