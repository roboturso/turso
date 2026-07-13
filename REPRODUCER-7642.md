# Reproducer: #7642 — checkpoint fails with DatabaseFull, then the next table read uses the leaked root mapping and short-reads page 3.

https://github.com/tursodatabase/turso/issues/7642

## Environment

- Commit: c390087d4bfb1350c66b404769e2c47a92a2b426
- Platform: linux x86_64
- Reproduced at: 2026-07-13T07:38:58Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests mvcc_checkpoint_dbfull
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed on the current tree: a wal_checkpoint(TRUNCATE) that fails with DatabaseFull in MVCC mode leaves a leaked root mapping, and the next SELECT on the table fails with 'short read on page 3: expected 4096 bytes, got 0' instead of returning the rows.

## Observed failure

RoboTurso ran the command above and observed:

```text
Compiling turso_core v0.7.0-pre.20 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7642/core)
   Compiling turso_ext v0.7.0-pre.20 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7642/extensions/core)
   Compiling turso_parser v0.7.0-pre.20 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7642/sqlite/parser)
   Compiling turso_sdk_kit v0.7.0-pre.20 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7642/sdk-kit)
   Compiling turso_sync_engine v0.7.0-pre.20 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7642/sync/engine)
   Compiling turso-dbhash v0.7.0-pre.20 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7642/tools/dbhash)
   Compiling sql_generation v0.7.0-pre.20 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7642/sql_generation)
   Compiling turso_sync_sdk_kit v0.7.0-pre.20 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7642/sync/sdk-kit)
   Compiling turso v0.7.0-pre.20 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7642/bindings/rust)
   Compiling core_tester v0.7.0-pre.20 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7642/tests)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1m 00s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-a0066e450608f031)

running 1 test
2026-07-13T07:38:57.990368Z ERROR turso_core::storage::sqlite3_ondisk: short read on page 3: expected 4096 bytes, got 0
2026-07-13T07:38:57.990437Z ERROR turso_core::statement: Error while draining pending IO during statement reset: I/O error: short read on page 3: expected 4096 bytes, got 0
2026-07-13T07:38:57.990458Z ERROR turso_core::statement: Statement reset failed during best-effort cleanup: I/O error: short read on page 3: expected 4096 bytes, got 0
test mvcc_checkpoint_dbfull::test_mvcc_read_after_checkpoint_database_full ... FAILED

failures:

---- mvcc_checkpoint_dbfull::test_mvcc_read_after_checkpoint_database_full stdout ----

thread 'mvcc_checkpoint_dbfull::test_mvcc_read_after_checkpoint_database_full' panicked at tests/integration/mvcc_checkpoint_dbfull.rs:10:1:
called `Result::unwrap()` on an `Err` value: I/O error: short read on page 3: expected 4096 bytes, got 0
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    mvcc_checkpoint_dbfull::test_mvcc_read_after_checkpoint_database_full

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 979 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/mvcc_checkpoint_dbfull.rs`
- `tests/integration/mod.rs`