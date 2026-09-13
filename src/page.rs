const PAGE_SIZE: usize = 4096;

pub type PageId = u64;

pub struct Page {
    id: PageId,
    data: [u8; PAGE_SIZE],
}

impl Page {
    pub fn new(id: PageId) -> Self {
        Self {
            id,
            data: [0; PAGE_SIZE],
        }
    }

    pub fn id(&self) -> PageId {
        self.id
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn page_offset(page_id: PageId) -> u64 {
        page_id * PAGE_SIZE as u64
    }   

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::create_database_file;
    use crate::database::open_database_file;
    use std::io::{Read, Write, Seek};

    #[test]
    fn page_has_correct_size() {
        let page = Page::new(0);

        assert_eq!(page.size(), 4096);
    }

    #[test]
    fn new_page_contains_zeroes() {
        let page = Page::new(0);

        println!("First byte: {}", page.data()[0]);
        println!("Second byte: {}", page.data()[1]);
        println!("Last byte: {}", page.data()[4095]);

        assert_eq!(page.data()[0], 0);
        assert_eq!(page.data()[4095], 0);
    }

    #[test]
    fn page_has_correct_id() {
        let page = Page::new(42);

        assert_eq!(page.id(), 42);
    }

    #[test]
    fn page_id_has_correct_offset() {
        assert_eq!(Page::page_offset(0), 0);
        assert_eq!(Page::page_offset(1), 4096);
        assert_eq!(Page::page_offset(2), 8192);
        assert_eq!(Page::page_offset(42), 172032);
    }

    #[test]
    fn database_file_can_be_created() {
        let path = "bmsql_test.db";
        let _file = create_database_file(path).unwrap();
        assert!(std::path::Path::new(path).exists());
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn existing_database_file_can_be_opened() {
        let path = "bmsql_open_test.db";
        let _file = create_database_file(path).unwrap();
        let _file = open_database_file(path).unwrap();
        assert!(std::path::Path::new(path).exists());

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn database_file_can_store_bytes() {
        let path = "bmsql_write_test.db";
        let mut file = create_database_file(path).unwrap();
        file.write_all(b"BMsql").unwrap();
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn database_file_can_read_bytes() {
        let path = "bmsql_read_test.db";
        let mut file = create_database_file(path).unwrap();
        file.write_all(b"BMsql").unwrap();
        drop(file);
        //Start reading from the file
        let mut file = open_database_file(path).unwrap();

        let mut buffer = [0u8; 5];
        file.read_exact(&mut buffer).unwrap();
        assert_eq!(&buffer, b"BMsql");
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn page_can_be_written_to_database_file() {
        let path = "bmsql_write_page_test.db";
        let page = Page::new(0);
        let mut file = create_database_file(path).unwrap();
        file.write_all(page.data()).unwrap();
        assert_eq!(std::fs::metadata(path).unwrap().len(), 4096);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn file_can_seek_to_page_offset() {
        let path = "bmsql_seek_test.db";
        let mut file = create_database_file(path).unwrap();
        file.seek(std::io::SeekFrom::Start(Page::page_offset(1))).unwrap();
        file.write_all(b"BMsql").unwrap();
        assert_eq!(std::fs::metadata(path).unwrap().len(), 4101);
        std::fs::remove_file(path).unwrap();
    }
}