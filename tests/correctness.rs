use trdbte::Database;

#[test]
fn test_open_database() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let db = Database::open(&db_path).unwrap();
    // execute and query return NotImplemented for now
    assert!(db.execute("CREATE TABLE t (id INTEGER)").is_err());
    assert!(db.query("SELECT 1").is_err());
}
