# Reproducer: #7918 — HAVING referencing an aggregate with a scalar subquery in its FILTER clause silently returns no rows

https://github.com/tursodatabase/turso/issues/7918

## Environment

- Commit: fae58dc3ee11aaad12dff21bbb27089083231319
- Platform: linux x86_64
- Reproduced at: 2026-07-23T06:15:22Z

## Reproduce

```sh
make -C sqlite/conformance run-rust ARGS="--snapshot-filter __never__ --filter 'issue-7918-*'"
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed on current main: `SELECT id FROM t GROUP BY id HAVING AVG(r) FILTER (WHERE (SELECT 1)) != 0` returns no rows in Turso while SQLite returns the row; the same aggregate works without HAVING, so the FILTER-subquery handling in the HAVING path silently drops rows.

## Observed failure

RoboTurso ran the command above and observed:

```text
make: Entering directory '/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7918/sqlite/conformance'
cargo build --manifest-path /home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7918/testing/sqltest/Cargo.toml 
warning: unused import: `crate::translate::collate::CollationSeq`
  --> core/vdbe/mod.rs:42:9
   |
42 | pub use crate::translate::collate::CollationSeq;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: `turso_core` (lib) generated 1 warning (run `cargo fix --lib -p turso_core` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
cargo run --manifest-path /home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7918/testing/sqltest/Cargo.toml --bin sqltest  run sqlite-sqltests --backend rust  --snapshot-filter __never__ --filter 'issue-7918-*'
warning: unused import: `crate::translate::collate::CollationSeq`
  --> core/vdbe/mod.rs:42:9
   |
42 | pub use crate::translate::collate::CollationSeq;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: `turso_core` (lib) generated 1 warning (run `cargo fix --lib -p turso_core` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
     Running `/home/penberg/src/tursodatabase/roboturso-turso/target/debug/sqltest run sqlite-sqltests --backend rust --snapshot-filter __never__ --filter 'issue-7918-*'`
Generating integrity-check fixtures...
Generating default databases...
[1msqlite-sqltests/having-aggregate-filter-subquery.sqltest[0m
  [[31mFAIL[0m] issue-7918-having-agg-filter-scalar-subquery [2m(4.47ms)[0m
  [[32mPASS[0m] issue-7918-agg-filter-scalar-subquery-no-having [2m(4.62ms)[0m

[1m[31mFailures:[39m[0m

[31m── issue-7918-having-agg-filter-scalar-subquery (sqlite-sqltests/having-aggregate-filter-subquery.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -1

[1mSummary:[0m
  [32m1 passed[39m, [31m1 failed[39m
  [2mTotal time: 9.34ms[0m

[1m[31mSome tests failed.[39m[0m
make: *** [Makefile:79: run-rust] Error 1
make: Leaving directory '/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7918/sqlite/conformance'
```

## Files

- `sqlite/conformance/sqlite-sqltests/having-aggregate-filter-subquery.sqltest`