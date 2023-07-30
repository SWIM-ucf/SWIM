use akin::akin;

use crate::emulation_core::datapath::Datapath;
use crate::emulation_core::mips::datapath::MipsDatapath;
use crate::parser::parser_assembler_main::parser;

pub mod arithmetic;
pub mod basic_immediate;
pub mod basic_operations;
pub mod branch_jump;
pub mod conditions;
pub mod coprocessor_move;
pub mod double_arithmetic;
pub mod double_immediate;
pub mod fibonacci;
pub mod floating_point_arithmetic;
pub mod floating_point_branch;
pub mod floating_point_comparison;
pub mod store_load_word;

#[test]
fn add_register_plus_itself() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    // Sets register $s0 to 5, then adds $s0 and itself to get 10,
    // leaving the result in register $s1.
    let instructions = String::from(
        r#"ori $s0, $zero, 5
add $s1, $s0, $s0"#,
    );

    // Parse instructions and load into emulation core memory.
    let (_, instruction_bits) = parser(instructions);
    datapath.initialize(instruction_bits)?;

    // Execute 2 instructions.
    for _ in 0..2 {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[16], 5); // $s0
    assert_eq!(datapath.registers.gpr[17], 10); // $s1

    Ok(())
}

#[test]
// Loading a 64-bit constant with only a 16-bit immediate field.
// This involves the use of 4 separate instructions to put each
// piece of the value into a register.
fn load_64_bit_constant() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    // The goal is to load the value 0xABCD 8765 CCCC EEEE
    // into register R1.
    let instructions = String::from(
        r#"lui r1, 52428
ori r1, r1, 61166
dahi r1, 34662
dati r1, 43982"#,
    );

    // Sample tracing:
    //    Instruction    |     Register R1     |     Notes
    // ------------------+---------------------+-----------------------------
    // lui r1, 52428     | FFFF FFFF CCCC 0000 | 52428 == 0xCCCC. C == 1100, so the value is sign-extended.
    // ori r1, r1, 61166 | FFFF FFFF CCCC EEEE | 61166 == 0xEEEE.
    // dahi r1, 34662    | FFFF 8765 CCCC EEEE | 34662 == 0x8766. FFFF + 8766 = 8765.
    // dati r1, 43982    | ABCD 8765 CCCC EEEE | 43982 == 0xABCE. FFFF + ABCE = ABCD.

    // Parse instructions and load into emulation core memory.
    let (_, instruction_bits) = parser(instructions);
    datapath.initialize(instruction_bits)?;

    // Execute 4 instructions.
    for _ in 0..4 {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[1], 0xABCD_8765_CCCC_EEEE);

    Ok(())
}

#[test]
// Basic program that adds two numbers then multiplies that result by 2.
// The parser should add a `syscall` instruction at the end of the program
// and automatically halt.
fn syscall_to_stop() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from(
        r#"ori r5, $zero, 4321
ori r6, $zero, 5678
dadd r7, r5, r6
dmuli r8, r7, 2"#,
    );

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize(instruction_bits)?;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[5], 4321);
    assert_eq!(datapath.registers.gpr[6], 5678);
    assert_eq!(datapath.registers.gpr[7], 9999); // 4321 + 5678
    assert_eq!(datapath.registers.gpr[8], 19998); // 9999 * 2

    Ok(())
}
