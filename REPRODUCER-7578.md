# Reproducer: #7578 — panics with index finger diverged from query_btree_version_is_valid

https://github.com/tursodatabase/turso/issues/7578

## Environment

- Commit: d4d6777a2685ab9a3ffc8733329743fdfe000c45
- Platform: linux x86_64
- Reproduced at: 2026-07-06T11:50:10Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests test_mvcc_index_scan_does_not_return_row_deleted_mid_scan
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Reproduced on current main: deleting a later durable index key while an MVCC index scan cursor is open panics with the `index finger diverged from query_btree_version_is_valid` assertion in core/mvcc/cursor.rs.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-2086b07511e41b85)

running 1 test
test mvcc::test_mvcc_index_scan_does_not_return_row_deleted_mid_scan ... FAILED

failures:

---- mvcc::test_mvcc_index_scan_does_not_return_row_deleted_mid_scan stdout ----

thread 'mvcc::test_mvcc_index_scan_does_not_return_row_deleted_mid_scan' panicked at core/mvcc/cursor.rs:570:9:
assertion `left == right` failed: index finger diverged from query_btree_version_is_valid
  left: true
 right: false
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    mvcc::test_mvcc_index_scan_does_not_return_row_deleted_mid_scan

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 962 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/mvcc.rs`