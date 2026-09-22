# Reproducer: #6864 — Read-Only JSON Parse Error Rolls Back the Active Transaction

https://github.com/tursodatabase/turso/issues/6864

## Environment

- Commit: a012590f552751754ea18a4226432664e06055ea
- Platform: linux x86_64
- Reproduced at: 2026-09-22T18:13:35Z

## Reproduce

```sh
cargo test -q -p core_tester --test integration_tests json_parse_error_in_select_keeps_transaction_open
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: after a malformed-JSON error in a SELECT inside an open transaction, Turso rolls back the transaction (count(*) returns 0 and COMMIT reports no active transaction), whereas SQLite keeps the transaction open and returns 1.

## Observed failure

RoboTurso ran the command above and observed:

```text
running 1 test
json_error_keeps_transaction::json_parse_error_in_select_keeps_transaction_open --- FAILED

failures:

---- json_error_keeps_transaction::json_parse_error_in_select_keeps_transaction_open stdout ----

thread 'json_error_keeps_transaction::json_parse_error_in_select_keeps_transaction_open' panicked at tests/integration/json_error_keeps_transaction.rs:20:5:
assertion `left == right` failed: the INSERT must still be visible inside the transaction after the SELECT failed
  left: [[Integer(0)]]
 right: [[Integer(1)]]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    json_error_keeps_transaction::json_parse_error_in_select_keeps_transaction_open

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1138 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/json_error_keeps_transaction.rs`
- `tests/integration/mod.rs`