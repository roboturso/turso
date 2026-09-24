# Reproducer: #6031 — DATABASE_MANAGER registry ignores OpenFlags on cache hit — readonly flag leaks across connections

https://github.com/tursodatabase/turso/issues/6031

## Environment

- Commit: cb1c3c8653e8a1a96ec0159ef69d9906c71940aa
- Platform: linux x86_64
- Reproduced at: 2026-09-24T13:10:08Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests registry_open_flags
```

Fails on the current tree; passes once the bug is fixed.

## What happens

When a file is already open in the process, the registry hands back that cached Database without checking OpenFlags, so a read-write open after a readonly open cannot write, and a readonly open after a read-write open can write.

## Observed failure

RoboTurso ran the command above and observed:

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-56909b28b7d44da0)

running 2 tests
test registry_open_flags::readonly_open_after_read_write_open_cannot_write ... FAILED
test registry_open_flags::read_write_open_after_readonly_open_can_write ... FAILED

failures:

---- registry_open_flags::readonly_open_after_read_write_open_cannot_write stdout ----

thread 'registry_open_flags::readonly_open_after_read_write_open_cannot_write' panicked at tests/integration/registry_open_flags.rs:57:5:
readonly open returned the cached read-write Database and allowed a write
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- registry_open_flags::read_write_open_after_readonly_open_can_write stdout ----

thread 'registry_open_flags::read_write_open_after_readonly_open_can_write' panicked at tests/integration/registry_open_flags.rs:37:5:
read-write open returned the cached readonly Database


failures:
    registry_open_flags::read_write_open_after_readonly_open_can_write
    registry_open_flags::readonly_open_after_read_write_open_cannot_write

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 1151 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/registry_open_flags.rs`
- `tests/integration/mod.rs`