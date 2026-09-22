# Reproducer: #9001 — simulator: wrong number of rows in table

https://github.com/tursodatabase/turso/issues/9001

## Environment

- Commit: 6e320d2e788f2c98333c43697c886f0f7ce9e8ff
- Platform: linux x86_64
- Reproduced at: 2026-09-22T16:44:27Z

## Reproduce

```sh
cargo test -q -p limbo_sim savepoint_snapshot_issue_9001
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Reproduced with the reported seed; the database's 14 rows are correct (verified against SQLite), and the simulator's shadow model is wrong because SAVEPOINT snapshots the committed tables immediately instead of at the first read, so it misses rows other connections commit before the savepoint's transaction actually reads.

## Observed failure

RoboTurso ran the command above and observed:

```text
running 2 tests
runner::env::savepoint_snapshot_issue_9001::insert_select_after_savepoint_copies_rows_committed_by_another_connection --- FAILED
runner::env::savepoint_snapshot_issue_9001::savepoint_without_reads_sees_rows_committed_later_by_another_connection --- FAILED

failures:

---- runner::env::savepoint_snapshot_issue_9001::insert_select_after_savepoint_copies_rows_committed_by_another_connection stdout ----

thread 'runner::env::savepoint_snapshot_issue_9001::insert_select_after_savepoint_copies_rows_committed_by_another_connection' panicked at testing/simulator/runner/savepoint_snapshot_issue_9001.rs:92:5:
assertion `left == right` failed: INSERT ... SELECT must copy every row the database would see, including the one committed after SAVEPOINT
  left: [[SimValue(Numeric(Integer(1)))], [SimValue(Numeric(Integer(1)))], [SimValue(Numeric(Integer(2)))]]
 right: [[SimValue(Numeric(Integer(1)))], [SimValue(Numeric(Integer(1)))], [SimValue(Numeric(Integer(2)))], [SimValue(Numeric(Integer(2)))]]

---- runner::env::savepoint_snapshot_issue_9001::savepoint_without_reads_sees_rows_committed_later_by_another_connection stdout ----

thread 'runner::env::savepoint_snapshot_issue_9001::savepoint_without_reads_sees_rows_committed_later_by_another_connection' panicked at testing/simulator/runner/savepoint_snapshot_issue_9001.rs:47:5:
assertion `left == right` failed: a connection that only ran SAVEPOINT has no read snapshot yet, so it must see rows committed afterwards
  left: [[SimValue(Numeric(Integer(1)))]]
 right: [[SimValue(Numeric(Integer(1)))], [SimValue(Numeric(Integer(2)))]]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    runner::env::savepoint_snapshot_issue_9001::insert_select_after_savepoint_copies_rows_committed_by_another_connection
    runner::env::savepoint_snapshot_issue_9001::savepoint_without_reads_sees_rows_committed_later_by_another_connection

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 32 filtered out; finished in 0.00s

error: test failed, to rerun pass `-p limbo_sim --bin limbo_sim`
```

## Files

- `testing/simulator/runner/savepoint_snapshot_issue_9001.rs`
- `testing/simulator/runner/env.rs`