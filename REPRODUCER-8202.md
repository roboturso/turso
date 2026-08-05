# Reproducer: #8202 — FULL JOIN emits spurious all-NULL row when WHERE filters out matched rows

https://github.com/tursodatabase/turso/issues/8202

## Environment

- Commit: 2bdeb831796f62b4ff2f8393f93ddc1a17ebba50
- Platform: linux x86_64
- Reproduced at: 2026-08-05T07:09:54Z

## Reproduce

```sh
make -C sqlite/conformance run-rust ARGS='--snapshot-filter __never__ --filter full-join-where-filters-matched-row'
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed on the current tree: the FULL JOIN query returns a spurious all-NULL row in Turso while SQLite returns no rows, because a WHERE-filtered matched row is treated as unmatched.

## Observed failure

RoboTurso ran the command above and observed:

```text
make: Entering directory '/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8202/sqlite/conformance'
cargo build --manifest-path /home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8202/testing/sqltest/Cargo.toml 
warning: unused import: `crate::translate::collate::CollationSeq`
  --> core/vdbe/mod.rs:43:9
   |
43 | pub use crate::translate::collate::CollationSeq;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: `turso_core` (lib) generated 1 warning (run `cargo fix --lib -p turso_core` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
cargo run --manifest-path /home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8202/testing/sqltest/Cargo.toml --bin sqltest  run sqlite-sqltests --backend rust  --snapshot-filter __never__ --filter full-join-where-filters-matched-row
warning: unused import: `crate::translate::collate::CollationSeq`
  --> core/vdbe/mod.rs:43:9
   |
43 | pub use crate::translate::collate::CollationSeq;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: `turso_core` (lib) generated 1 warning (run `cargo fix --lib -p turso_core` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `/home/penberg/src/tursodatabase/roboturso-turso/target/debug/sqltest run sqlite-sqltests --backend rust --snapshot-filter __never__ --filter full-join-where-filters-matched-row`
Generating integrity-check fixtures...
Generating default databases...
[1msqlite-sqltests/full-join-where-filters-matched-row.sqltest[0m
  [[31mFAIL[0m] full-join-where-filters-matched-row      [2m(5.36ms)[0m

[1m[31mFailures:[39m[0m

[31m── full-join-where-filters-matched-row (sqlite-sqltests/full-join-where-filters-matched-row.sqltest) - :memory:[39m
   --- expected
   +++ actual
   +|

[1mSummary:[0m
  [31m1 failed[39m
  [2mTotal time: 18.25ms[0m

[1m[31mSome tests failed.[39m[0m
make: *** [Makefile:79: run-rust] Error 1
make: Leaving directory '/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8202/sqlite/conformance'
```

## Files

- `sqlite/conformance/sqlite-sqltests/full-join-where-filters-matched-row.sqltest`