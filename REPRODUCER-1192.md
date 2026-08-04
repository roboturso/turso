# Reproducer: #1192 — Pretty mode needs some adjustments

https://github.com/tursodatabase/turso/issues/1192

## Environment

- Commit: 1dec7f943df24d6955a11b102b427f6c5b49937e
- Platform: linux x86_64
- Reproduced at: 2026-08-04T10:36:21Z

## Reproduce

```sh
cargo test -p turso_cli --test pretty_mode_multiline_values
```

Fails on the current tree; passes once the bug is fixed.

## What happens

Confirmed: in pretty mode a multi-line value like json_pretty output is truncated to its first line (shown as `{…`) because print_pretty_mode caps each row at max_height(1), while sqlite3's box mode shows every line of the cell.

## Observed failure

RoboTurso ran the command above and observed:

```text
warning: unused import: `crate::translate::collate::CollationSeq`
  --> core/vdbe/mod.rs:43:9
   |
43 | pub use crate::translate::collate::CollationSeq;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: `turso_core` (lib) generated 1 warning (run `cargo fix --lib -p turso_core` to apply 1 suggestion)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running tests/pretty_mode_multiline_values.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/pretty_mode_multiline_values-a922c5c2df04f45f)

running 1 test
test pretty_mode_shows_all_lines_of_a_multiline_value ... FAILED

failures:

---- pretty_mode_shows_all_lines_of_a_multiline_value stdout ----

thread 'pretty_mode_shows_all_lines_of_a_multiline_value' panicked at cli/tests/pretty_mode_multiline_values.rs:29:5:
pretty mode should show the indented JSON lines, got:
┌─────────────┐
│ j           │
├─────────────┤
│ {…          │
└─────────────┘

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    pretty_mode_shows_all_lines_of_a_multiline_value

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p turso_cli --test pretty_mode_multiline_values`
```

## Files

- `cli/tests/pretty_mode_multiline_values.rs`