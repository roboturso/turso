# Reproducer: #9406 — optimizer: use an index range for LIKE and GLOB with a literal prefix

https://github.com/tursodatabase/turso/issues/9406

## Environment

- Commit: 704b69cb5eaf73c018567788030f0a4502d17552
- Platform: linux x86_64
- Reproduced at: 2026-09-29T08:50:22Z

## Reproduce

```sh
cd sqlite/conformance && cargo run -q --manifest-path ../../testing/sqltest/Cargo.toml --bin sqltest -- run sqlite-sqltests/like_glob_prefix_index_range.sqltest --backend rust --snapshot-filter __never__
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Turso scans the table or the whole index for `f GLOB 'ghij*'`, `f LIKE 'ghij%'` with a NOCASE index, and an OR with a GLOB prefix branch, where SQLite searches the index with an `f>? AND f<?` range or uses a multi-index OR.

## Observed failure

RoboTurso ran the command above and observed:

```text
[1msqlite-sqltests/like_glob_prefix_index_range.sqltest[0m
  [[31mFAIL[0m] like-prefix-uses-nocase-index-range-plan [2m(10.26ms)[0m
  [[32mPASS[0m] like-prefix-nocase-results               [2m(10.56ms)[0m
  [[32mPASS[0m] glob-prefix-results                      [2m(11.77ms)[0m
  [[31mFAIL[0m] glob-prefix-in-multi-index-or-plan       [2m(11.90ms)[0m
  [[31mFAIL[0m] glob-prefix-uses-index-range-plan        [2m(12.09ms)[0m
  [[32mPASS[0m] glob-prefix-in-or-results                [2m(12.83ms)[0m

[1m[31mFailures:[39m[0m

[31m── like-prefix-uses-nocase-index-range-plan (sqlite-sqltests/like_glob_prefix_index_range.sqltest) - :memory:[39m
   output does not match pattern
   Pattern: SEARCH t3 USING COVERING INDEX t3f \(f>=?\? AND f<\?\)
   Actual:
   1|0|0|SCAN t3

[31m── glob-prefix-in-multi-index-or-plan (sqlite-sqltests/like_glob_prefix_index_range.sqltest) - :memory:[39m
   output does not match pattern
   Pattern: MULTI-INDEX OR t2
   Actual:
   1|0|0|SCAN t2

[31m── glob-prefix-uses-index-range-plan (sqlite-sqltests/like_glob_prefix_index_range.sqltest) - :memory:[39m
   output does not match pattern
   Pattern: SEARCH t2 USING COVERING INDEX t2f \(f>=?\? AND f<\?\)
   Actual:
   1|0|0|SCAN t2 USING COVERING INDEX t2f

[1mSummary:[0m
  [32m3 passed[39m, [31m3 failed[39m
  [2mTotal time: 13.06ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/sqlite-sqltests/like_glob_prefix_index_range.sqltest`