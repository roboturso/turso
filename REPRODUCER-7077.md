# Reproducer: #7077 — ALTER COLUMN rewrites table rows but not secondary indexes, later causes DELETE/UPDATE corruption.

https://github.com/tursodatabase/turso/issues/7077

## Environment

- Commit: 225388ab65ac31ce4ff348e5d4f1d1855425ff6c
- Platform: linux x86_64
- Reproduced at: 2026-07-07T08:24:17Z

## Reproduce

```sh
cd testing/sqltests && cargo run -q --bin test-runner -- run tests/alter-column-unique-index-rewrite-7077.sqltest --backend rust --snapshot-filter __never__
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed on current main: ALTER COLUMN on a UNIQUE column rewrites table rows but leaves stale entries in the automatic unique index, so a later DELETE/UPDATE fails with 'Corrupt database: IdxDelete: no matching index entry found'.

## Observed failure

RoboTurso ran the command above and observed:

```text
[1mtests/alter-column-unique-index-rewrite-7077.sqltest[0m
  [[31mFAIL[0m] alter-column-unique-index-update-no-corruption [2m(10.05ms)[0m
  [[31mFAIL[0m] alter-column-unique-index-delete-no-corruption [2m(10.20ms)[0m

[1m[31mFailures:[39m[0m

[31m── alter-column-unique-index-update-no-corruption (tests/alter-column-unique-index-rewrite-7077.sqltest) - :memory:[39m
   expected success but got error: IdxDelete: no matching index entry found for key [Value(Numeric(Integer(2))), Value(Numeric(Integer(1)))] while seeking

[31m── alter-column-unique-index-delete-no-corruption (tests/alter-column-unique-index-rewrite-7077.sqltest) - :memory:[39m
   expected success but got error: IdxDelete: no matching index entry found for key [Value(Numeric(Integer(2))), Value(Numeric(Integer(1)))] while seeking

[1mSummary:[0m
  [31m2 failed[39m
  [2mTotal time: 10.47ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `testing/sqltests/tests/alter-column-unique-index-rewrite-7077.sqltest`