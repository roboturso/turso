use std::num::NonZero;

use crate::common::TempDatabase;
use turso_core::{StepResult, Value};

fn count_rows_while_inserting_between_steps(order: &str) -> i64 {
    let tmp_db = TempDatabase::new_empty();
    let conn = tmp_db.connect_limbo();
    conn.execute("CREATE TABLE t(id INTEGER PRIMARY KEY)")
        .unwrap();
    conn.execute("CREATE TABLE other(id INTEGER PRIMARY KEY, v INTEGER)")
        .unwrap();
    conn.execute(
        "WITH RECURSIVE c(x) AS (SELECT 1 UNION ALL SELECT x + 1 FROM c WHERE x < 2000) \
         INSERT INTO t SELECT x FROM c",
    )
    .unwrap();

    let mut select = conn
        .prepare(format!("SELECT id FROM t ORDER BY id {order}"))
        .unwrap();
    let mut insert = conn.prepare("INSERT INTO other(v) VALUES (?)").unwrap();
    let mut rows = 0;
    loop {
        match select.step().unwrap() {
            StepResult::Row => {
                rows += 1;
                insert.reset().unwrap();
                insert
                    .bind_at(NonZero::new(1).unwrap(), Value::from_i64(rows))
                    .unwrap();
                insert.run_ignore_rows().unwrap();
            }
            StepResult::IO => select._io().step().unwrap(),
            StepResult::Done => break,
            other => panic!("unexpected step result: {other:?}"),
        }
    }
    rows
}

#[test]
fn test_forward_scan_returns_all_rows_after_auto_checkpoint_between_steps() {
    assert_eq!(count_rows_while_inserting_between_steps("ASC"), 2000);
}

#[test]
fn test_backward_scan_returns_all_rows_after_auto_checkpoint_between_steps() {
    assert_eq!(count_rows_while_inserting_between_steps("DESC"), 2000);
}
