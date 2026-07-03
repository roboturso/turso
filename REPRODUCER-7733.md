# Reproducer: #7733 — ALTER TABLE RENAME COLUMN fails for TEMP trigger with NEW/OLD column refs on main table

https://github.com/tursodatabase/turso/issues/7733

## Environment

- Commit: d4d6777a2685ab9a3ffc8733329743fdfe000c45
- Platform: linux x86_64
- Reproduced at: 2026-07-03T17:29:33Z

## Reproduce

```sh
cd testing/sqltests && cargo run -q --bin test-runner -- run tests/alter-rename-column-temp-trigger.sqltest --backend rust --snapshot-mode no
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed on current main: ALTER TABLE ... RENAME COLUMN is rejected (currently with 'trigger table not found: t') when a TEMP trigger references the column via NEW/OLD, while SQLite accepts the rename and rewrites the trigger.

## Observed failure

RoboTurso ran the command above and observed:

```text
[1mtests/alter-rename-column-temp-trigger.sqltest[0m
  [[31mFAIL[0m] rename-column-with-temp-trigger-new-ref  [2m(8.77ms)[0m
  [[31mFAIL[0m] rename-column-with-temp-trigger-old-ref-in-when [2m(9.07ms)[0m

[1m[31mFailures:[39m[0m

[31m── rename-column-with-temp-trigger-new-ref (tests/alter-rename-column-temp-trigger.sqltest) - :memory:[39m
   expected success but got error: Parse error: error in trigger trg after rename column: trigger table not found: t

[31m── rename-column-with-temp-trigger-old-ref-in-when (tests/alter-rename-column-temp-trigger.sqltest) - :memory:[39m
   expected success but got error: Parse error: error in trigger trg2 after rename column: trigger table not found: t2

[1mSummary:[0m
  [31m2 failed[39m
  [2mTotal time: 9.25ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `testing/sqltests/tests/alter-rename-column-temp-trigger.sqltest`