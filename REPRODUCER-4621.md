# Reproducer: #4621 — Panic: "attempt to negate with overflow"

https://github.com/tursodatabase/turso/issues/4621

## Environment

- Commit: d4d6777a2685ab9a3ffc8733329743fdfe000c45
- Platform: linux x86_64
- Reproduced at: 2026-07-02T06:56:29Z

## Reproduce

```sh
cd testing/sqltests && cargo run -q --bin test-runner -- run --backend rust --snapshot-filter __never__ tests/alter-add-column-default-negate-overflow.sqltest
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Reproduced the "attempt to negate with overflow" panic at core/translate/alter.rs:446: `ALTER TABLE t ADD COLUMN b DEFAULT -0x8000000000000000;` negates the hex literal that parses to i64::MIN without a checked_neg, whereas SQLite rejects this gracefully with "hex literal too big".

## Observed failure

RoboTurso ran the command above and observed:

```text
thread 'tokio-runtime-worker' panicked at core/translate/alter.rs:446:40:
attempt to negate with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[1mtests/alter-add-column-default-negate-overflow.sqltest[0m
  [[31;1mERROR[0m] alter-add-column-default-negate-min-int  [2m(5.11ms)[0m
  [[32mPASS[0m] alter-add-column-default-negate-ok       [2m(6.33ms)[0m

[1m[31mFailures:[39m[0m

[31m── alter-add-column-default-negate-min-int (tests/alter-add-column-default-negate-overflow.sqltest) - :memory:[39m
   [31mpanic: attempt to negate with overflow[39m

[1mSummary:[0m
  [32m1 passed[39m, [31m1 errors[39m
  [2mTotal time: 6.49ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `testing/sqltests/tests/alter-add-column-default-negate-overflow.sqltest`