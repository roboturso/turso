# Reproducer: #7150 — ORDER BY query panics when PRAGMA cache_size is set to `i32::MIN`

https://github.com/tursodatabase/turso/issues/7150

## Environment

- Commit: d4d6777a2685ab9a3ffc8733329743fdfe000c45
- Platform: linux x86_64
- Reproduced at: 2026-07-02T05:30:28Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests test_pragma_cache_size_i32_min_order_by
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Setting PRAGMA cache_size to i32::MIN and then running an ORDER BY query panics with 'attempt to negate with overflow' because op_sorter_open calls i32::abs() on i32::MIN in core/vdbe/execute.rs.

## Observed failure

RoboTurso ran the command above and observed:

```text
Compiling core_tester v0.7.0-pre.13 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-7150/tests)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 7.87s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-2086b07511e41b85)

running 2 tests
test pragma::test_pragma_cache_size_i32_min_order_by ... FAILED
test pragma::test_pragma_cache_size_i32_min_order_by_mvcc ... FAILED

failures:

---- pragma::test_pragma_cache_size_i32_min_order_by stdout ----

thread 'pragma::test_pragma_cache_size_i32_min_order_by' panicked at /home/penberg/.rustup/toolchains/1.88-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/mod.rs:294:5:
attempt to negate with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- pragma::test_pragma_cache_size_i32_min_order_by_mvcc stdout ----

thread 'pragma::test_pragma_cache_size_i32_min_order_by_mvcc' panicked at /home/penberg/.rustup/toolchains/1.88-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/mod.rs:294:5:
attempt to negate with overflow


failures:
    pragma::test_pragma_cache_size_i32_min_order_by
    pragma::test_pragma_cache_size_i32_min_order_by_mvcc

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 962 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/pragma.rs`