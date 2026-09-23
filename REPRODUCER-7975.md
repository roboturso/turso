# Reproducer: #7975 — REPLACE with delete trigger re-inserting the key corrupts UNIQUE index

https://github.com/tursodatabase/turso/issues/7975

## Environment

- Commit: 6d01be3604bbd2bc1066c2fef3f8049b98ed4523
- Platform: linux x86_64
- Reproduced at: 2026-09-23T18:19:19Z

## Reproduce

```sh
cargo run -q --manifest-path testing/sqltest/Cargo.toml --bin sqltest -- run sqlite/conformance/sqlite-sqltests/replace-delete-trigger-reinsert-unique.sqltest --backend rust
```

Fails on the current tree; passes once the bug is fixed.

## What happens

When a REPLACE deletes a conflicting row and a delete trigger re-inserts the same key, Turso never checks the UNIQUE constraint again: the statement succeeds instead of failing with 'UNIQUE constraint failed', and in the ON DELETE CASCADE case it leaves duplicate values and a corrupt unique index.

## Observed failure

RoboTurso ran the command above and observed:

```text
[1msqlite/conformance/sqlite-sqltests/replace-delete-trigger-reinsert-unique.sqltest[0m
  [[31mFAIL[0m] replace-recursive-delete-trigger-reinserts-unique-key [2m(9.33ms)[0m
  [[31mFAIL[0m] replace-cascade-trigger-reinserts-unique-key [2m(10.26ms)[0m

[1m[31mFailures:[39m[0m

[31m── replace-recursive-delete-trigger-reinserts-unique-key (sqlite/conformance/sqlite-sqltests/replace-delete-trigger-reinsert-unique.sqltest) - :memory:[39m
   expected error but query succeeded

[31m── replace-cascade-trigger-reinserts-unique-key (sqlite/conformance/sqlite-sqltests/replace-delete-trigger-reinsert-unique.sqltest) - :memory:[39m
   expected error but query succeeded

[1mSummary:[0m
  [31m2 failed[39m
  [2mTotal time: 10.44ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/sqlite-sqltests/replace-delete-trigger-reinsert-unique.sqltest`