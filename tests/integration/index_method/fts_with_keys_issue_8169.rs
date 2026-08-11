//! Regression tests for https://github.com/tursodatabase/turso/issues/8169.
//!
//! Keys in an FTS index `WITH` clause were never validated: a typo like
//! `tokenzier` or a mis-cased `TOKENIZER` was stored verbatim and ignored,
//! so the index silently got the default tokenizer instead of the one the
//! user asked for. Bad values already produce a helpful error; bad keys
//! must too.

use crate::common::{limbo_exec_rows, TempDatabase};

/// A one-character typo in the `tokenizer` key must be an error, not a
/// silently different index.
#[cfg(all(feature = "fts", not(target_family = "wasm")))]
#[turso_macros::test]
fn fts_with_clause_rejects_typo_key(tmp_db: TempDatabase) {
    let _ = env_logger::try_init();
    let conn = tmp_db.connect_limbo();

    conn.execute("CREATE TABLE docs(id INTEGER PRIMARY KEY, body TEXT)")
        .unwrap();
    let result =
        conn.execute("CREATE INDEX fts_docs ON docs USING fts (body) WITH (tokenzier = 'ngram')");
    assert!(
        result.is_err(),
        "typo key 'tokenzier' was accepted; the index silently uses the default tokenizer"
    );
}

/// An unrecognised extra key next to a valid one must also be an error.
#[cfg(all(feature = "fts", not(target_family = "wasm")))]
#[turso_macros::test]
fn fts_with_clause_rejects_unknown_key(tmp_db: TempDatabase) {
    let _ = env_logger::try_init();
    let conn = tmp_db.connect_limbo();

    conn.execute("CREATE TABLE docs(id INTEGER PRIMARY KEY, body TEXT)")
        .unwrap();
    let result = conn.execute(
        "CREATE INDEX fts_docs ON docs USING fts (body) WITH (tokenizer = 'ngram', completely_bogus_key = 42)",
    );
    assert!(
        result.is_err(),
        "unknown key 'completely_bogus_key' was accepted and ignored"
    );
}

/// A mis-cased key must not be silently ignored. Rejecting it or treating
/// keys case-insensitively are both fine; getting the default tokenizer
/// without any error is the bug.
#[cfg(all(feature = "fts", not(target_family = "wasm")))]
#[turso_macros::test]
fn fts_with_clause_does_not_silently_ignore_miscased_key(tmp_db: TempDatabase) {
    let _ = env_logger::try_init();
    let conn = tmp_db.connect_limbo();

    conn.execute("CREATE TABLE docs(id INTEGER PRIMARY KEY, body TEXT)")
        .unwrap();
    let result =
        conn.execute("CREATE INDEX fts_docs ON docs USING fts (body) WITH (TOKENIZER = 'ngram')");
    if result.is_ok() {
        // The key was accepted, so it must have configured the ngram
        // tokenizer. Only ngram can match the 2-character prefix 'al';
        // the default tokenizer indexes whole words and returns nothing.
        conn.execute("INSERT INTO docs VALUES (1, 'alpha')")
            .unwrap();
        let rows = limbo_exec_rows(&conn, "SELECT id FROM docs WHERE fts_match(body, 'al')");
        assert_eq!(
            rows.len(),
            1,
            "TOKENIZER = 'ngram' was accepted but ignored: the index got the default tokenizer"
        );
    }
}
