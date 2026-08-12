# Reproducer: #8341 — bindings/js: StepResult::Yield is mapped to STEP_IO — wasm statements hang forever when the engine yields with no I/O in flight

https://github.com/tursodatabase/turso/issues/8341

## Environment

- Commit: a94102c20b4c1c554f7c246606c2ed74db47199c
- Platform: linux x86_64
- Reproduced at: 2026-08-12T15:23:08Z

## Reproduce

```sh
cargo test -p turso_node --features test_helper --lib issue_8341
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: step_sync in bindings/javascript/src/lib.rs still maps StepResult::Yield to STEP_IO, so a yield with no I/O in flight tells the wasm driver to await an I/O notification that is never scheduled; a deterministic regression test using core's yield injector fails on the current tree and passes once Yield gets its own step handling.

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

warning: `turso_core` (lib) generated 2 warnings (run `cargo fix --lib -p turso_core` to apply 2 suggestions)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running unittests src/lib.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/turso_node-0aeae95c2ef0a1cd)

running 1 test
test issue_8341_yield_step_test::yield_with_no_pending_io_is_not_reported_as_step_io ... FAILED

failures:

---- issue_8341_yield_step_test::yield_with_no_pending_io_is_not_reported_as_step_io stdout ----

thread 'issue_8341_yield_step_test::yield_with_no_pending_io_is_not_reported_as_step_io' panicked at bindings/javascript/src/issue_8341_yield_step_test.rs:78:13:
assertion `left != right` failed: StepResult::Yield was mapped to STEP_IO: the wasm step loop would await an I/O completion that was never scheduled and hang forever
  left: 3
 right: 3
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    issue_8341_yield_step_test::yield_with_no_pending_io_is_not_reported_as_step_io

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p turso_node --lib`
```

## Files

- `bindings/javascript/src/issue_8341_yield_step_test.rs`