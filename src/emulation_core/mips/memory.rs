//! Data and instruction memory implementation and API.

use serde::{Deserialize, Serialize};

// pub const CAPACITY_BYTES: usize = 2^12; // 4KB
pub const CAPACITY_BYTES: usize = 64 * 1024; // 64 KB

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
            output.push_str(&format!("{byte:02x}"));
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
        } else if address >= self.memory.len() {
            Err(format!(
                "Address `{}` out of bounds of memory of size {}",
                address,
                self.memory.len()
            ))
        } else {
            Ok(())
        }
    }

    // A byte is 8 bits.
    pub fn store_byte(&mut self, address: u64, data: u8) -> Result<(), String> {
        let address = address as usize;

        self.check_valid_address(address)?;

        self.memory[address] = (data & 0b11111111) as u8;

        Ok(())
    }

    // A word is 32 bits.
    pub fn store_half(&mut self, address: u64, data: u16) -> Result<(), String> {
        let address = address as usize;

        self.check_valid_address(address)?;

        self.memory[address] = ((data >> 8) & 0b11111111) as u8;
        self.memory[address + 1] = (data & 0b11111111) as u8;

        Ok(())
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

    // A byte is 8 bits.
    pub fn load_byte(&self, address: u64) -> Result<u8, String> {
        let address = address as usize;

        self.check_valid_address(address)?;

        let mut result: u8 = 0;
        result |= self.memory[address];

        Ok(result)
    }

    // A half-word is 16 bits.
    pub fn load_half(&self, address: u64) -> Result<u16, String> {
        let address = address as usize;

        self.check_valid_address(address)?;

        let mut result: u16 = 0;
        result |= (self.memory[address] as u16) << 8;
        result |= self.memory[address + 1] as u16;

        Ok(result)
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

    pub fn generate_formatted_hex(&self) -> String {
        let mut string: String = "".to_string();

        let mut base = 0;
        while base < self.memory.len() {
            string.push_str(&format!("0x{base:04x}:\t\t"));
            let mut char_version: String = "".to_string();

            for offset in 0..4 {
                let word_address = base as u64 + (offset * 4);
                if let Ok(word) = self.load_word(word_address) {
                    string.push_str(&format!("{word:08x}\t"));
                    char_version.push_str(&convert_word_to_chars(word))
                };
            }
            string.push_str(&format!("{char_version}\n"));
            base += 16;
        }
        string
    }
}

fn convert_word_to_chars(word: u32) -> String {
    let mut chars = "".to_string();
    for shift in (0..4).rev() {
        let byte = (word >> (shift * 8)) as u8;
        if byte > 32 && byte < 127 {
            chars.push(byte as char);
        } else {
            chars.push('.');
        }
    }
    chars
}
