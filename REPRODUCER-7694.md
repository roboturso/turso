# Reproducer: #7694 — DELETE can fail with missing nullable index entry after overlapping yielded UPDATE

https://github.com/tursodatabase/turso/issues/7694

## Environment

- Commit: d4d6777a2685ab9a3ffc8733329743fdfe000c45
- Platform: linux x86_64
- Reproduced at: 2026-07-01T14:38:39Z

## Reproduce

```sh
rm -f idxdelete-reentrant-update.db* && cargo test -p turso_core --features io_memory_yield --test idxdelete_reentrant_update
```

Fails on the current tree; passes once the bug is fixed.

## What happens

A DELETE that runs re-entrantly during a mid-flight yielded UPDATE on the same NULL-valued unique-index row fails with Corrupt: "IdxDelete: no matching index entry found for key [Null, 322]", so the reproducer test panics.

## Observed failure

RoboTurso ran the command above and observed:

```text
Blocking waiting for file lock on build directory
   Compiling turso_ext v0.7.0-pre.13 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7694/extensions/core)
   Compiling turso_core v0.7.0-pre.13 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7694/core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 23.72s
     Running tests/idxdelete_reentrant_update.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/idxdelete_reentrant_update-32a65797896cc3a4)

running 1 test
test test_reentrant_delete_during_yielded_update_does_not_see_partial_index_state ... FAILED

failures:

---- test_reentrant_delete_during_yielded_update_does_not_see_partial_index_state stdout ----

thread 'test_reentrant_delete_during_yielded_update_does_not_see_partial_index_state' panicked at core/tests/idxdelete_reentrant_update.rs:10:27:
called `Result::unwrap()` on an `Err` value: Corrupt("IdxDelete: no matching index entry found for key [Value(Null), Value(Numeric(Integer(322)))] while seeking")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    test_reentrant_delete_during_yielded_update_does_not_see_partial_index_state

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p turso_core --test idxdelete_reentrant_update`
```

## Files

- `core/tests/idxdelete_reentrant_update.rs`