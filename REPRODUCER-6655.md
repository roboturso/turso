# Reproducer: #6655 — Stack overflow in arithmetic

https://github.com/tursodatabase/turso/issues/6655

## Environment

- Commit: b7b5d4765f56c1c8bd3fe7ee6d3f311b3b6f1aa4
- Platform: linux x86_64
- Reproduced at: 2026-07-28T05:27:28Z

## Reproduce

```sh
cargo test -p core_tester --test fuzz_tests issue_6655
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Reproduced with SEED=1777759667094: the recursive-descent parser (parse_expr_inner/parse_expr_operand in sqlite/parser/src/parser.rs) overflows the 2 MiB test-thread stack on a modestly nested unary-operator expression like `SELECT (+ + - - - ~ (- ((~ + ~ ~ - - ~ -4 << -3))))`, confirmed via gdb backtrace; a minimized regression test is added in tests/fuzz/issue_6655.rs.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.76s
     Running fuzz/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/fuzz_tests-147509cefa71c138)

running 2 tests

thread 'issue_6655::issue_6655_tests::arithmetic_unary_chain_no_stack_overflow' has overflowed its stack
fatal runtime error: stack overflow, aborting
error: test failed, to rerun pass `-p core_tester --test fuzz_tests`

Caused by:
  process didn't exit successfully: `/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/fuzz_tests-147509cefa71c138 issue_6655` (signal: 6, SIGABRT: process abort signal)
```

## Files

- `tests/fuzz/issue_6655.rs`
- `tests/fuzz/mod.rs`