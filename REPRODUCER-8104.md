# Reproducer: #8104 — Stack overflow on compound selects with many terms

https://github.com/tursodatabase/turso/issues/8104

## Environment

- Commit: c164beeb8af16ce9e04bb612e0727fb860d4030b
- Platform: linux x86_64
- Reproduced at: 2026-07-29T16:20:08Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests compound_select_with_many_terms
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: preparing a compound SELECT with many UNION ALL terms recurses once per term in the code generator (the parser collects terms iteratively) and aborts with a stack overflow; added a regression test that prepares a 50,000-term compound SELECT and currently dies with SIGABRT.

## Observed failure

RoboTurso ran the command above and observed:

```text
Compiling core_tester v0.8.0-pre.2 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8104/tests)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 8.80s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-e55ee224bbab46b8)

running 1 test

thread '<unknown>' has overflowed its stack
fatal runtime error: stack overflow, aborting
error: test failed, to rerun pass `-p core_tester --test integration_tests`

Caused by:
  process didn't exit successfully: `/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-e55ee224bbab46b8 compound_select_with_many_terms` (signal: 6, SIGABRT: process abort signal)
```

## Files

- `tests/integration/compound_select_stack_overflow.rs`
- `tests/integration/mod.rs`