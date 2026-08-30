use bmsql::database::Database;

#[test]
fn database_has_name() {
    let database = Database::new("BMSQL".to_string());
    assert_eq!(database.name(), "BMSQL");
}