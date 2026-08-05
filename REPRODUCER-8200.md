# Reproducer: #8200 — Aggregate query with HAVING but no GROUP BY returns no rows

https://github.com/tursodatabase/turso/issues/8200

## Environment

- Commit: 2bdeb831796f62b4ff2f8393f93ddc1a17ebba50
- Platform: linux x86_64
- Reproduced at: 2026-08-05T07:19:53Z

## Reproduce

```sh
cd sqlite/conformance && cargo run -q --manifest-path ../../testing/sqltest/Cargo.toml --bin sqltest -- run sqlite-sqltests/having-without-group-by.sqltest --backend rust
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: an aggregate query with HAVING on a bare column but no GROUP BY returns no rows in Turso while SQLite returns the single aggregate row (3); added a regression sqltest that fails on the current tree.

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

[1msqlite-sqltests/having-without-group-by.sqltest[0m
  [[31mFAIL[0m] having-bare-column-true                  [2m(6.64ms)[0m
  [[32mPASS[0m] having-bare-column-false                 [2m(6.70ms)[0m
  [[32mPASS[0m] having-aggregate-on-column               [2m(7.34ms)[0m
  [[32mPASS[0m] having-aggregate-no-group-by             [2m(7.42ms)[0m

[1m[31mFailures:[39m[0m

[31m── having-bare-column-true (sqlite-sqltests/having-without-group-by.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -3

[1mSummary:[0m
  [32m3 passed[39m, [31m1 failed[39m
  [2mTotal time: 7.63ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/sqlite-sqltests/having-without-group-by.sqltest`