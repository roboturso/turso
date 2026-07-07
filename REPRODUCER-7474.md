# Reproducer: #7474 — MVCC treats plain rowid allocation like AUTOINCREMENT after rollback

https://github.com/tursodatabase/turso/issues/7474

## Environment

- Commit: 225388ab65ac31ce4ff348e5d4f1d1855425ff6c
- Platform: linux x86_64
- Reproduced at: 2026-07-07T07:56:30Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests test_mvcc_rowid_not_sticky_after_rollback
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: in MVCC mode a rolled-back INSERT permanently bumps the next rowid on a table without AUTOINCREMENT, so the subsequent insert gets rowid 2 instead of 1 (SQLite and Turso's non-MVCC mode both return 1).

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.42s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-a796116438746cf7)

running 1 test
test issue_7474_mvcc_rowid_rollback::test_mvcc_rowid_not_sticky_after_rollback ... FAILED

failures:

---- issue_7474_mvcc_rowid_rollback::test_mvcc_rowid_not_sticky_after_rollback stdout ----

thread 'issue_7474_mvcc_rowid_rollback::test_mvcc_rowid_not_sticky_after_rollback' panicked at tests/integration/issue_7474_mvcc_rowid_rollback.rs:26:5:
assertion `left == right` failed: rowid allocation without AUTOINCREMENT must restart from max(rowid)+1 after a rolled-back insert; got [(2,)]
  left: [(2,)]
 right: [(1,)]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    issue_7474_mvcc_rowid_rollback::test_mvcc_rowid_not_sticky_after_rollback

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 966 filtered out; finished in 0.02s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/issue_7474_mvcc_rowid_rollback.rs`
- `tests/integration/mod.rs`