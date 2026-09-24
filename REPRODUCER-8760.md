# Reproducer: #8760 — A GROUP BY sorter drops COLLATE inside an aggregate, so a stored count is wrong

https://github.com/tursodatabase/turso/issues/8760

## Environment

- Commit: cb1c3c8653e8a1a96ec0159ef69d9906c71940aa
- Platform: linux x86_64
- Reproduced at: 2026-09-24T12:58:53Z

## Reproduce

```sh
cd sqlite/conformance && cargo run -q --manifest-path ../../testing/sqltest/Cargo.toml --bin sqltest -- run sqlite-sqltests/group-by-sorter-aggregate-collate.sqltest --backend rust
```

Fails on the current tree; passes once the bug is fixed.

## What happens

When a GROUP BY sorts its rows, comparisons inside an aggregate argument or FILTER lose the column's NOCASE collation, so sum(a='X') returns 1 instead of 2 and INSERT ... SELECT stores the wrong count (on the current tree this also happens with the literal on the left).

## Observed failure

RoboTurso ran the command above and observed:

```text
[1msqlite-sqltests/group-by-sorter-aggregate-collate.sqltest[0m
  [[31mFAIL[0m] group-by-sorter-aggregate-argument-literal-on-left-keeps-column-collation [2m(8.55ms)[0m
  [[31mFAIL[0m] group-by-sorter-aggregate-argument-keeps-column-collation [2m(8.66ms)[0m
  [[32mPASS[0m] group-by-with-full-index-keeps-column-collation [2m(9.56ms)[0m
  [[31mFAIL[0m] group-by-sorter-with-partial-index-keeps-column-collation [2m(9.83ms)[0m
  [[31mFAIL[0m] group-by-sorter-aggregate-filter-keeps-column-collation [2m(9.96ms)[0m

[1m[31mFailures:[39m[0m

[31m── group-by-sorter-aggregate-argument-literal-on-left-keeps-column-collation (sqlite-sqltests/group-by-sorter-aggregate-collate.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -1|2
   +1|1

[31m── group-by-sorter-aggregate-argument-keeps-column-collation (sqlite-sqltests/group-by-sorter-aggregate-collate.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -1|2
   +1|1

[31m── group-by-sorter-with-partial-index-keeps-column-collation (sqlite-sqltests/group-by-sorter-aggregate-collate.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -1|2
   +1|1

[31m── group-by-sorter-aggregate-filter-keeps-column-collation (sqlite-sqltests/group-by-sorter-aggregate-collate.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -2
   +1

[1mSummary:[0m
  [32m1 passed[39m, [31m4 failed[39m
  [2mTotal time: 10.16ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/sqlite-sqltests/group-by-sorter-aggregate-collate.sqltest`