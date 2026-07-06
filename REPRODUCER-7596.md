# Reproducer: #7596 — Nonblocking PRAGMA journal_mode=mvcc can write MVCC header before bootstrap and corrupt reopen

https://github.com/tursodatabase/turso/issues/7596

## Environment

- Commit: d4d6777a2685ab9a3ffc8733329743fdfe000c45
- Platform: linux x86_64
- Reproduced at: 2026-07-06T11:36:25Z

## Reproduce

```sh
cargo test -p turso_core --features io_memory_yield --test active_index_select_repro -- --nocapture
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Reproduced on current main: interrupting PRAGMA journal_mode=mvcc after its 6th I/O yield (via same-connection insert or statement drop) leaves page 1 marked MVCC without a logical-log header, and reopen fails with Corrupt("WAL has committed frames but logical log header is missing").

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running tests/active_index_select_repro.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/active_index_select_repro-9bee4cfcbcbbbc8e)

running 2 tests

thread 'active_journal_mode_mvcc_interleaved_insert_does_not_corrupt_reopen' panicked at core/tests/active_index_select_repro.rs:29:33:
called `Result::unwrap()` on an `Err` value: Corrupt("WAL has committed frames but logical log header is missing")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test active_journal_mode_mvcc_interleaved_insert_does_not_corrupt_reopen ... FAILED

thread 'writes_after_abandoned_journal_mode_mvcc_survive_fresh_reopen' panicked at core/tests/active_index_select_repro.rs:121:6:
called `Result::unwrap()` on an `Err` value: Corrupt("WAL has committed frames but logical log header is missing")
test writes_after_abandoned_journal_mode_mvcc_survive_fresh_reopen ... FAILED

failures:

failures:
    active_journal_mode_mvcc_interleaved_insert_does_not_corrupt_reopen
    writes_after_abandoned_journal_mode_mvcc_survive_fresh_reopen

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p turso_core --test active_index_select_repro`
```

## Files

- `core/tests/active_index_select_repro.rs`