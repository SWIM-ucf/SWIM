#![allow(clippy::unusual_byte_groupings)]

use crate::emulation_core::datapath::Datapath;
use crate::emulation_core::mips::datapath::MipsDatapath;
use crate::emulation_core::mips::registers::RegisterType;

#[test]
fn add_register_to_itself() {
    let mut datapath = MipsDatapath::default();

    // $t1 = $t1 + $t1
    //                       R-type  t1    t1    t1  (shamt)  ADD
    let instruction: u32 = 0b000000_01001_01001_01001_00000_100000;
    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    // Assume the register $t1 has the value 5.
    datapath.registers[RegisterType::T1] = 5;

    datapath.execute_instruction();

    // After the operation is finished, the register should be 10.
    assert_eq!(datapath.registers[RegisterType::T1], 10);
}

#[test]
fn add_register_to_another() {
    let mut datapath = MipsDatapath::default();

    // $s2 = $s0 + $s1
    //                       R-type  s0    s1    s2  (shamt)  ADD
    let instruction: u32 = 0b000000_10000_10001_10010_00000_100000;
    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    datapath.registers.gpr[16] = 15; // $s0
    datapath.registers.gpr[17] = 40; // $s1

    datapath.execute_instruction();

    // Register $s2 should contain 55.
    let result = datapath.registers.gpr[18] as u32;
    assert_eq!(result, 55);
}

#[test]
// This test attempts to write to register $zero. The datapath should
// not overwrite this register, and remain with a value of 0.
fn add_to_register_zero() {
    let mut datapath = MipsDatapath::default();

    // $zero = $t3 + $t3
    //                       R-type  t3    t3    zero (shamt) ADD
    let instruction: u32 = 0b000000_01011_01011_00000_00000_100000;
    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    datapath.registers.gpr[11] = 1234; // $t3

    datapath.execute_instruction();

    // $zero should still contain 0.
    assert_eq!(datapath.registers.gpr[0], 0);
}

#[test]
// NOTE: This test falls under our initial project design that there are no
// handled exceptions. Therefore, we would expect to see an updated value in
// register T1, rather than having the register unmodified per the MIPS64v6
// specification.
fn add_32_bit_with_overflow() {
    let mut datapath = MipsDatapath::default();

    // $t1 = $t4 + $t4
    //                       R-type  t4    t4    t1 (shamt) ADD
    let instruction: u32 = 0b000000_01100_01100_01001_00000_100000;
    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    // Assume register $t4 contains 2,454,267,026, a 32-bit integer.
    datapath.registers.gpr[12] = 0b10010010_01001001_00100100_10010010;

    datapath.execute_instruction();

    // Disregarding overflow, register $t4 would contain 4,908,534,052, or
    // 1_00100100_10010010_01001001_00100100 in binary. The result
    // should be truncated. Thus, we should expect the register to
    // contain 613,566,756, or 00100100_10010010_01001001_00100100 in binary.
    assert_eq!(datapath.registers.gpr[9], 613566756);
}

#[test]
// NOTE: This test falls under our initial project design that there are no
// handled exceptions. Therefore, we would expect to see an updated value in
// register T1, rather than having the register unmodified per the MIPS64v6
// specification.
fn add_32_bit_with_overflow_sign_extend() {
    let mut datapath = MipsDatapath::default();

    // $t1 = $t4 + $t4
    //                       R-type  t4    t4    t1 (shamt) ADD
    let instruction: u32 = 0b000000_01100_01100_01001_00000_100000;
    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    // Assume register $t4 contains 3,528,008,850, a 32-bit integer.
    datapath.registers.gpr[12] = 0b11010010_01001001_00100100_10010010;

    datapath.execute_instruction();

    // Disregarding overflow, register $t4 would contain 7,056,017,700, or
    // 1_10100100_10010010_01001001_00100100 in binary. The result
    // should be truncated. Thus, we should expect the register to
    // contain 2,761,050,404, or 10100100_10010010_01001001_00100100 in binary.
    assert_eq!(datapath.registers.gpr[9] as u32, 2761050404);
}

#[test]
fn sub_positive_result() {
    let mut datapath = MipsDatapath::default();

    // $s2 = $s3 - $s2
    //                       R-type  s3    s2    s2  (shamt) SUB
    let instruction: u32 = 0b000000_10011_10010_10010_00000_100010;
    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    datapath.registers.gpr[19] = 7; // $s3
    datapath.registers.gpr[18] = 3; // $s2

    datapath.execute_instruction();

    // Register $s2 should contain 4, as 7 - 3 = 4.
    assert_eq!(datapath.registers.gpr[18], 4);
}

