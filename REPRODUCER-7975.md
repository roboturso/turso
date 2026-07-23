# Reproducer: #7975 — REPLACE with delete trigger re-inserting the key corrupts UNIQUE index

https://github.com/tursodatabase/turso/issues/7975

## Environment

- Commit: fae58dc3ee11aaad12dff21bbb27089083231319
- Platform: linux x86_64
- Reproduced at: 2026-07-23T06:22:17Z

## Reproduce

```sh
cd sqlite/conformance && cargo run -q --manifest-path ../../testing/sqltest/Cargo.toml --bin sqltest -- run sqlite-sqltests/replace-delete-trigger-reinsert-unique.sqltest --backend rust --snapshot-filter __never__
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed on fae58dc3e: REPLACE with a delete trigger (via ON DELETE CASCADE) re-inserting the conflicting key succeeds without re-checking uniqueness, leaving two rows with b=1 and PRAGMA integrity_check reporting a non-unique entry in sqlite_autoindex_p1_1, whereas SQLite fails with 'UNIQUE constraint failed: p1.b'.

## Observed failure

RoboTurso ran the command above and observed:

```text
warning: unused import: `crate::translate::collate::CollationSeq`
  --> core/vdbe/mod.rs:42:9
   |
42 | pub use crate::translate::collate::CollationSeq;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

[1msqlite-sqltests/replace-delete-trigger-reinsert-unique.sqltest[0m
  [[31mFAIL[0m] replace-recursive-delete-trigger-reinsert-unique [2m(8.94ms)[0m
  [[31mFAIL[0m] replace-fk-cascade-trigger-reinsert-unique [2m(9.50ms)[0m

[1m[31mFailures:[39m[0m

[31m── replace-recursive-delete-trigger-reinsert-unique (sqlite-sqltests/replace-delete-trigger-reinsert-unique.sqltest) - :memory:[39m
   expected error but query succeeded

[31m── replace-fk-cascade-trigger-reinsert-unique (sqlite-sqltests/replace-delete-trigger-reinsert-unique.sqltest) - :memory:[39m
   expected error but query succeeded

[1mSummary:[0m
  [31m2 failed[39m
  [2mTotal time: 9.66ms[0m

[1m[31mSome tests failed.[39m[0m
```

## Files

- `sqlite/conformance/sqlite-sqltests/replace-delete-trigger-reinsert-unique.sqltest`