# Reproducer: #7944 — Turso silently ignores NULLS FIRST/LAST in CREATE INDEX statements

https://github.com/tursodatabase/turso/issues/7944

## Environment

- Commit: fae58dc3ee11aaad12dff21bbb27089083231319
- Platform: linux x86_64
- Reproduced at: 2026-07-23T05:09:58Z

## Reproduce

```sh
make -C sqlite/conformance run-rust ARGS="--snapshot-filter __never__ --filter 'create-index-nulls-*'"
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: Turso silently accepts NULLS FIRST/LAST in CREATE INDEX and writes a schema entry that SQLite then rejects as malformed; added a failing .sqltest regression covering NULLS FIRST, NULLS LAST, and DESC NULLS FIRST.

## Observed failure

RoboTurso ran the command above and observed:

```text
make: Entering directory '/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7944/sqlite/conformance'
cargo build --manifest-path /home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7944/testing/sqltest/Cargo.toml 
warning: unused import: `crate::translate::collate::CollationSeq`
  --> core/vdbe/mod.rs:42:9
   |
42 | pub use crate::translate::collate::CollationSeq;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: `turso_core` (lib) generated 1 warning (run `cargo fix --lib -p turso_core` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
cargo run --manifest-path /home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7944/testing/sqltest/Cargo.toml --bin sqltest  run sqlite-sqltests --backend rust  --snapshot-filter __never__ --filter 'create-index-nulls-*'
warning: unused import: `crate::translate::collate::CollationSeq`
  --> core/vdbe/mod.rs:42:9
   |
42 | pub use crate::translate::collate::CollationSeq;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: `turso_core` (lib) generated 1 warning (run `cargo fix --lib -p turso_core` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
     Running `/home/penberg/src/tursodatabase/roboturso-turso/target/debug/sqltest run sqlite-sqltests --backend rust --snapshot-filter __never__ --filter 'create-index-nulls-*'`
Generating integrity-check fixtures...
Generating default databases...
[1msqlite-sqltests/create-index-nulls-ordering-error.sqltest[0m
  [[31mFAIL[0m] create-index-nulls-first-with-desc-rejected [2m(4.89ms)[0m
  [[31mFAIL[0m] create-index-nulls-first-rejected        [2m(4.98ms)[0m
  [[31mFAIL[0m] create-index-nulls-last-rejected         [2m(5.22ms)[0m

[1m[31mFailures:[39m[0m

[31m── create-index-nulls-first-with-desc-rejected (sqlite-sqltests/create-index-nulls-ordering-error.sqltest) - :memory:[39m
   expected error but query succeeded

[31m── create-index-nulls-first-rejected (sqlite-sqltests/create-index-nulls-ordering-error.sqltest) - :memory:[39m
   expected error but query succeeded

[31m── create-index-nulls-last-rejected (sqlite-sqltests/create-index-nulls-ordering-error.sqltest) - :memory:[39m
   expected error but query succeeded

[1mSummary:[0m
  [31m3 failed[39m
  [2mTotal time: 8.65ms[0m

[1m[31mSome tests failed.[39m[0m
make: *** [Makefile:79: run-rust] Error 1
make: Leaving directory '/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7944/sqlite/conformance'
```

## Files

- `sqlite/conformance/sqlite-sqltests/create-index-nulls-ordering-error.sqltest`