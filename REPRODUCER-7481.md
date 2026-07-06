# Reproducer: #7481 — MVCC: CREATE INDEX can miss an already committed row before committed watermark is published

https://github.com/tursodatabase/turso/issues/7481

## Environment

- Commit: a01b2476cc2909208164dcf25488f35c18475827
- Platform: linux x86_64
- Reproduced at: 2026-07-06T12:29:19Z

## Reproduce

```sh
cargo test -p turso_core --lib test_create_index_exclusive_acquire_rechecks_committed_tx_before_watermark_publish
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: with a concurrent writer's commit paused between being marked Committed and publishing the committed watermark, an older transaction's CREATE INDEX succeeds (Ok instead of Busy), building the index from a stale snapshot that misses the committed row.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running unittests lib.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/turso_core-9f318bf332f0c513)

running 1 test
test mvcc::database::tests::test_create_index_exclusive_acquire_rechecks_committed_tx_before_watermark_publish ... FAILED

failures:

---- mvcc::database::tests::test_create_index_exclusive_acquire_rechecks_committed_tx_before_watermark_publish stdout ----
path: /tmp/.tmpvq7Pwo/test_5696911703715041964

thread 'mvcc::database::tests::test_create_index_exclusive_acquire_rechecks_committed_tx_before_watermark_publish' panicked at core/mvcc/database/tests.rs:16897:5:
DDL transaction older than an already-committed writer should return Busy, got Ok(())
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    mvcc::database::tests::test_create_index_exclusive_acquire_rechecks_committed_tx_before_watermark_publish

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2048 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p turso_core --lib`
```

## Files

- `core/mvcc/database/tests.rs`