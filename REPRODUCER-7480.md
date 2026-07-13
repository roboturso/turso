# Reproducer: #7480 — MVCC: checkpoint can truncate logical log before WAL and panic with  Corrupt("WAL has committed frames but logical log header is missing")

https://github.com/tursodatabase/turso/issues/7480

## Environment

- Commit: c390087d4bfb1350c66b404769e2c47a92a2b426
- Platform: linux x86_64
- Reproduced at: 2026-07-13T07:52:22Z

## Reproduce

```sh
cargo test -p turso_core --lib mvcc::database::issue_7480_tests
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: a simulated crash between the checkpoint's logical-log truncation and WAL truncation leaves committed WAL frames with a zero-byte log, and reopening then fails closed with Corrupt("WAL has committed frames but logical log header is missing") instead of completing the interrupted checkpoint.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.19s
     Running unittests lib.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/turso_core-8195e4f22bfd8dbf)

running 1 test
test mvcc::database::issue_7480_tests::test_checkpoint_crash_after_log_truncate_before_wal_truncate_recovers ... FAILED

failures:

---- mvcc::database::issue_7480_tests::test_checkpoint_crash_after_log_truncate_before_wal_truncate_recovers stdout ----
path: /tmp/.tmpxuYbWS/test_4035343825355403972

thread 'mvcc::database::issue_7480_tests::test_checkpoint_crash_after_log_truncate_before_wal_truncate_recovers' panicked at core/mvcc/database/issue_7480_tests.rs:93:25:
reopening after a crash between logical-log truncate and WAL truncate should succeed: Corrupt("WAL has committed frames but logical log header is missing")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    mvcc::database::issue_7480_tests::test_checkpoint_crash_after_log_truncate_before_wal_truncate_recovers

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2102 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p turso_core --lib`
```

## Files

- `core/mvcc/database/issue_7480_tests.rs`
- `core/mvcc/database/mod.rs`