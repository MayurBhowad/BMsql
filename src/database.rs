use std::fs::File;

pub struct Database {
    name: String,
}

pub fn create_database_file(path: &str) -> std::io::Result<File> {
    File::create(path)
}

pub fn open_database_file(path: &str) -> std::io::Result<File> {
    File::open(path)
}

impl Database {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use crate::database::Database;
    #[test]
    fn database_can_be_created() {
        let database = Database::new(String::from("BMsql"));
        assert_eq!(database.name(), "BMsql");
    }
}