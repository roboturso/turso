# Reproducer: #8195 — WAL frame lookup panics instead of returning not-found

https://github.com/tursodatabase/turso/issues/8195

## Environment

- Commit: 43c3eb631e1490c294f0f019f56a4d9439b29fd7
- Platform: linux x86_64
- Reproduced at: 2026-08-07T14:51:16Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests wal_watermark_below_backfill_floor_does_not_panic
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: after a passive checkpoint, a WAL page lookup with a watermark below the backfill floor hits the turso_assert at core/storage/wal.rs:3498 and panics instead of returning not-found; reproduced via Connection::try_wal_watermark_read_page in a new integration test.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 21.39s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-17ac5015bf27e00b)

running 1 test
test functions::test_wal_watermark_below_backfill::wal_watermark_below_backfill_floor_does_not_panic ... FAILED

failures:

---- functions::test_wal_watermark_below_backfill::wal_watermark_below_backfill_floor_does_not_panic stdout ----

thread 'functions::test_wal_watermark_below_backfill::wal_watermark_below_backfill_floor_does_not_panic' panicked at core/storage/wal.rs:3498:9:
frame_watermark must be >= than current WAL backfill amount | frame_watermark=Some(30), nbackfills=52
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    functions::test_wal_watermark_below_backfill::wal_watermark_below_backfill_floor_does_not_panic

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1033 filtered out; finished in 0.02s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/functions/test_wal_watermark_below_backfill.rs`
- `tests/integration/functions/mod.rs`