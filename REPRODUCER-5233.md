# Reproducer: #5233 — Panic on JOIN with Empty Table and Ungrouped Aggregate (Cursor Never Opened)

https://github.com/tursodatabase/turso/issues/5233

## Environment

- Commit: b7b5d4765f56c1c8bd3fe7ee6d3f311b3b6f1aa4
- Platform: linux x86_64
- Reproduced at: 2026-07-28T05:33:45Z

## Reproduce

```sh
cd sqlite/conformance && cargo run -q --manifest-path ../../testing/sqltest/Cargo.toml --bin sqltest -- run sqlite-sqltests --backend rust --snapshot-filter __never__ --filter '*join-empty-table-ungrouped-aggregate*'
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed on current main: `SELECT b.x, COUNT(*) FROM t a LEFT JOIN t b ON a.x=b.x` (and plain JOIN) on an empty table panics with `cursor id 2 is None` because the autoindex cursor is never opened when the outer loop body is skipped; added a regression sqltest that panics now and expects SQLite's `|0` output.

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

Generating integrity-check fixtures...
Generating default databases...

thread 'tokio-runtime-worker' panicked at core/vdbe/mod.rs:1168:32:
cursor id 2 is None
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

thread 'tokio-runtime-worker' panicked at core/vdbe/mod.rs:1168:32:
cursor id 2 is None
[1msqlite-sqltests/join-empty-table-ungrouped-aggregate.sqltest[0m
  [[31;1mERROR[0m] left-join-empty-table-ungrouped-aggregate [2m(4.32ms)[0m
  [[31;1mERROR[0m] inner-join-empty-table-ungrouped-aggregate [2m(4.54ms)[0m

[1m[31mFailures:[39m[0m

[31m── left-join-empty-table-ungrouped-aggregate (sqlite-sqltests/join-empty-table-ungrouped-aggregate.sqltest) - :memory:[39m
   [31mpanic: cursor id 2 is None[39m

[31m── inner-join-empty-table-ungrouped-aggregate (sqlite-sqltests/join-empty-table-ungrouped-aggregate.sqltest) - :memory:[39m
   [31mpanic: cursor id 2 is None[39m

[1mSummary:[0m
  [31m2 errors[39m
  [2mTotal time: 9.80ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/sqlite-sqltests/join-empty-table-ungrouped-aggregate.sqltest`