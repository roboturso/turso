# Reproducer: #7952 — Checkpoint backfill not crash-atomic under default durability

https://github.com/tursodatabase/turso/issues/7952

## Environment

- Commit: 7afaf5afd7a845f639d0d54ad73147177940030e
- Platform: linux x86_64
- Reproduced at: 2026-07-29T10:25:11Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests checkpoint_crash_atomicity
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: under synchronous=NORMAL a simulated power loss during checkpoint backfill recovers a torn database (neither the pre-transaction state nor the committed transaction), while the synchronous=FULL control at the identical crash point heals via WAL replay — checkpoint_inner still lacks the WAL fsync before backfill that its own comment promises.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-e55ee224bbab46b8)

running 2 tests
test checkpoint_crash_atomicity::checkpoint_backfill_crash_under_synchronous_normal_recovers_committed_prefix ... FAILED
test checkpoint_crash_atomicity::checkpoint_backfill_crash_under_synchronous_full_control ... ok

failures:

---- checkpoint_crash_atomicity::checkpoint_backfill_crash_under_synchronous_normal_recovers_committed_prefix stdout ----

thread 'checkpoint_crash_atomicity::checkpoint_backfill_crash_under_synchronous_normal_recovers_committed_prefix' panicked at tests/integration/checkpoint_crash_atomicity.rs:396:5:
assertion `left == right` failed: recovered database failed integrity_check
  left: ["*** in database main ***\nPage 9 (Normal) cell 30 has rowid=216 in wrong order. Parent cell has parent_rowid=211 and next_rowid=211\nPage 9 (Normal) cell 29 has rowid=215 in wrong order. Parent cell has parent_rowid=211 and next_rowid=216\nPage 9 (Normal) cell 28 has rowid=214 in wrong order. Parent cell has parent_rowid=211 and next_rowid=215\nPage 9 (Normal) cell 27 has rowid=213 in wrong order. Parent cell has parent_rowid=211 and next_rowid=214\nPage 9 (Normal) cell 26 has rowid=212 in wrong order. Parent cell has parent_rowid=211 and next_rowid=213"]
 right: ["ok"]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    checkpoint_crash_atomicity::checkpoint_backfill_crash_under_synchronous_normal_recovers_committed_prefix

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 1008 filtered out; finished in 0.06s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/checkpoint_crash_atomicity.rs`
- `tests/integration/mod.rs`