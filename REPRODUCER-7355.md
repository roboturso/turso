# Reproducer: #7355 — Hex Blob Literal in a Materialized View Definition Panics During Emission

https://github.com/tursodatabase/turso/issues/7355

## Environment

- Commit: ca8933a2e32b74edf6e24650c4f6ace9f0ad6f3c
- Platform: linux x86_64
- Reproduced at: 2026-09-22T11:18:43Z

## Reproduce

```sh
cd sqlite/conformance && cargo run -q --manifest-path ../../testing/sqltest/Cargo.toml --bin sqltest -- run turso-sqltests/matview_blob_literal.sqltest --backend rust
```

Fails on the current tree; passes once the bug is fixed.

## What happens

The panic itself no longer occurs since b56f7b865, but the same hex round trip still mishandles the blob: a materialized view over `SELECT x'FF'` returns the 2-byte blob 0x4646 (the text "FF") instead of 0xFF, because core/translate/logical.rs:1934 uses the hex digits as raw bytes instead of decoding them.

## Observed failure

RoboTurso ran the command above and observed:

```text
[1mturso-sqltests/matview_blob_literal.sqltest[0m
  [[31mFAIL[0m] matview-blob-literal-select-star         [2m(13.11ms)[0m
  [[31mFAIL[0m] matview-blob-literal-type-and-bytes      [2m(18.01ms)[0m

[1m[31mFailures:[39m[0m

[31m── matview-blob-literal-select-star (turso-sqltests/matview_blob_literal.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -FF
   +4646

[31m── matview-blob-literal-type-and-bytes (turso-sqltests/matview_blob_literal.sqltest) - :memory:[39m
   --- expected
   +++ actual
   -1|blob|FF|1|blob|0102AB|3
   +1|blob|4646|2|blob|303130326162|6

[1mSummary:[0m
  [31m2 failed[39m
  [2mTotal time: 18.19ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/turso-sqltests/matview_blob_literal.sqltest`