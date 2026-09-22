# Reproducer: #6860 — UPSERT OR ROLLBACK Rolls Back the Whole Transaction Instead of Only the Failed DO UPDATE Statement

https://github.com/tursodatabase/turso/issues/6860

## Environment

- Commit: a012590f552751754ea18a4226432664e06055ea
- Platform: linux x86_64
- Reproduced at: 2026-09-22T18:19:40Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests upsert_or_rollback_do_update_failure_keeps_transaction
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: after the UPSERT's DO UPDATE fails on the UNIQUE constraint, Turso rolls back the whole explicit transaction (count 0) while SQLite only aborts that statement and keeps row id=3 (count 1), because the INSERT's OR ROLLBACK policy is wrongly applied to the DO UPDATE branch.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.29s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-e1bb149c2239ec41)

running 1 test
test upsert_or_rollback_keeps_transaction::upsert_or_rollback_do_update_failure_keeps_transaction ... FAILED

failures:

---- upsert_or_rollback_keeps_transaction::upsert_or_rollback_do_update_failure_keeps_transaction stdout ----

thread 'upsert_or_rollback_keeps_transaction::upsert_or_rollback_do_update_failure_keeps_transaction' panicked at tests/integration/upsert_or_rollback_keeps_transaction.rs:33:5:
assertion `left == right` failed: row inserted earlier in the transaction must still be visible
  left: [[Integer(0)]]
 right: [[Integer(1)]]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    upsert_or_rollback_keeps_transaction::upsert_or_rollback_do_update_failure_keeps_transaction

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1138 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/upsert_or_rollback_keeps_transaction.rs`
- `tests/integration/mod.rs`