# Reproducer: #6859 — UPSERT OR REPLACE Uses REPLACE Journal Analysis For a DO UPDATE Arm That SQLite Requires To Be ABORT

https://github.com/tursodatabase/turso/issues/6859

## Environment

- Commit: d115b414ad83d05a1b3b66063eeefd07a5f51268
- Platform: linux x86_64
- Reproduced at: 2026-07-12T07:14:10Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests test_upsert_or_replace_do_update_unique_failure_no_index_corruption
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Reproduced on current main: the failed DO UPDATE arm of INSERT OR REPLACE ... ON CONFLICT leaves sqlite_autoindex_t_2 out of sync (integrity_check reports 'row 1 missing from index'), whereas SQLite aborts the statement cleanly and reports ok.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-41d957122089e1f9)

running 1 test
test upsert_or_replace_abort_integrity::test_upsert_or_replace_do_update_unique_failure_no_index_corruption ... FAILED

failures:

---- upsert_or_replace_abort_integrity::test_upsert_or_replace_do_update_unique_failure_no_index_corruption stdout ----

thread 'upsert_or_replace_abort_integrity::test_upsert_or_replace_do_update_unique_failure_no_index_corruption' panicked at tests/integration/upsert_or_replace_abort_integrity.rs:63:5:
assertion `left == right` failed: integrity_check must pass inside the transaction after the failed UPSERT
  left: "row 1 missing from index sqlite_autoindex_t_2\nwrong # of entries in index sqlite_autoindex_t_2"
 right: "ok"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    upsert_or_replace_abort_integrity::test_upsert_or_replace_do_update_unique_failure_no_index_corruption

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 979 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/upsert_or_replace_abort_integrity.rs`
- `tests/integration/mod.rs`