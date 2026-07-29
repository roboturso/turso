use std::sync::Arc;

use turso_core::{Database, MemoryIO, SqliteDialect, IO};

/// Stack size matching the default main-thread stack on Linux. The code
/// generator recurses once per compound SELECT term, so without a limit on
/// the number of terms (SQLite caps this at SQLITE_MAX_COMPOUND_SELECT = 500)
/// a query with many terms overflows the stack and aborts the process.
///
/// Regression test for https://github.com/tursodatabase/turso/issues/8104
const WORKER_STACK: usize = 8 << 20;

#[test]
fn compound_select_with_many_terms_does_not_overflow_stack() {
    let terms = 50_000;
    let sql = vec!["SELECT 1"; terms].join(" UNION ALL ");

    // Either a graceful "too many terms" error or a successful preparation is
    // acceptable; the process must not abort with a stack overflow.
    let result = std::thread::Builder::new()
        .stack_size(WORKER_STACK)
        .spawn(move || -> turso_core::Result<()> {
            let io: Arc<dyn IO> = Arc::new(MemoryIO::new());
            let db = Database::open_file(io, ":memory:", Arc::new(SqliteDialect))?;
            let conn = db.connect()?;
            conn.prepare(&sql)?;
            Ok(())
        })
        .expect("failed to spawn worker thread")
        .join()
        .expect("worker thread panicked while preparing compound select");

    if let Err(err) = result {
        // A graceful limit error is fine; just make sure it surfaced as a
        // regular error instead of crashing the process.
        eprintln!("compound select rejected gracefully: {err}");
    }
}
