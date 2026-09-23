# Reproducer: #8699 — ADD COLUMN drops DESC from a primary key and leaves the file unopenable

https://github.com/tursodatabase/turso/issues/8699

## Environment

- Commit: 6d01be3604bbd2bc1066c2fef3f8049b98ed4523
- Platform: linux x86_64
- Reproduced at: 2026-09-23T18:02:12Z

## Reproduce

```sh
cargo run -q --bin sqltest -- run sqlite/conformance/sqlite-sqltests/alter-column-keeps-pk-desc.sqltest --backend rust
```

Fails on the current tree; passes once the bug is fixed.

## What happens

ALTER TABLE ADD/DROP COLUMN rewrites the stored CREATE TABLE without the primary key's DESC, which leaves an unused automatic index, and reopening the database then panics in core/schema.rs.

## Observed failure

RoboTurso ran the command above and observed:

```text
[1msqlite/conformance/sqlite-sqltests/alter-column-keeps-pk-desc.sqltest[0m
  [[31mFAIL[0m] add-column-keeps-desc-on-text-primary-key [2m(10.16ms)[0m
  [[31mFAIL[0m] add-column-keeps-desc-on-integer-primary-key [2m(10.30ms)[0m
  [[31mFAIL[0m] drop-column-keeps-desc-on-integer-primary-key [2m(11.49ms)[0m

[1m[31mFailures:[39m[0m

[31m── add-column-keeps-desc-on-text-primary-key (sqlite/conformance/sqlite-sqltests/alter-column-keeps-pk-desc.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -1
   +0

[31m── add-column-keeps-desc-on-integer-primary-key (sqlite/conformance/sqlite-sqltests/alter-column-keeps-pk-desc.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -1
   +0

[31m── drop-column-keeps-desc-on-integer-primary-key (sqlite/conformance/sqlite-sqltests/alter-column-keeps-pk-desc.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -1
   +0

[1mSummary:[0m
  [31m3 failed[39m
  [2mTotal time: 11.73ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/sqlite-sqltests/alter-column-keeps-pk-desc.sqltest`