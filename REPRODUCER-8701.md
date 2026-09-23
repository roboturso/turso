# Reproducer: #8701 — ALTER TABLE ADD COLUMN writes REFERENCES p(), and no engine can reopen it

https://github.com/tursodatabase/turso/issues/8701

## Environment

- Commit: 6d01be3604bbd2bc1066c2fef3f8049b98ed4523
- Platform: linux x86_64
- Reproduced at: 2026-09-23T17:59:09Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests test_alter_table_keeps_fk_without_parent_columns_parseable
```

Fails on the current tree; passes once the bug is fixed.

## What happens

ALTER TABLE ADD/DROP COLUMN rewrites a foreign key with no parent column list as `REFERENCES p()`, which neither Turso nor SQLite can parse when the database is reopened.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-e1bb149c2239ec41)

running 1 test
test query_processing::test_alter_table_fk_no_parent_columns::test_alter_table_keeps_fk_without_parent_columns_parseable ... FAILED

failures:

---- query_processing::test_alter_table_fk_no_parent_columns::test_alter_table_keeps_fk_without_parent_columns_parseable stdout ----

thread 'query_processing::test_alter_table_fk_no_parent_columns::test_alter_table_keeps_fk_without_parent_columns_parseable' panicked at tests/integration/query_processing/test_alter_table_fk_no_parent_columns.rs:23:9:
assertion `left == right` failed
  left: [(2,)]
 right: [(0,)]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    query_processing::test_alter_table_fk_no_parent_columns::test_alter_table_keeps_fk_without_parent_columns_parseable

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1140 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/query_processing/test_alter_table_fk_no_parent_columns.rs`
- `tests/integration/query_processing/mod.rs`