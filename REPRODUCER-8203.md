# Reproducer: #8203 — PRAGMA count_changes is a no-op

https://github.com/tursodatabase/turso/issues/8203

## Environment

- Commit: 2bdeb831796f62b4ff2f8393f93ddc1a17ebba50
- Platform: linux x86_64
- Reproduced at: 2026-08-05T07:05:33Z

## Reproduce

```sh
cargo run -q --bin sqltest -- run sqlite/conformance/sqlite-sqltests/pragma-count-changes.sqltest --backend rust
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: PRAGMA count_changes = on is accepted but INSERT/UPDATE/DELETE return no change-count rows, while sqlite3 returns them (e.g. 2 for a two-row INSERT); added a regression sqltest that fails on the current tree.

## Observed failure

RoboTurso ran the command above and observed:

```text
[1msqlite/conformance/sqlite-sqltests/pragma-count-changes.sqltest[0m
  [[31mFAIL[0m] pragma-count-changes-insert              [2m(6.20ms)[0m
  [[32mPASS[0m] pragma-count-changes-off-is-silent       [2m(6.41ms)[0m
  [[31mFAIL[0m] pragma-count-changes-update-delete       [2m(6.96ms)[0m

[1m[31mFailures:[39m[0m

[31m── pragma-count-changes-insert (sqlite/conformance/sqlite-sqltests/pragma-count-changes.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -2

[31m── pragma-count-changes-update-delete (sqlite/conformance/sqlite-sqltests/pragma-count-changes.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -3
   -2
   -3

[1mSummary:[0m
  [32m1 passed[39m, [31m2 failed[39m
  [2mTotal time: 7.15ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/sqlite-sqltests/pragma-count-changes.sqltest`