# Reproducer: #5071 — Stack overflow in query translator due to unbounded recursion

https://github.com/tursodatabase/turso/issues/5071

## Environment

- Commit: d4d6777a2685ab9a3ffc8733329743fdfe000c45
- Platform: linux x86_64
- Reproduced at: 2026-07-01T09:21:26Z

## Reproduce

```sh
cargo test -p turso_cli --test expr_depth_stack_overflow
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Deeply nested OR chains, scalar subqueries, and CASE expressions crash tursodb with a stack overflow (SIGABRT) due to unbounded recursion in the translator/optimizer, with no equivalent of SQLite's SQLITE_MAX_EXPR_DEPTH guard.

## Observed failure

RoboTurso ran the command above and observed:

```text
Blocking waiting for file lock on build directory
    Finished `test` profile [unoptimized + debuginfo] target(s) in 11.39s
     Running tests/expr_depth_stack_overflow.rs (target/debug/deps/expr_depth_stack_overflow-cc547b6f5bad9a68)

running 3 tests

thread 'main' has overflowed its stack
fatal runtime error: stack overflow, aborting

thread 'main' has overflowed its stack
fatal runtime error: stack overflow, aborting

thread 'main' has overflowed its stack
fatal runtime error: stack overflow, aborting
test deep_nested_subqueries_do_not_stack_overflow ... FAILED
test deep_nested_case_does_not_stack_overflow ... FAILED
test deep_or_chain_does_not_stack_overflow ... FAILED

failures:

---- deep_nested_subqueries_do_not_stack_overflow stdout ----

thread 'deep_nested_subqueries_do_not_stack_overflow' panicked at cli/tests/expr_depth_stack_overflow.rs:50:5:
tursodb was killed by a signal (stack overflow) on deeply nested subqueries: ExitStatus(unix_wait_status(134))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- deep_nested_case_does_not_stack_overflow stdout ----

thread 'deep_nested_case_does_not_stack_overflow' panicked at cli/tests/expr_depth_stack_overflow.rs:66:5:
tursodb was killed by a signal (stack overflow) on deeply nested CASE: ExitStatus(unix_wait_status(134))

---- deep_or_chain_does_not_stack_overflow stdout ----

thread 'deep_or_chain_does_not_stack_overflow' panicked at cli/tests/expr_depth_stack_overflow.rs:34:5:
tursodb was killed by a signal (stack overflow) on a deep OR chain: ExitStatus(unix_wait_status(134))


failures:
    deep_nested_case_does_not_stack_overflow
    deep_nested_subqueries_do_not_stack_overflow
    deep_or_chain_does_not_stack_overflow

test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.04s

error: test failed, to rerun pass `-p turso_cli --test expr_depth_stack_overflow`
```

## Files

- `cli/tests/expr_depth_stack_overflow.rs`