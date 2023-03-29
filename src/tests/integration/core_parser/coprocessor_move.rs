//! Tests for the "move from/to Coprocessor 1" instructions: mtc1, dmtc1, mfc1, dmfc1

use super::*;

#[test]
fn basic_mtc1() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from("mtc1 $t2, $f5");
    let (_, instruction_bits) = parser(instructions);
    datapath.initialize(instruction_bits)?;

    datapath.registers.gpr[10] = 658461658; // $t2

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.coprocessor.fpr[5], 658461658);
    Ok(())
}

#[test]
fn truncate_32_bit_mtc1() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from("mtc1 $t3, $f6");
    let (_, instruction_bits) = parser(instructions);
    datapath.initialize(instruction_bits)?;

    datapath.registers.gpr[11] = 0x0000_02F2_AC71_AC41; // $t3

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.coprocessor.fpr[6], 0xAC71_AC41);
    Ok(())
}

#[test]
fn basic_mfc1() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from("mfc1 $t3, $f5");
    let (_, instruction_bits) = parser(instructions);
    datapath.initialize(instruction_bits)?;

    datapath.coprocessor.fpr[5] = 657861659;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[11], 657861659); // $t3
    Ok(())
}

#[test]
fn truncate_32_bit_mfc1() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from("mfc1 $t4, $f6");
    let (_, instruction_bits) = parser(instructions);
    datapath.initialize(instruction_bits)?;

    datapath.coprocessor.fpr[6] = 0x0003_7F80_E5E7_D785;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[12], 0xFFFF_FFFF_E5E7_D785); // $t4
    Ok(())
}

#[test]
fn basic_dmtc1() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from("dmtc1 $t3, $f6");
    let (_, instruction_bits) = parser(instructions);
    datapath.initialize(instruction_bits)?;

    datapath.registers.gpr[11] = 0x0120_02F2_AC71_AC41; // $t3

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.coprocessor.fpr[6], 0x0120_02F2_AC71_AC41);
    Ok(())
}

#[test]
fn basic_dmfc1() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from("dmfc1 $t4, $f6");
    let (_, instruction_bits) = parser(instructions);
    datapath.initialize(instruction_bits)?;

    datapath.coprocessor.fpr[6] = 0x0003_7F90_E5E7_D785;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[12], 0x0003_7F90_E5E7_D785); // $t4
    Ok(())
}
