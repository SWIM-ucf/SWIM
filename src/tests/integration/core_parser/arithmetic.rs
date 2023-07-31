//! Tests for additional arithmetic instructions: addu, move.

use super::*;

#[test]
fn basic_addu() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from("addu r20, r19, r18");

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize(instruction_bits)?;

    datapath.registers.gpr[18] = 6849841;
    datapath.registers.gpr[19] = 99816512;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[20], 106666353); // 6849841 + 99816512

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
    datapath.initialize(instruction_bits)?;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[21], 78); // $s5

    Ok(())
}
