//! Regression test for issue #9001.
//!
//! `SAVEPOINT` outside a transaction starts a deferred transaction. SQLite and
//! Turso take the read snapshot at the first read, not at the SAVEPOINT. The
//! shadow model copied the committed tables as soon as the SAVEPOINT ran, so
//! rows committed by other connections in between were invisible to the model
//! but visible to the database. The simulator then reported
//! "expected 12 rows but got 14".

use sql_generation::model::query::Insert;
use sql_generation::model::query::predicate::Predicate;
use sql_generation::model::query::select::Select;
use sql_generation::model::table::{Column, ColumnType, SimValue, Table};
use turso_core::Value;

use super::{ShadowSequence, ShadowTablesMut, TransactionTables};
use crate::generation::Shadow;
use crate::model::{Query, ReleaseSavepoint, Savepoint};

#[test]
fn savepoint_without_reads_sees_rows_committed_later_by_another_connection() {
    let mut committed = vec![table_with_rows(&[1])];
    let mut sequences = Vec::new();
    let mut conn_a = None;
    let mut conn_b = None;

    run(
        Query::Savepoint(Savepoint { name: "sp".into() }),
        &mut committed,
        &mut conn_a,
        &mut sequences,
    );

    run(insert_row(2), &mut committed, &mut conn_b, &mut sequences);
    assert_eq!(
        committed[0].rows.len(),
        2,
        "autocommit insert on another connection must reach the committed tables"
    );

    let seen = run(
        Query::Select(Select::simple("t".into(), Predicate::true_())),
        &mut committed,
        &mut conn_a,
        &mut sequences,
    );
    assert_eq!(
        seen,
        vec![row(1), row(2)],
        "a connection that only ran SAVEPOINT has no read snapshot yet, so it must see rows committed afterwards"
    );
}

#[test]
fn insert_select_after_savepoint_copies_rows_committed_by_another_connection() {
    let mut committed = vec![table_with_rows(&[1])];
    let mut sequences = Vec::new();
    let mut conn_a = None;
    let mut conn_b = None;

    run(
        Query::Savepoint(Savepoint { name: "sp".into() }),
        &mut committed,
        &mut conn_a,
        &mut sequences,
    );
    run(insert_row(2), &mut committed, &mut conn_b, &mut sequences);

    run(
        Query::Insert(Insert::Select {
            table: "t".into(),
            columns: Default::default(),
            select: Box::new(Select::simple("t".into(), Predicate::true_())),
        }),
        &mut committed,
        &mut conn_a,
        &mut sequences,
    );
    run(
        Query::ReleaseSavepoint(ReleaseSavepoint { name: "sp".into() }),
        &mut committed,
        &mut conn_a,
        &mut sequences,
    );

    assert!(
        conn_a.is_none(),
        "RELEASE of the first savepoint ends the transaction"
    );
    let mut rows = committed[0].rows.clone();
    rows.sort_by_key(|r| r[0].0.as_int().expect("column x holds integers"));
    assert_eq!(
        rows,
        vec![row(1), row(1), row(2), row(2)],
        "INSERT ... SELECT must copy every row the database would see, including the one committed after SAVEPOINT"
    );
}

fn run(
    query: Query,
    committed: &mut Vec<Table>,
    conn: &mut Option<TransactionTables>,
    sequences: &mut Vec<ShadowSequence>,
) -> Vec<Vec<SimValue>> {
    let mut tables = ShadowTablesMut {
        commited_tables: committed,
        transaction_tables: conn,
        sequences,
    };
    query
        .shadow(&mut tables)
        .unwrap_or_else(|err| panic!("shadow model rejected `{query}`: {err}"))
}

fn insert_row(value: i64) -> Query {
    Query::Insert(Insert::Values {
        table: "t".into(),
        values: vec![row(value)],
        on_conflict: None,
    })
}

fn table_with_rows(values: &[i64]) -> Table {
    Table {
        name: "t".into(),
        columns: vec![Column {
            name: "x".into(),
            column_type: ColumnType::Integer,
            constraints: vec![],
        }],
        rows: values.iter().map(|v| row(*v)).collect(),
        indexes: vec![],
    }
}

fn row(value: i64) -> Vec<SimValue> {
    vec![SimValue(Value::from_i64(value))]
}
