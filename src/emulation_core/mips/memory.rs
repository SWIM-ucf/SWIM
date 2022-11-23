pub const CAPACITY_BYTES: usize = 4 * 1024; // 4 KB

pub struct Memory {
    pub memory: Vec<u8>,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            memory: vec![0; CAPACITY_BYTES],
        }
    }
}

impl Memory {
    // Determines if an address is valid in a given instance of Memory.
    // If invalid, returns an instance of Err describing the problem with
    // the address.
    fn check_valid_address(&self, address: usize) -> Result<(), String> {
        if address % 4 != 0 {
            return Err(String::from("Address is not byte-aligned"));
        } else if address > self.memory.len() {
            return Err(String::from("Address out of bounds of memory"));
        }

        Ok(())
    }

    // A word is 32 bits.
    pub fn store_word(&mut self, address: usize, data: u32) -> Result<(), String> {
        self.check_valid_address(address)?;

        self.memory[address] = ((data >> 24) & 0b11111111) as u8;
        self.memory[address + 1] = ((data >> 16) & 0b11111111) as u8;
        self.memory[address + 2] = ((data >> 8) & 0b11111111) as u8;
        self.memory[address + 3] = (data & 0b11111111) as u8;

        Ok(())
    }

    // A word is 32 bits.
    pub fn load_word(&self, address: usize) -> Result<u32, String> {
        self.check_valid_address(address)?;

        let mut result: u32 = 0;
        result |= (self.memory[address] as u32) << 24;
        result |= (self.memory[address + 1] as u32) << 16;
        result |= (self.memory[address + 2] as u32) << 8;
        result |= self.memory[address + 3] as u32;

        Ok(result)
    }
}
