# Reproducer: #8746 — INSERT OR FAIL half-applies a batch when the error is not a conflict it covers

https://github.com/tursodatabase/turso/issues/8746

## Environment

- Commit: 6e320d2e788f2c98333c43697c886f0f7ce9e8ff
- Platform: linux x86_64
- Reproduced at: 2026-09-22T17:17:38Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests insert_or_fail_statement_rollback
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed on main: INSERT OR FAIL keeps rows already written when a multi-row insert fails on a foreign key violation or a rowid datatype mismatch in autocommit mode, whereas SQLite undoes the whole statement (count is 1 in Turso vs 0 in SQLite); the same statement inside an explicit transaction is correctly undone.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-e1bb149c2239ec41)

running 4 tests
test insert_or_fail_statement_rollback::insert_or_fail_rowid_type_mismatch_undoes_earlier_rows_in_autocommit ... FAILED
test insert_or_fail_statement_rollback::insert_or_fail_still_keeps_rows_before_a_unique_conflict ... ok
test insert_or_fail_statement_rollback::insert_or_fail_foreign_key_violation_undoes_earlier_rows_in_autocommit ... FAILED
test insert_or_fail_statement_rollback::insert_or_fail_foreign_key_violation_undoes_earlier_rows_in_transaction ... ok

failures:

---- insert_or_fail_statement_rollback::insert_or_fail_rowid_type_mismatch_undoes_earlier_rows_in_autocommit stdout ----

thread 'insert_or_fail_statement_rollback::insert_or_fail_rowid_type_mismatch_undoes_earlier_rows_in_autocommit' panicked at tests/integration/insert_or_fail_statement_rollback.rs:41:5:
assertion `left == right` failed
  left: 1
 right: 0
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- insert_or_fail_statement_rollback::insert_or_fail_foreign_key_violation_undoes_earlier_rows_in_autocommit stdout ----

thread 'insert_or_fail_statement_rollback::insert_or_fail_foreign_key_violation_undoes_earlier_rows_in_autocommit' panicked at tests/integration/insert_or_fail_statement_rollback.rs:24:5:
assertion `left == right` failed
  left: 1
 right: 0


failures:
    insert_or_fail_statement_rollback::insert_or_fail_foreign_key_violation_undoes_earlier_rows_in_autocommit
    insert_or_fail_statement_rollback::insert_or_fail_rowid_type_mismatch_undoes_earlier_rows_in_autocommit

test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 measured; 1138 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/insert_or_fail_statement_rollback.rs`
- `tests/integration/mod.rs`