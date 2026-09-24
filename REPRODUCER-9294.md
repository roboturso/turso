# Reproducer: #9294 — Self-referencing ON DELETE SET NULL makes DELETE stop after it deletes cell 0 of a leaf page

https://github.com/tursodatabase/turso/issues/9294

## Environment

- Commit: 173b8773ff7e35af7df51973d923212918851dd1
- Platform: linux x86_64
- Reproduced at: 2026-09-24T10:43:24Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests query_processing::test_delete_fk_action_first_cell
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Both reproducers fail on the current tree: after the DELETE removes cell 0 of a leaf page and an FK action writes back to the table, the scan stops and later matching rows stay (Turso keeps 6..12 where SQLite deletes them).

## Observed failure

RoboTurso ran the command above and observed:

```text
Compiling turso_ext v0.8.0-pre.12 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-9294/extensions/core)
   Compiling turso_core v0.8.0-pre.12 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-9294/core)
   Compiling turso_parser v0.8.0-pre.12 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-9294/sqlite/parser)
   Compiling turso_sdk_kit v0.8.0-pre.12 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-9294/sdk-kit)
   Compiling turso_sync_engine v0.8.0-pre.12 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-9294/sync/engine)
   Compiling turso-dbhash v0.8.0-pre.12 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-9294/tools/dbhash)
   Compiling sql_generation v0.8.0-pre.12 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-9294/sql_generation)
   Compiling turso_sync_sdk_kit v0.8.0-pre.12 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-9294/sync/sdk-kit)
   Compiling turso v0.8.0-pre.12 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-9294/bindings/rust)
   Compiling core_tester v0.8.0-pre.12 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-9294/tests)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1m 11s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-e1bb149c2239ec41)

running 2 tests
test query_processing::test_delete_fk_action_first_cell::delete_with_self_referencing_set_null_continues_after_first_cell_of_page ... FAILED
test query_processing::test_delete_fk_action_first_cell::delete_with_fk_update_write_back_continues_after_first_cell_of_page ... FAILED

failures:

---- query_processing::test_delete_fk_action_first_cell::delete_with_self_referencing_set_null_continues_after_first_cell_of_page stdout ----

thread 'query_processing::test_delete_fk_action_first_cell::delete_with_self_referencing_set_null_continues_after_first_cell_of_page' panicked at tests/integration/query_processing/test_delete_fk_action_first_cell.rs:24:5:
assertion `left == right` failed
  left: [[Text("1,2,3,4,6,7,8,9,10,11,12")]]
 right: [[Text("1,2,3,4")]]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- query_processing::test_delete_fk_action_first_cell::delete_with_fk_update_write_back_continues_after_first_cell_of_page stdout ----

thread 'query_processing::test_delete_fk_action_first_cell::delete_with_fk_update_write_back_continues_after_first_cell_of_page' panicked at tests/integration/query_processing/test_delete_fk_action_first_cell.rs:42:5:
assertion `left == right` failed
  left: [[Text("1,2,3,4,6,7,8,9,10,11,12,1005")]]
 right: [[Text("1,2,3,4,1005")]]


failures:
    query_processing::test_delete_fk_action_first_cell::delete_with_fk_update_write_back_continues_after_first_cell_of_page
    query_processing::test_delete_fk_action_first_cell::delete_with_self_referencing_set_null_continues_after_first_cell_of_page

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 1150 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/query_processing/test_delete_fk_action_first_cell.rs`
- `tests/integration/query_processing/mod.rs`