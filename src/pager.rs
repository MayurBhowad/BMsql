use crate::error::BmsqlError;
use crate::page::{Page, PageId};
use crate::storage::DatabaseFile;
use std::collections::HashMap;

pub struct Pager {
    database_file: DatabaseFile,
    page_cache: HashMap<PageId, Page>,
}

impl Pager {
    pub fn open(path: &str) -> Result<Self, BmsqlError> {
        let database_file = DatabaseFile::open(path)?;
        Ok(Self { database_file, page_cache: HashMap::new() })
    }

    pub fn read_page(&mut self, page_id: PageId) -> Result<Page, BmsqlError> {
        if let Some(page) = self.page_cache.get(&page_id) {
            return Ok(page.clone());
        }
        let page =  self.database_file.read_page(page_id)?;
        self.page_cache.insert(page_id, page.clone());

        Ok(page)
    }

    pub fn write_page(&mut self, page: &Page) -> Result<(), BmsqlError> {
        self.database_file.write_page(page)?;
        self.page_cache.insert(page.id(), page.clone());
        Ok(())
    }

    pub fn size(&self) -> Result<u64, BmsqlError> {
        self.database_file.size()
    }

    pub fn page_count(&self) -> Result<u64, BmsqlError> {
        Ok(self.size()? / crate::page::PAGE_SIZE as u64)
    }

    pub fn allocate_page(&self) -> Result<Page, BmsqlError> {
        let page_count = self.page_count()?;
        Ok(Page::new(page_count))
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

    #[test]
    fn pager_can_allocate_page() {
        let path = "bmsql_pager_allocate_test.db";
        File::create(path).unwrap();
        let pager = Pager::open(path).unwrap();

        let page = pager.allocate_page().unwrap();

        assert_eq!(page.id(), 0);
        assert_eq!(page.size(), crate::page::PAGE_SIZE);
        assert_eq!(page.data()[0], 0);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn allocating_page_does_not_write_to_disk() {
        let path = "bmsql_pager_allocate_no_write_test.db";
        File::create(path).unwrap();
        let pager = Pager::open(path).unwrap();
        let _page = pager.allocate_page().unwrap();

        assert_eq!(pager.size().unwrap(), 0);
        assert_eq!(pager.page_count().unwrap(), 0);

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn pager_allocates_next_page_id() {
        let path = "bmsql_pager_next_page_test.db";
        File::create(path).unwrap();
        let mut pager = Pager::open(path).unwrap();
        let page0 = Page::new(0);
        pager.write_page(&page0).unwrap();

        let page1 = pager.allocate_page().unwrap();

        assert_eq!(page1.id(), 1);
        assert_eq!(pager.page_count().unwrap(), 1);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn allocated_page_can_be_written_and_read() {
        let path = "bmsql_pager_allocated_page_test.db";
        File::create(path).unwrap();
        let mut pager = Pager::open(path).unwrap();

        let mut page = pager.allocate_page().unwrap();
        page.data_mut()[0] = 55;

        pager.write_page(&page).unwrap();

        let page = pager.read_page(0).unwrap();

        assert_eq!(page.id(), 0);
        assert_eq!(page.data()[0], 55);
        assert_eq!(pager.page_count().unwrap(), 1);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn pager_returns_cached_page() {
        let path = "bmsql_pager_cached_page_test.db";
        File::create(path).unwrap();

        let mut pager = Pager::open(path).unwrap();

        let mut page = Page::new(0);
        page.data_mut()[0] = 42;
        pager.write_page(&page).unwrap();

        let page = pager.read_page(0).unwrap();
        assert_eq!(page.data()[0], 42);

        let mut database_file = DatabaseFile::open(path).unwrap();

        let mut updated_page = Page::new(0);
        updated_page.data_mut()[0] = 99;
        database_file.write_page(&updated_page).unwrap();

        let page = pager.read_page(0).unwrap();

        assert_eq!(page.data()[0], 42);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn pager_write_updates_cached_page() {
        let path = "bmsql_pager_cache_write_test.db";
        File::create(path).unwrap();

        let mut pager = Pager::open(path).unwrap();

        let mut page = Page::new(0);
        page.data_mut()[0] = 42;
        pager.write_page(&page).unwrap();

        let _ = pager.read_page(0).unwrap();

        let mut updated_page = Page::new(0);
        updated_page.data_mut()[0] = 99;
        pager.write_page(&updated_page).unwrap();

        let page = pager.read_page(0).unwrap();

        assert_eq!(page.data()[0], 99);

        std::fs::remove_file(path).unwrap();
    }
}
