# Reproducer: #8745 — An aggregate under GROUP BY drops the column's COLLATE, so a rollup undercounts

https://github.com/tursodatabase/turso/issues/8745

## Environment

- Commit: cb1c3c8653e8a1a96ec0159ef69d9906c71940aa
- Platform: linux x86_64
- Reproduced at: 2026-09-24T12:55:45Z

## Reproduce

```sh
cd sqlite/conformance && cargo run -q --manifest-path ../../testing/sqltest/Cargo.toml --bin sqltest -- run sqlite-sqltests/aggregate-group-by-keeps-column-collation.sqltest --backend rust
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: with GROUP BY, an aggregate argument or FILTER predicate compares a COLLATE NOCASE column as BINARY, so Turso returns 1|1, 2|0 where SQLite returns 1|2, 2|1.

## Observed failure

RoboTurso ran the command above and observed:

```text
[1msqlite-sqltests/aggregate-group-by-keeps-column-collation.sqltest[0m
  [[31mFAIL[0m] aggregate-filter-under-group-by-uses-column-collation [2m(7.87ms)[0m
  [[31mFAIL[0m] aggregate-argument-under-group-by-uses-column-collation [2m(7.91ms)[0m
  [[31mFAIL[0m] aggregate-under-group-by-with-partial-index-uses-column-collation [2m(10.01ms)[0m
  [[31mFAIL[0m] aggregate-under-group-by-into-keyed-table-uses-column-collation [2m(10.51ms)[0m

[1m[31mFailures:[39m[0m

[31m── aggregate-filter-under-group-by-uses-column-collation (sqlite-sqltests/aggregate-group-by-keeps-column-collation.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -1|2
   -2|1
   +1|1
   +2|0

[31m── aggregate-argument-under-group-by-uses-column-collation (sqlite-sqltests/aggregate-group-by-keeps-column-collation.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -1|2
   -2|1
   +1|1
   +2|0

[31m── aggregate-under-group-by-with-partial-index-uses-column-collation (sqlite-sqltests/aggregate-group-by-keeps-column-collation.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -1|2
   -2|1
   +1|1
   +2|0

[31m── aggregate-under-group-by-into-keyed-table-uses-column-collation (sqlite-sqltests/aggregate-group-by-keeps-column-collation.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -2,1
   +1,0

[1mSummary:[0m
  [31m4 failed[39m
  [2mTotal time: 10.74ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/sqlite-sqltests/aggregate-group-by-keeps-column-collation.sqltest`