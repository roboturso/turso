# Reproducer: #7991 — Pending checksum not cleared after a failed deferred write

https://github.com/tursodatabase/turso/issues/7991

## Environment

- Commit: 7afaf5afd7a845f639d0d54ad73147177940030e
- Platform: linux x86_64
- Reproduced at: 2026-07-29T10:11:07Z

## Reproduce

```sh
cargo test -p turso_core --lib mvcc::persistent_storage::discard_pending_tests
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: `Storage` never overrides the default no-op `discard_pending_log_write`, so the pending running CRC staged by an aborted deferred log write is left in place and can be consumed by a later `advance_offset_after_success`; the new regression test fails on the current tree and passes once the discard clears the pending CRC.

## Observed failure

RoboTurso ran the command above and observed:

```text
warning: unused imports: `PROTO_WIRE_LENGTH_DELIMITED`, `PROTO_WIRE_VARINT`, `ProtoKey`, `ProtoSint64`, and `ProtoVarint`
   --> core/mvcc/persistent_storage/logical_log.rs:262:63
    |
262 |     log_write, LogBufferWrite, LogChunkStream, LogSerializer, ProtoKey, ProtoSint64, ProtoVarint,
    |                                                               ^^^^^^^^  ^^^^^^^^^^^  ^^^^^^^^^^^
263 |     PROTO_WIRE_LENGTH_DELIMITED, PROTO_WIRE_VARINT,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^
    |
    = note: `#[warn(unused_imports)]` on by default

warning: unused import: `crate::translate::collate::CollationSeq`
  --> core/vdbe/mod.rs:43:9
   |
43 | pub use crate::translate::collate::CollationSeq;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `turso_core` (lib test) generated 2 warnings (run `cargo fix --lib -p turso_core --tests` to apply 2 suggestions)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running unittests lib.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/turso_core-08106d93f71f83d5)

running 1 test
test mvcc::persistent_storage::discard_pending_tests::discard_pending_log_write_clears_staged_pending_crc ... FAILED

failures:

---- mvcc::persistent_storage::discard_pending_tests::discard_pending_log_write_clears_staged_pending_crc stdout ----

thread 'mvcc::persistent_storage::discard_pending_tests::discard_pending_log_write_clears_staged_pending_crc' panicked at core/mvcc/persistent_storage/discard_pending_tests.rs:51:5:
discard_pending_log_write left the staged pending running CRC in place: the success path consumed a CRC staged for an aborted deferred write
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    mvcc::persistent_storage::discard_pending_tests::discard_pending_log_write_clears_staged_pending_crc

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2240 filtered out; finished in 0.00s

error: test failed, to rerun pass `-p turso_core --lib`
```

## Files

- `core/mvcc/persistent_storage/discard_pending_tests.rs`
- `core/mvcc/persistent_storage/mod.rs`