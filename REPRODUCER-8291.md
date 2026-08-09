# Reproducer: #8291 — panic: "end_write_tx called while write lock not held according to connection state"

https://github.com/tursodatabase/turso/issues/8291

## Environment

- Commit: 964faddd16b20ed7d5c090a879a138dbabeee508
- Platform: linux x86_64
- Reproduced at: 2026-08-09T08:18:16Z

## Reproduce

```sh
cargo test -p turso_whopper --test regression_test_issue_8291
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Replaying seed 16095637479200742728 in btree-rebalance mode on the current tree deterministically hits the panic at core/storage/wal.rs:3450 (end_write_tx called while write lock not held); debug builds swallow it via a catch_unwind in Statement::reset_best_effort, so the regression test detects it with a panic hook.

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

warning: `turso_core` (lib) generated 1 warning (run `cargo fix --lib -p turso_core` to apply 1 suggestion)
   Compiling turso_whopper v0.8.0-pre.3 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8291/testing/concurrent-simulator)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.58s
     Running regression_test_issue_8291.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/regression_test_issue_8291-d1c3eb6f19607875)

running 1 test
test btree_rebalance_seed_16095637479200742728_does_not_panic ... FAILED

failures:

---- btree_rebalance_seed_16095637479200742728_does_not_panic stdout ----

thread 'btree_rebalance_seed_16095637479200742728_does_not_panic' panicked at core/storage/wal.rs:3450:9:
end_write_tx called while write lock not held according to connection state
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

thread 'btree_rebalance_seed_16095637479200742728_does_not_panic' panicked at testing/concurrent-simulator/regression_test_issue_8291.rs:80:5:
a panic fired during the run (see output above): issue #8291


failures:
    btree_rebalance_seed_16095637479200742728_does_not_panic

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 56.76s

error: test failed, to rerun pass `-p turso_whopper --test regression_test_issue_8291`
```

## Files

- `testing/concurrent-simulator/regression_test_issue_8291.rs`
- `testing/concurrent-simulator/Cargo.toml`