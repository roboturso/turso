# Reproducer: #4929 — `VACUUM INTO` fails with MVCC for certain cases

https://github.com/tursodatabase/turso/issues/4929

## Environment

- Commit: ca8933a2e32b74edf6e24650c4f6ace9f0ad6f3c
- Platform: linux x86_64
- Reproduced at: 2026-09-22T10:51:14Z

## Reproduce

```sh
cargo test -q -p core_tester --test integration_tests test_vacuum_into_mvcc
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Views, triggers, partial indexes, user_version, application_id, and page_count all work with MVCC on the current tree, but VACUUM INTO from an MVCC source still does not bump schema_version by one (SQLite and WAL mode give source+1; MVCC gives source+0) because the MVCC checkpoint overwrites the destination's schema cookie with the in-memory schema version, so test_vacuum_into_preserves_meta_values still cannot run with MVCC.

## Observed failure

RoboTurso ran the command above and observed:

```text
running 10 tests
. 1/10
query_processing::test_vacuum_into_mvcc::test_vacuum_into_preserves_meta_values_mvcc --- FAILED
........
failures:

---- query_processing::test_vacuum_into_mvcc::test_vacuum_into_preserves_meta_values_mvcc stdout ----

thread 'query_processing::test_vacuum_into_mvcc::test_vacuum_into_preserves_meta_values_mvcc' panicked at tests/integration/query_processing/test_vacuum_into_mvcc.rs:202:5:
assertion `left == right` failed: schema_version should be source schema_version + 1
  left: [(2,)]
 right: [(3,)]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    query_processing::test_vacuum_into_mvcc::test_vacuum_into_preserves_meta_values_mvcc

test result: FAILED. 9 passed; 1 failed; 0 ignored; 0 measured; 1138 filtered out; finished in 0.18s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/query_processing/test_vacuum_into_mvcc.rs`
- `tests/integration/query_processing/mod.rs`