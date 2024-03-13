//! Covering the store and load word instructions: sw, lw, swc1, lwc1.

use super::*;

#[test]
fn basic_sw() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from(
        r#"li r14, 500
li r25, 1234
sw r25, 0(r14)"#,
    );

    let (_, instruction_bits) = parser(instructions, AvailableDatapaths::MIPS);
    datapath.initialize_legacy(instruction_bits)?;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.memory.load_word(500).unwrap(), 1234);

    Ok(())
}

#[test]
fn basic_lw() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from(
        r#"li r14, 400
lw r25, 0(r14)"#,
    );

    let (_, instruction_bits) = parser(instructions, AvailableDatapaths::MIPS);
    datapath.initialize_legacy(instruction_bits)?;

    datapath.memory.memory[403] = 36;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[25], 36);

    Ok(())
}

#[test]
fn lw_sw_label() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from(
        r#".data
secret_number: .word 42

.text
lw $s1, secret_number
daddiu $s2, $s1, 1
sw $s2, secret_number"#,
    );

    let (_, instruction_bits) = parser(instructions, AvailableDatapaths::MIPS);
    datapath.initialize_legacy(instruction_bits)?;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    // .data contents are stored after the end of instructions. Thus, secret_number
    // will be at address 24, following the syscall instruction at address 20.

    assert_eq!(datapath.registers.gpr[17], 42); // $s1
    assert_eq!(datapath.memory.load_word(24).unwrap(), 43);

    Ok(())
}

#[test]
fn basic_swc1() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from(
        r#"li $s0, 500
li $s1, 1234
mtc1 $s1, $f25
swc1 $f25, 0($s0)"#,
    );

    let (_, instruction_bits) = parser(instructions, AvailableDatapaths::MIPS);
    datapath.initialize_legacy(instruction_bits)?;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.memory.load_word(500).unwrap(), 1234);

    Ok(())
}

#[test]
fn basic_lwc1() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from(
        r#"li $t4, 400
lwc1 $f12, 0($t4)"#,
    );

    let (_, instruction_bits) = parser(instructions, AvailableDatapaths::MIPS);
    datapath.initialize_legacy(instruction_bits)?;

    datapath.memory.memory[403] = 36;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.coprocessor.registers.fpr[12], 36);

    Ok(())
}
