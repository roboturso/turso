# Reproducer: #9069 — CDC: the `before` record of an upsert shows the new value instead of the old value

https://github.com/tursodatabase/turso/issues/9069

## Environment

- Commit: 6e320d2e788f2c98333c43697c886f0f7ce9e8ff
- Platform: linux x86_64
- Reproduced at: 2026-09-22T16:30:41Z

## Reproduce

```sh
cd sqlite/conformance && cargo run -q --manifest-path ../../testing/sqltest/Cargo.toml --bin sqltest -- run turso-sqltests/cdc-upsert-before-record.sqltest --backend rust
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed on the current tree: the upsert's CDC before record holds the new value ('000456') instead of the old one ('1.5') because the UPDATE path reads the before-image back through the table cursor after the row has already been rewritten, rather than from the saved before registers.

## Observed failure

RoboTurso ran the command above and observed:

```text
[1mturso-sqltests/cdc-upsert-before-record.sqltest[0m
  [[31mFAIL[0m] cdc-upsert-before-record-holds-old-value [2m(12.27ms)[0m

[1m[31mFailures:[39m[0m

[31m── cdc-upsert-before-record-holds-old-value (turso-sqltests/cdc-upsert-before-record.sqltest) - :memory:[39m
   --- expected
   +++ actual
    0|{"id":1,"value":"000123"}|{"id":1,"value":"1.5"}
   -0|{"id":1,"value":"1.5"}|{"id":1,"value":"000456"}
   +0|{"id":1,"value":"000456"}|{"id":1,"value":"000456"}

[1mSummary:[0m
  [31m1 failed[39m
  [2mTotal time: 12.51ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/turso-sqltests/cdc-upsert-before-record.sqltest`