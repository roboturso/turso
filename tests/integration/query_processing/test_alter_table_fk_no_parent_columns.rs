use crate::common::{ExecRows, TempDatabase};
use tempfile::TempDir;

#[test]
fn test_alter_table_keeps_fk_without_parent_columns_parseable() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("alter_fk_no_parent_columns.db");

    {
        let db = TempDatabase::new_with_existent(&path);
        let conn = db.connect_limbo();
        conn.execute("CREATE TABLE p(id INTEGER PRIMARY KEY)")
            .unwrap();
        conn.execute("CREATE TABLE t(a INTEGER, FOREIGN KEY(a) REFERENCES p)")
            .unwrap();
        conn.execute("CREATE TABLE u(a INTEGER REFERENCES p, b INTEGER, d INTEGER)")
            .unwrap();
        conn.execute("ALTER TABLE t ADD COLUMN c INTEGER").unwrap();
        conn.execute("ALTER TABLE u DROP COLUMN d").unwrap();

        let broken: Vec<(i64,)> =
            conn.exec_rows("SELECT count(*) FROM sqlite_schema WHERE sql LIKE '%REFERENCES p()%'");
        assert_eq!(broken, vec![(0,)]);
        conn.close().unwrap();
    }

    {
        let db = TempDatabase::new_with_existent(&path);
        let conn = db.connect_limbo();
        let rows: Vec<(i64,)> = conn.exec_rows("SELECT count(*) FROM t");
        assert_eq!(rows, vec![(0,)]);
        let rows: Vec<(i64,)> = conn.exec_rows("SELECT count(*) FROM u");
        assert_eq!(rows, vec![(0,)]);
    }
}