#[test]
fn sub_32_bit_negative_result() {
    let mut datapath = MipsDatapath::default();

    // $s0 = $s0 - $t0
    //                       R-type  s0    t0    s0  (shamt) SUB
    let instruction: u32 = 0b000000_10000_01000_10000_00000_100010;
    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    datapath.registers.gpr[16] = 5; // $s0
    datapath.registers.gpr[8] = 20; // $t0

    datapath.execute_instruction();

    // Register $s0 should contain -15, as 5 - 20 = -15.
    assert_eq!(datapath.registers.gpr[16] as i32, -15);
}

#[test]
fn sub_32_bit_underflow() {
    let mut datapath = MipsDatapath::default();

    // $s0 = $s0 - $t0
    //                       R-type  s0    t0    s0  (shamt) SUB
    let instruction: u32 = 0b000000_10000_01000_10000_00000_100010;
    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    datapath.registers.gpr[16] = 0; // $s0
    datapath.registers.gpr[8] = 1; // $t0

    datapath.execute_instruction();

    // Register $s0 should contain the largest unsigned 32-bit integer due to underflow.
    assert_eq!(
        datapath.registers.gpr[16] as u32,
        0b11111111_11111111_11111111_11111111
    );
}

#[test]
fn mul_positive_result() {
    let mut datapath = MipsDatapath::default();

    // $s5 = $t7 * $t6
    //                       R-type  t7    t6    s5    MUL   SOP30
    let instruction: u32 = 0b000000_01111_01110_10101_00010_011000;
    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    datapath.registers.gpr[15] = 8; // $t7
    datapath.registers.gpr[14] = 95; // $t6

    datapath.execute_instruction();

    assert_eq!(datapath.registers.gpr[21], 760); // $s5
}

#[test]
fn mul_32_bit_negative_result() {
    let mut datapath = MipsDatapath::default();

    // $s5 = $t7 * $t6
    //                       R-type  t7    t6    s5    MUL   SOP30
    let instruction: u32 = 0b000000_01111_01110_10101_00010_011000;
    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    datapath.registers.gpr[15] = 5; // $t7
    datapath.registers.gpr[14] = -5_i64 as u64; // $t6

    datapath.execute_instruction();

    assert_eq!(datapath.registers.gpr[21] as i64, -25); // $s5
}

#[test]
fn mul_result_truncate() {
    let mut datapath = MipsDatapath::default();

    // $s4 = $t6 * $t5
    //                       R-type  t6    t5    s4    MUL   SOP30
    let instruction: u32 = 0b000000_01110_01101_10100_00010_011000;
    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    datapath.registers.gpr[14] = 731_564_544; // $t6
    datapath.registers.gpr[13] = 8; // $t5

    datapath.execute_instruction();

    // The result, 5,852,516,352, is too large for a 32-bit integer.
    // (1 01011100 11010110 01010000 00000000)
    // The result should instead truncate to the lower 32 bits.
    assert_eq!(
        datapath.registers.gpr[20],
        0b01011100_11010110_01010000_00000000
    ); // $s5
}

#[test]
fn div_positive_result() {
    let mut datapath = MipsDatapath::default();

    // $s4 = $t6 / $t5
    //                       R-type  t6    t5    s4    DIV   SOP32
    let instruction: u32 = 0b000000_01110_01101_10100_00010_011010;
    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    datapath.registers.gpr[14] = 20; // $t6
    datapath.registers.gpr[13] = 2; // $t5

    datapath.execute_instruction();

    assert_eq!(datapath.registers.gpr[20], 10); // $s5
}

#[test]
fn div_negative_result() {
    let mut datapath = MipsDatapath::default();

    // $s4 = $t6 / $t5
    //                       R-type  t6    t5    s4    DIV   SOP32
    let instruction: u32 = 0b000000_01110_01101_10100_00010_011010;
    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    datapath.registers.gpr[14] = 20; // $t6
    datapath.registers.gpr[13] = -5_i64 as u64; // $t5

    datapath.execute_instruction();

    assert_eq!(datapath.registers.gpr[20] as i64, -4); // $s5
}
