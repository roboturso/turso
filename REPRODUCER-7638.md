# Reproducer: #7638 — MVCC abandoned post-commit auto-checkpoint plus GC resurrects deleted row and corrupts secondary index

https://github.com/tursodatabase/turso/issues/7638

## Environment

- Commit: d115b414ad83d05a1b3b66063eeefd07a5f51268
- Platform: linux x86_64
- Reproduced at: 2026-07-12T06:10:02Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests issue_7638_mvcc_gc_resurrects_row::abandoned_post_durable_checkpoint_gc_indexed_sibling_probe
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Reproduced deterministically: after an abandoned MVCC post-commit checkpoint, a subsequent UPDATE+DELETE followed by wal_checkpoint(TRUNCATE) resurrects the deleted row in the table while the secondary index disagrees, corrupting integrity_check.

## Observed failure

RoboTurso ran the command above and observed:

```text
Compiling core_tester v0.7.0-pre.19 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7638/tests)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 8.93s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-41d957122089e1f9)

running 1 test
test issue_7638_mvcc_gc_resurrects_row::abandoned_post_durable_checkpoint_gc_indexed_sibling_probe ... FAILED

failures:

---- issue_7638_mvcc_gc_resurrects_row::abandoned_post_durable_checkpoint_gc_indexed_sibling_probe stdout ----

thread 'issue_7638_mvcc_gc_resurrects_row::abandoned_post_durable_checkpoint_gc_indexed_sibling_probe' panicked at tests/integration/issue_7638_mvcc_gc_resurrects_row.rs:115:5:
assertion `left == right` failed
  left: [[Numeric(Integer(1500)), Text(Text { value: "old1500", subtype: Text })]]
 right: []
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    issue_7638_mvcc_gc_resurrects_row::abandoned_post_durable_checkpoint_gc_indexed_sibling_probe

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 979 filtered out; finished in 1.00s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/issue_7638_mvcc_gc_resurrects_row.rs`
- `tests/integration/mod.rs`