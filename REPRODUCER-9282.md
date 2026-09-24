# Reproducer: #9282 — A SQLite file with an FTS or R-Tree table loads only part of the schema, and writes then corrupt its indexes

https://github.com/tursodatabase/turso/issues/9282

## Environment

- Commit: 173b8773ff7e35af7df51973d923212918851dd1
- Platform: linux x86_64
- Reproduced at: 2026-09-24T10:52:55Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests unknown_vtab_module_schema
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: when a SQLite file has an fts5 or rtree table, Turso stops loading the schema at that row, so later tables are missing, indexes are never attached (INSERTs leave index ax missing entries), and PRAGMA integrity_check reports unknown b-tree pages as never used.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-e1bb149c2239ec41)

running 2 tests
test unknown_vtab_module_schema::tables_after_fts5_table_are_visible ... FAILED
test unknown_vtab_module_schema::writes_update_indexes_when_file_has_rtree_table ... FAILED

failures:

---- unknown_vtab_module_schema::tables_after_fts5_table_are_visible stdout ----

thread 'unknown_vtab_module_schema::tables_after_fts5_table_are_visible' panicked at tests/integration/common.rs:425:40:
called `Result::unwrap()` on an `Err` value: ParseError("no such table: z")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- unknown_vtab_module_schema::writes_update_indexes_when_file_has_rtree_table stdout ----

thread 'unknown_vtab_module_schema::writes_update_indexes_when_file_has_rtree_table' panicked at tests/integration/unknown_vtab_module_schema.rs:25:5:
assertion `left == right` failed
  left: [[Text("*** in database main ***\nPage 3: never used\nPage 4: never used\nPage 5: never used\nPage 6: never used")]]
 right: [[Text("ok")]]


failures:
    unknown_vtab_module_schema::tables_after_fts5_table_are_visible
    unknown_vtab_module_schema::writes_update_indexes_when_file_has_rtree_table

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 1150 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/unknown_vtab_module_schema.rs`
- `tests/integration/mod.rs`