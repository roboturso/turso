# Reproducer: #9383 — FTS: DROP TABLE leaves an orphaned backing-index page, confirmed by SQLite (0.7.2)

https://github.com/tursodatabase/turso/issues/9383

## Environment

- Commit: 632b58d522c73dd6b6ca34b3ecfde45b079651bd
- Platform: linux x86_64
- Reproduced at: 2026-09-28T06:25:22Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests index_method::drop_table_backing_btree
```

Fails on the current tree; passes once the bug is fixed.

## What happens

The FTS case now passes on the current tree, but dropping a table that has a `USING backing_btree` index still leaves that index's page unfreed, and both PRAGMA integrity_check and sqlite3 report 'Page 3: never used'.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-444bfbe90304a55b)

running 2 tests
test index_method::drop_table_backing_btree::drop_table_frees_pages_of_backing_btree_index ... FAILED
test index_method::drop_table_backing_btree::drop_table_frees_pages_of_fts_index ... ok

failures:

---- index_method::drop_table_backing_btree::drop_table_frees_pages_of_backing_btree_index stdout ----

thread 'index_method::drop_table_backing_btree::drop_table_frees_pages_of_backing_btree_index' panicked at tests/integration/index_method/drop_table_backing_btree.rs:12:5:
assertion `left == right` failed
  left: [("*** in database main ***\nPage 3: never used",)]
 right: [("ok",)]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    index_method::drop_table_backing_btree::drop_table_frees_pages_of_backing_btree_index

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 1158 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/index_method/drop_table_backing_btree.rs`
- `tests/integration/index_method/mod.rs`