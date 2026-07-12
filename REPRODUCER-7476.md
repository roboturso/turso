# Reproducer: #7476 — MVCC: same connection can run statements during yielded COMMIT and misread its own prepared writes

https://github.com/tursodatabase/turso/issues/7476

## Environment

- Commit: d115b414ad83d05a1b3b66063eeefd07a5f51268
- Platform: linux x86_64
- Reproduced at: 2026-07-12T06:59:48Z

## Reproduce

```sh
cargo test -p turso_core --lib mvcc::database::issue_7476_tests
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Reproduced: while a COMMIT of a BEGIN CONCURRENT transaction is suspended at CommitYieldPoint::LogRecordPrepared, a SELECT on the same connection panics with 'a txn cannot read its own row versions during prepare' instead of erroring cleanly or seeing the pending write.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.19s
     Running unittests lib.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/turso_core-63015736d236e26d)

running 1 test
test mvcc::database::issue_7476_tests::mvcc_same_connection_statement_during_yielded_commit ... FAILED

failures:

---- mvcc::database::issue_7476_tests::mvcc_same_connection_statement_during_yielded_commit stdout ----
path: /tmp/.tmpgAGTnH/test_18114904391471547162

thread 'mvcc::database::issue_7476_tests::mvcc_same_connection_statement_during_yielded_commit' panicked at core/mvcc/database/mod.rs:8893:29:
a txn cannot read its own row versions during prepare
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

thread 'mvcc::database::issue_7476_tests::mvcc_same_connection_statement_during_yielded_commit' panicked at core/mvcc/database/issue_7476_tests.rs:85:19:
SELECT during yielded COMMIT panicked instead of returning an error


failures:
    mvcc::database::issue_7476_tests::mvcc_same_connection_statement_during_yielded_commit

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2102 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p turso_core --lib`
```

## Files

- `core/mvcc/database/issue_7476_tests.rs`
- `core/mvcc/database/mod.rs`