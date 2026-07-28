// Regression test for https://github.com/tursodatabase/turso/issues/6655
// A moderately nested arithmetic expression with chained unary operators
// overflowed the stack (found by arithmetic_expression_fuzz_mvcc with
// SEED=1777759667094).
#[cfg(test)]
mod issue_6655_tests {
    use crate::helpers;
    use core_tester::common::TempDatabase;

    #[turso_macros::test(mvcc)]
    pub fn arithmetic_unary_chain_no_stack_overflow(db: TempDatabase) {
        let limbo_conn = db.connect_limbo();
        let sqlite_conn = rusqlite::Connection::open_in_memory().unwrap();
        helpers::assert_differential(
            &limbo_conn,
            &sqlite_conn,
            "SELECT (+ + - - - ~ (- ((~ + ~ ~ - - ~ -4 << -3))))",
            "",
        );
    }
}
