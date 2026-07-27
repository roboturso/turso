# Reproducer: #7976 — Scalar subquery over multi-row VALUES returns the last row instead of the first

https://github.com/tursodatabase/turso/issues/7976

## Environment

- Commit: 6372b5ab2131f0c1423fd56b1b946fb8e66acd14
- Platform: linux x86_64
- Reproduced at: 2026-07-27T08:29:39Z

## Reproduce

```sh
make -C sqlite/conformance run-rust ARGS='--snapshot-filter __never__ --filter scalar-subquery-multirow-values'
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: `SELECT (VALUES(1),(2),(3));` returns 3 in Turso instead of 1 as in SQLite, so a scalar subquery over a multi-row VALUES yields the last row rather than the first.

## Observed failure

RoboTurso ran the command above and observed:

```text
make: Entering directory '/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7976/sqlite/conformance'
cargo build --manifest-path /home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7976/testing/sqltest/Cargo.toml 
warning: unused import: `crate::translate::collate::CollationSeq`
  --> core/vdbe/mod.rs:43:9
   |
43 | pub use crate::translate::collate::CollationSeq;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: `turso_core` (lib) generated 1 warning (run `cargo fix --lib -p turso_core` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
cargo run --manifest-path /home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7976/testing/sqltest/Cargo.toml --bin sqltest  run sqlite-sqltests --backend rust  --snapshot-filter __never__ --filter scalar-subquery-multirow-values
warning: unused import: `crate::translate::collate::CollationSeq`
  --> core/vdbe/mod.rs:43:9
   |
43 | pub use crate::translate::collate::CollationSeq;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: `turso_core` (lib) generated 1 warning (run `cargo fix --lib -p turso_core` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `/home/penberg/src/tursodatabase/roboturso-turso/target/debug/sqltest run sqlite-sqltests --backend rust --snapshot-filter __never__ --filter scalar-subquery-multirow-values`
Generating integrity-check fixtures...
Generating default databases...
[1msqlite-sqltests/scalar-subquery-multirow-values.sqltest[0m
  [[31mFAIL[0m] scalar-subquery-multirow-values          [2m(2.66ms)[0m

[1m[31mFailures:[39m[0m

[31m── scalar-subquery-multirow-values (sqlite-sqltests/scalar-subquery-multirow-values.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -1
   +3

[1mSummary:[0m
  [31m1 failed[39m
  [2mTotal time: 6.76ms[0m

[1m[31mSome tests failed.[39m[0m
make: *** [Makefile:79: run-rust] Error 1
make: Leaving directory '/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7976/sqlite/conformance'
```

## Files

- `sqlite/conformance/sqlite-sqltests/scalar-subquery-multirow-values.sqltest`