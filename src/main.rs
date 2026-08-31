use bmsql::database::Database;

fn main() {
    let name = String::from("BMsql");

    let database = Database::new(name);

    println!("{}", database.name());
}
