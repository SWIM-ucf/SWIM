//! Tests for the branch and jump instructions: j, jr, jal, jalr, beq, bne

use super::*;

#[test]
fn basic_jump() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    // This program infinitely loops, increasing register $s1 by multiples of
    // 12345 on each iteration. While this program will never stop, the goal
    // of this is to simply demonstrate expected operation of the jump
    // instruction. This test goes through 5 iterations.
    let instructions = String::from(
        r#"ori $s0, $zero, 12345
loop: daddu $s1, $s1, $s0
j loop"#,
    );

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize_legacy(instruction_bits)?;

    // Execute the ori instruction.
    datapath.execute_instruction();

    for i in 1..=5 {
        // Execute the daddu and the j instructions.
        datapath.execute_instruction();
        datapath.execute_instruction();

        // The PC should be back at the daddu instruction.
        assert_eq!(datapath.registers.pc, 4);
        assert_eq!(datapath.registers.gpr[17], 12345 * i); // $s1
    }

    Ok(())
}

#[test]
fn basic_jr() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from(
        r#"ori r15, r0, 1200
jr r15"#,
    );

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize_legacy(instruction_bits)?;

    // Execute 2 instructions.
    for _ in 0..2 {
        datapath.execute_instruction();
    }

    // The PC should be at 1200.
    assert_eq!(datapath.registers.pc, 1200);

    Ok(())
}

#[test]
fn basic_jal() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from(
        r#"ori $t0, $zero, 12345
jal function
syscall
function: ori $t0, $zero, 5831"#,
    );

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize_legacy(instruction_bits)?;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[8], 5831); // $t0

    // The return address should be the instruction after the jal.
    assert_eq!(datapath.registers.gpr[31], 8); // $ra

    Ok(())
}

#[test]
fn basic_jalr() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    let instructions = String::from(
        r#"ori $s0, $zero, 12
jalr $s5, $s0
or $zero, $zero, $zero
function: ori $t1, $zero, 9548"#,
    );

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize_legacy(instruction_bits)?;

    // Execute 3 instructions.
    for _ in 0..3 {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[9], 9548); // $t1

    // The return address should be the instruction after the jalr.
    assert_eq!(datapath.registers.gpr[21], 8); // $ra

    // The PC should be right after the last instruction.
    assert_eq!(datapath.registers.pc, 16);

    Ok(())
}

#[test]
fn basic_b() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    // This program does the following if this was in C:
    //
    // int x = 5;
    // int y = 5;
    // int z = 40;
    //
    // goto change10;
    // z += 20;
    //
    // change10:
    // z += 10;
    //
    // $s0 contains x.
    // $s1 contains y.
    // $s2 contains z.
    //
    // This test determines if at the conclusion of this program, if z == 50.
    let instructions = String::from(
        r#"ori $s0, $zero, 5
ori $s1, $zero, 5
ori $s2, $zero, 40
b change10
daddiu $s2, $s2, 20
change10: daddiu $s2, $s2, 10"#,
    );

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize_legacy(instruction_bits)?;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[18], 50); // $s2

    Ok(())
}

#[test]
fn basic_beq() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    // This program does the following if this was in C:
    //
    // int x = 5;
    // int y = 5;
    // int z = 40;
    //
    // if (x == y)
    // {
    //     z += 20;
    // }
    //
    // $s0 contains x.
    // $s1 contains y.
    // $s2 contains z.
    //
    // This test determines if at the conclusion of this program, if z == 60.
    let instructions = String::from(
        r#"ori $s0, $zero, 5
ori $s1, $zero, 5
ori $s2, $zero, 40
beq $s0, $s1, changez
syscall
changez: daddiu $s2, $s2, 20"#,
    );

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize_legacy(instruction_bits)?;

    while !datapath.is_halted() {
        datapath.execute_instruction();
    }

    assert_eq!(datapath.registers.gpr[18], 60); // $s2

    Ok(())
}

#[test]
fn basic_bne() -> Result<(), String> {
    let mut datapath = MipsDatapath::default();

    // This program loops 5 times, increasing register $s1 by multiples of
    // 12345 on each iteration.
    // $s0 stores the iteration count.
    // $s1 stores the value 12345.
    // $s2 stores the value 5 (for ending the loop).
    // $s3 stores the value being incremented.
    let instructions = String::from(
        r#"or $s0, $zero, $zero
ori $s1, $zero, 12345
ori $s2, $zero, 5
or $s3, $zero, $zero
loop: daddu $s3, $s3, $s1
daddiu $s0, $s0, 1
bne $s0, $s2, loop"#,
    );

    let (_, instruction_bits) = parser(instructions);
    datapath.initialize_legacy(instruction_bits)?;

    let mut iterations = 0;

    while !datapath.is_halted() {
        datapath.execute_instruction();
        iterations += 1;

        // Catch an infinite loop. This program should not cause over 300 instructions to run.
        if iterations > 300 {
            return Err(String::from(
                "Infinite loop detected: {iterations} instructions executed.",
            ));
        }
    }

    assert_eq!(datapath.registers.gpr[19], 61725); // $s3 == 12345 * 5

    Ok(())
}
