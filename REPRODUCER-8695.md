# Reproducer: #8695 — A failed UPDATE keeps the rows it already wrote once inside a transaction

https://github.com/tursodatabase/turso/issues/8695

## Environment

- Commit: 6e320d2e788f2c98333c43697c886f0f7ce9e8ff
- Platform: linux x86_64
- Reproduced at: 2026-09-22T17:43:47Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests update_failure_in_tx_rollback
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed on current main: a failed `UPDATE t SET id = id + 7` inside BEGIN/COMMIT leaves the table as 3,8,9,10 instead of the original 1,2,3,10 that SQLite keeps, so the partially-applied rows from the failed statement are committed.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 18.16s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-e1bb149c2239ec41)

running 3 tests
test update_failure_in_tx_rollback::failed_rowid_update_outside_tx_leaves_table_unchanged ... ok
test update_failure_in_tx_rollback::failed_rowid_update_inside_tx_leaves_table_unchanged ... FAILED
test update_failure_in_tx_rollback::failed_rowid_update_with_unique_column_inside_tx_leaves_table_unchanged ... ok

failures:

---- update_failure_in_tx_rollback::failed_rowid_update_inside_tx_leaves_table_unchanged stdout ----

thread 'update_failure_in_tx_rollback::failed_rowid_update_inside_tx_leaves_table_unchanged' panicked at tests/integration/update_failure_in_tx_rollback.rs:47:5:
assertion `left == right` failed
  left: ["3,8,9,10"]
 right: ["1,2,3,10"]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    update_failure_in_tx_rollback::failed_rowid_update_inside_tx_leaves_table_unchanged

test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 1138 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/update_failure_in_tx_rollback.rs`
- `tests/integration/mod.rs`