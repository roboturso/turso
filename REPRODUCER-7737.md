# Reproducer: #7737 — ALTER TABLE Error Message Omits Table Qualifier for Column References

https://github.com/tursodatabase/turso/issues/7737

## Environment

- Commit: d4d6777a2685ab9a3ffc8733329743fdfe000c45
- Platform: linux x86_64
- Reproduced at: 2026-07-03T17:13:54Z

## Reproduce

```sh
cargo run -q --manifest-path testing/sqltests/Cargo.toml --bin test-runner -- run testing/sqltests/tests/alter-rename-column-qualified-trigger-error.sqltest --backend rust
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: when a trigger references a renamed column via a table-qualified name (t.b), Turso's ALTER TABLE error reports only the bare column name 'b' while SQLite reports 't.b' as written.

## Observed failure

RoboTurso ran the command above and observed:

```text
[1mtesting/sqltests/tests/alter-rename-column-qualified-trigger-error.sqltest[0m
  [[31mFAIL[0m] rename-column-trigger-when-qualified-ref-error-message [2m(13.78ms)[0m

[1m[31mFailures:[39m[0m

[31m── rename-column-trigger-when-qualified-ref-error-message (testing/sqltests/tests/alter-rename-column-qualified-trigger-error.sqltest) - :memory:[39m
   error message 'Parse error: error in trigger tr after rename: no such column: b' does not contain expected pattern 'no such column: t\.b'

[1mSummary:[0m
  [31m1 failed[39m
  [2mTotal time: 13.98ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `testing/sqltests/tests/alter-rename-column-qualified-trigger-error.sqltest`