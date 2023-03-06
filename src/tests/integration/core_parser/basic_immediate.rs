//! Covering the basic immediate arithmetic instructions: addi, subi, muli, divi, ori, andi.
//!
//! Note that some of these instructions are pseudo-instructions.

use super::*;

#[test]
fn basic_addi() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from("addi r11, r15, 2");

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize(instruction_bits)?;

    datapath.registers.gpr[15] = 100;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[11], 102); // 100 + 2

    Ok(())
}

#[test]
fn basic_subi() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from("subi r11, r15, 2");

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize(instruction_bits)?;

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

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize(instruction_bits)?;

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

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize(instruction_bits)?;

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

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize(instruction_bits)?;

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

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize(instruction_bits)?;

    datapath.registers.gpr[15] = 100;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[11], 4); // 100 & 4 (0110 0100 & 0000 0100)

    Ok(())
}
