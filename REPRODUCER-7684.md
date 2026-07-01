# Reproducer: #7684 — Abandoned CREATE INDEX can poison freelist state and panic during later page allocation

https://github.com/tursodatabase/turso/issues/7684

## Environment

- Commit: d4d6777a2685ab9a3ffc8733329743fdfe000c45
- Platform: linux x86_64
- Reproduced at: 2026-07-01T08:41:47Z

## Reproduce

```sh
cargo test -p core_tester --features io_memory_yield --test integration_tests test_abandoned_create_index_does_not_poison_later_allocation
```

Fails on the current tree; passes once the bug is fixed.

## What happens

An abandoned CREATE INDEX inside a transaction leaves freelist state poisoned, so a later page allocation panics with "Freelist leaf page has overflow cells" at core/storage/pager.rs:5266.

## Observed failure

RoboTurso ran the command above and observed:

```text
Blocking waiting for file lock on build directory
    Finished `test` profile [unoptimized + debuginfo] target(s) in 19.62s
     Running integration/mod.rs (target/debug/deps/integration_tests-d2a0c7ad5c96c94b)

running 1 test
test abandoned_create_index::test_abandoned_create_index_does_not_poison_later_allocation ... FAILED

failures:

---- abandoned_create_index::test_abandoned_create_index_does_not_poison_later_allocation stdout ----

thread 'abandoned_create_index::test_abandoned_create_index_does_not_poison_later_allocation' panicked at core/storage/pager.rs:5266:21:
Freelist leaf page has overflow cells | page_id=47
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    abandoned_create_index::test_abandoned_create_index_does_not_poison_later_allocation

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 962 filtered out; finished in 0.03s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/abandoned_create_index.rs`
- `tests/integration/mod.rs`
- `tests/Cargo.toml`