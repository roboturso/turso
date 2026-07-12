# Reproducer: #7603 — BEGIN CONCURRENT: COMMIT returns success after a write-write conflict aborts the transaction, silently discarding its writes

https://github.com/tursodatabase/turso/issues/7603

## Environment

- Commit: d115b414ad83d05a1b3b66063eeefd07a5f51268
- Platform: linux x86_64
- Reproduced at: 2026-07-12T06:38:51Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests test_mvcc_commit_after_write_write_conflict_rollback
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Reproduced on current main: after a sequence write-write conflict rolls back a BEGIN CONCURRENT transaction on the in-memory MVCC backend, COMMIT still returns OK while the transaction's DELETE is silently discarded; added a failing regression test covering this.

## Observed failure

RoboTurso ran the command above and observed:

```text
Compiling turso_ext v0.7.0-pre.19 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7603/extensions/core)
   Compiling turso_core v0.7.0-pre.19 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7603/core)
   Compiling turso_parser v0.7.0-pre.19 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7603/sqlite/parser)
   Compiling turso_sdk_kit v0.7.0-pre.19 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7603/sdk-kit)
   Compiling turso_sync_engine v0.7.0-pre.19 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7603/sync/engine)
   Compiling sql_generation v0.7.0-pre.19 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7603/sql_generation)
   Compiling turso-dbhash v0.7.0-pre.19 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7603/tools/dbhash)
   Compiling turso_sync_sdk_kit v0.7.0-pre.19 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7603/sync/sdk-kit)
   Compiling turso v0.7.0-pre.19 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7603/bindings/rust)
   Compiling core_tester v0.7.0-pre.19 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7603/tests)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1m 02s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-41d957122089e1f9)

running 1 test
test mvcc_commit_after_conflict_rollback::test_mvcc_commit_after_write_write_conflict_rollback_must_not_report_success ... FAILED

failures:

---- mvcc_commit_after_conflict_rollback::test_mvcc_commit_after_write_write_conflict_rollback_must_not_report_success stdout ----

thread 'mvcc_commit_after_conflict_rollback::test_mvcc_commit_after_write_write_conflict_rollback_must_not_report_success' panicked at tests/integration/mvcc_commit_after_conflict_rollback.rs:75:13:
assertion `left == right` failed: COMMIT reported success but conn1's DELETE was silently discarded (write-write conflict already rolled the transaction back)
  left: 1
 right: 0
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    mvcc_commit_after_conflict_rollback::test_mvcc_commit_after_write_write_conflict_rollback_must_not_report_success

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 979 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/mvcc_commit_after_conflict_rollback.rs`
- `tests/integration/mod.rs`