const CAPACITY_BYTES: usize = 4 * 1024; // 4 KB

pub struct Memory {
    pub memory: Vec<u8>,
}

impl Default for Memory {
    fn default() -> Self {
        let memory: Vec<u8> = vec![0; CAPACITY_BYTES];
        Self { memory }
    }
}
