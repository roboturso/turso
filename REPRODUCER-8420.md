# Reproducer: #8420 — json_each() returns JSON string values still escaped instead of the decoded SQL text

https://github.com/tursodatabase/turso/issues/8420

## Environment

- Commit: 6e320d2e788f2c98333c43697c886f0f7ce9e8ff
- Platform: linux x86_64
- Reproduced at: 2026-09-22T17:50:58Z

## Reproduce

```sh
cd sqlite/conformance && cargo run -q --manifest-path ../../testing/sqltest/Cargo.toml --bin sqltest -- run sqlite-sqltests/json/json-each-decodes-string-escapes.sqltest --backend rust --snapshot-mode no
```

Fails on the current tree; passes once the bug is fixed.

## What happens

The value/atom half of this was fixed in 605308d8, but object keys still only decode \" — json_each('{"a\nb":1}') returns key hex 615C6E62 where SQLite returns 610A62, and \\, \t, and \uXXXX in keys are left verbatim too, for both json_each and json_tree.

## Observed failure

RoboTurso ran the command above and observed:

```text
[1msqlite-sqltests/json/json-each-decodes-string-escapes.sqltest[0m
  [[32mPASS[0m] json-each-value-decodes-newline-and-unicode-escapes [2m(5.85ms)[0m
  [[32mPASS[0m] json-each-atom-and-cast-decode-string-escapes [2m(5.84ms)[0m
  [[32mPASS[0m] json-each-value-decodes-string-escapes   [2m(5.85ms)[0m
  [[31mFAIL[0m] json-each-key-decodes-all-string-escapes [2m(5.84ms)[0m
  [[31mFAIL[0m] json-tree-key-decodes-newline-escape     [2m(5.79ms)[0m
  [[31mFAIL[0m] json-each-key-decodes-newline-escape     [2m(5.85ms)[0m
  [[31mFAIL[0m] json-each-key-decodes-tab-escape         [2m(5.82ms)[0m

[1m[31mFailures:[39m[0m

[31m── json-each-key-decodes-all-string-escapes (sqlite-sqltests/json/json-each-decodes-string-escapes.sqltest) - :memory:[39m
   --- expected
   +++ actual
    q"q|712271
   -b\s|625C73
   +b\\s|625C5C73
    uA|7541

[31m── json-tree-key-decodes-newline-escape (sqlite-sqltests/json/json-each-decodes-string-escapes.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -610A62
   +615C6E62

[31m── json-each-key-decodes-newline-escape (sqlite-sqltests/json/json-each-decodes-string-escapes.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -610A62
   +615C6E62

[31m── json-each-key-decodes-tab-escape (sqlite-sqltests/json/json-each-decodes-string-escapes.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -740974
   +745C7474

[1mSummary:[0m
  [32m3 passed[39m, [31m4 failed[39m
  [2mTotal time: 6.14ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/sqlite-sqltests/json/json-each-decodes-string-escapes.sqltest`