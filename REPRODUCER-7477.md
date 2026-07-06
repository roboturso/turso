# Reproducer: #7477 — MVCC: dropped committed DELETE leaves tombstones pointing at removed TxID and panics later writers

https://github.com/tursodatabase/turso/issues/7477

## Environment

- Commit: d4d6777a2685ab9a3ffc8733329743fdfe000c45
- Platform: linux x86_64
- Reproduced at: 2026-07-06T10:43:36Z

## Reproduce

```sh
cargo test -p turso_core mvcc_bug_repro_dropped_committed_delete_rewrites_all_tombstone_txids
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Reproduced on current main: dropping a committed large DELETE's COMMIT statement after LogRecordPrepared leaves tombstones referencing the removed TxID, and the next concurrent writer panics with "check_version_conflicts: tombstone end TxID not found in txn map" at core/mvcc/database/mod.rs:1725.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running unittests lib.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/turso_core-34fdf59f579c350e)

running 1 test
test mvcc::database::tests::mvcc_bug_repro_dropped_committed_delete_rewrites_all_tombstone_txids ... FAILED

failures:

---- mvcc::database::tests::mvcc_bug_repro_dropped_committed_delete_rewrites_all_tombstone_txids stdout ----
path: /tmp/.tmpL0O27L/test_4336448699796916570

thread 'mvcc::database::tests::mvcc_bug_repro_dropped_committed_delete_rewrites_all_tombstone_txids' panicked at core/mvcc/database/mod.rs:1725:73:
check_version_conflicts: tombstone end TxID not found in txn map
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

thread 'mvcc::database::tests::mvcc_bug_repro_dropped_committed_delete_rewrites_all_tombstone_txids' panicked at core/mvcc/database/tests.rs:16838:5:
later public writer must not panic on a stale removed tombstone TxID


failures:
    mvcc::database::tests::mvcc_bug_repro_dropped_committed_delete_rewrites_all_tombstone_txids

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2034 filtered out; finished in 0.27s

error: test failed, to rerun pass `-p turso_core --lib`
```

## Files

- `core/mvcc/database/tests.rs`