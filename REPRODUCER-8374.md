# Reproducer: #8374 — Collation-opaque expression wrappers on a compound-SELECT arm lose the BINARY default (silent wrong rows)

https://github.com/tursodatabase/turso/issues/8374

## Environment

- Commit: 93fe3cebbf1d7d69e5052d3339ffd5b67be2b795
- Platform: linux x86_64
- Reproduced at: 2026-09-24T12:49:37Z

## Reproduce

```sh
cargo run -q --manifest-path testing/sqltest/Cargo.toml --bin sqltest -- run sqlite/conformance/sqlite-sqltests/compound-select-opaque-expression-collation.sqltest --backend rust
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Wrapping a COLLATE NOCASE column in `||` or a scalar function on a compound SELECT arm makes turso remove UNION/INTERSECT/EXCEPT duplicates under NOCASE instead of BINARY, so it returns different rows than SQLite.

## Observed failure

RoboTurso ran the command above and observed:

```text
[1msqlite/conformance/sqlite-sqltests/compound-select-opaque-expression-collation.sqltest[0m
  [[31mFAIL[0m] intersect-concat-uses-binary             [2m(9.75ms)[0m
  [[31mFAIL[0m] except-concat-uses-binary                [2m(9.81ms)[0m
  [[32mPASS[0m] union-cast-keeps-column-collation        [2m(9.83ms)[0m
  [[31mFAIL[0m] union-concat-uses-binary                 [2m(10.02ms)[0m
  [[31mFAIL[0m] union-function-call-uses-binary          [2m(10.17ms)[0m

[1m[31mFailures:[39m[0m

[31m── intersect-concat-uses-binary (sqlite/conformance/sqlite-sqltests/compound-select-opaque-expression-collation.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -0
   +2

[31m── except-concat-uses-binary (sqlite/conformance/sqlite-sqltests/compound-select-opaque-expression-collation.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -A
   -b

[31m── union-concat-uses-binary (sqlite/conformance/sqlite-sqltests/compound-select-opaque-expression-collation.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -A
   -B
    a
   -b
   +B

[31m── union-function-call-uses-binary (sqlite/conformance/sqlite-sqltests/compound-select-opaque-expression-collation.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -A
   -B
    a
   -b
   +B

[1mSummary:[0m
  [32m1 passed[39m, [31m4 failed[39m
  [2mTotal time: 10.43ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/sqlite-sqltests/compound-select-opaque-expression-collation.sqltest`