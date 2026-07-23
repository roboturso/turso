# Reproducer: #7978 — RIGHT/FULL OUTER JOIN: missing NULL-extended rows and misplaced output columns

https://github.com/tursodatabase/turso/issues/7978

## Environment

- Commit: 6b713c27c34e1fa554dd126b5d6abf9055f7b62a
- Platform: linux x86_64
- Reproduced at: 2026-07-23T09:09:59Z

## Reproduce

```sh
make -C sqlite/conformance run-one FILE=sqlite-sqltests/right-full-join-null-extended-rows.sqltest
```

Fails on the current tree; passes once the bug is fixed.

## What happens

All three divergences reproduce on the current tree: the FULL OUTER JOIN query returns no rows instead of `||1`, the RIGHT JOIN + LEFT JOIN USING query returns `||3` instead of `|3|`, and unmatched right-table rows are emitted after (rather than interleaved with) the scan order SQLite uses.

## Observed failure

RoboTurso ran the command above and observed:

```text
make: Entering directory '/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7978/sqlite/conformance'
cd /home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7978 && cargo build  --package turso_cli --features test_helper
warning: unused import: `crate::translate::collate::CollationSeq`
  --> core/vdbe/mod.rs:42:9
   |
42 | pub use crate::translate::collate::CollationSeq;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: `turso_core` (lib) generated 1 warning (run `cargo fix --lib -p turso_core` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.39s
cargo build --manifest-path /home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7978/testing/sqltest/Cargo.toml 
warning: unused import: `crate::translate::collate::CollationSeq`
  --> core/vdbe/mod.rs:42:9
   |
42 | pub use crate::translate::collate::CollationSeq;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: `turso_core` (lib) generated 1 warning (run `cargo fix --lib -p turso_core` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
cargo run --manifest-path /home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7978/testing/sqltest/Cargo.toml --bin sqltest  run sqlite-sqltests/right-full-join-null-extended-rows.sqltest --binary /home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7978/target/debug/tursodb 
warning: unused import: `crate::translate::collate::CollationSeq`
  --> core/vdbe/mod.rs:42:9
   |
42 | pub use crate::translate::collate::CollationSeq;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: `turso_core` (lib) generated 1 warning (run `cargo fix --lib -p turso_core` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `/home/penberg/src/tursodatabase/roboturso-turso/target/debug/sqltest run sqlite-sqltests/right-full-join-null-extended-rows.sqltest --binary /home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7978/target/debug/tursodb`
[1msqlite-sqltests/right-full-join-null-extended-rows.sqltest[0m
  [[31mFAIL[0m] right-join-unmatched-row-order           [2m(11.10ms)[0m
  [[31mFAIL[0m] full-outer-join-null-extended-row-on-0   [2m(12.18ms)[0m
  [[31mFAIL[0m] full-outer-join-null-extended-row-on-1eq0 [2m(12.88ms)[0m
  [[31mFAIL[0m] right-join-then-left-join-using-column-order [2m(13.00ms)[0m
  [[31mFAIL[0m] full-outer-join-null-extended-row-on-false [2m(17.50ms)[0m

[1m[31mFailures:[39m[0m

[31m── right-join-unmatched-row-order (sqlite-sqltests/right-full-join-null-extended-rows.sqltest) - :memory:[39m
   --- expected
   +++ actual
    a|a
   -|c
   -|
   +|
   +|c

[31m── full-outer-join-null-extended-row-on-0 (sqlite-sqltests/right-full-join-null-extended-rows.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -||1

[31m── full-outer-join-null-extended-row-on-1eq0 (sqlite-sqltests/right-full-join-null-extended-rows.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -||1

[31m── right-join-then-left-join-using-column-order (sqlite-sqltests/right-full-join-null-extended-rows.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -|3|
   +||3

[31m── full-outer-join-null-extended-row-on-false (sqlite-sqltests/right-full-join-null-extended-rows.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -||1

[1mSummary:[0m
  [31m5 failed[39m
  [2mTotal time: 17.67ms[0m

[1m[31mSome tests failed.[39m[0m
make: *** [Makefile:101: run-one] Error 1
make: Leaving directory '/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7978/sqlite/conformance'
```

## Files

- `sqlite/conformance/sqlite-sqltests/right-full-join-null-extended-rows.sqltest`