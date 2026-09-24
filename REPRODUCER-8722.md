# Reproducer: #8722 — GROUP BY over a wrapper keeps NOCASE, so a dedup DELETE removes a live row

https://github.com/tursodatabase/turso/issues/8722

## Environment

- Commit: cb1c3c8653e8a1a96ec0159ef69d9906c71940aa
- Platform: linux x86_64
- Reproduced at: 2026-09-24T12:52:46Z

## Reproduce

```sh
cargo run -q --manifest-path testing/sqltest/Cargo.toml --bin sqltest -- run sqlite/conformance/sqlite-sqltests/group-by-wrapped-nocase-column.sqltest --backend rust
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: a GROUP BY or DISTINCT key that wraps a NOCASE column in an expression (e.g. `e || ''` or `trim(e)`) still groups with NOCASE instead of BINARY, so 'a' and 'A' end up in one group and the dedup DELETE removes a row that is not a duplicate.

## Observed failure

RoboTurso ran the command above and observed:

```text
[1msqlite/conformance/sqlite-sqltests/group-by-wrapped-nocase-column.sqltest[0m
  [[31mFAIL[0m] group-by-concat-uses-binary              [2m(8.63ms)[0m
  [[32mPASS[0m] group-by-bare-column-uses-nocase         [2m(8.64ms)[0m
  [[31mFAIL[0m] group-by-trim-uses-binary                [2m(8.82ms)[0m
  [[32mPASS[0m] group-by-cast-keeps-nocase               [2m(8.86ms)[0m
  [[31mFAIL[0m] distinct-trim-uses-binary                [2m(9.01ms)[0m
  [[31mFAIL[0m] dedup-delete-keeps-distinct-rows         [2m(9.50ms)[0m

[1m[31mFailures:[39m[0m

[31m── group-by-concat-uses-binary (sqlite/conformance/sqlite-sqltests/group-by-wrapped-nocase-column.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -1
   -1
   +2

[31m── group-by-trim-uses-binary (sqlite/conformance/sqlite-sqltests/group-by-wrapped-nocase-column.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -1
   -1
   +2

[31m── distinct-trim-uses-binary (sqlite/conformance/sqlite-sqltests/group-by-wrapped-nocase-column.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -A
    a

[31m── dedup-delete-keeps-distinct-rows (sqlite/conformance/sqlite-sqltests/group-by-wrapped-nocase-column.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -a
   -A
   +a

[1mSummary:[0m
  [32m2 passed[39m, [31m4 failed[39m
  [2mTotal time: 9.74ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/sqlite-sqltests/group-by-wrapped-nocase-column.sqltest`