# Reproducer: #9279 — Self-referencing ON DELETE SET NULL makes DELETE remove rows that did not match

https://github.com/tursodatabase/turso/issues/9279

## Environment

- Commit: 173b8773ff7e35af7df51973d923212918851dd1
- Platform: linux x86_64
- Reproduced at: 2026-09-24T10:49:21Z

## Reproduce

```sh
cd sqlite/conformance && cargo run -q --manifest-path ../../testing/sqltest/Cargo.toml --bin sqltest -- run sqlite-sqltests/delete-self-fk-set-null-keeps-unmatched-rows.sqltest --backend rust
```

Fails on the current tree; passes once the bug is fixed.

## What happens

With a self-referencing ON DELETE SET NULL foreign key, DELETE also removes the child rows that the SET NULL action changed to match the WHERE clause, so no rows remain, while SQLite keeps rows 2 and 3 with p set to NULL.

## Observed failure

RoboTurso ran the command above and observed:

```text
[1msqlite-sqltests/delete-self-fk-set-null-keeps-unmatched-rows.sqltest[0m
  [[31mFAIL[0m] delete-self-fk-set-null-keeps-rows-that-did-not-match [2m(9.97ms)[0m

[1m[31mFailures:[39m[0m

[31m── delete-self-fk-set-null-keeps-rows-that-did-not-match (sqlite-sqltests/delete-self-fk-set-null-keeps-unmatched-rows.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -2|1
   -3|1

[1mSummary:[0m
  [31m1 failed[39m
  [2mTotal time: 10.20ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/sqlite-sqltests/delete-self-fk-set-null-keeps-unmatched-rows.sqltest`