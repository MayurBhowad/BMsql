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
}


#[cfg(test)]
mod tests {
    use super::*;

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
}