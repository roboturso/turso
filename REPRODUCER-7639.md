# Reproducer: #7639 — MVCC abandoned chunked DELETE commit can checkpoint/reopen into table/index corruption

https://github.com/tursodatabase/turso/issues/7639

## Environment

- Commit: d4d6777a2685ab9a3ffc8733329743fdfe000c45
- Platform: linux x86_64
- Reproduced at: 2026-06-26T15:41:04Z

## Reproduce

```sh
cargo test -p turso_core --lib abandoned_chunked_delete_with_two_indexes_must_not_corrupt_after_checkpoint_reopen
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: abandoning a chunked DELETE under MVCC commits partial work that survives checkpoint and reopen, leaving the unique index corrupt — PRAGMA integrity_check reports 'row 1 missing from index sqlite_autoindex_t_2'.

## Observed failure

RoboTurso ran the command above and observed:

```text
Compiling turso_core v0.7.0-pre.13 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7639/core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 13.26s
     Running unittests lib.rs (target/debug/deps/turso_core-34fdf59f579c350e)

running 1 test
test mvcc::database::tests::abandoned_chunked_delete_with_two_indexes_must_not_corrupt_after_checkpoint_reopen ... FAILED

failures:

---- mvcc::database::tests::abandoned_chunked_delete_with_two_indexes_must_not_corrupt_after_checkpoint_reopen stdout ----

thread 'mvcc::database::tests::abandoned_chunked_delete_with_two_indexes_must_not_corrupt_after_checkpoint_reopen' panicked at core/mvcc/database/tests.rs:16863:5:
assertion `left == right` failed
  left: "row 1 missing from index sqlite_autoindex_t_2"
 right: "ok"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    mvcc::database::tests::abandoned_chunked_delete_with_two_indexes_must_not_corrupt_after_checkpoint_reopen

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2034 filtered out; finished in 0.71s

error: test failed, to rerun pass `-p turso_core --lib`
```

## Files

- `core/mvcc/database/tests.rs`