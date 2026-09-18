use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use crate::page::{Page, PAGE_SIZE};

pub struct DatabaseFile {
    file: File,
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
}
