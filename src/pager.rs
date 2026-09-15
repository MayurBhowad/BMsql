use crate::error::BmsqlError;
use crate::page::{Page, PageId};
use crate::storage::DatabaseFile;

pub struct Pager {
    database_file: DatabaseFile,
}

impl Pager {
    pub fn open(path: &str) -> Result<Self, BmsqlError> {
        let database_file = DatabaseFile::open(path)?;
        Ok(Self { database_file })
    }

    pub fn read_page(&mut self, page_id: PageId) -> Result<Page, BmsqlError> {
        self.database_file.read_page(page_id)
    }

    pub fn write_page(&mut self, page: &Page) -> Result<(), BmsqlError> {
        self.database_file.write_page(page)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn pager_can_be_opened() {
        let path = "bmsql_pager_test.db";
        File::create(path).unwrap();
        let pager = Pager::open(path);
        assert!(pager.is_ok());
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn pager_can_read_page() {
        let path = "bmsql_pager_read_test.db";
        File::create(path).unwrap();
        let mut database_file = DatabaseFile::open(path).unwrap();
        let mut page = crate::page::Page::new(0);
        page.data_mut()[0] = 42;
        database_file.write_page(&page).unwrap();
        let mut pager = Pager::open(path).unwrap();
        let page = pager.read_page(0).unwrap();

        assert_eq!(page.data()[0], 42);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn pager_can_write_page() {
        let path = "bmsql_pager_write_test.db";
        File::create(path).unwrap();
        let mut pager = Pager::open(path).unwrap();
        let mut page = crate::page::Page::new(0);
        page.data_mut()[0] = 99;
        pager.write_page(&page).unwrap();

        let mut database_file = DatabaseFile::open(path).unwrap();
        let page = database_file.read_page(0).unwrap();
        assert_eq!(page.data()[0], 99);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn pager_returns_error_when_database_file_does_not_exist() {
        let path = "bmsql_pager_missing_test.db";
        let result = Pager::open(path);
        assert!(result.is_err());
    }
}
