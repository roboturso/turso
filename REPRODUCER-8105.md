# Reproducer: #8105 — Stack overflow from nested CTEs

https://github.com/tursodatabase/turso/issues/8105

## Environment

- Commit: f41afa754ff55f8b484070dc98c57802afb246d5
- Platform: linux x86_64
- Reproduced at: 2026-07-29T16:28:17Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests deeply_nested_ctes_do_not_smash_the_stack
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Deeply nested CTEs still crash the process with a stack overflow (the 100-level case from the report was mitigated by the expression-depth guard, but the same query at 1000+ levels aborts tursodb while sqlite3 handles it gracefully).

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.21s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-e55ee224bbab46b8)

running 1 test

thread '<unknown>' has overflowed its stack
fatal runtime error: stack overflow, aborting
error: test failed, to rerun pass `-p core_tester --test integration_tests`

Caused by:
  process didn't exit successfully: `/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-e55ee224bbab46b8 deeply_nested_ctes_do_not_smash_the_stack` (signal: 6, SIGABRT: process abort signal)
```

## Files

- `tests/integration/nested_cte_stack_overflow.rs`
- `tests/integration/mod.rs`