#![allow(clippy::unusual_byte_groupings)]

use crate::emulation_core::datapath::Datapath;
use crate::emulation_core::mips::datapath::MipsDatapath;
use crate::emulation_core::mips::registers::GpRegisterType;

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
    datapath.registers[GpRegisterType::T1] = 5;

    datapath.execute_instruction();

    // After the operation is finished, the register should be 10.
    assert_eq!(datapath.registers[GpRegisterType::T1], 10);
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

#[test]
fn or_immediate_with_zero() {
    let mut datapath = MipsDatapath::default();

    // $s0 = $zero | 12345
    //                       ori    $zero  $s0   12345
    let instruction: u32 = 0b001101_00000_10000_0011000000111001;
    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    datapath.execute_instruction();

    assert_eq!(datapath.registers.gpr[16], 12345); // $s0
}

#[test]
fn or_immediate_with_value() {
    let mut datapath = MipsDatapath::default();

    // $s0 = $t0 | 12345
    //                       ori     $t0   $s0   12345
    let instruction: u32 = 0b001101_01000_10000_0011000000111001;
    datapath
        .memory
        .store_word(0, instruction)
        .expect("Failed to store instruction.");

    // In binary: 00111010 11011110 01101000 10110001
    datapath.registers.gpr[8] = 987654321; // $t0

    datapath.execute_instruction();

    // The result should be as follows:
    //         $t0:  00111010 11011110 01101000 10110001
    // OR   12,345:                    00110000 00111001
    // =================================================
    // 987,658,425:  00111010 11011110 01111000 10111001

    assert_eq!(datapath.registers.gpr[16], 987658425); // $s0
}

pub mod load_word {
    use super::*;
    #[test]
    fn lw_zero_offset_test() {
        // for this test the lw instruction will load itself from
        // memory
        let mut datapath = MipsDatapath::default();

        //                        lw     $t0   $s0      offset = 0
        let instruction: u32 = 0b100011_01000_10000_0000000000000000;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");
        datapath.execute_instruction();
        assert_eq!(datapath.registers.gpr[16], instruction as u64);
    }

    #[test]
    fn lw_offset_at_4_test() {
        // For this test the lw instruction will load 0x4 from memory
        // by using the offset address plus zero
        let mut datapath = MipsDatapath::default();

        //                        lw     $t0   $s0      offset = 4
        let instruction: u32 = 0b100011_01000_10000_0000000000000100;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        // place data at address
        datapath
            .memory
            .store_word(0b100, 0x10000)
            .expect("failed to store test data");

        datapath.registers.gpr[8] = 0;
        datapath.execute_instruction();
        assert_eq!(datapath.registers.gpr[16], 0x10000);
    }

    #[test]
    fn lw_gpr_8_at_4_offset_at_0_test() {
        // for this test the lw instruction will load 0x4 from memory
        // by using (offset = 0) + (gpr[8] = 4)
        let mut datapath = MipsDatapath::default();

        //                        lw     $t0   $s0      offset = 0
        let instruction: u32 = 0b100011_01000_10000_0000000000000000;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        // place data at address
        datapath
            .memory
            .store_word(0b100, 0x10000)
            .expect("failed to store test data");

        datapath.registers.gpr[8] = 4;
        datapath.execute_instruction();
        assert_eq!(datapath.registers.gpr[16], 0x10000);
    }

    #[test]
    fn lw_gpr_8_at_4_offset_at_4_test() {
        // for this test the lw instruction will load 0x8 from memory
        // by adding the offset to gpr[8]
        let mut datapath = MipsDatapath::default();
    
        //                        lw     $t0   $s0      offset = 0
        let instruction: u32 = 0b100011_01000_10000_0000000000000100;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");
    
        // place data at address
        datapath
            .memory
            .store_word(0b1000, 0x10000)
            .expect("failed to store test data");
    
        datapath.registers.gpr[8] = 4;
        datapath.execute_instruction();
        assert_eq!(datapath.registers.gpr[16], 0x10000);
    }

