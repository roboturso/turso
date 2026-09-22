# Reproducer: #8373 — DROP INDEX does not persist for mixed-case indexes imported from SQLite, allowing DROP COLUMN to corrupt schema

https://github.com/tursodatabase/turso/issues/8373

## Environment

- Commit: a012590f552751754ea18a4226432664e06055ea
- Platform: linux x86_64
- Reproduced at: 2026-09-22T17:58:55Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests drop_index_mixed_case_sqlite_schema
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: DROP INDEX lowercases the index name and compares it case-sensitively against sqlite_schema.name, so a SQLite-created row like CustomerLookupMixedCase is never deleted (while its root page is still destroyed), and a following DROP COLUMN commits a schema that neither Turso nor sqlite3 can load.

## Observed failure

RoboTurso ran the command above and observed:

```text
Compiling core_tester v0.8.0-pre.12 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8373/tests)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 16.76s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-e1bb149c2239ec41)

running 2 tests
test drop_index_mixed_case_sqlite_schema::drop_index_removes_sqlite_created_mixed_case_row_from_sqlite_schema ... FAILED
test drop_index_mixed_case_sqlite_schema::drop_index_then_drop_column_on_sqlite_created_database_keeps_schema_loadable ... FAILED

failures:

---- drop_index_mixed_case_sqlite_schema::drop_index_removes_sqlite_created_mixed_case_row_from_sqlite_schema stdout ----

thread 'drop_index_mixed_case_sqlite_schema::drop_index_removes_sqlite_created_mixed_case_row_from_sqlite_schema' panicked at tests/integration/drop_index_mixed_case_sqlite_schema.rs:50:5:
DROP INDEX left the SQLite-created index row in sqlite_schema
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- drop_index_mixed_case_sqlite_schema::drop_index_then_drop_column_on_sqlite_created_database_keeps_schema_loadable stdout ----

thread 'drop_index_mixed_case_sqlite_schema::drop_index_then_drop_column_on_sqlite_created_database_keeps_schema_loadable' panicked at tests/integration/drop_index_mixed_case_sqlite_schema.rs:87:5:
stale index row survived: [[Text("CustomerLookupMixedCase")]]


failures:
    drop_index_mixed_case_sqlite_schema::drop_index_removes_sqlite_created_mixed_case_row_from_sqlite_schema
    drop_index_mixed_case_sqlite_schema::drop_index_then_drop_column_on_sqlite_created_database_keeps_schema_loadable

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 1138 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/drop_index_mixed_case_sqlite_schema.rs`
- `tests/integration/mod.rs`