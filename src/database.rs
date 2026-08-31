pub struct Database {
    name: String,
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