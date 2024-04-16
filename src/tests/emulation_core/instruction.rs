use std::collections::HashMap;

use crate::emulation_core::mips::instruction::MipsInstruction;

// ** R-TYPE INSTRUCTIONS ** //
#[test]
fn get_string_version_from_r_type() {
    // R-type instructions:
    // add, sub, mul, div
    // addu (TODO)
    // (TODO) dadd, dsub, dmul, ddiv
    // (TODO) daddu, dsubu, dmulu, ddivu
    // or, and, sll
    // slt, sltu (TODO)
    // jalr, jr
    let instruction: u32 = 0b00000001111110000111000000100000;
    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("add $t6, $t7, $t8")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b00000011110010011110100000100010;

    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("sub $sp, $fp, $t1")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b00000001100010111101000010011000;

    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("mul $k0, $t4, $t3")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b00000000101001100010000010011010;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("div $a0, $a1, $a2")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b00000011111000000000000000001001;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("jr $ra")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b00000000010100010001000000101010;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("slt $v0, $v0, $s1")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b00000000000010000100101010000000;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("sll $t1, $t0, 10")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b00000000000111011111000000100101;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("or $fp, $zero, $sp")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b00000000000000001000000000100100;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("and $s0, $zero, $zero")
            }
            _ => false,
        }
    );
}

// ** I-TYPE INSTRUCTIONS ** //
#[test]
fn get_string_version_from_i_type() {
    // I-Type instructions:
    // addi, addiu, daddi, daddiu
    // lw, sw
    // lui
    // ori, andi (TODO)
    // regimm (TODO)
    // beq, bne (FIX)
    let instruction: u32 = 0b00100000000100010000000000000010;
    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("addi $s1, $zero, 2")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b00100111101111011111111111011000;

    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("addiu $sp, $sp, -40")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b01100011011011011111111111111101;

    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("daddi $t5, $k1, -3")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b01100110000100000000000000000001;

    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("daddiu $s0, $s0, 1")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b00010000010000000000000000000100;
    let mut labels: HashMap<String, usize> = HashMap::<String, usize>::new();
    labels.insert("loop".to_string(), 0x00000004);

    // TODO: Fix this test to include labels

    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("beq $v0, $zero, ")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b00010100010000000000000000000100;
    let mut labels: HashMap<String, usize> = HashMap::<String, usize>::new();
    labels.insert("loop".to_string(), 0x00000004);

    // TODO: Fix this test to include labels

    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("bne $v0, $zero, ")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b10101101110110010000000000000000;

    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("sw $t9, 0($t6)")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b10001111110000100000000000101000;

    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("lw $v0, 40($fp)")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b00111100000100000011111110000000;

    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("lui $s0, 0x3f80")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b00110100000011100000000111110100;

    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("ori $t6, $zero, 500")
            }
            _ => false,
        }
    );
}

// ** J-TYPE INSTRUCTIONS ** //
#[test]
fn get_string_version_from_j_type() {
    let mut labels: HashMap<String, usize> = HashMap::<String, usize>::new();
    labels.insert(String::from("fib(int)"), 0x0);
    let instruction: u32 = 0b00001100000000000000000000000000;

    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("jal fib(int)")
            }
            _ => false,
        }
    );
}

// ** SYSCALL-TYPE INSTRUCTIONS ** //
#[test]
fn get_string_version_from_syscall_type() {
    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();
    let instruction: u32 = 0b00000000000000000000000000001100;

    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("syscall")
            }
            _ => false,
        }
    );
}
