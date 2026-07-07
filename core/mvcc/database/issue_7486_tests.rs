//! Regression test for https://github.com/tursodatabase/turso/issues/7486
//!
//! An autocommit SELECT must not surface a row written by a concurrent
//! writer whose commit is still in the `Preparing` state if that writer
//! later aborts. The reader must either wait until the dependency
//! resolves, fail before returning any row, or return the stable
//! committed value.

use std::sync::Arc;

use rustc_hash::FxHashSet as HashSet;

use crate::mvcc::database::tests::MvccTestDbNoConn;
use crate::mvcc::database::CommitYieldPoint;
use crate::mvcc::yield_hooks::YieldPointMarker;
use crate::mvcc::yield_points::{YieldInjector, YieldPoint};
use crate::sync::Mutex;
use crate::{StepResult, Value};

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
}

impl YieldInjector for FixedYieldInjector {
    fn should_yield(&self, _instance_id: u64, _selection_key: u64, point: YieldPoint) -> bool {
        self.remaining.lock().remove(&point)
    }
}

#[test]
fn test_select_does_not_stream_row_from_aborted_preparing_writer() {
    let db = MvccTestDbNoConn::new_with_random_db();
    let setup = db.connect();

    setup
        .execute("CREATE TABLE t(id INTEGER PRIMARY KEY, value TEXT)")
        .unwrap();
    setup
        .execute("INSERT INTO t VALUES (1, 'initial')")
        .unwrap();

    // Writer updates the row and pauses mid-commit in the `Preparing`
    // state (after the log record is prepared but before commit finishes).
    let writer = db.connect();
    writer.execute("BEGIN CONCURRENT").unwrap();
    writer
        .execute("UPDATE t SET value = 'modified' WHERE id = 1")
        .unwrap();
    writer.set_yield_injector(Some(FixedYieldInjector::new([
        CommitYieldPoint::LogRecordPrepared.point(),
    ])));

    let mut commit_stmt = writer.prepare("COMMIT").unwrap();
    assert!(
        matches!(commit_stmt.step().unwrap(), StepResult::Yield),
        "writer commit should pause in the Preparing state"
    );

    // Reader runs an autocommit SELECT while the writer is still Preparing.
    let reader = db.connect();
    let mut read_stmt = reader.prepare("SELECT value FROM t WHERE id = 1").unwrap();

    let mut streamed: Vec<Value> = Vec::new();
    let mut done = false;

    // Phase 1: step the reader while the writer is still Preparing. It is
    // fine for the reader to wait (Yield/Busy) for the in-flight commit to
    // resolve, but any row surfaced here comes from an uncommitted writer.
    for _ in 0..64 {
        match read_stmt.step() {
            Ok(StepResult::Row) => {
                streamed.push(read_stmt.row().unwrap().get_value(0).clone());
            }
            Ok(StepResult::IO) => db.get_db().io.step().unwrap(),
            Ok(StepResult::Yield) | Ok(StepResult::Busy) => break,
            Ok(StepResult::Done) => {
                done = true;
                break;
            }
            Ok(StepResult::Interrupt) => panic!("unexpected interrupt"),
            // Failing before surfacing an uncommitted row is acceptable.
            Err(_) => {
                done = true;
                break;
            }
        }
    }

    // Abort the writer while it is still Preparing: dropping the yielded
    // COMMIT statement rolls the transaction back.
    drop(commit_stmt);

    // Phase 2: let the reader finish now that the dependency has aborted.
    if !done {
        for _ in 0..1024 {
            match read_stmt.step() {
                Ok(StepResult::Row) => {
                    streamed.push(read_stmt.row().unwrap().get_value(0).clone());
                }
                Ok(StepResult::IO) => db.get_db().io.step().unwrap(),
                Ok(StepResult::Yield) | Ok(StepResult::Busy) => {}
                Ok(StepResult::Done) => {
                    done = true;
                    break;
                }
                Ok(StepResult::Interrupt) => panic!("unexpected interrupt"),
                // Failing without having surfaced an uncommitted row is
                // acceptable; the assertions below catch the bad case.
                Err(_) => {
                    done = true;
                    break;
                }
            }
        }
    }
    assert!(done, "reader statement should eventually finish or fail");
    drop(read_stmt);

    // The writer aborted, so the only value the reader may ever have
    // surfaced to the user is the stable committed value 'initial'.
    for value in &streamed {
        assert_eq!(
            value.to_text(),
            Some("initial"),
            "reader surfaced a row from a writer that later aborted: {streamed:?}"
        );
    }

    // Sanity check: the aborted writer's change must not be visible.
    let observer = db.connect();
    let mut stmt = observer
        .prepare("SELECT value FROM t WHERE id = 1")
        .unwrap();
    let mut final_rows = Vec::new();
    stmt.run_with_row_callback(|row| {
        final_rows.push(row.get_value(0).clone());
        Ok(())
    })
    .unwrap();
    assert_eq!(final_rows.len(), 1);
    assert_eq!(final_rows[0].to_text(), Some("initial"));
}
