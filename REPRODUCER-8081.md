# Reproducer: #8081 — MVCC: cancelling a published COMMIT lets TRUNCATE discard committed rows

https://github.com/tursodatabase/turso/issues/8081

## Environment

- Commit: 6d01be3604bbd2bc1066c2fef3f8049b98ed4523
- Platform: linux x86_64
- Reproduced at: 2026-09-23T18:13:12Z

## Reproduce

```sh
cargo test -p turso --test issue_8081_cancelled_commit_truncate
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Cancelling a COMMIT after other connections can already see its rows, then running a TRUNCATE checkpoint and reopening, loses all 1500 rows every time (count is 0 after restart).

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running tests/issue_8081_cancelled_commit_truncate.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/issue_8081_cancelled_commit_truncate-4eaf873609e3d0e9)

running 1 test
test test_mvcc_dropped_published_commit_survives_truncate_restart ... FAILED

failures:

---- test_mvcc_dropped_published_commit_survives_truncate_restart stdout ----

thread 'test_mvcc_dropped_published_commit_survives_truncate_restart' panicked at bindings/rust/tests/issue_8081_cancelled_commit_truncate.rs:100:9:
assertion `left == right` failed: a published commit cancelled after scheduler yield 2 must survive TRUNCATE and restart
  left: 0
 right: 1500
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    test_mvcc_dropped_published_commit_survives_truncate_restart

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.13s

error: test failed, to rerun pass `-p turso --test issue_8081_cancelled_commit_truncate`
```

## Files

- `bindings/rust/tests/issue_8081_cancelled_commit_truncate.rs`