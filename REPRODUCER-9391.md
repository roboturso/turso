# Reproducer: #9391 — Aborted transaction stays readable after COMMIT fails — and on v0.8.0-pre.13 the never-committed rows persist (experimental=views)

https://github.com/tursodatabase/turso/issues/9391

## Environment

- Commit: 632b58d522c73dd6b6ca34b3ecfde45b079651bd
- Platform: linux x86_64
- Reproduced at: 2026-09-28T06:19:42Z

## Reproduce

```sh
cd tests && cargo test --test integration_tests issue_9391_aborted_commit_views
```

Fails on the current tree; passes once the bug is fixed.

## What happens

With views enabled, after COMMIT fails with 'cannot commit - no transaction is active' at 29,000 rows, the same connection still reads all 29,000 rows even though only 28,000 committed.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-444bfbe90304a55b)

running 1 test
test issue_9391_aborted_commit_views::failed_commit_with_materialized_view_leaves_no_rows_behind ... FAILED

failures:

---- issue_9391_aborted_commit_views::failed_commit_with_materialized_view_leaves_no_rows_behind stdout ----

thread 'issue_9391_aborted_commit_views::failed_commit_with_materialized_view_leaves_no_rows_behind' panicked at tests/integration/issue_9391_aborted_commit_views.rs:64:5:
assertion `left == right` failed: rows from the transaction whose COMMIT failed (Transaction error: cannot commit - no transaction is active) are still visible (same connection, fresh connection)
  left: (29000, 28000)
 right: (28000, 28000)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    issue_9391_aborted_commit_views::failed_commit_with_materialized_view_leaves_no_rows_behind

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1158 filtered out; finished in 15.58s

error: test failed, to rerun pass `--test integration_tests`
```

## Files

- `tests/integration/issue_9391_aborted_commit_views.rs`
- `tests/integration/mod.rs`