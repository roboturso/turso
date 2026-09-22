# Reproducer: #8862 — PRAGMA query_only=1 still permits DROP TRIGGER and persistent user_version/application_id writes (core 0.8.0-pre.9)

https://github.com/tursodatabase/turso/issues/8862

## Environment

- Commit: e7e77e595725b2d3d6114a634e17cebaf1e9f3fb
- Platform: linux x86_64
- Reproduced at: 2026-09-22T14:16:06Z

## Reproduce

```sh
cd sqlite/conformance && cargo run -q --manifest-path ../../testing/sqltest/Cargo.toml --bin sqltest -- run sqlite-sqltests/pragma_query_only_rejects_metadata_and_drop_trigger.sqltest --backend rust
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed on current main: with PRAGMA query_only=1, PRAGMA user_version=…, PRAGMA application_id=…, and DROP TRIGGER all succeed and persist because DropTrigger is missing from the is_write list in core/translate/mod.rs and the cookie-writing pragmas never check query_only, whereas SQLite rejects all three.

## Observed failure

RoboTurso ran the command above and observed:

```text
[1msqlite-sqltests/pragma_query_only_rejects_metadata_and_drop_trigger.sqltest[0m
  [[32mPASS[0m] query-only-still-allows-reading-user-version [2m(8.57ms)[0m
  [[32mPASS[0m] query-only-rejects-insert                [2m(8.86ms)[0m
  [[31mFAIL[0m] query-only-rejects-application-id-write  [2m(9.07ms)[0m
  [[31mFAIL[0m] query-only-rejects-user-version-write    [2m(9.13ms)[0m
  [[31mFAIL[0m] query-only-rejects-drop-trigger          [2m(9.25ms)[0m

[1m[31mFailures:[39m[0m

[31m── query-only-rejects-application-id-write (sqlite-sqltests/pragma_query_only_rejects_metadata_and_drop_trigger.sqltest) - :memory:[39m
   expected error but query succeeded

[31m── query-only-rejects-user-version-write (sqlite-sqltests/pragma_query_only_rejects_metadata_and_drop_trigger.sqltest) - :memory:[39m
   expected error but query succeeded

[31m── query-only-rejects-drop-trigger (sqlite-sqltests/pragma_query_only_rejects_metadata_and_drop_trigger.sqltest) - :memory:[39m
   expected error but query succeeded

[1mSummary:[0m
  [32m2 passed[39m, [31m3 failed[39m
  [2mTotal time: 9.49ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/sqlite-sqltests/pragma_query_only_rejects_metadata_and_drop_trigger.sqltest`