use crate::common::{ExecRows, TempDatabase};

const SCHEMA: &[&str] = &[
    "CREATE TABLE t(id INTEGER PRIMARY KEY, k INTEGER)",
    "CREATE UNIQUE INDEX i1 ON t(k)",
    "CREATE TABLE u(x INTEGER)",
    "INSERT INTO t VALUES(1,10),(2,20)",
    "INSERT INTO u VALUES(1)",
    "CREATE TRIGGER tg0 BEFORE INSERT ON t BEGIN DELETE FROM u WHERE x=1; END",
    "CREATE TRIGGER tg1 AFTER DELETE ON u BEGIN UPDATE t SET k=k*2 WHERE id=1; END",
];

#[turso_macros::test]
fn insert_or_ignore_fails_when_a_trigger_delete_fires_a_unique_violation(db: TempDatabase) {
    assert_nested_delete_trigger_violation_rolls_back_statement(
        &db,
        "INSERT OR IGNORE INTO t VALUES(3,30)",
    );
}

#[turso_macros::test]
fn insert_or_fail_rolls_back_when_a_trigger_delete_fires_a_unique_violation(db: TempDatabase) {
    assert_nested_delete_trigger_violation_rolls_back_statement(
        &db,
        "INSERT OR FAIL INTO t VALUES(3,30)",
    );
}

fn assert_nested_delete_trigger_violation_rolls_back_statement(db: &TempDatabase, insert: &str) {
    let conn = db.connect_limbo();
    for stmt in SCHEMA {
        conn.execute(stmt).unwrap();
    }

    let err = conn.execute(insert).expect_err("outer statement must fail");
    assert!(
        err.to_string().contains("UNIQUE constraint failed: t.k"),
        "unexpected error: {err}"
    );

    let u_rows: Vec<(i64,)> = conn.exec_rows("SELECT count(*) FROM u");
    assert_eq!(
        u_rows,
        vec![(1,)],
        "trigger DELETE on u must be rolled back"
    );

    let t_rows: Vec<(i64, i64)> = conn.exec_rows("SELECT id, k FROM t ORDER BY id");
    assert_eq!(
        t_rows,
        vec![(1, 10), (2, 20)],
        "outer INSERT and trigger UPDATE must be rolled back"
    );
}
