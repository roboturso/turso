# Reproducer: #8703 — ALTER TABLE erases ON CONFLICT, so a rolled-back transaction half-commits

https://github.com/tursodatabase/turso/issues/8703

## Environment

- Commit: 6e320d2e788f2c98333c43697c886f0f7ce9e8ff
- Platform: linux x86_64
- Reproduced at: 2026-09-22T17:28:26Z

## Reproduce

```sh
cd sqlite/conformance && cargo run -q --manifest-path ../../testing/sqltest/Cargo.toml --bin sqltest -- run sqlite-sqltests/alter-table-keeps-on-conflict-clause.sqltest --backend rust --snapshot-filter __never__
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: ALTER TABLE ADD COLUMN and DROP COLUMN rebuild the CREATE TABLE text via BTreeTable::to_sql, which omits every constraint's ON CONFLICT clause, so a reopened connection enforces ABORT instead of ROLLBACK and a conflicting transaction keeps its earlier rows.

## Observed failure

RoboTurso ran the command above and observed:

```text
[1msqlite-sqltests/alter-table-keeps-on-conflict-clause.sqltest[0m
  [[31mFAIL[0m] alter-add-column-keeps-primary-key-on-conflict-replace [2m(8.31ms)[0m
  [[31mFAIL[0m] alter-add-column-keeps-not-null-on-conflict-ignore [2m(8.34ms)[0m
  [[31mFAIL[0m] alter-add-column-keeps-unique-on-conflict-rollback [2m(8.54ms)[0m
  [[31mFAIL[0m] alter-drop-column-keeps-unique-on-conflict-rollback [2m(9.76ms)[0m
  [[32mPASS[0m] alter-rename-column-keeps-unique-on-conflict-rollback [2m(10.42ms)[0m

[1m[31mFailures:[39m[0m

[31m── alter-add-column-keeps-primary-key-on-conflict-replace (sqlite-sqltests/alter-table-keeps-on-conflict-clause.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -1
   +0

[31m── alter-add-column-keeps-not-null-on-conflict-ignore (sqlite-sqltests/alter-table-keeps-on-conflict-clause.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -1
   +0

[31m── alter-add-column-keeps-unique-on-conflict-rollback (sqlite-sqltests/alter-table-keeps-on-conflict-clause.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -1
   +0

[31m── alter-drop-column-keeps-unique-on-conflict-rollback (sqlite-sqltests/alter-table-keeps-on-conflict-clause.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -1
   +0

[1mSummary:[0m
  [32m1 passed[39m, [31m4 failed[39m
  [2mTotal time: 10.64ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/sqlite-sqltests/alter-table-keeps-on-conflict-clause.sqltest`