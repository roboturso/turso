# Reproducer: #7977 — HAVING without GROUP BY referencing a bare column returns no rows

https://github.com/tursodatabase/turso/issues/7977

## Environment

- Commit: fae58dc3ee11aaad12dff21bbb27089083231319
- Platform: linux x86_64
- Reproduced at: 2026-07-23T07:11:28Z

## Reproduce

```sh
cargo run -q --manifest-path testing/sqltest/Cargo.toml --bin sqltest -- run sqlite/conformance/sqlite-sqltests/having-without-group-by-bare-column.sqltest --backend rust --snapshot-filter __never__
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed on fae58dc3e: `SELECT count(*) FROM t HAVING a != 400` returns no rows in Turso while SQLite returns 5, so bare-column HAVING without GROUP BY wrongly drops the implicit group; added a failing .sqltest regression covering this.

## Observed failure

RoboTurso ran the command above and observed:

```text
warning: unused import: `crate::translate::collate::CollationSeq`
  --> core/vdbe/mod.rs:42:9
   |
42 | pub use crate::translate::collate::CollationSeq;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

[1msqlite/conformance/sqlite-sqltests/having-without-group-by-bare-column.sqltest[0m
  [[32mPASS[0m] having-no-group-by-bare-column-false     [2m(6.36ms)[0m
  [[31mFAIL[0m] having-no-group-by-bare-column-true      [2m(6.43ms)[0m
  [[32mPASS[0m] having-no-group-by-agg-alias             [2m(6.47ms)[0m
  [[31mFAIL[0m] having-no-group-by-bare-column-with-agg  [2m(6.77ms)[0m

[1m[31mFailures:[39m[0m

[31m── having-no-group-by-bare-column-true (sqlite/conformance/sqlite-sqltests/having-without-group-by-bare-column.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -5

[31m── having-no-group-by-bare-column-with-agg (sqlite/conformance/sqlite-sqltests/having-without-group-by-bare-column.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -5|15

[1mSummary:[0m
  [32m2 passed[39m, [31m2 failed[39m
  [2mTotal time: 6.98ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/sqlite-sqltests/having-without-group-by-bare-column.sqltest`