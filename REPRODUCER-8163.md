# Reproducer: #8163 — Persistent checkpoint write failures can hang, panic, or mark unwritten frames as backfilled

https://github.com/tursodatabase/turso/issues/8163

## Environment

- Commit: cb1c3c8653e8a1a96ec0159ef69d9906c71940aa
- Platform: linux x86_64
- Reproduced at: 2026-09-24T13:03:33Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests checkpoint_write_failure::checkpoint_with_persistent_write_failure_returns_error_and_keeps_connection_usable
```

Fails on the current tree; passes once the bug is fixed.

## What happens

A checkpoint whose main-database write fails through the default File::pwritev hangs forever, because the child completion ignores the Err result and never finishes the parent completion.

## Observed failure

RoboTurso ran the command above and observed:

```text
Compiling turso_core v0.8.0-pre.12 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8163/core)
   Compiling turso_sync_engine v0.8.0-pre.12 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8163/sync/engine)
   Compiling sql_generation v0.8.0-pre.12 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8163/sql_generation)
   Compiling turso-dbhash v0.8.0-pre.12 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8163/tools/dbhash)
   Compiling turso_sdk_kit v0.8.0-pre.12 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8163/sdk-kit)
   Compiling turso_sync_sdk_kit v0.8.0-pre.12 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8163/sync/sdk-kit)
   Compiling turso v0.8.0-pre.12 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8163/bindings/rust)
   Compiling core_tester v0.8.0-pre.12 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8163/tests)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 40.30s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-56909b28b7d44da0)

running 1 test
test checkpoint_write_failure::checkpoint_with_persistent_write_failure_returns_error_and_keeps_connection_usable ... FAILED

failures:

---- checkpoint_write_failure::checkpoint_with_persistent_write_failure_returns_error_and_keeps_connection_usable stdout ----

thread 'checkpoint_write_failure::checkpoint_with_persistent_write_failure_returns_error_and_keeps_connection_usable' panicked at tests/integration/checkpoint_write_failure.rs:170:19:
checkpoint hung after a persistent database write failure
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    checkpoint_write_failure::checkpoint_with_persistent_write_failure_returns_error_and_keeps_connection_usable

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1151 filtered out; finished in 20.00s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/checkpoint_write_failure.rs`
- `tests/integration/mod.rs`