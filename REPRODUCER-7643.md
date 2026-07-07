# Reproducer: #7643 — MVCC abandoned auto-checkpoint after CREATE TABLE leaks stale root mapping and short reads

https://github.com/tursodatabase/turso/issues/7643

## Environment

- Commit: ed5032f6723a3123e9492c958f6e01c9fa8d8351
- Platform: linux x86_64
- Reproduced at: 2026-07-07T06:52:51Z

## Reproduce

```sh
cargo test -p turso_core --features io_memory_yield --test mvcc_abandoned_create_auto_checkpoint_rootmap_repro
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed on the current tree: abandoning the auto-checkpoint mid-CREATE TABLE in MVCC mode (abandon_after_io=3) leaves a stale root mapping, so a subsequent SELECT fails with ShortRead { page_idx: 3, expected: 4096, actual: 0 }.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running tests/mvcc_abandoned_create_auto_checkpoint_rootmap_repro.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/mvcc_abandoned_create_auto_checkpoint_rootmap_repro-711e5c1da6cb4e1c)

running 1 test
test abandoned_auto_checkpoint_create_table_leaks_root_mapping ... FAILED

failures:

---- abandoned_auto_checkpoint_create_table_leaks_root_mapping stdout ----

thread 'abandoned_auto_checkpoint_create_table_leaks_root_mapping' panicked at core/tests/mvcc_abandoned_create_auto_checkpoint_rootmap_repro.rs:91:25:
abandon_after_io=3: public read failed after abandoned auto-checkpoint: CompletionError(ShortRead { page_idx: 3, expected: 4096, actual: 0 })
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    abandoned_auto_checkpoint_create_table_leaks_root_mapping

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

error: test failed, to rerun pass `-p turso_core --test mvcc_abandoned_create_auto_checkpoint_rootmap_repro`
```

## Files

- `core/tests/mvcc_abandoned_create_auto_checkpoint_rootmap_repro.rs`