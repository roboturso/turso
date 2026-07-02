# Reproducer: #5164 — SIGSEGV (UB) from from_utf8_unchecked when database contains non-UTF-8 TEXT

https://github.com/tursodatabase/turso/issues/5164

## Environment

- Commit: d4d6777a2685ab9a3ffc8733329743fdfe000c45
- Platform: linux x86_64
- Reproduced at: 2026-07-02T09:28:18Z

## Reproduce

```sh
cargo test -p core_tester test_like_on_non_utf8_text
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Reproduced: reading a TEXT value containing invalid UTF-8 (e.g. sqlite3's CAST(X'FF' AS TEXT)) hits from_utf8_unchecked in core/vdbe/mod.rs, which returns 'Internal error: Invalid UTF-8 in TEXT serial type' in debug builds and is UB (SIGSEGV) in release builds.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 26.32s
     Running unittests lib.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/core_tester-8e71706007abbd93)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 31 filtered out; finished in 0.00s

     Running fuzz/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/fuzz_tests-38abc9f832931d6b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 142 filtered out; finished in 0.00s

     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-2086b07511e41b85)

running 1 test
test query_processing::test_non_utf8_text::test_like_on_non_utf8_text ... FAILED

failures:

---- query_processing::test_non_utf8_text::test_like_on_non_utf8_text stdout ----

thread 'query_processing::test_non_utf8_text::test_like_on_non_utf8_text' panicked at tests/integration/query_processing/test_non_utf8_text.rs:9:1:
called `Result::unwrap()` on an `Err` value: Internal error: Invalid UTF-8 in TEXT serial type: invalid utf-8 sequence of 1 bytes from index 0
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    query_processing::test_non_utf8_text::test_like_on_non_utf8_text

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 962 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/query_processing/test_non_utf8_text.rs`
- `tests/integration/query_processing/mod.rs`