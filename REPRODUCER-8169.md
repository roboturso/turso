# Reproducer: #8169 — FTS: documented per-column `WITH tokenizer=` does not parse, and unknown/mis-cased WITH keys are silently ignored

https://github.com/tursodatabase/turso/issues/8169

## Environment

- Commit: 79163249538197d01dec5ea7f65519454ed792e2
- Platform: linux x86_64
- Reproduced at: 2026-08-11T06:55:53Z

## Reproduce

```sh
cargo test -p core_tester --test integration_tests fts_with_keys_issue_8169
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed on current main: FTS WITH keys are never validated (only `tokenizer`/`weights` are read in core/index_method/fts.rs), so the typo `tokenzier`, mis-cased `TOKENIZER`, and arbitrary bogus keys are silently accepted while the index gets the default tokenizer, and the documented per-column `WITH tokenizer=` form is a parse error.

## Observed failure

RoboTurso ran the command above and observed:

```text
Blocking waiting for file lock on build directory
   Compiling turso_ext v0.8.0-pre.3 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8169/extensions/core)
   Compiling turso_core v0.8.0-pre.3 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8169/core)
   Compiling turso_parser v0.8.0-pre.3 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8169/sqlite/parser)
   Compiling turso_sdk_kit v0.8.0-pre.3 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8169/sdk-kit)
   Compiling turso_sync_engine v0.8.0-pre.3 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8169/sync/engine)
   Compiling sql_generation v0.8.0-pre.3 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8169/sql_generation)
   Compiling turso-dbhash v0.8.0-pre.3 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8169/tools/dbhash)
   Compiling turso_sync_sdk_kit v0.8.0-pre.3 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8169/sync/sdk-kit)
   Compiling turso v0.8.0-pre.3 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8169/bindings/rust)
   Compiling core_tester v0.8.0-pre.3 (/home/penberg/src/tursodatabase/roboturso-turso/worktrees/issue-8169/tests)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1m 26s
     Running integration/mod.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/integration_tests-17ac5015bf27e00b)

running 3 tests
test index_method::fts_with_keys_issue_8169::fts_with_clause_rejects_unknown_key ... FAILED
test index_method::fts_with_keys_issue_8169::fts_with_clause_rejects_typo_key ... FAILED
test index_method::fts_with_keys_issue_8169::fts_with_clause_does_not_silently_ignore_miscased_key ... FAILED

failures:

---- index_method::fts_with_keys_issue_8169::fts_with_clause_rejects_unknown_key stdout ----

thread 'index_method::fts_with_keys_issue_8169::fts_with_clause_rejects_unknown_key' panicked at tests/integration/index_method/fts_with_keys_issue_8169.rs:41:5:
unknown key 'completely_bogus_key' was accepted and ignored
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- index_method::fts_with_keys_issue_8169::fts_with_clause_rejects_typo_key stdout ----

thread 'index_method::fts_with_keys_issue_8169::fts_with_clause_rejects_typo_key' panicked at tests/integration/index_method/fts_with_keys_issue_8169.rs:23:5:
typo key 'tokenzier' was accepted; the index silently uses the default tokenizer

---- index_method::fts_with_keys_issue_8169::fts_with_clause_does_not_silently_ignore_miscased_key stdout ----

thread 'index_method::fts_with_keys_issue_8169::fts_with_clause_does_not_silently_ignore_miscased_key' panicked at tests/integration/index_method/fts_with_keys_issue_8169.rs:67:9:
assertion `left == right` failed: TOKENIZER = 'ngram' was accepted but ignored: the index got the default tokenizer
  left: 0
 right: 1


failures:
    index_method::fts_with_keys_issue_8169::fts_with_clause_does_not_silently_ignore_miscased_key
    index_method::fts_with_keys_issue_8169::fts_with_clause_rejects_typo_key
    index_method::fts_with_keys_issue_8169::fts_with_clause_rejects_unknown_key

test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 1046 filtered out; finished in 0.04s

error: test failed, to rerun pass `-p core_tester --test integration_tests`
```

## Files

- `tests/integration/index_method/fts_with_keys_issue_8169.rs`
- `tests/integration/index_method/mod.rs`