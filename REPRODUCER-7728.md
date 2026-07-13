# Reproducer: #7728 — Active SELECT can read recycled root page after DROP TABLE, causing TableInterior panic / possible stale data

https://github.com/tursodatabase/turso/issues/7728

## Environment

- Commit: c390087d4bfb1350c66b404769e2c47a92a2b426
- Platform: linux x86_64
- Reproduced at: 2026-07-13T05:47:46Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests table_interior_drop_reuse
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed on current main: an active SELECT resumed after DROP TABLE plus root-page reuse panics at core/storage/pager.rs:395 with the TableInterior page-type assertion, exactly as reported.

## Observed failure

RoboTurso ran the command above and observed:

```text
Blocking waiting for file lock on build directory
   Compiling turso_ext v0.7.0-pre.20 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7728/extensions/core)
   Compiling turso_core v0.7.0-pre.20 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7728/core)
   Compiling turso_sync_engine v0.7.0-pre.20 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7728/sync/engine)
   Compiling sql_generation v0.7.0-pre.20 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7728/sql_generation)
   Compiling turso-dbhash v0.7.0-pre.20 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7728/tools/dbhash)
   Compiling turso_sdk_kit v0.7.0-pre.20 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7728/sdk-kit)
   Compiling turso_sync_sdk_kit v0.7.0-pre.20 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7728/sync/sdk-kit)
   Compiling turso v0.7.0-pre.20 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7728/bindings/rust)
   Compiling core_tester v0.7.0-pre.20 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7728/tests)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1m 51s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-a0066e450608f031)

running 1 test
test table_interior_drop_reuse::active_table_seek_after_drop_reuse_must_not_use_recycled_root_page ... FAILED

failures:

---- table_interior_drop_reuse::active_table_seek_after_drop_reuse_must_not_use_recycled_root_page stdout ----

thread 'table_interior_drop_reuse::active_table_seek_after_drop_reuse_must_not_use_recycled_root_page' panicked at core/storage/pager.rs:395:9:
matches! (self.page_type(), Ok(PageType::TableInterior))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    table_interior_drop_reuse::active_table_seek_after_drop_reuse_must_not_use_recycled_root_page

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 979 filtered out; finished in 0.02s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/table_interior_drop_reuse.rs`
- `tests/integration/mod.rs`