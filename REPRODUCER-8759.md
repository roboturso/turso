# Reproducer: #8759 — OR IGNORE crosses a trigger's DELETE, so a violation below it loses the row

https://github.com/tursodatabase/turso/issues/8759

## Environment

- Commit: 6e320d2e788f2c98333c43697c886f0f7ce9e8ff
- Platform: linux x86_64
- Reproduced at: 2026-09-22T17:09:03Z

## Reproduce

```sh
cargo run -q --manifest-path testing/sqltest/Cargo.toml --bin sqltest -- run sqlite/conformance/sqlite-sqltests/or-ignore-does-not-cross-delete-trigger.sqltest --backend rust && cargo test -q --test integration_tests trigger_or_ignore_nested_delete
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: INSERT OR IGNORE (and OR FAIL) lets the outer conflict policy cross a trigger's DELETE into the AFTER DELETE trigger, so the nested UPDATE's UNIQUE violation is ignored and the deleted row is lost, whereas SQLite aborts and rolls back the whole statement.

## Observed failure

RoboTurso ran the command above and observed:

```text
[1msqlite/conformance/sqlite-sqltests/or-ignore-does-not-cross-delete-trigger.sqltest[0m
  [[32mPASS[0m] or-ignore-still-reaches-a-trigger-update-directly [2m(11.02ms)[0m
  [[31mFAIL[0m] or-ignore-does-not-cross-delete-trigger  [2m(12.09ms)[0m

[1m[31mFailures:[39m[0m

[31m── or-ignore-does-not-cross-delete-trigger (sqlite/conformance/sqlite-sqltests/or-ignore-does-not-cross-delete-trigger.sqltest) - :memory:[39m
   expected error but query succeeded

[1mSummary:[0m
  [32m1 passed[39m, [31m1 failed[39m
  [2mTotal time: 12.28ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/sqlite-sqltests/or-ignore-does-not-cross-delete-trigger.sqltest`
- `tests/integration/trigger_or_ignore_nested_delete.rs`
- `tests/integration/mod.rs`