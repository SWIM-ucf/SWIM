use std::collections::HashMap;

use crate::emulation_core::mips::instruction::MipsInstruction;

// ** R-TYPE INSTRUCTIONS ** //
#[test]
fn get_string_version_from_r_type() {
    // R-type instructions:
    // add, sub, mul, div
    // addu
    // dadd, dsub, dmul, ddiv
    // daddu, dsubu, dmulu, ddivu
    // or, and, sll
    // slt, sltu
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

    let instruction: u32 = 0b00000010000100010100100000100001;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("addu $t1, $s0, $s1")
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
    let instruction: u32 = 0b00000010000100010100100000101100;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("dadd $t1, $s0, $s1")
            }
            _ => false,
        }
    );
    let instruction: u32 = 0b00000010000100010100100000101101;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("daddu $t1, $s0, $s1")
            }
            _ => false,
        }
    );
    let instruction: u32 = 0b00000010000100010100100000101110;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("dsub $t1, $s0, $s1")
            }
            _ => false,
        }
    );
    let instruction: u32 = 0b00000010000100010100100000101111;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("dsubu $t1, $s0, $s1")
            }
            _ => false,
        }
    );
    let instruction: u32 = 0b00000001001010100100000010011100;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("dmul $t0, $t1, $t2")
            }
            _ => false,
        }
    );
    let instruction: u32 = 0b00000001001010100100000010011101;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("dmulu $t0, $t1, $t2")
            }
            _ => false,
        }
    );
    let instruction: u32 = 0b00000001001010100100000010011110;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("ddiv $t0, $t1, $t2")
            }
            _ => false,
        }
    );
    let instruction: u32 = 0b00000001001010100100000010011111;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("ddivu $t0, $t1, $t2")
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
    let instruction: u32 = 0b00000010000100010100100000101011;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("sltu $t1, $s0, $s1")
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
    // ori, andi
    // dahi, dati
    // beq, bne (FIX)
    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    let instruction: u32 = 0b00100000000100010000000000000010;
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

    let instruction: u32 = 0b00110001010010010000000011111111;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("andi $t1, $t2, 255")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b00000101001001100000000000001111;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("dahi $t1, 15")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b00000101001111100000000000001111;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("dati $t1, 15")
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

// ** FLOATING POINT INSTRUCTIONS ** //

// ** FPU R-Type ** //
// add.fmt, sub.fmt, mul.fmt, div.fmt
#[test]
fn get_string_version_from_fpu_r_type() {
    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();
    let instruction: u32 = 0b01000110001001100010000010000000;

    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("add.d $f2, $f4, $f6")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b01000110000001100010000010000000;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("add.s $f2, $f4, $f6")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b01000110001001100010000010000001;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("sub.d $f2, $f4, $f6")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b01000110000001100010000010000001;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("sub.s $f2, $f4, $f6")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b01000110001001100010000010000010;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("mul.d $f2, $f4, $f6")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b01000110000001100010000010000010;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("mul.s $f2, $f4, $f6")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b01000110001001100010000010000011;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("div.d $f2, $f4, $f6")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b01000110000001100010000010000011;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("div.s $f2, $f4, $f6")
            }
            _ => false,
        }
    );
}

// ** FPU I-Type ** //
// lwc1, swc1
#[test]
fn get_string_version_from_fpu_i_type() {
    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();
    let instruction: u32 = 0b11000101001000100000000000000000;

    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("lwc1 $f2, 0($t1)")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b11100101001000100000000000000000;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("swc1 $f2, 0($t1)")
            }
            _ => false,
        }
    );
}

// ** FPU Register-Immediate Type ** //
// mtc1, dmtc1, mfc1, dmfc1
#[test]
fn get_string_version_from_fpu_register_immediate_type() {
    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();
    let instruction: u32 = 0b01000100100010010001000000000000;

    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("mtc1 $t1, $f2")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b01000100000010010001000000000000;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("mfc1 $t1, $f2")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b01000100101010010001000000000000;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("dmtc1 $t1, $f2")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b01000100001010010001000000000000;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("dmfc1 $t1, $f2")
            }
            _ => false,
        }
    );
}

// ** FPU Branch Type ** //
// bc1t, bc1f
#[test]
fn get_string_version_from_fpu_branch_type() {
    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();
    let instruction: u32 = 0b01000101000000010000000000000001;

    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("bc1t 1")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b01000101000000000000000000000000;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("bc1f 0")
            }
            _ => false,
        }
    );
}

// ** FPU Compare Type ** //
// c.eq.fmt, c.lt.fmt, c.le.fmt, c.ngt.fmt, c.nge.fmt
#[test]
fn get_string_version_from_fpu_compare_type() {
    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();
    let instruction: u32 = 0b01000110001001000001000000110010;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("c.eq.d $f2, $f4")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b01000110000001000001000000110010;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("c.eq.s $f2, $f4")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b01000110001001000001000000111110;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("c.le.d $f2, $f4")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b01000110000001000001000000111110;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("c.le.s $f2, $f4")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b01000110001001000001000000111100;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("c.lt.d $f2, $f4")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b01000110000001000001000000111100;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("c.lt.s $f2, $f4")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b01000110001001000001000000111101;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("c.nge.d $f2, $f4")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b01000110000001000001000000111101;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("c.nge.s $f2, $f4")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b01000110001001000001000000111111;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("c.ngt.d $f2, $f4")
            }
            _ => false,
        }
    );

    let instruction: u32 = 0b01000110000001000001000000111111;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("c.ngt.s $f2, $f4")
            }
            _ => false,
        }
    );
}

// ** Test empty instruction ** //
#[test]
fn get_string_version_from_empty_instruction() {
    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();
    let instruction: u32 = 0b00000000000000000000000000000000;
    assert!(
        match MipsInstruction::get_string_version(instruction, labels.clone(), 0) {
            Ok(string) => {
                string.contains("nop")
            }
            _ => false,
        }
    );
}
