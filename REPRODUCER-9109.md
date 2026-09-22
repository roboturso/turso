# Reproducer: #9109 — MVCC does not reload planner statistics after ANALYZE

https://github.com/tursodatabase/turso/issues/9109

## Environment

- Commit: 665cf5b2c80c3f76b6652ab102d7da2cd470f904
- Platform: linux x86_64
- Reproduced at: 2026-09-22T07:26:19Z

## Reproduce

```sh
cd sqlite/conformance && cargo run -q --manifest-path ../../testing/sqltest/Cargo.toml --bin sqltest -- run sqlite-sqltests/mvcc-analyze-refreshes-planner-stats.sqltest --backend rust
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed on the current tree: in MVCC mode the plan after an autocommit ANALYZE still uses the default 1000000 row estimate while sqlite_stat1 holds t||100, whereas WAL mode correctly reports 100.

## Observed failure

RoboTurso ran the command above and observed:

```text
[1msqlite-sqltests/mvcc-analyze-refreshes-planner-stats.sqltest[0m
  [[32mPASS[0m] wal-analyze-next-plan-uses-new-row-estimate [2m(15.87ms)[0m
  [[32mPASS[0m] mvcc-analyze-writes-sqlite-stat1         [2m(20.78ms)[0m
  [[31mFAIL[0m] mvcc-analyze-next-plan-uses-new-row-estimate [2m(21.33ms)[0m

[1m[31mFailures:[39m[0m

[31m── mvcc-analyze-next-plan-uses-new-row-estimate (sqlite-sqltests/mvcc-analyze-refreshes-planner-stats.sqltest) - :temp:[39m
   output does not match pattern
   Pattern: "rows_per_input":100,"output_rows":100,
   Actual:
   mvcc
   {"version":1,"sql":"EXPLAIN QUERY PLAN FORMAT=JSON SELECT * FROM t;","result_columns":["id","value"],"nodes":[{"id":1,"parent":null,"detail":"SCAN t","op":{"type":"scan","table":"t","source":"table","estimate":{"input_rows":1,"rows_per_input":1000000,"output_rows":1000000,"access_cost":23000,"total_cost":23000}}}]}

[1mSummary:[0m
  [32m2 passed[39m, [31m1 failed[39m
  [2mTotal time: 21.48ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/sqlite-sqltests/mvcc-analyze-refreshes-planner-stats.sqltest`