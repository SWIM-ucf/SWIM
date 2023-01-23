use crate::emulation_core::datapath::Datapath;
use crate::emulation_core::mips::datapath::MipsDatapath;
use crate::parser::parser_main::parser;

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
    datapath.load_instructions(instruction_bits)?;

    // Execute 2 instructions.
    for _ in 0..2 {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[16], 5); // $s0
    assert_eq!(datapath.registers.gpr[17], 10); // $s1

    Ok(())
}
