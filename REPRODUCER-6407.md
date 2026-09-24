# Reproducer: #6407 — Compound SELECT (3+ arms) Drops First-Arm Collation

https://github.com/tursodatabase/turso/issues/6407

## Environment

- Commit: c0c6f663109382ab770fe8b98a9622e9ef81ca64
- Platform: linux x86_64
- Reproduced at: 2026-09-24T12:46:25Z

## Reproduce

```sh
make -C sqlite/conformance run-rust SQLITE_TESTS=sqlite-sqltests/compound-select-three-arms-keeps-collation.sqltest TURSO_TESTS=sqlite-sqltests/compound-select-three-arms-keeps-collation.sqltest ARGS='--snapshot-filter __never__'
```

Fails on the current tree; passes once the bug is fixed.

## What happens

A UNION with three arms ignores the first arm's COLLATE NOCASE and returns HELLO, Hello and hello, while SQLite returns only HELLO.

## Observed failure

RoboTurso ran the command above and observed:

```text
make: Entering directory '/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-6407/sqlite/conformance'
cargo build --manifest-path /home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-6407/testing/sqltest/Cargo.toml 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
cargo run --manifest-path /home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-6407/testing/sqltest/Cargo.toml --bin sqltest  run sqlite-sqltests/compound-select-three-arms-keeps-collation.sqltest --backend rust  --snapshot-filter __never__
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `/home/penberg/src/tursodatabase/roboturso-turso/target/debug/sqltest run sqlite-sqltests/compound-select-three-arms-keeps-collation.sqltest --backend rust --snapshot-filter __never__`
[1msqlite-sqltests/compound-select-three-arms-keeps-collation.sqltest[0m
  [[31mFAIL[0m] union-three-arms-keeps-first-arm-collation-count [2m(5.55ms)[0m
  [[31mFAIL[0m] union-three-arms-keeps-first-arm-collation [2m(5.55ms)[0m

[1m[31mFailures:[39m[0m

[31m── union-three-arms-keeps-first-arm-collation-count (sqlite-sqltests/compound-select-three-arms-keeps-collation.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -1
   +3

[31m── union-three-arms-keeps-first-arm-collation (sqlite-sqltests/compound-select-three-arms-keeps-collation.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -HELLO
   +HELLO
   +Hello
   +hello

[1mSummary:[0m
  [31m2 failed[39m
  [2mTotal time: 5.80ms[0m

[1m[31mSome tests failed.[39m[0m
make: *** [Makefile:79: run-rust] Error 1
make: Leaving directory '/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-6407/sqlite/conformance'
```

## Files

- `sqlite/conformance/sqlite-sqltests/compound-select-three-arms-keeps-collation.sqltest`