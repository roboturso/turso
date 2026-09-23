# Reproducer: #9285 — MVCC: an index keeps the old key after an UPDATE to a value that compares as equal

https://github.com/tursodatabase/turso/issues/9285

## Environment

- Commit: 6d01be3604bbd2bc1066c2fef3f8049b98ed4523
- Platform: linux x86_64
- Reproduced at: 2026-09-23T18:30:34Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests mvcc_index_equal_key_update
```

Fails on the current tree; passes once the bug is fixed.

## What happens

In MVCC mode, an UPDATE to a value the index treats as equal (a case change under NOCASE, or 1 to 1.0) leaves the old key in the index, so queries that use the index return the old value; WAL mode returns the new value.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-e1bb149c2239ec41)

running 2 tests
test mvcc_index_equal_key_update::mvcc_update_to_nocase_equal_value_replaces_index_key ... FAILED
test mvcc_index_equal_key_update::mvcc_update_integer_to_equal_real_replaces_index_key ... FAILED

failures:

---- mvcc_index_equal_key_update::mvcc_update_to_nocase_equal_value_replaces_index_key stdout ----

thread 'mvcc_index_equal_key_update::mvcc_update_to_nocase_equal_value_replaces_index_key' panicked at tests/integration/mvcc_index_equal_key_update.rs:21:5:
assertion `left == right` failed
  left: [[Text("bob@example.com")]]
 right: [[Text("Bob@Example.com")]]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- mvcc_index_equal_key_update::mvcc_update_integer_to_equal_real_replaces_index_key stdout ----

thread 'mvcc_index_equal_key_update::mvcc_update_integer_to_equal_real_replaces_index_key' panicked at tests/integration/mvcc_index_equal_key_update.rs:42:5:
assertion `left == right` failed
  left: [[Integer(1), Text("integer")]]
 right: [[Real(1.0), Text("real")]]


failures:
    mvcc_index_equal_key_update::mvcc_update_integer_to_equal_real_replaces_index_key
    mvcc_index_equal_key_update::mvcc_update_to_nocase_equal_value_replaces_index_key

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 1140 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/mvcc_index_equal_key_update.rs`
- `tests/integration/mod.rs`