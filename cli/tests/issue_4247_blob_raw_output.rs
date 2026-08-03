//! Regression test for https://github.com/tursodatabase/turso/issues/4247
//!
//! Blobs holding bytes that are not valid UTF-8 must be written to stdout
//! as raw bytes, exactly like sqlite3 does in list mode. tursodb used to
//! replace every invalid byte with U+FFFD (EF BF BD), so blob output was
//! mangled and could not be round-tripped.

use std::process::Command;

/// sqlite3 prints `SELECT X'900A6280'` as the raw bytes 90 0A 62 80
/// followed by a newline. tursodb must produce the same bytes.
#[test]
fn blob_with_invalid_utf8_is_printed_as_raw_bytes() {
    let output = Command::new(env!("CARGO_BIN_EXE_tursodb"))
        .arg("-q")
        .arg(":memory:")
        .arg("SELECT X'900A6280';")
        .arg("--output-mode")
        .arg("list")
        .output()
        .expect("failed to run tursodb");

    assert!(output.status.success());
    assert_eq!(
        output.stdout, b"\x90\x0a\x62\x80\x0a",
        "blob bytes must be written raw, not replaced with U+FFFD; got {:02x?}",
        output.stdout
    );
}

/// Same check for blobs that mix printable ASCII with invalid UTF-8
/// bytes, taken from the trigger-inserted row in the issue's repro
/// script. sqlite3 prints these as b6 65 52 c3 7c 64 ac 76 7b 0a.
/// (Blobs with ASCII control bytes are left out on purpose: sqlite3
/// escapes those as ^X in list mode, which is a separate behavior.)
#[test]
fn mixed_ascii_and_invalid_utf8_blob_is_printed_as_raw_bytes() {
    let output = Command::new(env!("CARGO_BIN_EXE_tursodb"))
        .arg("-q")
        .arg(":memory:")
        .arg("SELECT X'B66552C3', X'64AC767B';")
        .arg("--output-mode")
        .arg("list")
        .output()
        .expect("failed to run tursodb");

    assert!(output.status.success());
    assert_eq!(
        output.stdout, b"\xb6\x65\x52\xc3|\x64\xac\x76\x7b\x0a",
        "blob bytes must be written raw, not replaced with U+FFFD; got {:02x?}",
        output.stdout
    );
}
