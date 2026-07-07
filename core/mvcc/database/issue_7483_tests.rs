//! Regression test for https://github.com/tursodatabase/turso/issues/7483
//!
//! Dropping a `PRAGMA journal_mode = WAL` statement mid-checkpoint (while the
//! MVCC checkpoint state machine is suspended at a yield point) must release
//! the checkpoint lock so that other connections do not remain permanently
//! Busy.

use rustc_hash::FxHashSet as HashSet;

use crate::mvcc::database::checkpoint_state_machine::CheckpointYieldPoint;
use crate::mvcc::database::tests::MvccTestDbNoConn;
use crate::mvcc::yield_hooks::YieldPointMarker;
use crate::mvcc::yield_points::{YieldInjector, YieldPoint};
use crate::sync::Mutex;
use crate::vdbe::StepResult;
use std::sync::Arc;

#[derive(Debug)]
struct FixedYieldInjector {
    remaining: Mutex<HashSet<YieldPoint>>,
}

impl FixedYieldInjector {
    fn new(points: impl IntoIterator<Item = YieldPoint>) -> Arc<Self> {
        Arc::new(Self {
            remaining: Mutex::new(points.into_iter().collect()),
        })
    }

    fn is_empty(&self) -> bool {
        self.remaining.lock().is_empty()
    }
}

impl YieldInjector for FixedYieldInjector {
    fn should_yield(&self, _instance_id: u64, _selection_key: u64, point: YieldPoint) -> bool {
        self.remaining.lock().remove(&point)
    }
}

#[test]
fn dropped_journal_mode_mvcc_checkpoint_releases_lock() {
    let db = MvccTestDbNoConn::new_with_random_db();
    let conn = db.connect();

    conn.execute("CREATE TABLE t(id INTEGER PRIMARY KEY, v TEXT)")
        .unwrap();
    conn.execute("INSERT INTO t VALUES (1, 'a')").unwrap();

    let injector =
        FixedYieldInjector::new([CheckpointYieldPoint::AfterDurableBoundaryAdvanced.point()]);

    conn.set_yield_injector(Some(injector.clone()));

    let mut stmt = conn.prepare("PRAGMA journal_mode = WAL").unwrap();

    // Step the statement until the injected yield fires, then abandon it.
    let first_yield = loop {
        match stmt.step() {
            Ok(StepResult::IO | StepResult::Yield) => {
                if injector.is_empty() {
                    break true;
                }
            }
            Ok(StepResult::Done) => break false,
            Ok(StepResult::Row) => {}
            Ok(other) => panic!("unexpected journal_mode step result: {other:?}"),
            Err(err) => panic!("unexpected journal_mode error: {err:?}"),
        }
    };
    assert!(
        first_yield,
        "injected checkpoint yield point never fired; scenario not exercised"
    );

    // Abandon the in-flight PRAGMA mid-checkpoint.
    drop(stmt);
    conn.set_yield_injector(None);

    // Another connection must still be able to use the database.
    let observer = db.connect();

    observer
        .execute("INSERT INTO t VALUES (2, 'b')")
        .expect("insert after abandoned journal_mode checkpoint must not be Busy");

    let rows = {
        let mut stmt = observer.query("SELECT count(*) FROM t").unwrap().unwrap();
        stmt.run_collect_rows()
            .expect("select after abandoned journal_mode checkpoint must not be Busy")
    };
    assert_eq!(rows.len(), 1);

    observer
        .execute("PRAGMA wal_checkpoint(TRUNCATE)")
        .expect("checkpoint after abandoned journal_mode checkpoint must not be Busy");
}
