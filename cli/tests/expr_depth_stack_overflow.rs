//! Regression tests for https://github.com/tursodatabase/turso/issues/5071
//!
//! Several recursive functions in the SQL translator/optimizer have no depth
//! limit, so a crafted (or fuzz-generated) query can crash the whole process
//! with a stack overflow. SQLite guards against this with `SQLITE_MAX_EXPR_DEPTH`
//! (default 1000); tursodb had no equivalent.
//!
//! A deeply nested expression must never abort the process. It may succeed or
//! return a graceful error, but it must not be killed by a signal. We run
//! tursodb as a subprocess on purpose: a stack overflow is a SIGABRT/SIGSEGV
//! that cannot be caught in-process, so an in-process assertion would take the
//! test harness down with it.
//!
//! When the process is terminated by a signal, `ExitStatus::code()` returns
//! `None`; a clean exit (success or graceful error) returns `Some(_)`.

use std::process::Command;

fn run_sql(sql: &str) -> std::process::ExitStatus {
    Command::new(env!("CARGO_BIN_EXE_tursodb"))
        .arg(":memory:")
        .arg(sql)
        .status()
        .expect("failed to run tursodb")
}

/// A long `OR` chain overflows `flatten_or_expr_owned` / `translate_condition_expr`.
#[test]
fn deep_or_chain_does_not_stack_overflow() {
    let or_chain = vec!["1"; 10_000].join(" OR ");
    let sql = format!("CREATE TABLE t(x); SELECT * FROM t WHERE {or_chain};");

    let status = run_sql(&sql);
    assert!(
        status.code().is_some(),
        "tursodb was killed by a signal (stack overflow) on a deep OR chain: {status:?}"
    );
}

/// Deeply nested scalar subqueries overflow `translate_expr`.
#[test]
fn deep_nested_subqueries_do_not_stack_overflow() {
    let mut inner = String::from("1");
    for _ in 0..500 {
        inner = format!("(SELECT {inner} FROM t)");
    }
    let sql = format!("CREATE TABLE t(x); SELECT {inner} FROM t;");

    let status = run_sql(&sql);
    assert!(
        status.code().is_some(),
        "tursodb was killed by a signal (stack overflow) on deeply nested subqueries: {status:?}"
    );
}

/// Deeply nested `CASE` expressions overflow `translate_expr`.
#[test]
fn deep_nested_case_does_not_stack_overflow() {
    let mut inner = String::from("1");
    for _ in 0..1_000 {
        inner = format!("CASE WHEN 1 THEN {inner} ELSE 0 END");
    }
    let sql = format!("CREATE TABLE t(x); SELECT {inner} FROM t;");

    let status = run_sql(&sql);
    assert!(
        status.code().is_some(),
        "tursodb was killed by a signal (stack overflow) on deeply nested CASE: {status:?}"
    );
}
