use crate::error::BmsqlError;
use crate::page::{Page, PageId};
use crate::storage::PageManager;
use std::collections::HashMap;

pub struct Pager {
    page_manager: PageManager,
    page_cache: HashMap<PageId, Page>,
    cache_capacity: usize,
    cache_order: Vec<PageId>,
}

impl Pager {
    pub fn open(path: &str, cache_capacity: usize) -> Result<Self, BmsqlError> {
        if cache_capacity == 0 {
            return Err(BmsqlError::InvalidInput(
                "cache capacity must be greater than zero".to_string(),
            ));
        }
        let page_manager = PageManager::open(path)?;
        Ok(Self {
            page_manager,
            page_cache: HashMap::new(),
            cache_capacity,
            cache_order: Vec::new(),
        })
    }

    pub fn read_page(&mut self, page_id: PageId) -> Result<Page, BmsqlError> {
        if let Some(page) = self.page_cache.get(&page_id) {
            return Ok(page.clone());
        }
        let page = self.page_manager.read_page(page_id)?;

        if self.page_cache.len() >= self.cache_capacity {
            let oldest_page_id = self.cache_order.remove(0);
            self.page_cache.remove(&oldest_page_id);
        }

        self.page_cache.insert(page_id, page.clone());
        self.cache_order.push(page_id);

        Ok(page)
    }

    pub fn write_page(&mut self, page: &Page) -> Result<(), BmsqlError> {
        self.page_manager.write_page(page)?;

        if self.page_cache.contains_key(&page.id()) {
            self.page_cache.insert(page.id(), page.clone());
            return Ok(());
        }

        if self.page_cache.len() >= self.cache_capacity {
            let oldest_page_id = self.cache_order.remove(0);
            self.page_cache.remove(&oldest_page_id);
        }

        self.page_cache.insert(page.id(), page.clone());
        self.cache_order.push(page.id());
        Ok(())
    }

    pub fn size(&self) -> Result<u64, BmsqlError> {
        self.page_manager.size()
    }

    pub fn page_count(&self) -> Result<u64, BmsqlError> {
        self.page_manager.page_count()
    }

    pub fn allocate_page(&mut self) -> Result<Page, BmsqlError> {
        self.page_manager.allocate_page()
    }

    pub fn cache_size(&self) -> usize {
        self.page_cache.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::DatabaseFile;
    use std::fs::File;

    #[test]
    fn pager_can_be_opened() {
        let path = "bmsql_pager_test.db";
        File::create(path).unwrap();
        let pager = Pager::open(path, 3);
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
        let mut pager = Pager::open(path, 3).unwrap();
        let page = pager.read_page(0).unwrap();

        assert_eq!(page.data()[0], 42);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn pager_can_write_page() {
        let path = "bmsql_pager_write_test.db";
        File::create(path).unwrap();
        let mut pager = Pager::open(path, 3).unwrap();
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
        let result = Pager::open(path, 3);
        assert!(result.is_err());
    }

    #[test]
    fn pager_reports_database_file_size() {
        let path = "bmsql_pager_size_test.db";
        File::create(path).unwrap();
        let mut pager = Pager::open(path, 3).unwrap();
        let page = Page::new(0);
        pager.write_page(&page).unwrap();
        assert_eq!(pager.size().unwrap(), 4096);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn pager_reports_page_count() {
        let path = "bmsql_pager_page_count_test.db";
        File::create(path).unwrap();
        let mut pager = Pager::open(path, 3).unwrap();

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
        let mut pager = Pager::open(path, 3).unwrap();

        let page = pager.allocate_page().unwrap();

        assert_eq!(page.id(), 0);
        assert_eq!(page.size(), crate::page::PAGE_SIZE);
        assert_eq!(page.data()[0], 0);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn allocating_page_writes_to_disk() {
        let path = "bmsql_pager_allocate_write_test.db";
        File::create(path).unwrap();
        let mut pager = Pager::open(path, 3).unwrap();
        let _page = pager.allocate_page().unwrap();

        assert_eq!(pager.size().unwrap(), crate::page::PAGE_SIZE as u64);
        assert_eq!(pager.page_count().unwrap(), 1);

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn pager_allocates_next_page_id() {
        let path = "bmsql_pager_next_page_test.db";
        File::create(path).unwrap();
        let mut pager = Pager::open(path, 3).unwrap();
        let page0 = Page::new(0);
        pager.write_page(&page0).unwrap();

        let page1 = pager.allocate_page().unwrap();

        assert_eq!(page1.id(), 1);
        assert_eq!(pager.page_count().unwrap(), 2);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn allocated_page_can_be_written_and_read() {
        let path = "bmsql_pager_allocated_page_test.db";
        File::create(path).unwrap();
        let mut pager = Pager::open(path, 3).unwrap();

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

        let mut pager = Pager::open(path, 3).unwrap();

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

        let mut pager = Pager::open(path, 3).unwrap();

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

    #[test]
    fn pager_cache_respects_capacity() {
        let path = "bmsql_pager_cache_capacity_test.db";
        File::create(path).unwrap();

        let mut pager = Pager::open(path, 2).unwrap();

        let page0 = Page::new(0);
        let page1 = Page::new(1);
        let page2 = Page::new(2);

        pager.write_page(&page0).unwrap();
        pager.write_page(&page1).unwrap();
        pager.write_page(&page2).unwrap();

        pager.read_page(0).unwrap();
        pager.read_page(1).unwrap();
        pager.read_page(2).unwrap();

        assert_eq!(pager.cache_size(), 2);

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn writing_cached_page_does_not_increase_cache_size() {
        let path = "bmsql_pager_cache_existing_write_test.db";
        File::create(path).unwrap();

        let mut pager = Pager::open(path, 2).unwrap();

        let page0 = Page::new(0);
        pager.write_page(&page0).unwrap();

        assert_eq!(pager.cache_size(), 1);

        let mut updated_page0 = Page::new(0);
        updated_page0.data_mut()[0] = 99;
        pager.write_page(&updated_page0).unwrap();

        assert_eq!(pager.cache_size(), 1);

        let page = pager.read_page(0).unwrap();
        assert_eq!(page.data()[0], 99);

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn pager_evicts_oldest_cached_page() {
        let path = "bmsql_pager_fifo_test.db";
        File::create(path).unwrap();

        let mut pager = Pager::open(path, 2).unwrap();

        let mut page0 = Page::new(0);
        page0.data_mut()[0] = 10;

        let mut page1 = Page::new(1);
        page1.data_mut()[0] = 20;

        let mut page2 = Page::new(2);
        page2.data_mut()[0] = 30;

        pager.write_page(&page0).unwrap();
        pager.write_page(&page1).unwrap();
        pager.write_page(&page2).unwrap();

        pager.read_page(0).unwrap();
        pager.read_page(1).unwrap();

        pager.read_page(2).unwrap();

        let mut updated_page0 = Page::new(0);
        updated_page0.data_mut()[0] = 99;

        let mut database_file = DatabaseFile::open(path).unwrap();
        database_file.write_page(&updated_page0).unwrap();

        let page0 = pager.read_page(0).unwrap();

        assert_eq!(page0.data()[0], 99);

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn pager_rejects_zero_size_capacity() {
        let path = "bmsql_pager_zero_size_capacity_test.db";
        File::create(path).unwrap();

        let result = Pager::open(path, 0);
        assert!(result.is_err());

        std::fs::remove_file(path).unwrap();
    }
}
