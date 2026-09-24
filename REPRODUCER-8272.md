# Reproducer: #8272 — UNION/INTERSECT/EXCEPT return wrong rows: a plain column on the left arm loses its BINARY collation to the right arm

https://github.com/tursodatabase/turso/issues/8272

## Environment

- Commit: c0c6f663109382ab770fe8b98a9622e9ef81ca64
- Platform: linux x86_64
- Reproduced at: 2026-09-24T12:43:23Z

## Reproduce

```sh
cd sqlite/conformance && cargo run -q --manifest-path ../../testing/sqltest/Cargo.toml --bin sqltest -- run sqlite-sqltests/compound-select-left-column-binary-collation.sqltest --backend rust
```

Fails on the current tree; passes once the bug is fixed.

## What happens

When the left side is a plain column with no COLLATE, UNION/INTERSECT/EXCEPT compare rows using the right side's NOCASE collation instead of BINARY, so they return different rows from SQLite (for example, INTERSECT returns an extra 'c').

## Observed failure

RoboTurso ran the command above and observed:

```text
[1msqlite-sqltests/compound-select-left-column-binary-collation.sqltest[0m
  [[31mFAIL[0m] except-uses-left-column-binary-collation [2m(9.68ms)[0m
  [[31mFAIL[0m] union-all-order-by-uses-left-column-binary-collation [2m(9.69ms)[0m
  [[31mFAIL[0m] intersect-uses-left-column-binary-collation [2m(9.80ms)[0m
  [[31mFAIL[0m] union-uses-left-column-binary-collation  [2m(9.83ms)[0m

[1m[31mFailures:[39m[0m

[31m── except-uses-left-column-binary-collation (sqlite-sqltests/compound-select-left-column-binary-collation.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -c
    z

[31m── union-all-order-by-uses-left-column-binary-collation (sqlite-sqltests/compound-select-left-column-binary-collation.sqltest) - :memory:[39m
   --- expected
   +++ actual
   +a
    A
   +a
    B
   +b
    B
   +c
    C
   -a
   -a
   -b
   -c
    z

[31m── intersect-uses-left-column-binary-collation (sqlite-sqltests/compound-select-left-column-binary-collation.sqltest) - :memory:[39m
   --- expected
   +++ actual
   +a
    B
   -a
   +c

[31m── union-uses-left-column-binary-collation (sqlite-sqltests/compound-select-left-column-binary-collation.sqltest) - :memory:[39m
   --- expected
   +++ actual
    
   -A
   +a
    B
    C
   -a
   -b
   -c
    z

[1mSummary:[0m
  [31m4 failed[39m
  [2mTotal time: 10.02ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/sqlite-sqltests/compound-select-left-column-binary-collation.sqltest`