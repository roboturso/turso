# Reproducer: #7486 — MVCC: SELECT can return a row from a writer that later aborts

https://github.com/tursodatabase/turso/issues/7486

## Environment

- Commit: 225388ab65ac31ce4ff348e5d4f1d1855425ff6c
- Platform: linux x86_64
- Reproduced at: 2026-07-07T07:36:47Z

## Reproduce

```sh
cargo test -p turso_core issue_7486_tests::test_select_does_not_stream_row_from_aborted_preparing_writer
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: an autocommit SELECT streams the uncommitted value 'modified' from a writer paused in the Preparing commit state, and the writer's subsequent abort leaves the user having observed a row that was never committed; a regression test capturing this now fails on the current tree.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.38s
     Running unittests lib.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/turso_core-9f318bf332f0c513)

running 1 test
test mvcc::database::issue_7486_tests::test_select_does_not_stream_row_from_aborted_preparing_writer ... FAILED

failures:

---- mvcc::database::issue_7486_tests::test_select_does_not_stream_row_from_aborted_preparing_writer stdout ----
path: /tmp/.tmpiLci4M/test_18052562586694382632

thread 'mvcc::database::issue_7486_tests::test_select_does_not_stream_row_from_aborted_preparing_writer' panicked at core/mvcc/database/issue_7486_tests.rs:131:9:
assertion `left == right` failed: reader surfaced a row from a writer that later aborted: [Text(Text { value: "modified", subtype: Text })]
  left: Some("modified")
 right: Some("initial")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    mvcc::database::issue_7486_tests::test_select_does_not_stream_row_from_aborted_preparing_writer

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 2052 filtered out; finished in 0.02s

error: test failed, to rerun pass `-p turso_core --lib`
```

## Files

- `core/mvcc/database/issue_7486_tests.rs`
- `core/mvcc/database/mod.rs`