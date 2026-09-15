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

    pub fn size(&self) -> Result<u64, BmsqlError> {
        self.database_file.size()
    }

    pub fn page_count(&self) -> Result<u64, BmsqlError> {
        Ok(self.size()? / crate::page::PAGE_SIZE as u64)
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

    #[test]
    fn pager_reports_database_file_size() {
        let path = "bmsql_pager_size_test.db";
        File::create(path).unwrap();
        let mut pager = Pager::open(path).unwrap();
        let page = Page::new(0);
        pager.write_page(&page).unwrap();
        assert_eq!(pager.size().unwrap(), 4096);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn pager_reports_page_count() {
        let path = "bmsql_pager_page_count_test.db";
        File::create(path).unwrap();
        let mut pager = Pager::open(path).unwrap();

        assert_eq!(pager.page_count().unwrap(), 0);

        let page0 = Page::new(0);
        pager.write_page(&page0).unwrap();

        assert_eq!(pager.page_count().unwrap(), 1);

        let page1 = Page::new(1);
        pager.write_page(&page1).unwrap();

        assert_eq!(pager.page_count().unwrap(), 2);

        std::fs::remove_file(path).unwrap();
    }
}
