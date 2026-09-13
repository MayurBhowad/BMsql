use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use crate::page::Page;

pub struct DatabaseFile {
    file: File,
}

impl DatabaseFile {
    pub fn open(path: &str) -> std::io::Result<Self> {
        let file = File::options().read(true).write(true).open(path)?;
        Ok(Self { file })
    }

    pub fn write_page(&mut self, page: &Page) -> std::io::Result<()> {
        let offset = crate::page::page_offset(page.id());

        self.file.seek(SeekFrom::Start(offset))?;

        self.file.write_all(page.data())?;

        Ok(())
    }

    pub fn read_page(&mut self, page_id: crate::page::PageId) -> std::io::Result<Page> {
        let offset = crate::page::page_offset(page_id);
        self.file.seek(SeekFrom::Start(offset))?;

        let mut data = [0u8; 4096];

        self.file.read_exact(&mut data)?;

        Ok(Page::from_data(page_id, data))
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
}