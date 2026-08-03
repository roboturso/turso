# Reproducer: #4247 — blobs are weird in turso vs sqlite

https://github.com/tursodatabase/turso/issues/4247

## Environment

- Commit: d97b9c007b0474a01b3557410e0444073d711899
- Platform: linux x86_64
- Reproduced at: 2026-08-03T09:50:05Z

## Reproduce

```sh
cargo test -p turso_cli --test issue_4247_blob_raw_output
```

Fails on the current tree; passes once the bug is fixed.

## What happens

The stored data is intact (hex() output matches sqlite byte-for-byte), but the tursodb CLI replaces every non-UTF8 byte in a blob with U+FFFD when printing (via String::from_utf8_lossy in Value's Display impl), whereas sqlite3 writes the raw blob bytes, so blob output is mangled and not round-trippable.

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
     Running tests/issue_4247_blob_raw_output.rs (/home/penberg/src/tursodatabase/roboturso-turso/target/debug/deps/issue_4247_blob_raw_output-ce955e7eb312918f)

running 2 tests
test blob_with_invalid_utf8_is_printed_as_raw_bytes ... FAILED
test mixed_ascii_and_invalid_utf8_blob_is_printed_as_raw_bytes ... FAILED

failures:

---- blob_with_invalid_utf8_is_printed_as_raw_bytes stdout ----

thread 'blob_with_invalid_utf8_is_printed_as_raw_bytes' panicked at cli/tests/issue_4247_blob_raw_output.rs:24:5:
assertion `left == right` failed: blob bytes must be written raw, not replaced with U+FFFD; got [ef, bf, bd, 0a, 62, ef, bf, bd, 0a]
  left: [239, 191, 189, 10, 98, 239, 191, 189, 10]
 right: [144, 10, 98, 128, 10]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- mixed_ascii_and_invalid_utf8_blob_is_printed_as_raw_bytes stdout ----

thread 'mixed_ascii_and_invalid_utf8_blob_is_printed_as_raw_bytes' panicked at cli/tests/issue_4247_blob_raw_output.rs:48:5:
assertion `left == right` failed: blob bytes must be written raw, not replaced with U+FFFD; got [ef, bf, bd, 65, 52, ef, bf, bd, 7c, 64, ef, bf, bd, 76, 7b, 0a]
  left: [239, 191, 189, 101, 82, 239, 191, 189, 124, 100, 239, 191, 189, 118, 123, 10]
 right: [182, 101, 82, 195, 124, 100, 172, 118, 123, 10]


failures:
    blob_with_invalid_utf8_is_printed_as_raw_bytes
    mixed_ascii_and_invalid_utf8_blob_is_printed_as_raw_bytes

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p turso_cli --test issue_4247_blob_raw_output`
```

## Files

- `cli/tests/issue_4247_blob_raw_output.rs`