use std::io::Write;
use std::process::{Command, Stdio};

/// Pretty mode must show every line of a multi-line value, like sqlite3's
/// box mode does, instead of cutting the cell down to its first line.
///
/// Regression test for issue #1192: `SELECT json_pretty(...)` in pretty
/// mode printed only `{…` instead of the indented JSON.
#[test]
fn pretty_mode_shows_all_lines_of_a_multiline_value() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_tursodb"))
        .arg("-q")
        .arg(":memory:")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to run tursodb");

    let mut stdin = child.stdin.take().unwrap();
    stdin
        .write_all(b".mode pretty\nSELECT json_pretty('{\"a\":1,\"b\":[1,2]}') AS j;\n")
        .unwrap();
    drop(stdin);

    let output = child.wait_with_output().expect("failed to wait");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains("\"a\": 1"),
        "pretty mode should show the indented JSON lines, got:\n{stdout}"
    );
    assert!(
        stdout.contains("\"b\": ["),
        "pretty mode should show the indented JSON lines, got:\n{stdout}"
    );
    assert!(
        !stdout.contains('…'),
        "pretty mode should not cut off the multi-line value, got:\n{stdout}"
    );
}
