# Reproducer: #8196 — R-cand-018: experimental_features silently discarded on a process-wide registry hit

https://github.com/tursodatabase/turso/issues/8196

## Environment

- Commit: cb1c3c8653e8a1a96ec0159ef69d9906c71940aa
- Platform: linux x86_64
- Reproduced at: 2026-09-24T13:12:42Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests registry_db_opts
```

Fails on the current tree; passes once the bug is fixed.

## What happens

When a file is already open in the process, a second open returns the cached Database without comparing db_opts, so experimental features like passive checkpoint are silently given to, or dropped for, the second caller.

## Observed failure

RoboTurso ran the command above and observed:

```text
Compiling core_tester v0.8.0-pre.12 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8196/tests)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 10.15s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-56909b28b7d44da0)

running 2 tests
test registry_db_opts::registry_hit_does_not_drop_passive_checkpoint_the_caller_asked_for ... FAILED
test registry_db_opts::registry_hit_does_not_give_passive_checkpoint_to_caller_that_did_not_ask ... FAILED

failures:

---- registry_db_opts::registry_hit_does_not_drop_passive_checkpoint_the_caller_asked_for stdout ----

thread 'registry_db_opts::registry_hit_does_not_drop_passive_checkpoint_the_caller_asked_for' panicked at tests/integration/registry_db_opts.rs:58:23:
second open asked for passive checkpoint but the registry silently dropped it
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- registry_db_opts::registry_hit_does_not_give_passive_checkpoint_to_caller_that_did_not_ask stdout ----

thread 'registry_db_opts::registry_hit_does_not_give_passive_checkpoint_to_caller_that_did_not_ask' panicked at tests/integration/registry_db_opts.rs:33:23:
second open asked for no experimental features but got passive checkpoint from the registry


failures:
    registry_db_opts::registry_hit_does_not_drop_passive_checkpoint_the_caller_asked_for
    registry_db_opts::registry_hit_does_not_give_passive_checkpoint_to_caller_that_did_not_ask

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 1151 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/registry_db_opts.rs`
- `tests/integration/mod.rs`