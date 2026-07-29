# Reproducer: #7993 — Log offset and checksum advance before durable write

https://github.com/tursodatabase/turso/issues/7993

## Environment

- Commit: 7afaf5afd7a845f639d0d54ad73147177940030e
- Platform: linux x86_64
- Reproduced at: 2026-07-29T10:03:05Z

## Reproduce

```sh
cargo test -p turso_core --lib issue_7993
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: `truncate_to_zero` in logical_log.rs reseeds the running CRC before issuing the file truncate and resets the offset before the completion fires, ignoring failures, so under an injected/unconfirmed truncate the in-memory offset and CRC diverge from the durable tail; the added fault-injection test demonstrates this.

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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.19s
     Running unittests lib.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/turso_core-08106d93f71f83d5)

running 2 tests
test mvcc::persistent_storage::issue_7993_truncate_durability_test::issue_7993_failed_truncate_must_not_move_offset_and_crc ... FAILED
test mvcc::persistent_storage::issue_7993_truncate_durability_test::issue_7993_truncate_must_not_move_offset_and_crc_before_completion ... FAILED

failures:

---- mvcc::persistent_storage::issue_7993_truncate_durability_test::issue_7993_failed_truncate_must_not_move_offset_and_crc stdout ----

thread 'mvcc::persistent_storage::issue_7993_truncate_durability_test::issue_7993_failed_truncate_must_not_move_offset_and_crc' panicked at core/mvcc/persistent_storage/issue_7993_truncate_durability_test.rs:166:5:
assertion `left == right` failed: durable-extent offset/running CRC moved past the durable tail even though the truncate write failed
  left: (0, 2489655040)
 right: (108, 3970656038)

---- mvcc::persistent_storage::issue_7993_truncate_durability_test::issue_7993_truncate_must_not_move_offset_and_crc_before_completion stdout ----

thread 'mvcc::persistent_storage::issue_7993_truncate_durability_test::issue_7993_truncate_must_not_move_offset_and_crc_before_completion' panicked at core/mvcc/persistent_storage/issue_7993_truncate_durability_test.rs:143:5:
assertion `left == right` failed: durable-extent offset/running CRC moved past the durable tail before the truncate completion fired
  left: (0, 3953537078)
 right: (108, 2088933233)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    mvcc::persistent_storage::issue_7993_truncate_durability_test::issue_7993_failed_truncate_must_not_move_offset_and_crc
    mvcc::persistent_storage::issue_7993_truncate_durability_test::issue_7993_truncate_must_not_move_offset_and_crc_before_completion

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 2240 filtered out; finished in 0.00s

error: test failed, to rerun pass `-p turso_core --lib`
```

## Files

- `core/mvcc/persistent_storage/issue_7993_truncate_durability_test.rs`
- `core/mvcc/persistent_storage/mod.rs`