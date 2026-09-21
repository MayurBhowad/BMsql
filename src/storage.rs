use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use crate::page::{Page, PAGE_SIZE};

pub struct DatabaseFile {
    file: File,
}

pub struct PageManager {
    database_file: DatabaseFile,
}

impl DatabaseFile {
    pub fn open(path: &str) -> Result<Self, crate::error::BmsqlError> {
        let file = File::options().read(true).write(true).open(path)?;
        Ok(Self { file })
    }

    pub fn write_page(&mut self, page: &Page) -> Result<(), crate::error::BmsqlError> {
        let offset = crate::page::page_offset(page.id());

        self.file.seek(SeekFrom::Start(offset))?;

        self.file.write_all(&page.to_bytes())?;

        Ok(())
    }


    /// Reads a complete page from the database file.
    /// 
    /// Returns an IO error if the requested page does not contain enough bytes to fill a complete page.
    pub fn read_page(&mut self, page_id: crate::page::PageId) -> Result<Page, crate::error::BmsqlError> {
        let offset = crate::page::page_offset(page_id);
        self.file.seek(SeekFrom::Start(offset))?;

        let mut data = [0u8; PAGE_SIZE];

        self.file.read_exact(&mut data)?;

        Ok(Page::from_data(page_id, data))
    }

    pub fn size(&self) -> Result<u64, crate::error::BmsqlError> {
        Ok(self.file.metadata()?.len())
    }
}

impl PageManager {
    pub fn open(path: &str) -> Result<Self, crate::error::BmsqlError> {
        let database_file = DatabaseFile::open(path)?;
        Ok(Self { database_file })
    }

    pub fn size(&self) -> Result<u64, crate::error::BmsqlError> {
        self.database_file.size()
    }

    pub fn allocate_page(&mut self) -> Result<Page, crate::error::BmsqlError> {
        let page_id = self.page_count()?;
        let page = Page::new(page_id);
        self.write_page(&page)?;
        Ok(page)
    }

    pub fn read_page(
        &mut self,
        page_id: crate::page::PageId,
    ) -> Result<Page, crate::error::BmsqlError> {
        self.database_file.read_page(page_id)
    }

    pub fn write_page(
        &mut self,
        page: &Page,
    ) -> Result<(), crate::error::BmsqlError> {
        self.database_file.write_page(page)
    }

