//! Data and instruction memory implementation and API.

// pub const CAPACITY_BYTES: usize = 2^12; // 4KB
pub const CAPACITY_BYTES: usize = 64 * 1024; // 64 KB

#[derive(PartialEq)]
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

impl ToString for Memory {
    fn to_string(&self) -> String {
        let mut output = String::new();

        for byte in self.memory.iter() {
            output.push_str(&format!("{byte:x}"));
        }

        output
    }
}

impl Memory {
    /// Determines if an address is valid in a given instance of Memory.
    /// If invalid, returns an instance of Err describing the problem with
    /// the address.
    fn check_valid_address(&self, address: usize) -> Result<(), String> {
        if address % 4 != 0 {
            Err(format!("Address `{address}` is not word-aligned"))
        } else if address > self.memory.len() {
            Err(format!(
                "Address `{}` out of bounds of memory of size {}",
                address,
                self.memory.len()
            ))
        } else {
            Ok(())
        }
    }

    // A word is 32 bits.
    pub fn store_word(&mut self, address: u64, data: u32) -> Result<(), String> {
        let address = address as usize;

        self.check_valid_address(address)?;

        self.memory[address] = ((data >> 24) & 0b11111111) as u8;
        self.memory[address + 1] = ((data >> 16) & 0b11111111) as u8;
        self.memory[address + 2] = ((data >> 8) & 0b11111111) as u8;
        self.memory[address + 3] = (data & 0b11111111) as u8;

        Ok(())
    }

    pub fn store_double_word(&mut self, address: u64, data: u64) -> Result<(), String> {
        // Storing a doubleword is the same as storing two words.
        let data_upper = (data >> 32) as u32;
        let data_lower = data as u32;

        self.store_word(address, data_upper)?;
        self.store_word(address + 4, data_lower)?;

        Ok(())
    }

    // A word is 32 bits.
    pub fn load_word(&self, address: u64) -> Result<u32, String> {
        let address = address as usize;

        self.check_valid_address(address)?;

        let mut result: u32 = 0;
        result |= (self.memory[address] as u32) << 24;
        result |= (self.memory[address + 1] as u32) << 16;
        result |= (self.memory[address + 2] as u32) << 8;
        result |= self.memory[address + 3] as u32;

        Ok(result)
    }

    pub fn load_double_word(&self, address: u64) -> Result<u64, String> {
        // Loading a doubleword is the same as loading two words.
        let mut result: u64 = 0;

        // Get the upper 32 bits of the doubleword.
        match self.load_word(address) {
            Ok(upper_data) => {
                result |= (upper_data as u64) << 32;
            }
            Err(e) => return Err(e),
        }

        // Get the lower 32 bits of the doubleword.
        match self.load_word(address + 4) {
            Ok(lower_data) => {
                result |= lower_data as u64;
            }
            Err(e) => return Err(e),
        }

        Ok(result)
    }
}
