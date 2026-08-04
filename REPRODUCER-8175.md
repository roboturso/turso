# Reproducer: #8175 — CREATE TEMP VIEW silently creates a persistent view in the main schema

https://github.com/tursodatabase/turso/issues/8175

## Environment

- Commit: 1dec7f943df24d6955a11b102b427f6c5b49937e
- Platform: linux x86_64
- Reproduced at: 2026-08-04T09:36:07Z

## Reproduce

```sh
cd sqlite/conformance && cargo run --manifest-path ../../testing/sqltest/Cargo.toml --bin sqltest run sqlite-sqltests/temp-view.sqltest --backend rust
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: CREATE TEMP VIEW persists the view into the main sqlite_schema instead of the temp schema (temp.tv is unresolvable and CREATE TEMP VIEW main.bad is not rejected), while SQLite keeps it session-scoped in sqlite_temp_schema.

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

warning: `turso_core` (lib) generated 1 warning (run `cargo fix --lib -p turso_core` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
     Running `/home/penberg/src/tursodatabase/roboturso-turso/target/debug/sqltest run sqlite-sqltests/temp-view.sqltest --backend rust`
[1msqlite-sqltests/temp-view.sqltest[0m
  [[31mFAIL[0m] create-temp-view-qualified-main-is-rejected [2m(5.15ms)[0m
  [[31mFAIL[0m] create-temporary-view-lives-in-temp-schema-not-main [2m(11.29ms)[0m
  [[31mFAIL[0m] create-temp-view-lives-in-temp-schema-not-main [2m(11.42ms)[0m

[1m[31mFailures:[39m[0m

[31m── create-temp-view-qualified-main-is-rejected (sqlite-sqltests/temp-view.sqltest) - :memory:[39m
   expected error but query succeeded

[31m── create-temporary-view-lives-in-temp-schema-not-main (sqlite-sqltests/temp-view.sqltest) - :memory:[39m
   --- expected
   +++ actual
   +1
    0
   -1
    2

[31m── create-temp-view-lives-in-temp-schema-not-main (sqlite-sqltests/temp-view.sqltest) - :memory:[39m
   expected success but got error: Parse error: no such table: tv

[1mSummary:[0m
  [31m3 failed[39m
  [2mTotal time: 11.57ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/sqlite-sqltests/temp-view.sqltest`