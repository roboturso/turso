# Reproducer: #7934 — Confusing error message on binary operand arity mismatch

https://github.com/tursodatabase/turso/issues/7934

## Environment

- Commit: c267663bbb43f6b318f0374c53c0baffb31f63a9
- Platform: linux x86_64
- Reproduced at: 2026-07-21T04:54:27Z

## Reproduce

```sh
cargo run -q --bin sqltest -- run sqlite/conformance/sqlite-sqltests/row-value-arity-mismatch-error.sqltest --backend rust
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: `SELECT count(*) > ('abc', 'def')` produces `all arguments to binary operator > must return the same number of values. Got: (1) > (2)` instead of SQLite's `row value misused`; added a regression sqltest expecting the SQLite-compatible message.

## Observed failure

RoboTurso ran the command above and observed:

```text
[1msqlite/conformance/sqlite-sqltests/row-value-arity-mismatch-error.sqltest[0m
  [[31mFAIL[0m] binary-op-row-value-arity-mismatch-eq-error-message [2m(8.76ms)[0m
  [[31mFAIL[0m] binary-op-row-value-arity-mismatch-error-message [2m(8.79ms)[0m
  [[31mFAIL[0m] binary-op-row-value-arity-mismatch-lhs-error-message [2m(8.78ms)[0m

[1m[31mFailures:[39m[0m

[31m── binary-op-row-value-arity-mismatch-eq-error-message (sqlite/conformance/sqlite-sqltests/row-value-arity-mismatch-error.sqltest) - :memory:[39m
   error message 'Parse error: all arguments to binary operator = must return the same number of values. Got: (1) = (2)' does not contain expected pattern 'row value misused'

[31m── binary-op-row-value-arity-mismatch-error-message (sqlite/conformance/sqlite-sqltests/row-value-arity-mismatch-error.sqltest) - :memory:[39m
   error message 'Parse error: all arguments to binary operator > must return the same number of values. Got: (1) > (2)' does not contain expected pattern 'row value misused'

[31m── binary-op-row-value-arity-mismatch-lhs-error-message (sqlite/conformance/sqlite-sqltests/row-value-arity-mismatch-error.sqltest) - :memory:[39m
   error message 'Parse error: all arguments to binary operator < must return the same number of values. Got: (2) < (1)' does not contain expected pattern 'row value misused'

[1mSummary:[0m
  [31m3 failed[39m
  [2mTotal time: 9.01ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/sqlite-sqltests/row-value-arity-mismatch-error.sqltest`