# Reproducer: #8536 — multiprocess_wal: empty WAL + leftover -tshm fails open with short read on WAL frame

https://github.com/tursodatabase/turso/issues/8536

## Environment

- Commit: ca8933a2e32b74edf6e24650c4f6ace9f0ad6f3c
- Platform: linux x86_64
- Reproduced at: 2026-09-22T10:34:39Z

## Reproduce

```sh
cargo test -p turso_core --lib multiprocess_empty_wal_stale_tshm_tests::multiprocess_open_recovers_when_wal_is_empty_but_tshm_claims_frames -- --exact
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed on the current tree: after a multiprocess_wal writer exits leaving a populated -tshm, truncating the -wal to 0 bytes makes the next open fail with ShortReadWalFrame { offset: 3773952, expected: 4096, actual: 0 } because the stale -tshm frame index is still used even though the disk scan finds no WAL frames; a regression test that fails now and passes once open recovers is added.

## Observed failure

RoboTurso ran the command above and observed:

```text
warning: unused import: `HashMap`
 --> core/vdbe/statement_lifecycle_tests.rs:2:24
  |
2 | use std::collections::{HashMap, HashSet};
  |                        ^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `turso_core` (lib test) generated 1 warning (run `cargo fix --lib -p turso_core --tests` to apply 1 suggestion)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.21s
     Running unittests lib.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/turso_core-e97da60813f710c0)

running 1 test
test multiprocess_empty_wal_stale_tshm_tests::multiprocess_open_recovers_when_wal_is_empty_but_tshm_claims_frames ... FAILED

failures:

---- multiprocess_empty_wal_stale_tshm_tests::multiprocess_open_recovers_when_wal_is_empty_but_tshm_claims_frames stdout ----

thread 'multiprocess_empty_wal_stale_tshm_tests::multiprocess_open_recovers_when_wal_is_empty_but_tshm_claims_frames' panicked at core/multiprocess_empty_wal_stale_tshm_tests.rs:54:10:
open must succeed when the WAL is empty even if -tshm still claims frames: CompletionError(ShortReadWalFrame { offset: 3773952, expected: 4096, actual: 0 })
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    multiprocess_empty_wal_stale_tshm_tests::multiprocess_open_recovers_when_wal_is_empty_but_tshm_claims_frames

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2497 filtered out; finished in 0.67s

error: test failed, to rerun pass `-p turso_core --lib`
```

## Files

- `core/multiprocess_empty_wal_stale_tshm_tests.rs`
- `core/lib.rs`