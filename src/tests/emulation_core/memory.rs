use crate::emulation_core::mips::memory;

// Attempt to read at an address not byte-aligned.
#[test]
fn read_non_aligned_address() {
    let memory = memory::Memory::default();

    let address: usize = 1;
    let result = memory.load_word(address);

    assert!(match result {
        Err(e) => e.contains("align"),
        _ => false,
    });
}

// Attempt to read at an address larger than the available amount of space.
#[test]
fn read_out_of_bounds_address() {
    let memory = memory::Memory::default();

    // This test assumes that `CAPACITY_BYTES + 500` does not overflow.
    let address: usize = memory::CAPACITY_BYTES + 500;
    let result = memory.load_word(address);

    assert!(match result {
        Err(e) => e.contains("bounds"),
        _ => false,
    })
}

// Attempt to write at an address not byte-aligned.
#[test]
fn write_non_aligned_address() {
    let mut memory = memory::Memory::default();

    let address: usize = 1;
    let result = memory.store_word(address, 0);

    assert!(match result {
        Err(e) => e.contains("align"),
        _ => false,
    });
}

// Attempt to write at an address larger than the available amount of space.
#[test]
fn write_out_of_bounds_address() {
    let mut memory = memory::Memory::default();

    // This test assumes that `CAPACITY_BYTES + 500` does not overflow.
    let address: usize = memory::CAPACITY_BYTES + 500;
    let result = memory.store_word(address, 0);

    assert!(match result {
        Err(e) => e.contains("bounds"),
        _ => false,
    })
}
