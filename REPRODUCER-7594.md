# Reproducer: #7594 — Abandoned non-aborting indexed UPDATE can commit table/index corruption

https://github.com/tursodatabase/turso/issues/7594

## Environment

- Commit: d4d6777a2685ab9a3ffc8733329743fdfe000c45
- Platform: linux x86_64
- Reproduced at: 2026-07-06T11:38:49Z

## Reproduce

```sh
cargo test -p turso_core --features io_memory_yield --test active_index_select_repro abandoned_indexed_update_without_stmt_journal_does_not_corrupt_index
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Reproduced on the current tree: dropping an in-flight indexed UPDATE at IO boundary 24 and then committing leaves the index corrupted (integrity_check reports 'row 1 missing from index t_b'), because statement subjournaling is gated on is_multi_write && may_abort in core/vdbe/builder.rs and this UPDATE is not flagged may_abort.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running tests/active_index_select_repro.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/active_index_select_repro-9bee4cfcbcbbbc8e)

running 1 test
test abandoned_indexed_update_without_stmt_journal_does_not_corrupt_index ... FAILED

failures:

---- abandoned_indexed_update_without_stmt_journal_does_not_corrupt_index stdout ----

thread 'abandoned_indexed_update_without_stmt_journal_does_not_corrupt_index' panicked at core/tests/active_index_select_repro.rs:92:9:
assertion `left == right` failed: abandon_after_io=24: abandoned indexed UPDATE corrupted the index
  left: ["row 1 missing from index t_b", "wrong # of entries in index t_b"]
 right: ["ok"]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    abandoned_indexed_update_without_stmt_journal_does_not_corrupt_index

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.75s

error: test failed, to rerun pass `-p turso_core --test active_index_select_repro`
```

## Files

- `core/tests/active_index_select_repro.rs`