//! Covering the basic immediate arithmetic instructions: addi, subi, muli, divi, ori, andi, li, lui, aui.
//!
//! Note that some of these instructions are pseudo-instructions.

use crate::emulation_core::architectures::AvailableDatapaths;

use super::*;

#[test]
fn basic_addi() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from("addi r11, r15, 2");

    let (_, instruction_bits, _labels) = parser(instructions, AvailableDatapaths::MIPS);
    datapath.initialize_legacy(instruction_bits)?;

    datapath.registers.gpr[15] = 100;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[11], 102); // 100 + 2

    Ok(())
}

#[test]
fn basic_addiu() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from("addiu r14, r17, 5");

    let (_, instruction_bits, _labels) = parser(instructions, AvailableDatapaths::MIPS);
    datapath.initialize_legacy(instruction_bits)?;

    datapath.registers.gpr[17] = 500;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[14], 505); // 500 + 5

    Ok(())
}

#[test]
fn basic_subi() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from("subi r11, r15, 2");

    let (_, instruction_bits, _labels) = parser(instructions, AvailableDatapaths::MIPS);
    datapath.initialize_legacy(instruction_bits)?;

    datapath.registers.gpr[15] = 100;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[11], 98); // 100 - 2

    Ok(())
}

#[test]
fn basic_muli() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from("muli r11, r15, 2");

    let (_, instruction_bits, _labels) = parser(instructions, AvailableDatapaths::MIPS);
    datapath.initialize_legacy(instruction_bits)?;

    datapath.registers.gpr[15] = 100;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[11], 200); // 100 * 2

    Ok(())
}

#[test]
fn basic_divi() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from("divi r11, r15, 2");

    let (_, instruction_bits, _labels) = parser(instructions, AvailableDatapaths::MIPS);
    datapath.initialize_legacy(instruction_bits)?;

    datapath.registers.gpr[15] = 100;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[11], 50); // 100 / 2

    Ok(())
}

#[test]
fn basic_ori() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from("ori r11, r15, 2");

    let (_, instruction_bits, _labels) = parser(instructions, AvailableDatapaths::MIPS);
    datapath.initialize_legacy(instruction_bits)?;

    datapath.registers.gpr[15] = 100;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[11], 102); // 100 | 2 (0110 0100 | 0000 0010)

    Ok(())
}

#[test]
fn basic_andi() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from("andi r11, r15, 4");

    let (_, instruction_bits, _labels) = parser(instructions, AvailableDatapaths::MIPS);
    datapath.initialize_legacy(instruction_bits)?;

    datapath.registers.gpr[15] = 100;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[11], 4); // 100 & 4 (0110 0100 & 0000 0100)

    Ok(())
}

#[test]
fn basic_li() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from("li r15, 56");

    let (_, instruction_bits, _labels) = parser(instructions, AvailableDatapaths::MIPS);
    datapath.initialize_legacy(instruction_bits)?;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[15], 56);

    Ok(())
}

#[test]
fn basic_lui() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    // 65530 == 0xFFFA
    let instructions = String::from("lui r20, 65530");

    let (_, instruction_bits, _labels) = parser(instructions, AvailableDatapaths::MIPS);
    datapath.initialize_legacy(instruction_bits)?;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    // The value 0xFFFA should be sign-extended to the remaining 32 bits of the register.
    assert_eq!(datapath.registers.gpr[20], 18446744073709158400); // 129 << 16 (0xFFFF FFFF FFFA 0000)

    Ok(())
}

#[test]
fn basic_aui() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    // 4612 == 0x1204
    let instructions = String::from("aui r15, r18, 4612");

    let (_, instruction_bits, _labels) = parser(instructions, AvailableDatapaths::MIPS);
    datapath.initialize_legacy(instruction_bits)?;

    datapath.registers.gpr[18] = 0x0000_0000_0030_ABCD;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    //   0x0000 0000 0030 ABCD  (r18)
    // + 0x0000 0000 1204 0000  (immediate << 16)
    // ========================
    //   0x0000 0000 1234 ABCD

    assert_eq!(datapath.registers.gpr[15], 0x0000_0000_1234_ABCD);

    Ok(())
}
