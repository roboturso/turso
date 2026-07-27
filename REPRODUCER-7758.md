# Reproducer: #7758 — json_insert() and json_replace() accept path-only calls that SQLite rejects

https://github.com/tursodatabase/turso/issues/7758

## Environment

- Commit: a2b7c8d02b14336d84b2c1859095b14c4363b03d
- Platform: linux x86_64
- Reproduced at: 2026-07-27T06:54:26Z

## Reproduce

```sh
make -C sqlite/conformance run-one FILE=sqlite-sqltests/json/json-insert-replace-odd-args.sqltest
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed that json_insert, json_replace, jsonb_insert, and jsonb_replace with a document and a path but no value silently return the unchanged document instead of raising SQLite's 'needs an odd number of arguments' error.

## Observed failure

RoboTurso ran the command above and observed:

```text
make: Entering directory '/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7758/sqlite/conformance'
cd /home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7758 && cargo build  --package turso_cli --features test_helper
warning: unused import: `crate::translate::collate::CollationSeq`
  --> core/vdbe/mod.rs:43:9
   |
43 | pub use crate::translate::collate::CollationSeq;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: `turso_core` (lib) generated 1 warning (run `cargo fix --lib -p turso_core` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
cargo build --manifest-path /home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7758/testing/sqltest/Cargo.toml 
warning: unused import: `crate::translate::collate::CollationSeq`
  --> core/vdbe/mod.rs:43:9
   |
43 | pub use crate::translate::collate::CollationSeq;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: `turso_core` (lib) generated 1 warning (run `cargo fix --lib -p turso_core` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
cargo run --manifest-path /home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7758/testing/sqltest/Cargo.toml --bin sqltest  run sqlite-sqltests/json/json-insert-replace-odd-args.sqltest --binary /home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7758/target/debug/tursodb 
warning: unused import: `crate::translate::collate::CollationSeq`
  --> core/vdbe/mod.rs:43:9
   |
43 | pub use crate::translate::collate::CollationSeq;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: `turso_core` (lib) generated 1 warning (run `cargo fix --lib -p turso_core` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `/home/penberg/src/tursodatabase/roboturso-turso/target/debug/sqltest run sqlite-sqltests/json/json-insert-replace-odd-args.sqltest --binary /home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7758/target/debug/tursodb`
[1msqlite-sqltests/json/json-insert-replace-odd-args.sqltest[0m
  [[31mFAIL[0m] json-replace-path-only-errors            [2m(4.38ms)[0m
  [[32mPASS[0m] json-insert-pair-still-works             [2m(4.33ms)[0m
  [[31mFAIL[0m] json-insert-path-only-errors             [2m(4.37ms)[0m
  [[31mFAIL[0m] jsonb-insert-path-only-errors            [2m(4.36ms)[0m
  [[31mFAIL[0m] jsonb-replace-path-only-errors           [2m(4.33ms)[0m
  [[32mPASS[0m] json-replace-pair-still-works            [2m(4.35ms)[0m

[1m[31mFailures:[39m[0m

[31m── json-replace-path-only-errors (sqlite-sqltests/json/json-insert-replace-odd-args.sqltest) - :memory:[39m
   expected error but query succeeded

[31m── json-insert-path-only-errors (sqlite-sqltests/json/json-insert-replace-odd-args.sqltest) - :memory:[39m
   expected error but query succeeded

[31m── jsonb-insert-path-only-errors (sqlite-sqltests/json/json-insert-replace-odd-args.sqltest) - :memory:[39m
   expected error but query succeeded

[31m── jsonb-replace-path-only-errors (sqlite-sqltests/json/json-insert-replace-odd-args.sqltest) - :memory:[39m
   expected error but query succeeded

[1mSummary:[0m
  [32m2 passed[39m, [31m4 failed[39m
  [2mTotal time: 4.60ms[0m

[1m[31mSome tests failed.[39m[0m
make: *** [Makefile:101: run-one] Error 1
make: Leaving directory '/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7758/sqlite/conformance'
```

## Files

- `sqlite/conformance/sqlite-sqltests/json/json-insert-replace-odd-args.sqltest`