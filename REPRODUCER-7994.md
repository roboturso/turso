# Reproducer: #7994 — "no such column" error echoes the schema casing of the identifier instead of the query text

https://github.com/tursodatabase/turso/issues/7994

## Environment

- Commit: a9926eb465babb7fcd74b500e3fe64d11e7c1e39
- Platform: linux x86_64
- Reproduced at: 2026-07-23T10:20:47Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests no_such_column_error_uses_query_casing
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: Turso reports `no such column: main.Id` (schema casing) where SQLite echoes the query's spelling `main.id`; added a regression test that fails on the current tree.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-86392ed0f6cdde61)

running 1 test
test issue_7994_no_such_column_casing::no_such_column_error_uses_query_casing ... FAILED

failures:

---- issue_7994_no_such_column_casing::no_such_column_error_uses_query_casing stdout ----

thread 'issue_7994_no_such_column_casing::no_such_column_error_uses_query_casing' panicked at tests/integration/issue_7994_no_such_column_casing.rs:27:5:
error must echo the identifier as written in the query (`main.id`), got: Parse error: no such column: main.Id
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    issue_7994_no_such_column_casing::no_such_column_error_uses_query_casing

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1001 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/issue_7994_no_such_column_casing.rs`
- `tests/integration/mod.rs`