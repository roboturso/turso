# Reproducer: #7579 — Exact-Seek IdxDelete On The Same MVCC Index Cursor Skips MVCC-Only Rows

https://github.com/tursodatabase/turso/issues/7579

## Environment

- Commit: d4d6777a2685ab9a3ffc8733329743fdfe000c45
- Platform: linux x86_64
- Reproduced at: 2026-07-06T11:47:03Z

## Reproduce

```sh
cargo run -q -p test-runner --bin test-runner -- run testing/sqltests/tests/mvcc-idxdelete-exact-seek.sqltest --backend rust
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed on current main: in MVCC mode, after a checkpoint followed by INSERT OR REPLACE, DELETE FROM t WHERE x>=1 leaves the replaced row behind (count is 1 instead of 0), while non-MVCC tursodb and sqlite3 both correctly return 0 rows.

## Observed failure

RoboTurso ran the command above and observed:

```text
[1mtesting/sqltests/tests/mvcc-idxdelete-exact-seek.sqltest[0m
  [[31mFAIL[0m] mvcc-idxdelete-exact-seek                [2m(14.91ms)[0m

[1m[31mFailures:[39m[0m

[31m── mvcc-idxdelete-exact-seek (testing/sqltests/tests/mvcc-idxdelete-exact-seek.sqltest) - :temp:[39m
   --- expected
   +++ actual
    mvcc
    0|0|0
   -0
   +1

[1mSummary:[0m
  [31m1 failed[39m
  [2mTotal time: 15.16ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `testing/sqltests/tests/mvcc-idxdelete-exact-seek.sqltest`