# Reproducer: #6878 — DML RETURNING Errors Do Not Roll Back Already-Written Rows

https://github.com/tursodatabase/turso/issues/6878

## Environment

- Commit: f7aac1fc5e02e352f32b4a86349b636870a68e53
- Platform: linux x86_64
- Reproduced at: 2026-09-22T12:50:05Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests returning_error_rollback
```

Fails on the current tree; passes once the bug is fixed.

## What happens

The issue's own multi-row reproducers already match SQLite on main (RETURNING now marks the statement as needing a savepoint), but single-row INSERT/UPDATE/DELETE with a failing RETURNING expression inside BEGIN…COMMIT still commit the written row because no statement savepoint is opened when is_multi_write is false — SQLite undoes the row in all three cases.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-bafd09175e2df6dc)

running 8 tests
test returning_error_rollback::single_row_delete_returning_error_in_tx_keeps_rows ... FAILED
test returning_error_rollback::single_row_insert_returning_error_in_tx_keeps_rows ... FAILED
test returning_error_rollback::multi_row_insert_returning_error_in_tx_keeps_rows ... ok
test returning_error_rollback::single_row_update_returning_error_in_tx_keeps_rows ... FAILED
test returning_error_rollback::update_returning_error_in_tx_keeps_rows ... ok
test returning_error_rollback::delete_returning_error_in_tx_keeps_rows ... ok
test returning_error_rollback::insert_or_replace_returning_error_in_tx_keeps_old_row ... ok
test returning_error_rollback::delete_returning_row_dependent_error_in_tx_keeps_rows ... ok

failures:

---- returning_error_rollback::single_row_delete_returning_error_in_tx_keeps_rows stdout ----

thread 'returning_error_rollback::single_row_delete_returning_error_in_tx_keeps_rows' panicked at tests/integration/returning_error_rollback.rs:66:5:
assertion `left == right` failed
  left: ["2", "3"]
 right: ["1", "2", "3"]

---- returning_error_rollback::single_row_insert_returning_error_in_tx_keeps_rows stdout ----

thread 'returning_error_rollback::single_row_insert_returning_error_in_tx_keeps_rows' panicked at tests/integration/returning_error_rollback.rs:47:5:
assertion `left == right` failed
  left: ["1", "2"]
 right: ["1"]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- returning_error_rollback::single_row_update_returning_error_in_tx_keeps_rows stdout ----

thread 'returning_error_rollback::single_row_update_returning_error_in_tx_keeps_rows' panicked at tests/integration/returning_error_rollback.rs:90:5:
assertion `left == right` failed
  left: ["1|11", "2|20"]
 right: ["1|10", "2|20"]


failures:
    returning_error_rollback::single_row_delete_returning_error_in_tx_keeps_rows
    returning_error_rollback::single_row_insert_returning_error_in_tx_keeps_rows
    returning_error_rollback::single_row_update_returning_error_in_tx_keeps_rows

test result: FAILED. 5 passed; 3 failed; 0 ignored; 0 measured; 1138 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/returning_error_rollback.rs`
- `tests/integration/mod.rs`