//! Regression test for issue #8341: `StepResult::Yield` must not be reported
//! to the JS drivers as `STEP_IO`.
//!
//! A yield carries no pending I/O completion by design. The wasm driver
//! handles `STEP_IO` by awaiting an I/O notification, so folding `Yield` into
//! `STEP_IO` makes browser statements hang forever whenever the engine yields
//! while no I/O is in flight (nothing ever notifies the waiter). The driver
//! must instead be told to simply step again, e.g. via `STEP_SLEEP` or a
//! dedicated yield step code.
//!
//! Run with: cargo test -p turso_node --features test_helper

use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use turso_core::mvcc::yield_points::{YieldInjector, YieldPoint};

use crate::{step_sync, StatementHandle, STEP_DONE, STEP_IO};

/// Yields exactly once, at the first yield point the engine consults.
#[derive(Debug, Default)]
struct YieldOnce {
    fired: AtomicBool,
}

impl YieldOnce {
    fn fired(&self) -> bool {
        self.fired.load(Ordering::SeqCst)
    }
}

impl YieldInjector for YieldOnce {
    fn should_yield(&self, _instance_id: u64, _selection_key: u64, _point: YieldPoint) -> bool {
        !self.fired.swap(true, Ordering::SeqCst)
    }
}

#[test]
fn yield_with_no_pending_io_is_not_reported_as_step_io() {
    #[allow(clippy::arc_with_non_send_sync)]
    let io: Arc<dyn turso_core::IO> = Arc::new(turso_core::MemoryIO::new());
    let db = turso_core::Database::open_file(
        io.clone(),
        "issue-8341-yield-step.db",
        Arc::new(turso_core::SqliteDialect),
    )
    .unwrap();
    let conn = db.connect().unwrap();

    conn.execute("CREATE TABLE t (id INTEGER PRIMARY KEY, payload BLOB)")
        .unwrap();
    // Fill leaf pages so the next big insert overflows a page and reaches the
    // btree write yield point (overflow cell inserted, balance pending).
    for id in 1..20 {
        conn.execute(format!("INSERT INTO t VALUES ({id}, zeroblob(3600))"))
            .unwrap();
    }

    let injector = Arc::new(YieldOnce::default());
    conn.set_yield_injector(Some(injector.clone()));

    let stmt = conn
        .prepare("INSERT INTO t VALUES (1000, zeroblob(7200))")
        .unwrap();
    #[allow(clippy::arc_with_non_send_sync)]
    let handle: StatementHandle = Arc::new(RefCell::new(Some(stmt)));

    let mut saw_yield = false;
    for _ in 0..10_000 {
        let fired_before = injector.fired();
        let (code, _sleep_ms) = step_sync(&handle).unwrap();
        if !fired_before && injector.fired() {
            // This step_sync call observed core's StepResult::Yield. There is
            // no pending I/O completion, so reporting STEP_IO here makes the
            // wasm driver wait forever on an I/O notification that will never
            // arrive (issue #8341).
            assert_ne!(
                code, STEP_IO,
                "StepResult::Yield was mapped to STEP_IO: the wasm step loop \
                 would await an I/O completion that was never scheduled and \
                 hang forever"
            );
            saw_yield = true;
            break;
        }
        match code {
            STEP_DONE => break,
            STEP_IO => io.step().unwrap(),
            _ => {}
        }
    }
    assert!(
        saw_yield,
        "test setup failed: the injected yield never fired, so the \
         Yield-to-step-code mapping was not exercised"
    );

    // Drive the statement to completion so the insert resumes cleanly after
    // the yield.
    for _ in 0..10_000 {
        let (code, _sleep_ms) = step_sync(&handle).unwrap();
        match code {
            STEP_DONE => break,
            STEP_IO => io.step().unwrap(),
            _ => {}
        }
    }
    conn.set_yield_injector(None);
}
