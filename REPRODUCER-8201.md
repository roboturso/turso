# Reproducer: #8201 — ORDER BY before compound operator reports generic syntax error instead of SQLite's message

https://github.com/tursodatabase/turso/issues/8201

## Environment

- Commit: 2bdeb831796f62b4ff2f8393f93ddc1a17ebba50
- Platform: linux x86_64
- Reproduced at: 2026-08-05T07:14:48Z

## Reproduce

```sh
cd sqlite/conformance && cargo run --manifest-path ../../testing/sqltest/Cargo.toml --bin sqltest -q -- run sqlite-sqltests/compound-select-order-by-before-operator-error.sqltest --backend rust
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: Turso reports a generic `near "EXCEPT": syntax error` for ORDER BY before a compound operator, while SQLite reports `ORDER BY clause should come after EXCEPT not before` (same for UNION and INTERSECT); added a failing regression sqltest.

## Observed failure

RoboTurso ran the command above and observed:

```text
warning: unused import: `crate::translate::collate::CollationSeq`
  --> core/vdbe/mod.rs:43:9
   |
43 | pub use crate::translate::collate::CollationSeq;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

[1msqlite-sqltests/compound-select-order-by-before-operator-error.sqltest[0m
  [[31mFAIL[0m] order-by-before-intersect-error-message  [2m(5.76ms)[0m
  [[31mFAIL[0m] order-by-before-union-error-message      [2m(5.77ms)[0m
  [[31mFAIL[0m] order-by-before-except-error-message     [2m(5.80ms)[0m

[1m[31mFailures:[39m[0m

[31m── order-by-before-intersect-error-message (sqlite-sqltests/compound-select-order-by-before-operator-error.sqltest) - :memory:[39m
   error message 'Parse error: near "INTERSECT": syntax error' does not contain expected pattern 'ORDER BY clause should come after INTERSECT not before'

[31m── order-by-before-union-error-message (sqlite-sqltests/compound-select-order-by-before-operator-error.sqltest) - :memory:[39m
   error message 'Parse error: near "UNION": syntax error' does not contain expected pattern 'ORDER BY clause should come after UNION not before'

[31m── order-by-before-except-error-message (sqlite-sqltests/compound-select-order-by-before-operator-error.sqltest) - :memory:[39m
   error message 'Parse error: near "EXCEPT": syntax error' does not contain expected pattern 'ORDER BY clause should come after EXCEPT not before'

[1mSummary:[0m
  [31m3 failed[39m
  [2mTotal time: 6.00ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/sqlite-sqltests/compound-select-order-by-before-operator-error.sqltest`