# Reproducer: #6876 — ALTER COLUMN Rewrites Table Records Without Rebuilding Secondary Indexes

https://github.com/tursodatabase/turso/issues/6876

## Environment

- Commit: c76789266d7e4f6a78e37a01862497f3182df733
- Platform: linux x86_64
- Reproduced at: 2026-07-09T09:28:50Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests alter_column_rebuilds_secondary_index
```

Fails on the current tree; passes once the bug is fixed.

## What happens

ALTER TABLE ... ALTER COLUMN physically rewrites the table rows without rebuilding secondary indexes, so PRAGMA integrity_check reports every row missing from idx_x after the reproducer runs.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-a796116438746cf7)

running 1 test
test alter_column_index::alter_column_rebuilds_secondary_index ... FAILED

failures:

---- alter_column_index::alter_column_rebuilds_secondary_index stdout ----

thread 'alter_column_index::alter_column_rebuilds_secondary_index' panicked at tests/integration/alter_column_index.rs:25:5:
assertion `left == right` failed: integrity_check must pass after ALTER COLUMN on an indexed column, got: [[Text("row 1 missing from index idx_x")], [Text("row 2 missing from index idx_x")], [Text("row 3 missing from index idx_x")]]
  left: [[Text("row 1 missing from index idx_x")], [Text("row 2 missing from index idx_x")], [Text("row 3 missing from index idx_x")]]
 right: [[Text("ok")]]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    alter_column_index::alter_column_rebuilds_secondary_index

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 979 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/alter_column_index.rs`
- `tests/integration/mod.rs`