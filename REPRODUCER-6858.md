# Reproducer: #6858 — UPSERT DO UPDATE Deletes Secondary Index Entries Before Proving the Replacement Row Is Valid

https://github.com/tursodatabase/turso/issues/6858

## Environment

- Commit: c390087d4bfb1350c66b404769e2c47a92a2b426
- Platform: linux x86_64
- Reproduced at: 2026-07-13T05:56:15Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests test_upsert_do_update_failure_preserves_indexes
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed on current main: the failed UPSERT DO UPDATE removes the target row's secondary index entries before the replacement row is validated, leaving integrity_check reporting missing index entries where SQLite reports ok.

## Observed failure

RoboTurso ran the command above and observed:

```text
Compiling core_tester v0.7.0-pre.20 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-6858/tests)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 8.95s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-a0066e450608f031)

running 1 test
test upsert_do_update_index_corruption::test_upsert_do_update_failure_preserves_indexes ... FAILED

failures:

---- upsert_do_update_index_corruption::test_upsert_do_update_failure_preserves_indexes stdout ----

thread 'upsert_do_update_index_corruption::test_upsert_do_update_failure_preserves_indexes' panicked at tests/integration/upsert_do_update_index_corruption.rs:56:5:
assertion `left == right` failed: integrity_check inside transaction: row 1 missing from index idx_b
row 1 missing from index sqlite_autoindex_t_2
wrong # of entries in index sqlite_autoindex_t_2
  left: "row 1 missing from index idx_b\nrow 1 missing from index sqlite_autoindex_t_2\nwrong # of entries in index sqlite_autoindex_t_2"
 right: "ok"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    upsert_do_update_index_corruption::test_upsert_do_update_failure_preserves_indexes

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 979 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/upsert_do_update_index_corruption.rs`
- `tests/integration/mod.rs`