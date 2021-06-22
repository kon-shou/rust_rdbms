pub struct DiskManager {
    heap_file: File,
    next_page_id: u64,
}

impl DiskManager {
    pub fn new(data_file: File) -> io::Result<Self> {}
    pub fn open(data_file_path: impl AsRef<Path>) {}
    pub fn allocate_page(&mut self) -> PageId {}
    pub fn read_page_data(&mut self, page_id: PageId, data: &mut[u8]) -> io::Result<()>{}
}
