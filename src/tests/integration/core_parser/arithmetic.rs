//! Tests for additional arithmetic instructions: addu.

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
