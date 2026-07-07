# Reproducer: #7483 — MVCC: dropping PRAGMA journal_mode=WAL during checkpoint leaves database permanently Busy

https://github.com/tursodatabase/turso/issues/7483

## Environment

- Commit: 225388ab65ac31ce4ff348e5d4f1d1855425ff6c
- Platform: linux x86_64
- Reproduced at: 2026-07-07T07:42:15Z

## Reproduce

```sh
cargo test -p turso_core --lib mvcc::database::issue_7483_tests::dropped_journal_mode_mvcc_checkpoint_releases_lock -- --exact
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: abandoning a PRAGMA journal_mode=WAL statement while the MVCC checkpoint state machine is suspended at AfterDurableBoundaryAdvanced leaves the checkpoint lock held, so all other connections get permanent Busy on INSERT/SELECT/checkpoint.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running unittests lib.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/turso_core-9f318bf332f0c513)

running 1 test
test mvcc::database::issue_7483_tests::dropped_journal_mode_mvcc_checkpoint_releases_lock ... FAILED

failures:

---- mvcc::database::issue_7483_tests::dropped_journal_mode_mvcc_checkpoint_releases_lock stdout ----
path: /tmp/.tmpzL7e9l/test_3605173373532161340

thread 'mvcc::database::issue_7483_tests::dropped_journal_mode_mvcc_checkpoint_releases_lock' panicked at core/mvcc/database/issue_7483_tests.rs:85:10:
insert after abandoned journal_mode checkpoint must not be Busy: Busy
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    mvcc::database::issue_7483_tests::dropped_journal_mode_mvcc_checkpoint_releases_lock

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2052 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p turso_core --lib`
```

## Files

- `core/mvcc/database/issue_7483_tests.rs`
- `core/mvcc/database/mod.rs`