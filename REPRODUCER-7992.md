# Reproducer: #7992 — Logical-log header published before durable write

https://github.com/tursodatabase/turso/issues/7992

## Environment

- Commit: 6d01be3604bbd2bc1066c2fef3f8049b98ed4523
- Platform: linux x86_64
- Reproduced at: 2026-09-23T18:16:07Z

## Reproduce

```sh
cargo test -p turso_core --features conn_raw_api --lib header_upgrade_failure_tests
```

Fails on the current tree; passes once the bug is fixed.

## What happens

When the on-disk header write fails during an upgrade, the in-memory logical-log header has already been set to version 3 and is not rolled back, so the next portable commit skips the upgrade and appends v3 data to a file whose header on disk is still v2.

## Observed failure

RoboTurso ran the command above and observed:

```text
Compiling turso_core v0.8.0-pre.12 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7992/core)
warning: unused import: `HashMap`
 --> core/vdbe/statement_lifecycle_tests.rs:2:24
  |
2 | use std::collections::{HashMap, HashSet};
  |                        ^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `turso_core` (lib test) generated 1 warning (run `cargo fix --lib -p turso_core --tests` to apply 1 suggestion)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 16.03s
     Running unittests lib.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/turso_core-4ee84db7cd5b228b)

running 1 test
test mvcc::persistent_storage::logical_log::header_upgrade_failure_tests::failed_header_upgrade_write_keeps_in_memory_header_at_old_version ... FAILED

failures:

---- mvcc::persistent_storage::logical_log::header_upgrade_failure_tests::failed_header_upgrade_write_keeps_in_memory_header_at_old_version stdout ----

thread 'mvcc::persistent_storage::logical_log::header_upgrade_failure_tests::failed_header_upgrade_write_keeps_in_memory_header_at_old_version' panicked at core/mvcc/persistent_storage/logical_log/header_upgrade_failure_tests.rs:89:5:
assertion `left == right` failed: in-memory header advanced to version 3 although the on-disk header write failed
  left: 3
 right: 2
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    mvcc::persistent_storage::logical_log::header_upgrade_failure_tests::failed_header_upgrade_write_keeps_in_memory_header_at_old_version

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2539 filtered out; finished in 0.00s

error: test failed, to rerun pass `-p turso_core --lib`
```

## Files

- `core/mvcc/persistent_storage/logical_log/header_upgrade_failure_tests.rs`
- `core/mvcc/persistent_storage/logical_log.rs`