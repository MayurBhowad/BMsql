use crate::error::BmsqlError;
pub const  PAGE_SIZE: usize = 4096;
pub const PAGE_HEADER_SIZE: usize = 7;
pub const PAGE_DATA_SIZE: usize = PAGE_SIZE - PAGE_HEADER_SIZE;
pub const SLOT_SIZE: usize = 4;
pub type PageId = u64;

#[derive(Clone, Copy)]
pub struct Slot {
    offset: u16,
    length: u16,
}

#[derive(Clone)]
pub struct PageHeader {
    page_type: u8,
    record_count: u16,
    free_space_offset: u16,
    slot_directory_offset: u16,
}

#[derive(Clone)]
pub struct Page {
    id: PageId,
    header: PageHeader,
    data: [u8; PAGE_DATA_SIZE],
}

impl Slot {
    pub fn new(offset: u16, length: u16) -> Self {
        Self { offset, length }
    }

    pub fn offset(&self) -> u16 {
        self.offset
    }

    pub fn length(&self) -> u16 {
        self.length
    }
}

impl PageHeader {
    pub fn new(page_type: u8, free_space_offset: u16, slot_directory_offset: u16) -> Self {
        Self { page_type, record_count: 0, free_space_offset, slot_directory_offset }
    }

    pub fn page_type(&self) -> u8 {
        self.page_type
    }

    pub fn record_count(&self) -> u16 {
        self.record_count
    }

    pub fn free_space_offset(&self) -> u16 {
        self.free_space_offset
    }

    pub fn slot_directory_offset(&self) -> u16 {
        self.slot_directory_offset
    }

    pub fn to_bytes(&self) -> [u8; PAGE_HEADER_SIZE] {
        let mut data = [0u8; PAGE_HEADER_SIZE];

        data[0] = self.page_type;
        data[1..3].copy_from_slice(&self.record_count.to_le_bytes());
        data[3..5].copy_from_slice(&self.free_space_offset.to_le_bytes());

        data
    }

    pub fn from_bytes(data: [u8; PAGE_HEADER_SIZE]) -> Self {
        let record_count = u16::from_le_bytes([data[1], data[2]]);
        let free_space_offset = u16::from_le_bytes([data[3], data[4]]);
        let slot_directory_offset = u16::from_le_bytes([data[5], data[6]]);

        Self {
            page_type: data[0],
            record_count,
            free_space_offset,
            slot_directory_offset
        }
    }
}

impl Page {
    pub fn new(id: PageId) -> Self {
        Self {
            id,
            header: PageHeader::new(0, PAGE_HEADER_SIZE as u16, PAGE_SIZE as u16),
            data: [0; PAGE_DATA_SIZE],
        }
    }

    pub fn id(&self) -> PageId {
        self.id
    }

    pub fn size(&self) -> usize {
        PAGE_SIZE
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }   

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn insert_record(&mut self, record: &[u8]) -> Result<(), BmsqlError> {
        let offset = self.header.free_space_offset() as usize - PAGE_HEADER_SIZE;
        let end = offset + record.len();

        if end > PAGE_DATA_SIZE {
            return Err(BmsqlError::InvalidInput(
                "record does not fit in page".to_string(),
            ));
        }

        self.data[offset..end].copy_from_slice(record);

        self.header.record_count += 1;
        self.header.free_space_offset = (end + PAGE_HEADER_SIZE) as u16;

        Ok(())
    }

    pub fn from_data(id: PageId, data: [u8; PAGE_SIZE]) -> Self {
        let mut page_data = [0u8; PAGE_DATA_SIZE];

        page_data.copy_from_slice(&data[PAGE_HEADER_SIZE..]);

        Self {
            id,
            header: PageHeader::from_bytes(data[..PAGE_HEADER_SIZE].try_into().unwrap()),
            data: page_data,
        }
    }

    pub fn to_bytes(&self) -> [u8; PAGE_SIZE] {
        let mut data = [0u8; PAGE_SIZE];

        data[..PAGE_HEADER_SIZE].copy_from_slice(&self.header.to_bytes());
        data[PAGE_HEADER_SIZE..].copy_from_slice(&self.data);

        data
    }
}

