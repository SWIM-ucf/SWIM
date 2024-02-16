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

    // // Returns instructions that were updated with their string versions and line numbers
    // pub fn store_hexdump(&mut self, instructions: Vec<u32>) -> Result<Vec<UpdatedLine>, String> {
    //     let mut changed_lines: Vec<UpdatedLine> = vec![];
    //     for (i, data) in instructions.iter().enumerate() {
    //         let address = i as u64;
    //         let line = match get_string_version(*data) {
    //             Ok(string) => string,
    //             Err(string) => string,
    //         };
    //         let curr_word = match self.load_word(address * 4) {
    //             Ok(data) => data,
    //             Err(e) => {
    //                 debug!("{:?}", e);
    //                 0
    //             }
    //         };
    //         if curr_word != *data {
    //             changed_lines.push(UpdatedLine::new(line, i));
    //             self.store_word(address * 4, *data)?
    //         }
    //     }

    //     Ok(changed_lines)
    // }

    pub fn generate_formatted_hex(&self) -> String {
        let iterator = MemoryIter::new(&self);

        let mut string: String = "".to_string();

        for (address, words) in iterator {
            string.push_str(&format!("0x{address:04x}:\t\t"));
            let mut char_version: String = "".to_string();

            for word in words {
                string.push_str(&format!("{:08x}\t", word));
                char_version.push_str(&Self::convert_word_to_chars(word));
            }

            string.push_str(&format!("{char_version}\n"));
        }

        string
    }

    pub fn convert_word_to_chars(word: u32) -> String {
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
}

pub struct MemoryIter<'a> {
    memory: &'a Memory,
    current_address: usize,
}

impl<'a> MemoryIter<'a> {
    pub fn new(memory: &'a Memory) -> MemoryIter<'a> {
        MemoryIter {
            memory,
            current_address: 0,
        }
    }
}

impl<'a> Iterator for MemoryIter<'a> {
    // Words are 32 bits
    type Item = (usize, Vec<u32>);
    fn next(&mut self) -> Option<Self::Item> {
        self.current_address = (self.current_address + 3) & !3;
        if self.current_address + 16 <= self.memory.memory.len() {
            let address = self.current_address;
            let words = (0..4)
                .map(|i| self.memory.load_word(address as u64 + (i * 4)).unwrap())
                .collect();

            self.current_address += 16;
            Some((address, words))
        } else {
            None
        }
    }
}
