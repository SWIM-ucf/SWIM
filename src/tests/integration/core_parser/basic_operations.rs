//! Covering the basic arithmetic instructions: add, sub, mul, div, or, and.

use super::*;

#[test]
fn basic_add() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from("add r11, r7, r8");

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize(instruction_bits)?;

    datapath.registers.gpr[7] = 51;
    datapath.registers.gpr[8] = 5;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[11], 56); // 51 + 5

    Ok(())
}

#[test]
fn basic_sub() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from("sub r12, r7, r8");

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize(instruction_bits)?;

    datapath.registers.gpr[7] = 51;
    datapath.registers.gpr[8] = 5;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[12], 46); // 51 - 5

    Ok(())
}

#[test]
fn basic_mul() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from("mul r13, r7, r8");

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize(instruction_bits)?;

    datapath.registers.gpr[7] = 51;
    datapath.registers.gpr[8] = 5;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[13], 255); // 51 * 5

    Ok(())
}

#[test]
fn basic_div() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from("div r14, r7, r8");

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize(instruction_bits)?;

    datapath.registers.gpr[7] = 51;
    datapath.registers.gpr[8] = 5;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[14], 255); // 51 / 5 (10.2 truncated to 10)

    Ok(())
}

#[test]
fn basic_or() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from("or r15, r7, r8");

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize(instruction_bits)?;

    datapath.registers.gpr[7] = 51;
    datapath.registers.gpr[8] = 5;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[15], 55); // 51 | 5 (0011 0011 | 0000 0101)

    Ok(())
}

#[test]
fn basic_and() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from("and r16, r7, r8");

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize(instruction_bits)?;

    datapath.registers.gpr[7] = 51;
    datapath.registers.gpr[8] = 5;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[16], 1); // 51 & 5 (0011 0011 & 0000 0101)

    Ok(())
}
