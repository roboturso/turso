# Reproducer: #9068 — ADD COLUMN unquotes a keyword column name inside a table-level PRIMARY KEY, and the file can no longer be opened

https://github.com/tursodatabase/turso/issues/9068

## Environment

- Commit: e7e77e595725b2d3d6114a634e17cebaf1e9f3fb
- Platform: linux x86_64
- Reproduced at: 2026-09-22T14:10:51Z

## Reproduce

```sh
cd sqlite/conformance && cargo run -q --manifest-path ../../testing/sqltest/Cargo.toml --bin sqltest -- run sqlite-sqltests/alter-add-column-quoted-pk-column.sqltest --backend rust
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed on the current tree: ADD COLUMN rewrites the schema text as PRIMARY KEY (id, order) without quotes, so the next ALTER fails with near "order": syntax error and the file can no longer be opened by turso or sqlite3.

## Observed failure

RoboTurso ran the command above and observed:

```text
[1msqlite-sqltests/alter-add-column-quoted-pk-column.sqltest[0m
  [[31mFAIL[0m] add-column-keeps-quotes-in-table-level-primary-key [2m(8.40ms)[0m
  [[32mPASS[0m] add-second-column-after-add-column-with-quoted-pk-column [2m(8.99ms)[0m
  [[31mFAIL[0m] rename-column-after-add-column-with-quoted-pk-column [2m(9.48ms)[0m

[1m[31mFailures:[39m[0m

[31m── add-column-keeps-quotes-in-table-level-primary-key (sqlite-sqltests/alter-add-column-quoted-pk-column.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -1
   +0

[31m── rename-column-after-add-column-with-quoted-pk-column (sqlite-sqltests/alter-add-column-quoted-pk-column.sqltest) - :memory:[39m
   expected success but got error: near "order": syntax error

[1mSummary:[0m
  [32m1 passed[39m, [31m2 failed[39m
  [2mTotal time: 9.68ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/sqlite-sqltests/alter-add-column-quoted-pk-column.sqltest`