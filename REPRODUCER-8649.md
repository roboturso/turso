# Reproducer: #8649 — Checkpoint from a stale page-size connection corrupts the database

https://github.com/tursodatabase/turso/issues/8649

## Environment

- Commit: 6d01be3604bbd2bc1066c2fef3f8049b98ed4523
- Platform: linux x86_64
- Reproduced at: 2026-09-23T18:10:43Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests checkpoint_stale_page_size::test_checkpoint_uses_database_page_size
```

Fails on the current tree; passes once the bug is fixed.

## What happens

A TRUNCATE checkpoint from a connection that still has PRAGMA page_size=1024 uses that size instead of the database's 4096-byte pages, which causes a short write and leaves a file SQLite reports as malformed.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-e1bb149c2239ec41)

running 1 test
2026-09-23T18:10:43.683396Z ERROR turso_core::storage::sqlite3_ondisk: write_pages_vectored: short write: wrote(12288) != expected(3072)
test checkpoint_stale_page_size::test_checkpoint_uses_database_page_size ... FAILED

failures:

---- checkpoint_stale_page_size::test_checkpoint_uses_database_page_size stdout ----

thread 'checkpoint_stale_page_size::test_checkpoint_uses_database_page_size' panicked at tests/integration/checkpoint_stale_page_size.rs:25:10:
called `Result::unwrap()` on an `Err` value: SqliteFailure(Error { code: DatabaseCorrupt, extended_code: 11 }, Some("malformed database schema (?)"))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    checkpoint_stale_page_size::test_checkpoint_uses_database_page_size

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1140 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/checkpoint_stale_page_size.rs`
- `tests/integration/mod.rs`