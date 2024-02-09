//! Tests for additional arithmetic instructions: addu, sll, move, nop.

use super::*;

#[test]
fn basic_addu() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from("addu r20, r19, r18");

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize_legacy(instruction_bits)?;

    datapath.registers.gpr[18] = 6849841;
    datapath.registers.gpr[19] = 99816512;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[20], 106666353); // 6849841 + 99816512

    Ok(())
}

#[test]
fn basic_sll() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from(
        r#"ori $s1, $zero, 8
sll $s1, $s1, 3"#,
    );

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize_legacy(instruction_bits)?;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[17], 64); // $s1

    Ok(())
}

#[test]
fn basic_move() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from(
        r#"ori $s4, $zero, 78
move $s5, $s4"#,
    );

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize_legacy(instruction_bits)?;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[21], 78); // $s5

    Ok(())
}

#[test]
fn basic_nop() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from(r#"nop"#);

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize_legacy(instruction_bits)?;

    let mut expected_registers = datapath.registers;
    expected_registers.pc = 4;
    let expected_memory = datapath.memory.clone();

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    // Register and memory contents should be unchanged, except for the PC.
    assert_eq!(datapath.registers, expected_registers);
    assert_eq!(datapath.memory, expected_memory);

    Ok(())
}
