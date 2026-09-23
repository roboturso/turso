# Reproducer: #9284 — MVCC: a dropped trigger or view comes back after a checkpoint

https://github.com/tursodatabase/turso/issues/9284

## Environment

- Commit: 6d01be3604bbd2bc1066c2fef3f8049b98ed4523
- Platform: linux x86_64
- Reproduced at: 2026-09-23T18:27:12Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests mvcc_dropped_schema_object_after_checkpoint
```

Fails on the current tree; passes once the bug is fixed.

## What happens

In MVCC mode, the checkpoint does not write the delete of a dropped trigger or view to the database file, so the object is back in sqlite_schema after the checkpoint and the dropped trigger fires again after a restart.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-e1bb149c2239ec41)

running 3 tests
test mvcc_dropped_schema_object_after_checkpoint::test_mvcc_dropped_view_stays_dropped_after_rename_and_checkpoint ... FAILED
test mvcc_dropped_schema_object_after_checkpoint::test_mvcc_dropped_trigger_stays_dropped_after_rename_and_checkpoint ... FAILED
test mvcc_dropped_schema_object_after_checkpoint::test_mvcc_dropped_trigger_stays_dropped_after_restart_and_checkpoint ... FAILED

failures:

---- mvcc_dropped_schema_object_after_checkpoint::test_mvcc_dropped_view_stays_dropped_after_rename_and_checkpoint stdout ----

thread 'mvcc_dropped_schema_object_after_checkpoint::test_mvcc_dropped_view_stays_dropped_after_rename_and_checkpoint' panicked at tests/integration/mvcc_dropped_schema_object_after_checkpoint.rs:52:5:
assertion `left == right` failed
  left: [("v",)]
 right: []
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- mvcc_dropped_schema_object_after_checkpoint::test_mvcc_dropped_trigger_stays_dropped_after_rename_and_checkpoint stdout ----

thread 'mvcc_dropped_schema_object_after_checkpoint::test_mvcc_dropped_trigger_stays_dropped_after_rename_and_checkpoint' panicked at tests/integration/mvcc_dropped_schema_object_after_checkpoint.rs:26:9:
assertion `left == right` failed
  left: [("tr",)]
 right: []

---- mvcc_dropped_schema_object_after_checkpoint::test_mvcc_dropped_trigger_stays_dropped_after_restart_and_checkpoint stdout ----

thread 'mvcc_dropped_schema_object_after_checkpoint::test_mvcc_dropped_trigger_stays_dropped_after_restart_and_checkpoint' panicked at tests/integration/mvcc_dropped_schema_object_after_checkpoint.rs:79:5:
assertion `left == right` failed
  left: [("tr1",)]
 right: []


failures:
    mvcc_dropped_schema_object_after_checkpoint::test_mvcc_dropped_trigger_stays_dropped_after_rename_and_checkpoint
    mvcc_dropped_schema_object_after_checkpoint::test_mvcc_dropped_trigger_stays_dropped_after_restart_and_checkpoint
    mvcc_dropped_schema_object_after_checkpoint::test_mvcc_dropped_view_stays_dropped_after_rename_and_checkpoint

test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 1140 filtered out; finished in 0.02s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/mvcc_dropped_schema_object_after_checkpoint.rs`
- `tests/integration/mod.rs`