# Reproducer: #8700 — RENAME TO a digit-leading name drops the quotes and bricks the file

https://github.com/tursodatabase/turso/issues/8700

## Environment

- Commit: 6e320d2e788f2c98333c43697c886f0f7ce9e8ff
- Platform: linux x86_64
- Reproduced at: 2026-09-22T17:35:53Z

## Reproduce

```sh
cd sqlite/conformance && cargo run -q --manifest-path ../../testing/sqltest/Cargo.toml --bin sqltest -- run sqlite-sqltests/alter-rename-table-to-digit-leading-name.sqltest --backend rust
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed on main: RENAME TO "1a" writes the new name unquoted into both the CREATE TABLE and CREATE INDEX rows, so the next ALTER fails with `bad number '1' at offset 13` and the file can no longer be opened by tursodb or sqlite3.

## Observed failure

RoboTurso ran the command above and observed:

```text
[1msqlite-sqltests/alter-rename-table-to-digit-leading-name.sqltest[0m
  [[32mPASS[0m] rename-to-name-with-space-keeps-quotes   [2m(10.27ms)[0m
  [[31mFAIL[0m] rename-to-digit-leading-name-keeps-quotes-in-index-sql [2m(12.86ms)[0m
  [[31mFAIL[0m] rename-to-digit-leading-name-keeps-quotes-in-table-sql [2m(13.48ms)[0m
  [[31mFAIL[0m] renamed-digit-leading-table-can-be-renamed-again [2m(14.31ms)[0m
  [[31mFAIL[0m] next-alter-after-rename-to-digit-leading-name-still-works [2m(14.42ms)[0m

[1m[31mFailures:[39m[0m

[31m── rename-to-digit-leading-name-keeps-quotes-in-index-sql (sqlite-sqltests/alter-rename-table-to-digit-leading-name.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -1
   +0

[31m── rename-to-digit-leading-name-keeps-quotes-in-table-sql (sqlite-sqltests/alter-rename-table-to-digit-leading-name.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -1
   +0

[31m── renamed-digit-leading-table-can-be-renamed-again (sqlite-sqltests/alter-rename-table-to-digit-leading-name.sqltest) - :memory:[39m
   expected success but got error: bad number '1' at offset 13

[31m── next-alter-after-rename-to-digit-leading-name-still-works (sqlite-sqltests/alter-rename-table-to-digit-leading-name.sqltest) - :memory:[39m
   expected success but got error: bad number '1' at offset 13

[1mSummary:[0m
  [32m1 passed[39m, [31m4 failed[39m
  [2mTotal time: 14.64ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/sqlite-sqltests/alter-rename-table-to-digit-leading-name.sqltest`