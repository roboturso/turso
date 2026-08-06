# Reproducer: #8242 — timediff() is sign-symmetric: datetime(B, timediff(A,B)) != A for 15% of date pairs

https://github.com/tursodatabase/turso/issues/8242

## Environment

- Commit: 74752acfd958177046c9ec4a5be93e9c42bc153d
- Platform: linux x86_64
- Reproduced at: 2026-08-06T15:03:47Z

## Reproduce

```sh
cd sqlite/conformance && cargo run -q --manifest-path ../../testing/sqltest/Cargo.toml --bin sqltest -- run sqlite-sqltests/timediff-negative-month-roundtrip.sqltest --backend rust
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: timediff() is sign-symmetric, so negative results are anchored on the earlier date and walked forward, and datetime(B, timediff(A,B)) does not return A whenever SQLite's month-overflow clamping makes the two directions differ (e.g. timediff('2000-01-31','2000-03-02') gives -0000-01-00 instead of -0000-01-02).

## Observed failure

RoboTurso ran the command above and observed:

```text
warning: unused import: `crate::translate::collate::CollationSeq`
  --> core/vdbe/mod.rs:43:9
   |
43 | pub use crate::translate::collate::CollationSeq;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

Generating default databases...
[1msqlite-sqltests/timediff-negative-month-roundtrip.sqltest[0m
  [[32mPASS[0m] timediff-positive-with-time-of-day       [2m(4.53ms)[0m
  [[31mFAIL[0m] timediff-negative-two-months-back-roundtrip [2m(4.94ms)[0m
  [[31mFAIL[0m] timediff-negative-month-overflow-across-years [2m(5.16ms)[0m
  [[31mFAIL[0m] timediff-negative-month-overflow         [2m(5.22ms)[0m
  [[32mPASS[0m] timediff-positive-month-overflow-roundtrip [2m(4.88ms)[0m
  [[31mFAIL[0m] timediff-negative-month-overflow-across-years-roundtrip [2m(5.15ms)[0m
  [[31mFAIL[0m] timediff-negative-month-overflow-roundtrip [2m(5.29ms)[0m
  [[32mPASS[0m] timediff-positive-month-overflow         [2m(5.01ms)[0m
  [[31mFAIL[0m] timediff-negative-two-months-back        [2m(5.15ms)[0m
  [[31mFAIL[0m] timediff-negative-short-february         [2m(5.24ms)[0m
  [[31mFAIL[0m] timediff-negative-short-february-roundtrip [2m(5.21ms)[0m
  [[32mPASS[0m] timediff-equal-dates                     [2m(5.14ms)[0m

[1m[31mFailures:[39m[0m

[31m── timediff-negative-two-months-back-roundtrip (sqlite-sqltests/timediff-negative-month-roundtrip.sqltest) - :default:[39m
   --- expected
   +++ actual
   -1999-12-31 00:00:00
   +2000-01-02 00:00:00

[31m── timediff-negative-month-overflow-across-years (sqlite-sqltests/timediff-negative-month-roundtrip.sqltest) - :default:[39m
   --- expected
   +++ actual
   --0001-01-02 00:00:00.000
   +-0001-01-00 00:00:00.000

[31m── timediff-negative-month-overflow (sqlite-sqltests/timediff-negative-month-roundtrip.sqltest) - :default:[39m
   --- expected
   +++ actual
   --0000-01-02 00:00:00.000
   +-0000-01-00 00:00:00.000

[31m── timediff-negative-month-overflow-across-years-roundtrip (sqlite-sqltests/timediff-negative-month-roundtrip.sqltest) - :default:[39m
   --- expected
   +++ actual
   -1999-01-31 00:00:00
   +1999-02-02 00:00:00

[31m── timediff-negative-month-overflow-roundtrip (sqlite-sqltests/timediff-negative-month-roundtrip.sqltest) - :default:[39m
   --- expected
   +++ actual
   -2000-01-31 00:00:00
   +2000-02-02 00:00:00

[31m── timediff-negative-two-months-back (sqlite-sqltests/timediff-negative-month-roundtrip.sqltest) - :default:[39m
   --- expected
   +++ actual
   --0000-02-01 00:00:00.000
   +-0000-01-30 00:00:00.000

[31m── timediff-negative-short-february (sqlite-sqltests/timediff-negative-month-roundtrip.sqltest) - :default:[39m
   --- expected
   +++ actual
   --0000-01-30 00:00:00.000
   +-0000-01-28 00:00:00.000

[31m── timediff-negative-short-february-roundtrip (sqlite-sqltests/timediff-negative-month-roundtrip.sqltest) - :default:[39m
   --- expected
   +++ actual
   -2000-01-02 00:00:00
   +2000-01-04 00:00:00

[1mSummary:[0m
  [32m4 passed[39m, [31m8 failed[39m
  [2mTotal time: 5.82ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/sqlite-sqltests/timediff-negative-month-roundtrip.sqltest`