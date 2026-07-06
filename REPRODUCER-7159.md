# Reproducer: #7159 — Slow Commit & Corrupt database: Missing MVCC metadata table while logical log state exists

https://github.com/tursodatabase/turso/issues/7159

## Environment

- Commit: d4d6777a2685ab9a3ffc8733329743fdfe000c45
- Platform: linux x86_64
- Reproduced at: 2026-07-06T10:40:07Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests test_mvcc_logical_log_bounded_under_concurrent_commits
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Reproduced the slow commit: under sustained BEGIN CONCURRENT load the commit-time auto-checkpoint always fails Busy (every transaction read-holds blocking_checkpoint_lock) and the error is swallowed, so the logical log grows unbounded past the 4MB threshold and the last commit at shutdown checkpoints the whole log inline (28s at 5M rows, extrapolating to the reported 560s at 1.4GB); the separate 'Missing MVCC metadata table' open error no longer manifests on current main, where recovery after SIGKILL with a 213MB log succeeds.

## Observed failure

RoboTurso ran the command above and observed:

```text
Compiling core_tester v0.7.0-pre.13 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7159/tests)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 10.35s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-2086b07511e41b85)

running 1 test
test mvcc::test_mvcc_logical_log_bounded_under_concurrent_commits ... FAILED

failures:

---- mvcc::test_mvcc_logical_log_bounded_under_concurrent_commits stdout ----

thread 'mvcc::test_mvcc_logical_log_bounded_under_concurrent_commits' panicked at tests/integration/mvcc.rs:1503:5:
MVCC auto-checkpoint starved under concurrent load: logical log grew to 2634112 bytes despite a checkpoint threshold of 65536 bytes (allowed at most 1310720). Commits pay for the whole accumulated log at once when load drops (issue #7159).
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    mvcc::test_mvcc_logical_log_bounded_under_concurrent_commits

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 962 filtered out; finished in 6.00s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/mvcc.rs`