pub fn page_offset(page_id: PageId) -> u64 {
    page_id * PAGE_SIZE as u64
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
        println!("Last byte: {}", page.data()[PAGE_DATA_SIZE - 1]);

        assert_eq!(page.data()[0], 0);
        assert_eq!(page.data()[PAGE_DATA_SIZE - 1], 0);
    }

    #[test]
    fn page_has_correct_id() {
        let page = Page::new(42);

        assert_eq!(page.id(), 42);
    }

    #[test]
    fn page_id_has_correct_offset() {
        assert_eq!(page_offset(0), 0);
        assert_eq!(page_offset(1), 4096);
        assert_eq!(page_offset(2), 8192);
        assert_eq!(page_offset(42), 172032);
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
        file.write_all(&page.to_bytes()).unwrap();
        assert_eq!(std::fs::metadata(path).unwrap().len(), 4096);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn file_can_seek_to_page_offset() {
        let path = "bmsql_seek_test.db";
        let mut file = create_database_file(path).unwrap();
        file.seek(std::io::SeekFrom::Start(page_offset(1))).unwrap();
        file.write_all(b"BMsql").unwrap();
        assert_eq!(std::fs::metadata(path).unwrap().len(), 4101);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn page_header_has_correct_values() {
        let header = PageHeader::new(1, 5, PAGE_SIZE as u16);

        assert_eq!(header.page_type(), 1);
        assert_eq!(header.record_count(), 0);
        assert_eq!(header.free_space_offset(), 5);
        assert_eq!(header.slot_directory_offset(), PAGE_SIZE as u16);
    }

    #[test]
    fn page_header_has_correct_size() {
        assert_eq!(PAGE_HEADER_SIZE, 7);
    }

    #[test]
    fn page_from_data_preserves_data_after_header() {
        let mut data = [0u8; PAGE_SIZE];

        data[PAGE_HEADER_SIZE] = 42;

        let page = Page::from_data(0, data);

        assert_eq!(page.data()[0], 42);
    }

    #[test]
    fn page_total_size_is_4096_bytes() {
        assert_eq!(PAGE_HEADER_SIZE + PAGE_DATA_SIZE, PAGE_SIZE);
    }

    #[test]
    fn page_header_can_be_serialized() {
        let header = PageHeader::new(1, 5, PAGE_SIZE as u16);

        let data = header.to_bytes();

        assert_eq!(data[0], 1);
        assert_eq!(&data[1..3], &0u16.to_le_bytes());
        assert_eq!(&data[3..5], &5u16.to_le_bytes());
    }

    #[test]
    fn page_can_be_serialized_to_4096_bytes() {
        let mut page = Page::new(0);
        page.data_mut()[0] = 42;

        let data = page.to_bytes();

        assert_eq!(data.len(), PAGE_SIZE);
        assert_eq!(data[PAGE_HEADER_SIZE], 42);
    }

    #[test]
    fn page_header_can_be_deserialized() {
        let header = PageHeader::new(1, 5, PAGE_SIZE as u16);

        let data = header.to_bytes();
        let restored = PageHeader::from_bytes(data);

        assert_eq!(restored.page_type(), 1);
        assert_eq!(restored.record_count(), 0);
        assert_eq!(restored.free_space_offset(), 5);
    }

    #[test]
    fn page_from_data_preserves_header() {
        let header = PageHeader::new(1, 100, PAGE_SIZE as u16);

        let mut data = [0u8; PAGE_SIZE];

        data[..PAGE_HEADER_SIZE].copy_from_slice(&header.to_bytes());

        let page = Page::from_data(0, data);

        assert_eq!(page.header.page_type(), 1);
        assert_eq!(page.header.record_count(), 0);
        assert_eq!(page.header.free_space_offset(), 100);
    }

    #[test]
    fn page_can_insert_record() {
        let mut page = Page::new(0);

        let record = b"hello";

        page.insert_record(record).unwrap();

        assert_eq!(&page.data()[0..5], b"hello");
        assert_eq!(page.header.record_count(), 1);
        assert_eq!(page.header.free_space_offset(), 12);
    }

    #[test]
    fn page_can_insert_multiple_records() {
        let mut page = Page::new(0);

        page.insert_record(b"hello").unwrap();
        page.insert_record(b"world").unwrap();

        assert_eq!(&page.data()[0..5], b"hello");
        assert_eq!(&page.data()[5..10], b"world");

        assert_eq!(page.header.record_count(), 2);
        assert_eq!(page.header.free_space_offset(), 17);
    }

    #[test]
    fn page_rejects_record_when_not_enough_space() {
        let mut page = Page::new(0);

        let record = vec![0u8; PAGE_DATA_SIZE + 1];

        let result = page.insert_record(&record);

        assert!(result.is_err());
    }

    #[test]
    fn page_inserts_records_after_existing_data(){
        let mut page = Page::new(0);

        page.insert_record(b"hello").unwrap();
        page.insert_record(b"BMsql").unwrap();

        assert_eq!(&page.data()[0..5], b"hello");
        assert_eq!(&page.data()[5..10], b"BMsql");
        assert_eq!(page.header.record_count(), 2);
        assert_eq!(page.header.free_space_offset(), 17);
    }

    #[test]
    fn slot_has_correct_values() {
        let slot = Slot::new(7, 5);

        assert_eq!(slot.offset(), 7);
        assert_eq!(slot.length(), 5);
    }
}
