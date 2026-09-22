# Reproducer: #9005 — panic "shared WAL frame ids must increase monotonically"

https://github.com/tursodatabase/turso/issues/9005

## Environment

- Commit: 665cf5b2c80c3f76b6652ab102d7da2cd470f904
- Platform: linux x86_64
- Reproduced at: 2026-09-22T09:29:16Z

## Reproduce

```sh
cargo test -p turso_whopper --test regression_issue_9005
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed on the current tree: the seeded multiprocess btree-rebalance run panics deterministically on step 2443 with "shared WAL frame ids must increase monotonically: new_frame_id=157, previous_frame_id=344, slot=344, shared_max_frame=156" after workers are killed and respawned.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running regression_issue_9005.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/regression_issue_9005-f495addd9fa23184)

running 1 test
WORKER PANIC: panicked at core/storage/wal.rs:2430:24:
shared WAL frame ids must increase monotonically: new_frame_id=157, previous_frame_id=344, slot=344, shared_max_frame=156
worker panic during SQL execution: shared WAL frame ids must increase monotonically: new_frame_id=157, previous_frame_id=344, slot=344, shared_max_frame=156
WORKER SQL ERROR [Panic]: worker panicked: shared WAL frame ids must increase monotonically: new_frame_id=157, previous_frame_id=344, slot=344, shared_max_frame=156 (sql: INSERT OR REPLACE INTO btree_rebalance(id, k, pad, tag) VALUES (313290000, 735114, zeroblob(16384), 'tag_43'))
test issue_9005_respawned_worker_does_not_panic_on_shared_wal_frame_order ... FAILED

failures:

---- issue_9005_respawned_worker_does_not_panic_on_shared_wal_frame_order stdout ----

thread 'issue_9005_respawned_worker_does_not_panic_on_shared_wal_frame_order' panicked at testing/concurrent-simulator/regression_issue_9005.rs:67:14:
multiprocess whopper step must not fail: connection 1 fatal error on step 2443: Internal error: Panic: worker panicked: shared WAL frame ids must increase monotonically: new_frame_id=157, previous_frame_id=344, slot=344, shared_max_frame=156
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    issue_9005_respawned_worker_does_not_panic_on_shared_wal_frame_order

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.01s

error: test failed, to rerun pass `-p turso_whopper --test regression_issue_9005`
```

## Files

- `testing/concurrent-simulator/regression_issue_9005.rs`
- `testing/concurrent-simulator/Cargo.toml`