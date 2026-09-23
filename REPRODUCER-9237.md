# Reproducer: #9237 — MVCC: group commit with synchronous=OFF leader does not sync the log for a FULL waiter

https://github.com/tursodatabase/turso/issues/9237

## Environment

- Commit: 6d01be3604bbd2bc1066c2fef3f8049b98ed4523
- Platform: linux x86_64
- Reproduced at: 2026-09-23T17:43:38Z

## Reproduce

```sh
cargo test -p turso_core --lib full_waiter_is_synced_when_group_leader_uses_sync_off
```

Fails on the current tree; passes once the bug is fixed.

## What happens

When the group commit leader uses synchronous=OFF, the log is never synced, even though a FULL waiter in the same group is reported as committed, because the sync decision only looks at the leader's sync mode.

## Observed failure

RoboTurso ran the command above and observed:

```text
Compiling turso_core v0.8.0-pre.12 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-9237/core)
warning: unused import: `HashMap`
 --> core/vdbe/statement_lifecycle_tests.rs:2:24
  |
2 | use std::collections::{HashMap, HashSet};
  |                        ^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `turso_core` (lib test) generated 1 warning (run `cargo fix --lib -p turso_core --tests` to apply 1 suggestion)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 15.59s
     Running unittests lib.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/turso_core-32056d647a50aa07)

running 1 test
test mvcc::database::tests::group_commit_sync_mode_tests::full_waiter_is_synced_when_group_leader_uses_sync_off ... FAILED

failures:

---- mvcc::database::tests::group_commit_sync_mode_tests::full_waiter_is_synced_when_group_leader_uses_sync_off stdout ----

thread 'mvcc::database::tests::group_commit_sync_mode_tests::full_waiter_is_synced_when_group_leader_uses_sync_off' panicked at core/mvcc/database/group_commit_sync_mode_tests.rs:16:5:
FULL waiter returned success with zero sync calls
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    mvcc::database::tests::group_commit_sync_mode_tests::full_waiter_is_synced_when_group_leader_uses_sync_off

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2514 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p turso_core --lib`
```

## Files

- `core/mvcc/database/group_commit_sync_mode_tests.rs`
- `core/mvcc/database/tests.rs`