# Reproducer: #9296 — Auto-checkpoint on the same connection ends an active SELECT early, and a backward scan panics

https://github.com/tursodatabase/turso/issues/9296

## Environment

- Commit: c00b02e55fb14d93c57896f5b63dc55cef3ff4de
- Platform: linux x86_64
- Reproduced at: 2026-09-24T07:01:41Z

## Reproduce

```sh
cd tests && cargo test --test integration_tests auto_checkpoint_active_select
```

Fails on the current tree; passes once the bug is fixed.

## What happens

When a write on the same connection starts an auto-checkpoint during a SELECT, the forward scan stops after 986 of 2000 rows with no error, and the backward scan panics with `self.current_page >= 0` in btree.rs.

## Observed failure

RoboTurso ran the command above and observed:

```text
Compiling core_tester v0.8.0-pre.12 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-9296/tests)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 10.04s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-e1bb149c2239ec41)

running 2 tests
test auto_checkpoint_active_select::test_forward_scan_returns_all_rows_after_auto_checkpoint_between_steps ... FAILED
test auto_checkpoint_active_select::test_backward_scan_returns_all_rows_after_auto_checkpoint_between_steps ... FAILED

failures:

---- auto_checkpoint_active_select::test_forward_scan_returns_all_rows_after_auto_checkpoint_between_steps stdout ----

thread 'auto_checkpoint_active_select::test_forward_scan_returns_all_rows_after_auto_checkpoint_between_steps' panicked at tests/integration/auto_checkpoint_active_select.rs:44:5:
assertion `left == right` failed
  left: 986
 right: 2000
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- auto_checkpoint_active_select::test_backward_scan_returns_all_rows_after_auto_checkpoint_between_steps stdout ----

thread 'auto_checkpoint_active_select::test_backward_scan_returns_all_rows_after_auto_checkpoint_between_steps' panicked at core/storage/btree.rs:8840:9:
self.current_page >= 0


failures:
    auto_checkpoint_active_select::test_backward_scan_returns_all_rows_after_auto_checkpoint_between_steps
    auto_checkpoint_active_select::test_forward_scan_returns_all_rows_after_auto_checkpoint_between_steps

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 1150 filtered out; finished in 0.43s

error: test failed, to rerun pass `--test integration_tests`
```

## Files

- `tests/integration/auto_checkpoint_active_select.rs`
- `tests/integration/mod.rs`