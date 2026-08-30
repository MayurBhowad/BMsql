use bmsql::database::Database;

fn main() {
    let database = Database::new("BMSQL".to_string());

    println!("{} v{}", database.name(), env!("CARGO_PKG_VERSION"));
}
