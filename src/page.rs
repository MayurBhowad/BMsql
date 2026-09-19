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
    slots: Vec<Slot>,
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

    pub fn to_bytes(&self) -> [u8; SLOT_SIZE] {
        let mut data = [0u8; SLOT_SIZE];

        data[0..2].copy_from_slice(&self.offset.to_le_bytes());
        data[2..4].copy_from_slice(&self.length.to_le_bytes());

        data
    }

    pub fn from_bytes(data: [u8; SLOT_SIZE]) -> Self {
        let offset = u16::from_le_bytes([data[0], data[1]]);
        let length = u16::from_le_bytes([data[2], data[3]]);

        Self { offset, length }
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
            slots: Vec::new(),
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

        if (end + PAGE_HEADER_SIZE + SLOT_SIZE) > self.header.slot_directory_offset() as usize {
            return Err(BmsqlError::InvalidInput(
                "record does not fit in page".to_string(),
            ));
        }

        self.data[offset..end].copy_from_slice(record);

        self.header.record_count += 1;
        self.header.free_space_offset = (end + PAGE_HEADER_SIZE) as u16;

        let slot = Slot::new(
            self.header.free_space_offset() - record.len() as u16,
            record.len() as u16,
        );

        self.header.slot_directory_offset -= SLOT_SIZE as u16;

        self.write_slot(slot);

        self.slots.push(slot);

        Ok(())
    }

    pub fn from_data(id: PageId, data: [u8; PAGE_SIZE]) -> Self {
        let mut page_data = [0u8; PAGE_DATA_SIZE];

        page_data.copy_from_slice(&data[PAGE_HEADER_SIZE..]);

        let header = PageHeader::from_bytes(data[..PAGE_HEADER_SIZE].try_into().unwrap());
        let mut slots = Vec::new();

        for i in 0..header.record_count() as usize {
            let offset = PAGE_SIZE - ((i + 1) * SLOT_SIZE);
            let data_offset = offset - PAGE_HEADER_SIZE;
            let slot_data = page_data[data_offset..data_offset + SLOT_SIZE].try_into().unwrap();
            slots.push(Slot::from_bytes(slot_data));
        }

        Self {
            id,
            header,
            data: page_data,
            slots,
        }
    }

    pub fn to_bytes(&self) -> [u8; PAGE_SIZE] {
        let mut data = [0u8; PAGE_SIZE];

        data[..PAGE_HEADER_SIZE].copy_from_slice(&self.header.to_bytes());
        data[PAGE_HEADER_SIZE..].copy_from_slice(&self.data);

        data
    }

    pub fn slot_count(&self) -> usize {
        self.slots.len()
    }

    pub fn slots(&self, index: usize) -> Option<Slot> {
        self.slots.get(index).cloned()
    }

    fn write_slot(&mut self, slot: Slot) {
        let slot_data = slot.to_bytes();

        let slot_directory_offset = self.header.slot_directory_offset() as usize - PAGE_HEADER_SIZE;

        self.data[slot_directory_offset..slot_directory_offset + SLOT_SIZE].copy_from_slice(&slot_data);
    }

    pub fn read_record(&self, index: usize) -> Option<&[u8]> {
        let slot = self.slots.get(index)?;
        let offset = slot.offset() as usize - PAGE_HEADER_SIZE;
        let end = offset + slot.length() as usize;

        Some(&self.data[offset..end])
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

    #[test]
    fn slot_can_be_serialized() {
        let slot = Slot::new(7, 5);

        let data = slot.to_bytes();

        assert_eq!(&data[0..2], &7u16.to_le_bytes());
        assert_eq!(&data[2..4], &5u16.to_le_bytes());
    }

    #[test]
    fn slot_can_be_deserialized() {
        let data = [
            7u16.to_le_bytes()[0],
            7u16.to_le_bytes()[1],
            5u16.to_le_bytes()[0],
            5u16.to_le_bytes()[1],
        ];

        let slot = Slot::from_bytes(data);

        assert_eq!(slot.offset(), 7);
        assert_eq!(slot.length(), 5);
    }

    #[test]
    fn slot_round_trip() {
        let original = Slot::new(123, 45);

        let date = original.to_bytes();
        let restored = Slot::from_bytes(date);

        assert_eq!(restored.offset(), 123);
        assert_eq!(restored.length(), 45);
    }

    #[test]
    fn new_page_has_no_slots() {
        let page = Page::new(1);

        assert_eq!(page.slot_count(), 0);
    }

    #[test]
    fn page_insert_creates_slot() {
        let mut page = Page::new(1);

        page.insert_record(b"hello").unwrap();

        let slot = page.slots(0).unwrap();

        assert_eq!(slot.offset(), 7);
        assert_eq!(slot.length(), 5);
    }

    #[test]
    fn page_insert_moves_solt_directory() {
        let mut page = Page::new(1);
        page.insert_record(b"hello").unwrap();
        assert_eq!(page.header.slot_directory_offset(), 4092);
    }

    #[test]
    fn page_insert_moves_slot_directory_for_multiple_records() {
        let mut page = Page::new(1);

        page.insert_record(b"hello").unwrap();
        assert_eq!(page.header.slot_directory_offset(), 4092);

        page.insert_record(b"world").unwrap();
        assert_eq!(page.header.slot_directory_offset(), 4088);
    }

    #[test]
    fn page_stores_slot_in_data() {
        let mut page = Page::new(1);
        page.insert_record(b"hello").unwrap();
        let data = page.data();
        let slot_offset = 4092 - PAGE_HEADER_SIZE;

        assert_eq!(
            &data[slot_offset..slot_offset + SLOT_SIZE],
            &Slot::new(7, 5).to_bytes()
        );
    }

    #[test]
    fn page_stores_multiple_slots_in_data() {
        let mut page = Page::new(1);

        page.insert_record(b"hello").unwrap();
        page.insert_record(b"world").unwrap();
        let data = page.data();
        let first_slot_offset = 4092 - PAGE_HEADER_SIZE;
        let second_slot_offset = 4088 - PAGE_HEADER_SIZE;

        assert_eq!(
            &data[first_slot_offset..first_slot_offset + SLOT_SIZE],
            &Slot::new(7, 5).to_bytes()
        );
        assert_eq!(
            &data[second_slot_offset..second_slot_offset + SLOT_SIZE],
            &Slot::new(12, 5).to_bytes()
        );
    }

    #[test]
    fn page_rejects_record_when_slot_does_not_fit() {
        let mut page = Page::new(1);
        let record = vec![0u8; PAGE_DATA_SIZE - SLOT_SIZE + 1];
        let result = page.insert_record(&record);
        assert!(result.is_err());
    }

    #[test]
    fn rejected_record_does_not_change_page() {
        let mut page = Page::new(1);

        page.insert_record(b"hello").unwrap();

        let record_count = page.header.record_count();
        let free_space_offset = page.header.free_space_offset();
        let slot_directory_offset = page.header.slot_directory_offset();
        let slot_count = page.slot_count();

        let record = vec![0u8; PAGE_DATA_SIZE];

        let result = page.insert_record(&record);

        assert!(result.is_err());
        assert_eq!(page.header.record_count(), record_count);
        assert_eq!(page.header.free_space_offset(), free_space_offset);
        assert_eq!(page.header.slot_directory_offset(), slot_directory_offset);
        assert_eq!(page.slot_count(), slot_count);
    }

    #[test]
    fn page_can_read_record() {
        let mut page = Page::new(1);
        page.insert_record(b"hello").unwrap();
        let record = page.read_record(0).unwrap();
        assert_eq!(record, b"hello");
    }

    #[test]
    fn page_can_read_multiple_records() {
        let mut page = Page::new(1);

        page.insert_record(b"hello").unwrap();
        page.insert_record(b"world").unwrap();

        assert_eq!(page.read_record(0).unwrap(), b"hello");
        assert_eq!(page.read_record(1).unwrap(), b"world");
    }

    #[test]
    fn page_returns_none_for_invalid_record_index() {
        let mut page = Page::new(1);
        page.insert_record(b"hello").unwrap();
        assert!(page.read_record(1).is_none());
    }

    #[test]
    fn page_slots_can_survive_serialization() {
        let mut page = Page::new(1);

        page.insert_record(b"hello").unwrap();
        page.insert_record(b"world").unwrap();

        let data = page.to_bytes();

        let restored = Page::from_data(1, data);

        assert_eq!(restored.slot_count(), 2);

        let first = restored.slots(0).unwrap();
        let second = restored.slots(1).unwrap();

        assert_eq!(first.offset(), 7);
        assert_eq!(first.length(), 5);

        assert_eq!(second.offset(), 12);
        assert_eq!(second.length(), 5);
    }

    #[test]
    fn page_records_survive_serialization() {
        let mut page = Page::new(1);
        page.insert_record(b"hello").unwrap();
        page.insert_record(b"world").unwrap();
        let data = page.to_bytes();
        let restored = Page::from_data(1, data);
        assert_eq!(restored.read_record(0).unwrap(), b"hello");
        assert_eq!(restored.read_record(1).unwrap(), b"world");
    }
}
