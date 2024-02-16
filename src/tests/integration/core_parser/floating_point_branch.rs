//! Tests for the floating-point branch instructions: bc1t, bc1f

use super::*;

#[test]
fn bc1t_branch() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    // Load the single-precision floating-point value 1.0 into $s0.
    // Load the single-precision floating-point value 5.0 into $s1.
    // Load bitwise zero to $s2.
    // Move bitwise zero to $f0.
    // Move $s0 (1.0) to $f1.
    // Move $s1 (5.0) to $f2.
    // Add $f1 (1.0) to $f0.
    // Add 7 to $s2.
    // Repeat the previous two instructions if $f0 is less than $f2 (5.0).
    let instructions = String::from(
        r#"lui $s0, 0x3F80
lui $s1, 0x40A0
ori $s2, $zero, 0
mtc1 $zero, $f0
mtc1 $s0, $f1
mtc1 $s1, $f2

loop:
add.s $f0, $f0, $f1
addiu $s2, $s2, 7
c.lt.s $f0, $f2
bc1t loop"#,
    );

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize_legacy(instruction_bits)?;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    // This should end when the loop has iterated 5 times.
    // Thus, $s2 should be 35 and $f0 should be 5.0.
    assert_eq!(datapath.registers.gpr[18], 35); // $s2
    assert_eq!(f32::from_bits(datapath.coprocessor.fpr[0] as u32), 5.0); // $f0

    Ok(())
}

#[test]
fn bc1f_branch() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    // Load the single-precision floating-point value 1.0 into $s0.
    // Load the single-precision floating-point value 5.0 into $s1.
    // Load bitwise zero to $s2.
    // Move bitwise zero to $f0.
    // Move $s0 (1.0) to $f1.
    // Move $s1 (5.0) to $f2.
    // Add $f1 (1.0) to $f0.
    // Add 7 to $s2.
    // Repeat the previous two instructions if $f2 (5.0) is not less than $f0.
    let instructions = String::from(
        r#"lui $s0, 0x3F80
lui $s1, 0x40A0
ori $s2, $zero, 0
mtc1 $zero, $f0
mtc1 $s0, $f1
mtc1 $s1, $f2

loop:
add.s $f0, $f0, $f1
addiu $s2, $s2, 7
c.lt.s $f2, $f0
bc1f loop"#,
    );

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize_legacy(instruction_bits)?;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    // This should end when the loop has iterated 6 times.
    // Thus, $s2 should be 42 and $f0 should be 6.0.
    assert_eq!(datapath.registers.gpr[18], 42); // $s2
    assert_eq!(f32::from_bits(datapath.coprocessor.fpr[0] as u32), 6.0); // $f0

    Ok(())
}
