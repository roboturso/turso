# Reproducer: #7536 — Pager/WAL statement rollback after abandoned StepResult::IO corrupts overflow pages under cache spill

https://github.com/tursodatabase/turso/issues/7536

## Environment

- Commit: d4d6777a2685ab9a3ffc8733329743fdfe000c45
- Platform: linux x86_64
- Reproduced at: 2026-07-06T11:54:50Z

## Reproduce

```sh
cargo test -p core_tester --features io_memory_yield --test integration_tests stmt_rollback_after_abandoned_io_with_cache_spill
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Reproduced on current main: after dropping a DELETE statement mid-execution (StepResult::IO) inside an open transaction with cache spill enabled, COMMIT leaves the database corrupted — PRAGMA integrity_check reports an overflow page referenced multiple times and several never-used pages instead of "ok".

## Observed failure

RoboTurso ran the command above and observed:

```text
Compiling turso_core v0.7.0-pre.13 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7536/core)
   Compiling turso_sync_engine v0.7.0-pre.13 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7536/sync/engine)
   Compiling sql_generation v0.7.0-pre.13 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7536/sql_generation)
   Compiling turso-dbhash v0.7.0-pre.13 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7536/tools/dbhash)
   Compiling turso_sdk_kit v0.7.0-pre.13 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7536/sdk-kit)
   Compiling turso_sync_sdk_kit v0.7.0-pre.13 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7536/sync/sdk-kit)
   Compiling turso v0.7.0-pre.13 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7536/bindings/rust)
   Compiling core_tester v0.7.0-pre.13 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7536/tests)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 31.01s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-d2a0c7ad5c96c94b)

running 1 test
test pager_savepoint_yield::stmt_rollback_after_abandoned_io_with_cache_spill ... FAILED

failures:

---- pager_savepoint_yield::stmt_rollback_after_abandoned_io_with_cache_spill stdout ----

thread 'pager_savepoint_yield::stmt_rollback_after_abandoned_io_with_cache_spill' panicked at tests/integration/pager_savepoint_yield.rs:56:5:
assertion `left == right` failed: integrity_check failed: *** in database main ***
Page 124 referenced multiple times (references=[3, 24], page_category=Overflow)
Page 126: never used
Page 127: never used
Page 128: never used
Page 129: never used
Page 130: never used
  left: "*** in database main ***\nPage 124 referenced multiple times (references=[3, 24], page_category=Overflow)\nPage 126: never used\nPage 127: never used\nPage 128: never used\nPage 129: never used\nPage 130: never used"
 right: "ok"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    pager_savepoint_yield::stmt_rollback_after_abandoned_io_with_cache_spill

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 962 filtered out; finished in 0.02s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/pager_savepoint_yield.rs`
- `tests/integration/mod.rs`
- `tests/Cargo.toml`