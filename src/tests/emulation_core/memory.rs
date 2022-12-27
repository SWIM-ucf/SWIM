use crate::emulation_core::mips::memory::{Memory, CAPACITY_BYTES};

// Attempt to read at an address not byte-aligned.
#[test]
fn read_non_aligned_address() {
    let memory = Memory::default();

    let address: u64 = 1;

    assert!(match memory.load_word(address) {
        Err(e) => e.contains("align"),
        _ => false,
    });
}

// Attempt to read at an address larger than the available amount of space.
#[test]
fn read_out_of_bounds_address() {
    let memory = Memory::default();

    // This test assumes that `CAPACITY_BYTES + 500` does not overflow.
    let address = (CAPACITY_BYTES as u64) + 500;

    assert!(match memory.load_word(address) {
        Err(e) => e.contains("bounds"),
        _ => false,
    });
}

// Attempt to write at an address not byte-aligned.
#[test]
fn write_non_aligned_address() {
    let mut memory = Memory::default();

    let address: u64 = 1;

    assert!(match memory.store_word(address, 0) {
        Err(e) => e.contains("align"),
        _ => false,
    });
}

// Attempt to write at an address larger than the available amount of space.
#[test]
fn write_out_of_bounds_address() {
    let mut memory = Memory::default();

    // This test assumes that `CAPACITY_BYTES + 500` does not overflow.
    let address = (CAPACITY_BYTES as u64) + 500;

    assert!(match memory.store_word(address, 0) {
        Err(e) => e.contains("bounds"),
        _ => false,
    });
}

#[test]
fn store_and_load_word() {
    let mut memory = Memory::default();

    let address = 0;
    let data = 500;
    memory.store_word(address, data).ok();

    assert!(match memory.load_word(address) {
        Ok(loaded_data) => loaded_data == data,
        _ => false,
    });
}

// Stores a 64-bit doubleword in memory and attempts to read it back
// with the same value.
#[test]
fn store_and_load_double_word() -> Result<(), String> {
    let mut memory = Memory::default();

    let address = 0;
    // Large number that fits in 64 bits.
    let data: u64 = 2_791_928_321_507;
    memory.store_double_word(address, data)?;

    let loaded_data = memory.load_double_word(address)?;

    if loaded_data == data {
        Ok(())
    } else {
        Err(String::from(
            "Loaded data from memory did not match expected data",
        ))
    }
}