    #[test]
    fn lw_gpr_8_at_12_offset_at_neg_4_test() {
        // for this test the lw instruction will load 0x8 from memory
        // by adding the offset to gpr[8]
        let mut datapath = MipsDatapath::default();
    
        //                        lw     $t0   $s0      offset = 0
        let instruction: u32 = 0b100011_01000_10000_1111111111111100;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");
    
        // place data at address
        datapath
            .memory
            .store_word(0b1000, 0x10000)
            .expect("failed to store test data");
    
        datapath.registers.gpr[8] = 12;
        datapath.execute_instruction();
        assert_eq!(datapath.registers.gpr[16], 0x10000);
    }
}

pub mod store_word {
    use super::*;
    #[test]
    fn sw_zero_offset_test() {
        let mut datapath = MipsDatapath::default();

        //                        lw     $t0   $s0      offset = 0
        let instruction: u32 = 0b101011_01000_10000_0000000000000000;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");
        datapath.execute_instruction();

        let t = datapath
            .memory
            .load_word(0)
            .expect("Could not load from memory");
        assert_eq!(t, 0);
    }

    #[test]
    fn sw_offset_at_4_test() {
        let mut datapath = MipsDatapath::default();

        //                        sw     $t0   $s0      offset = 4
        let instruction: u32 = 0b101011_01000_10000_0000000000000100;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.registers.gpr[8] = 0;
        datapath.registers.gpr[16] = 0xff;
        datapath.execute_instruction();

        let t = datapath
            .memory
            .load_word(4)
            .expect("Could not load from memory");
        assert_eq!(t, 0xff);
    }

    #[test]
    fn lw_gpr_8_at_4_offset_at_4_test() {
        let mut datapath = MipsDatapath::default();

        //                        sw     $t0   $s0      offset = 4
        let instruction: u32 = 0b101011_01000_10000_0000000000000100;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.registers.gpr[8] = 4;
        datapath.registers.gpr[16] = 0xff;
        datapath.execute_instruction();

        let t = datapath
            .memory
            .load_word(8)
            .expect("Could not load from memory");
        assert_eq!(t, 0xff);
    }

    #[test]
    fn lw_gpr_8_at_4_offset_at_neg_4_test() {
        let mut datapath = MipsDatapath::default();

        //                        sw     $t0   $s0      offset = -4
        let instruction: u32 = 0b101011_01000_10000_1111111111111100;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.registers.gpr[8] = 12;
        datapath.registers.gpr[16] = 0xff;
        datapath.execute_instruction();

        let t = datapath
            .memory
            .load_word(8)
            .expect("Could not load from memory");
        assert_eq!(t, 0xff);
    }
}

pub mod coprocessor {
    use crate::emulation_core::datapath::Datapath;
    use crate::emulation_core::mips::datapath::MipsDatapath;

    #[test]
    pub fn add_float_single_precision() {
        let mut datapath = MipsDatapath::default();

        // add.s fd, fs, ft
        // add.s $f2, $f1, $f0
        // FPR[2] = FPR[1] + FPR[0]
        //                       COP1   fmt   ft    fs    fd    function
        //                              s     $f0   $f1   $f2   ADD
        let instruction: u32 = 0b010001_10000_00000_00001_00010_000000;
        datapath
            .memory
            .store_word(0, instruction)
            .expect("Failed to store instruction.");

        datapath.coprocessor.fpr[0] = f32::to_bits(0.25f32) as u64;
        datapath.coprocessor.fpr[1] = f32::to_bits(0.5f32) as u64;

        datapath.execute_instruction();

        // The result should be 0.75, represented in a 32-bit value as per the
        // IEEE 754 single-precision floating-point specification.
        assert_eq!(f32::from_bits(datapath.coprocessor.fpr[2] as u32), 0.75);
    }
}
