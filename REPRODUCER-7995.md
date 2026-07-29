# Reproducer: #7995 — Never-checkpointed WAL discarded on crash → whole-epoch committed-data loss (integrity_check falsely reports OK)

https://github.com/tursodatabase/turso/issues/7995

## Environment

- Commit: 7afaf5afd7a845f639d0d54ad73147177940030e
- Platform: linux x86_64
- Reproduced at: 2026-07-29T09:52:12Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests test_committed_wal_survives_power_loss_before_first_checkpoint
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: turso never fsyncs the main .db before the first checkpoint, so a power loss leaves a 0-byte .db plus a fully fsync'd WAL that recovery then discards on reopen — all committed synchronous=FULL transactions are lost while integrity_check reports ok (SQLite avoids this by fsyncing the db header at creation).

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.21s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-e55ee224bbab46b8)

running 1 test
test wal::test_power_loss_before_first_checkpoint::test_committed_wal_survives_power_loss_before_first_checkpoint ... FAILED

failures:

---- wal::test_power_loss_before_first_checkpoint::test_committed_wal_survives_power_loss_before_first_checkpoint stdout ----

thread 'wal::test_power_loss_before_first_checkpoint::test_committed_wal_survives_power_loss_before_first_checkpoint' panicked at tests/integration/common.rs:406:40:
called `Result::unwrap()` on an `Err` value: ParseError("no such table: t")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    wal::test_power_loss_before_first_checkpoint::test_committed_wal_survives_power_loss_before_first_checkpoint

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1008 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/wal/test_power_loss_before_first_checkpoint.rs`
- `tests/integration/wal/mod.rs`