    pub fn page_count(&self) -> Result<u64, crate::error::BmsqlError> {
        Ok(self.size()? / PAGE_SIZE as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::page::{Page, page_offset};
    use std::io::Read;

    #[test]
    fn database_file_can_be_opened() {
        let path = "bmsql_test.db";
        File::create(path).unwrap();
        let _database_file = DatabaseFile::open(path).unwrap();
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn database_file_can_write_page() {
        let path = "bmsql_write_page_test.db";
        File::create(path).unwrap();
        let mut database_file = DatabaseFile::open(path).unwrap();
        let page = Page::new(0);
        database_file.write_page(&page).unwrap();
        assert_eq!(std::fs::read(path).unwrap().len(), 4096);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn database_file_can_write_multiple_pages() {
        let path = "bmsql_write_multiple_pages_test.db";

        File::create(path).unwrap();

        let mut database_file = DatabaseFile::open(path).unwrap();

        let mut page0 = Page::new(0);
        let mut page1 = Page::new(1);

        page0.data_mut()[0] = 10;
        page1.data_mut()[0] = 20;

        database_file.write_page(&page0).unwrap();
        database_file.write_page(&page1).unwrap();

        drop(database_file);

        let mut file = File::open(path).unwrap();

        file.seek(std::io::SeekFrom::Start(page_offset(0))).unwrap();

        let mut buffer0 = [0u8; 1];

        file.read_exact(&mut buffer0).unwrap();

        assert_eq!(std::fs::metadata(path).unwrap().len(), 8192);

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn database_file_can_read_page() {
        let path = "bmsql_read_page_test.db";

        File::create(path).unwrap();

        let mut database_file = DatabaseFile::open(path).unwrap();

        let mut page = Page::new(0);
        page.data_mut()[0] = 42;
        database_file.write_page(&page).unwrap();

        let page = database_file.read_page(0).unwrap();

        assert_eq!(page.id(), 0);
        assert_eq!(page.data()[0], 42);
        assert_eq!(page.size(), 4096);

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn database_file_can_read_second_page() {
        let path = "bmsql_read_second_page_test.db";
        File::create(path).unwrap();

        let mut database_file = DatabaseFile::open(path).unwrap();

        let mut page0 = Page::new(0);
        let mut page1 = Page::new(1);

        page0.data_mut()[0] = 42;
        page1.data_mut()[0] = 99;

        database_file.write_page(&page0).unwrap();
        database_file.write_page(&page1).unwrap();

        let page = database_file.read_page(1).unwrap();

        assert_eq!(page.id(), 1);
        assert_eq!(page.data()[0], 99);
        assert_eq!(page.size(), 4096);

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn page_data_persists_after_reopening_database() {
        let path = "bmsql_persistence_test.db";
        File::create(path).unwrap();

        {
            let mut database_file = DatabaseFile::open(path).unwrap();

            let mut page = Page::new(1);
            page.data_mut()[0] = 123;

            database_file.write_page(&page).unwrap();
        }

        {
            let mut database_file = DatabaseFile::open(path).unwrap();

            let page = database_file.read_page(1).unwrap();

            assert_eq!(page.id(), 1);
            assert_eq!(page.data()[0], 123);
        }

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn reading_missing_page_returns_error() {
        let path = "bmsql_reading_missing_page_test.db";
        File::create(path).unwrap();
        let mut database_file = DatabaseFile::open(path).unwrap();
        let result = database_file.read_page(1);

        assert!(result.is_err());

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn reading_missing_page_returns_io_error() {
        let path = "bmsql_missing_page_test.db";
        File::create(path).unwrap();
        let mut database_file = DatabaseFile::open(path).unwrap();
        let result = database_file.read_page(1);
        match result {
            Err(crate::error::BmsqlError::Io(_)) => {}
            _ => panic!("Expected BmsqlError::Io"),
        }
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn database_file_reports_size() {
        let path = "bmsql_file_size_test.db";
        File::create(path).unwrap();
        let mut database_file = DatabaseFile::open(path).unwrap();
        let page = Page::new(0);
        database_file.write_page(&page).unwrap();
        assert_eq!(database_file.size().unwrap(), 4096);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn empty_database_file_has_size_zero() {
        let path = "bmsql_empty_file_size_test.db";
        File::create(path).unwrap();
        let database_file = DatabaseFile::open(path).unwrap();
        assert_eq!(database_file.size().unwrap(), 0);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn page_records_persist_after_reopening_database() {
        let path = "bmsql_page_records_persist_test.db";
        File::create(path).unwrap();
        let mut database_file = DatabaseFile::open(path).unwrap();
        let mut page = Page::new(0);
        page.insert_record(b"hello").unwrap();
        page.insert_record(b"world").unwrap();
        database_file.write_page(&page).unwrap();
        drop(database_file);
        let mut database_file = DatabaseFile::open(path).unwrap();
        let restored = database_file.read_page(0).unwrap();

        assert_eq!(restored.read_record(0).unwrap(), b"hello");
        assert_eq!(restored.read_record(1).unwrap(), b"world");
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn page_manager_can_be_opened() {
        let path = "bmsql_page_manager_test.db";
        File::create(path).unwrap();
        let page_manager = PageManager::open(path).unwrap();
        assert_eq!(page_manager.size().unwrap(), 0);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn page_manager_allocates_first_page_id() {
        let path = "bmsql_page_manager_allocate_first_page_id_test.db";
        File::create(path).unwrap();
        let mut page_manager = PageManager::open(path).unwrap();
        assert_eq!(page_manager.allocate_page().unwrap().id(), 0);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn page_manager_allocates_sequential_page_ids() {
        let path = "bmsql_page_manager_allocate_sequential_page_ids_test.db";
        File::create(path).unwrap();
        let mut page_manager = PageManager::open(path).unwrap();

        let page0 = page_manager.allocate_page().unwrap();

        let page1 = page_manager.allocate_page().unwrap();
        assert_eq!(page0.id(), 0);
        assert_eq!(page1.id(), 1);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn allocated_page_persists_after_reopening(){
        let path = "bmsql_page_manager_reopen_test.db";
        File::create(path).unwrap();
        {
            let mut page_manager = PageManager::open(path).unwrap();
            let page = page_manager.allocate_page().unwrap();
            assert_eq!(page.id(), 0);
        }
        {
            let mut page_manager = PageManager::open(path).unwrap();
            assert_eq!(page_manager.size().unwrap(), PAGE_SIZE as u64);
            let page = page_manager.read_page(0).unwrap();
            assert_eq!(page.id(), 0);
        }
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn page_manager_can_write_page() {
        let path = "bmsql_page_manager_write_test.db";

        File::create(path).unwrap();

        let mut page_manager = PageManager::open(path).unwrap();

        let mut page = Page::new(0);
        page.data_mut()[0] = 42;

        page_manager.write_page(&page).unwrap();

        let restored = page_manager.read_page(0).unwrap();

        assert_eq!(restored.id(), 0);
        assert_eq!(restored.data()[0], 42);

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn page_manager_reports_page_count() {
        let path = "bmsql_page_manager_page_count_test.db";

        File::create(path).unwrap();

        let mut page_manager = PageManager::open(path).unwrap();

        assert_eq!(page_manager.page_count().unwrap(), 0);

        page_manager.allocate_page().unwrap();

        assert_eq!(page_manager.page_count().unwrap(), 1);

        page_manager.allocate_page().unwrap();

        assert_eq!(page_manager.page_count().unwrap(), 2);

        std::fs::remove_file(path).unwrap();
    }
}
