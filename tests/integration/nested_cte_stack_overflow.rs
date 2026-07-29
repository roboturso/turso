use std::sync::Arc;

use turso_core::{Database, MemoryIO, SqliteDialect, IO};

// Regression test for https://github.com/tursodatabase/turso/issues/8105.
//
// Deeply nested CTEs (`WITH n1 AS (WITH n0 AS (...) ...) ...`) used to smash
// the stack and abort the whole process instead of either executing the query
// or returning a graceful "query too deep" error. SQLite handles this shape
// without crashing, so must we: any `Result` is acceptable, a crash is not.

/// Deep enough to overflow the worker stack below in both debug and release
/// builds while the recursion is unbounded.
const DEPTH: usize = 4000;

/// Deterministic stack size so the test does not depend on the platform's
/// default thread stack size.
const WORKER_STACK: usize = 8 << 20;

fn nested_cte_sql(depth: usize) -> String {
    let mut sql = "SELECT 1".to_owned();
    for i in 0..depth {
        sql = format!("WITH n{i} AS ({sql}) SELECT * FROM n{i}");
    }
    sql
}

#[test]
fn deeply_nested_ctes_do_not_smash_the_stack() {
    let sql = nested_cte_sql(DEPTH);
    let result = std::thread::Builder::new()
        .stack_size(WORKER_STACK)
        .spawn(move || -> turso_core::Result<()> {
            let io: Arc<dyn IO> = Arc::new(MemoryIO::new());
            let db = Database::open_file(io, ":memory:", Arc::new(SqliteDialect))?;
            let conn = db.connect()?;
            conn.execute(&sql)?;
            Ok(())
        })
        .expect("failed to spawn worker thread")
        .join()
        .expect("worker thread panicked while executing nested CTE query");
    // Both a successful execution and a graceful depth-limit error are fine;
    // reaching this point at all means the process did not crash.
    drop(result);
}
