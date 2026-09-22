# Reproducer: #9029 — MVCC: NULL rowid is assigned from a stale pre-DELETE maximum instead of the current max rowid

https://github.com/tursodatabase/turso/issues/9029

## Environment

- Commit: 665cf5b2c80c3f76b6652ab102d7da2cd470f904
- Platform: linux x86_64
- Reproduced at: 2026-09-22T09:38:24Z

## Reproduce

```sh
cd sqlite/conformance && cargo run -q --manifest-path ../../testing/sqltest/Cargo.toml --bin sqltest -- run sqlite-sqltests/mvcc-null-rowid-after-delete.sqltest --backend rust --snapshot-filter __never__
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: under MVCC the RowidAllocator is a high-water mark that is never lowered on DELETE, so a NULL INTEGER PRIMARY KEY gets pre-delete max + 1 (e.g. 1003 instead of 12), while SQLite and non-MVCC Turso both use the largest rowid currently in the table.

## Observed failure

RoboTurso ran the command above and observed:

```text
[1msqlite-sqltests/mvcc-null-rowid-after-delete.sqltest[0m
  [[31mFAIL[0m] mvcc-null-rowid-after-delete-max-row     [2m(12.99ms)[0m
  [[31mFAIL[0m] mvcc-null-rowid-after-delete-all         [2m(13.24ms)[0m
  [[31mFAIL[0m] mvcc-null-rowid-after-delete-in-separate-statements [2m(13.26ms)[0m

[1m[31mFailures:[39m[0m

[31m── mvcc-null-rowid-after-delete-max-row (sqlite-sqltests/mvcc-null-rowid-after-delete.sqltest) - :temp:[39m
   --- expected
   +++ actual
    mvcc
    1|a
    2|b
   -3|d
   +501|d

[31m── mvcc-null-rowid-after-delete-all (sqlite-sqltests/mvcc-null-rowid-after-delete.sqltest) - :temp:[39m
   --- expected
   +++ actual
    mvcc
    10|x
    11|y
   -12|z
   +1003|z

[31m── mvcc-null-rowid-after-delete-in-separate-statements (sqlite-sqltests/mvcc-null-rowid-after-delete.sqltest) - :temp:[39m
   --- expected
   +++ actual
    mvcc
    5|b
   -6|c
   +1001|c

[1mSummary:[0m
  [31m3 failed[39m
  [2mTotal time: 13.49ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/sqlite-sqltests/mvcc-null-rowid-after-delete.sqltest`