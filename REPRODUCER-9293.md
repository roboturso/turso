# Reproducer: #9293 — Self-referencing ON DELETE SET NULL makes DELETE keep rows that match (skip_advance is lost on restore)

https://github.com/tursodatabase/turso/issues/9293

## Environment

- Commit: 173b8773ff7e35af7df51973d923212918851dd1
- Platform: linux x86_64
- Reproduced at: 2026-09-24T10:46:30Z

## Reproduce

```sh
cargo test -q -p core_tester --test integration_tests test_delete_fk_write_back
```

Fails on the current tree; passes once the bug is fixed.

## What happens

A DELETE on a table whose foreign key action writes back to the same table (self-referencing ON DELETE SET NULL, or through an ON UPDATE SET NULL action) skips every second matching row, leaving 1,3,5 where SQLite leaves only 1.

## Observed failure

RoboTurso ran the command above and observed:

```text
running 2 tests
query_processing::test_delete_fk_write_back::test_delete_with_self_referencing_set_null_deletes_every_row --- FAILED
query_processing::test_delete_fk_write_back::test_delete_with_fk_update_write_back_deletes_every_row --- FAILED

failures:

---- query_processing::test_delete_fk_write_back::test_delete_with_self_referencing_set_null_deletes_every_row stdout ----

thread 'query_processing::test_delete_fk_write_back::test_delete_with_self_referencing_set_null_deletes_every_row' panicked at tests/integration/query_processing/test_delete_fk_write_back.rs:15:5:
assertion `left == right` failed
  left: [(1,), (3,), (5,)]
 right: [(1,)]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- query_processing::test_delete_fk_write_back::test_delete_with_fk_update_write_back_deletes_every_row stdout ----

thread 'query_processing::test_delete_fk_write_back::test_delete_with_fk_update_write_back_deletes_every_row' panicked at tests/integration/query_processing/test_delete_fk_write_back.rs:38:5:
assertion `left == right` failed
  left: [(1,), (3,), (5,), (1002,), (1003,), (1004,), (1005,), (1006,)]
 right: [(1,), (1002,), (1003,), (1004,), (1005,), (1006,)]


failures:
    query_processing::test_delete_fk_write_back::test_delete_with_fk_update_write_back_deletes_every_row
    query_processing::test_delete_fk_write_back::test_delete_with_self_referencing_set_null_deletes_every_row

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 1150 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/query_processing/test_delete_fk_write_back.rs`
- `tests/integration/query_processing/mod.rs`