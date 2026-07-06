# Reproducer: #7475 — MVCC recovery detects virtual tables by substring-matching schema SQL and panics with  Corrupt database: sqlite_schema root_page must be 0 for table, got -3

https://github.com/tursodatabase/turso/issues/7475

## Environment

- Commit: d4d6777a2685ab9a3ffc8733329743fdfe000c45
- Platform: linux x86_64
- Reproduced at: 2026-07-06T11:25:56Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests mvcc::test_recover_table_with_create_virtual_substring_in_sql
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: MVCC log recovery misclassifies any table whose schema SQL contains the substring "create virtual" (e.g. in a column DEFAULT literal) as a virtual table, so its negative pre-checkpoint root page triggers the 'sqlite_schema root_page must be 0 for table, got -3' corruption error on reopen.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-2086b07511e41b85)

running 1 test
test mvcc::test_recover_table_with_create_virtual_substring_in_sql ... FAILED

failures:

---- mvcc::test_recover_table_with_create_virtual_substring_in_sql stdout ----

thread 'mvcc::test_recover_table_with_create_virtual_substring_in_sql' panicked at tests/integration/mvcc.rs:1033:1:
called `Result::unwrap()` on an `Err` value: Corrupt database: sqlite_schema root_page must be 0 for table, got -3
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    mvcc::test_recover_table_with_create_virtual_substring_in_sql

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 962 filtered out; finished in 0.02s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/mvcc.rs`