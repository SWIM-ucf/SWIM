#![allow(clippy::unusual_byte_groupings)]

use crate::emulation_core::datapath::Datapath;
use crate::emulation_core::mips::datapath::MipsDatapath;
use crate::emulation_core::mips::registers::GpRegisterType;

pub mod api {
    use super::*;
    use crate::parser::parser_assembler_main::parser;

    #[test]
    fn reset_datapath() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // Add instruction into emulation core memory.
        let instruction = String::from("ori $s0, $zero, 5");
        let (_, instruction_bits) = parser(instruction);
        datapath.initialize(instruction_bits)?;

        datapath.execute_instruction();

        // Datapath should now have some data in it.
        assert_ne!(datapath.memory.memory[0], 0);
        assert_ne!(datapath.registers.gpr[16], 0); // $s0
        assert_ne!(datapath.registers.pc, 0);

        datapath.reset();

        // After resetting, these values should all be back to bitwise zero.
        assert_eq!(datapath.memory.memory[0], 0);
        assert_eq!(datapath.registers.gpr[16], 0); // $s0
        assert_eq!(datapath.registers.pc, 0);

        Ok(())
    }
}

pub mod add {
    use super::*;
    #[test]
    fn add_register_to_itself() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $t1 = $t1 + $t1
        //                                  R-type  t1    t1    t1  (shamt)  ADD
        let instructions: Vec<u32> = vec![0b000000_01001_01001_01001_00000_100000];
        datapath.initialize(instructions)?;

        // Assume the register $t1 has the value 5.
        datapath.registers[GpRegisterType::T1] = 5;

        datapath.execute_instruction();

        // After the operation is finished, the register should be 10.
        assert_eq!(datapath.registers[GpRegisterType::T1], 10);
        Ok(())
    }

    #[test]
    fn add_register_to_another() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s2 = $s0 + $s1
        //                                  R-type  s0    s1    s2  (shamt)  ADD
        let instructions: Vec<u32> = vec![0b000000_10000_10001_10010_00000_100000];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[16] = 15; // $s0
        datapath.registers.gpr[17] = 40; // $s1

        datapath.execute_instruction();

        // Register $s2 should contain 55.
        let result = datapath.registers.gpr[18] as u32;
        assert_eq!(result, 55);
        Ok(())
    }

    #[test]
    // This test attempts to write to register $zero. The datapath should
    // not overwrite this register, and remain with a value of 0.
    fn add_to_register_zero() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $zero = $t3 + $t3
        //                                  R-type  t3    t3    zero (shamt) ADD
        let instructions: Vec<u32> = vec![0b000000_01011_01011_00000_00000_100000];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[11] = 1234; // $t3

        datapath.execute_instruction();

        // $zero should still contain 0.
        assert_eq!(datapath.registers.gpr[0], 0);
        Ok(())
    }

    #[test]
    // NOTE: This test falls under our initial project design that there are no
    // handled exceptions. Therefore, we would expect to see an updated value in
    // register T1, rather than having the register unmodified per the MIPS64v6
    // specification.
    fn add_32_bit_with_overflow() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $t1 = $t4 + $t4
        //                                  R-type  t4    t4    t1 (shamt) ADD
        let instructions: Vec<u32> = vec![0b000000_01100_01100_01001_00000_100000];
        datapath.initialize(instructions)?;

        // Assume register $t4 contains 2,454,267,026, a 32-bit integer.
        datapath.registers.gpr[12] = 0b10010010_01001001_00100100_10010010;

        datapath.execute_instruction();

        // Disregarding overflow, register $t4 would contain 4,908,534,052, or
        // 1_00100100_10010010_01001001_00100100 in binary. The result
        // should be truncated. Thus, we should expect the register to
        // contain 613,566,756, or 00100100_10010010_01001001_00100100 in binary.
        assert_eq!(datapath.registers.gpr[9], 613566756);
        Ok(())
    }

    #[test]
    // NOTE: This test falls under our initial project design that there are no
    // handled exceptions. Therefore, we would expect to see an updated value in
    // register T1, rather than having the register unmodified per the MIPS64v6
    // specification.
    fn add_32_bit_with_overflow_sign_extend() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $t1 = $t4 + $t4
        //                                  R-type  t4    t4    t1 (shamt) ADD
        let instructions: Vec<u32> = vec![0b000000_01100_01100_01001_00000_100000];
        datapath.initialize(instructions)?;

        // Assume register $t4 contains 3,528,008,850, a 32-bit integer.
        datapath.registers.gpr[12] = 0b11010010_01001001_00100100_10010010;

        datapath.execute_instruction();

        // Disregarding overflow, register $t4 would contain 7,056,017,700, or
        // 1_10100100_10010010_01001001_00100100 in binary. The result
        // should be truncated. Thus, we should expect the register to
        // contain 2,761,050,404, or 10100100_10010010_01001001_00100100 in binary.
        assert_eq!(datapath.registers.gpr[9] as u32, 2761050404);
        Ok(())
    }
}

pub mod sub {
    use super::*;

    #[test]
    fn sub_positive_result() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s2 = $s3 - $s2
        //                                  R-type  s3    s2    s2  (shamt) SUB
        let instructions: Vec<u32> = vec![0b000000_10011_10010_10010_00000_100010];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[19] = 7; // $s3
        datapath.registers.gpr[18] = 3; // $s2

        datapath.execute_instruction();

        // Register $s2 should contain 4, as 7 - 3 = 4.
        assert_eq!(datapath.registers.gpr[18], 4);
        Ok(())
    }

    #[test]
    fn sub_32_bit_negative_result() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s0 = $s0 - $t0
        //                                  R-type  s0    t0    s0  (shamt) SUB
        let instructions: Vec<u32> = vec![0b000000_10000_01000_10000_00000_100010];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[16] = 5; // $s0
        datapath.registers.gpr[8] = 20; // $t0

        datapath.execute_instruction();

        // Register $s0 should contain -15, as 5 - 20 = -15.
        assert_eq!(datapath.registers.gpr[16] as i32, -15);
        Ok(())
    }

    #[test]
    // NOTE: This test falls under our initial project design that there are no
    // handled exceptions. Therefore, we would expect to see an updated value in
    // register $s0, rather than having the register unmodified per the MIPS64v6
    // specification.
    fn sub_32_bit_underflow() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s0 = $s0 - $t0
        //                                  R-type  s0    t0    s0  (shamt) SUB
        let instructions: Vec<u32> = vec![0b000000_10000_01000_10000_00000_100010];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[16] = 0; // $s0
        datapath.registers.gpr[8] = 1; // $t0

        datapath.execute_instruction();

        // Register $s0 should contain the largest unsigned 32-bit integer due to underflow.
        assert_eq!(
            datapath.registers.gpr[16] as u32,
            0b11111111_11111111_11111111_11111111
        );
        Ok(())
    }
}

pub mod mul {
    use super::*;

    #[test]
    fn mul_positive_result() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s5 = $t7 * $t6
        //                                  R-type  t7    t6    s5    MUL   SOP30
        let instructions: Vec<u32> = vec![0b000000_01111_01110_10101_00010_011000];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[15] = 8; // $t7
        datapath.registers.gpr[14] = 95; // $t6

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[21], 760); // $s5
        Ok(())
    }

    #[test]
    fn mul_32_bit_negative_result() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s5 = $t7 * $t6
        //                                  R-type  t7    t6    s5    MUL   SOP30
        let instructions: Vec<u32> = vec![0b000000_01111_01110_10101_00010_011000];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[15] = 5; // $t7
        datapath.registers.gpr[14] = -5_i64 as u64; // $t6

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[21] as i64, -25); // $s5
        Ok(())
    }

    #[test]
    fn mul_result_truncate() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s4 = $t6 * $t5
        //                                  R-type  t6    t5    s4    MUL   SOP30
        let instructions: Vec<u32> = vec![0b000000_01110_01101_10100_00010_011000];
        datapath.initialize(instructions)?;

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
        Ok(())
    }
}

pub mod div {
    use super::*;

    #[test]
    fn div_positive_result() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s4 = $t6 / $t5
        //                                  R-type  t6    t5    s4    DIV   SOP32
        let instructions: Vec<u32> = vec![0b000000_01110_01101_10100_00010_011010];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[14] = 20; // $t6
        datapath.registers.gpr[13] = 2; // $t5

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[20], 10); // $s5
        Ok(())
    }

    #[test]
    fn div_negative_result() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s4 = $t6 / $t5
        //                                  R-type  t6    t5    s4    DIV   SOP32
        let instructions: Vec<u32> = vec![0b000000_01110_01101_10100_00010_011010];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[14] = 20; // $t6
        datapath.registers.gpr[13] = -5_i64 as u64; // $t5

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[20] as i64, -4); // $s5
        Ok(())
    }
}

pub mod or {
    use super::*;

    #[test]
    fn or_register_to_itself() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $t1 = $t1 & $t1
        //                                  R-type  t1    t1    t1  (shamt)  OR
        let instructions: Vec<u32> = vec![0b000000_01001_01001_01001_00000_100101];
        datapath.initialize(instructions)?;

        // Assume the register $t1 has the value 5.
        datapath.registers[GpRegisterType::T1] = 0x5;

        datapath.execute_instruction();
        assert_eq!(datapath.registers[GpRegisterType::T1], 0x5);
        Ok(())
    }

    #[test]
    fn or_register_to_another16() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s2 = $s0 & $s1
        //                                  R-type  s0    s1    s2  (shamt)  OR
        let instructions: Vec<u32> = vec![0b000000_10000_10001_10010_00000_100101];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[16] = 0x1234; // $s0
        datapath.registers.gpr[17] = 0x4321; // $s1

        datapath.execute_instruction();

        // Register $s2 should contain 55.
        let result = datapath.registers.gpr[18];
        assert_eq!(result, 0x5335);
        Ok(())
    }

    #[test]
    fn or_register_to_another32() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s2 = $s0 & $s1
        //                                  R-type  s0    s1    s2  (shamt)  OR
        let instructions: Vec<u32> = vec![0b000000_10000_10001_10010_00000_100101];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[16] = 0x12341234; // $s0
        datapath.registers.gpr[17] = 0x43214321; // $s1

        datapath.execute_instruction();

        // Register $s2 should contain something...
        let result = datapath.registers.gpr[18];
        assert_eq!(result, 0x53355335);
        Ok(())
    }

    #[test]
    fn or_register_to_another64() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s2 = $s0 & $s1
        //                                  R-type  s0    s1    s2  (shamt)  OR
        let instructions: Vec<u32> = vec![0b000000_10000_10001_10010_00000_100101];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[16] = 0x1234123412341234; // $s0
        datapath.registers.gpr[17] = 0x4321432143214321; // $s1

        datapath.execute_instruction();

        // Register $s2 should contain something...
        let result = datapath.registers.gpr[18];
        assert_eq!(result, 0x5335533553355335);
        Ok(())
    }

    #[test]
    // This test attempts to write to register $zero. The datapath should
    // not overwrite this register, and remain with a value of 0.
    fn or_to_register_zero() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $zero = $t3 & $t3
        //                                  R-type  t3    t3    zero (shamt) OR
        let instructions: Vec<u32> = vec![0b000000_01011_01011_00000_00000_100101];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[11] = 1234; // $t3

        datapath.execute_instruction();

        // $zero should still contain 0.
        assert_eq!(datapath.registers.gpr[0], 0);
        Ok(())
    }
}
pub mod and {
    use super::*;

    #[test]
    fn and_register_to_itself() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $t1 = $t1 & $t1
        //                                  R-type  t1    t1    t1  (shamt)  AND
        let instructions: Vec<u32> = vec![0b000000_01001_01001_01001_00000_100100];
        datapath.initialize(instructions)?;

        // Assume the register $t1 has the value 5.
        datapath.registers[GpRegisterType::T1] = 0x5;

        datapath.execute_instruction();
        assert_eq!(datapath.registers[GpRegisterType::T1], 0x5);
        Ok(())
    }

    #[test]
    fn and_register_to_another16() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s2 = $s0 & $s1
        //                                  R-type  s0    s1    s2  (shamt)  AND
        let instructions: Vec<u32> = vec![0b000000_10000_10001_10010_00000_100100];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[16] = 0x1234; // $s0
        datapath.registers.gpr[17] = 0x4321; // $s1

        datapath.execute_instruction();

        // Register $s2 should contain 55.
        let result = datapath.registers.gpr[18];
        assert_eq!(result, 0x0220);
        Ok(())
    }

    #[test]
    fn and_register_to_another32() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s2 = $s0 & $s1
        //                                  R-type  s0    s1    s2  (shamt)  AND
        let instructions: Vec<u32> = vec![0b000000_10000_10001_10010_00000_100100];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[16] = 0x12341234; // $s0
        datapath.registers.gpr[17] = 0x43214321; // $s1

        datapath.execute_instruction();

        // Register $s2 should contain 55.
        let result = datapath.registers.gpr[18];
        assert_eq!(result, 0x02200220);
        Ok(())
    }

    #[test]
    fn and_register_to_another64() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s2 = $s0 & $s1
        //                                  R-type  s0    s1    s2  (shamt)  AND
        let instructions: Vec<u32> = vec![0b000000_10000_10001_10010_00000_100100];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[16] = 0x1234123412341234; // $s0
        datapath.registers.gpr[17] = 0x4321432143214321; // $s1

        datapath.execute_instruction();

        // Register $s2 should contain 55.
        let result = datapath.registers.gpr[18];
        assert_eq!(result, 0x0220022002200220);
        Ok(())
    }

    #[test]
    // This test attempts to write to register $zero. The datapath should
    // not overwrite this register, and remain with a value of 0.
    fn and_to_register_zero() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $zero = $t3 & $t3
        //                                  R-type  t3    t3    zero (shamt) AND
        let instructions: Vec<u32> = vec![0b000000_01011_01011_00000_00000_100100];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[11] = 1234; // $t3

        datapath.execute_instruction();

        // $zero should still contain 0.
        assert_eq!(datapath.registers.gpr[0], 0);
        Ok(())
    }
}

// Shift Word Left Logical
pub mod sll {
    use super::*;
    #[test]
    fn easy_test() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // something
        //                                R-type        s1    s2  (shamt) SLL
        let instructions: Vec<u32> = vec![0b000000_00000_10001_10010_00000_000000];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[0b10001] = 123;
        datapath.registers.gpr[0b10010] = 321;

        datapath.execute_instruction();
        assert_eq!(datapath.registers.gpr[0b10010], 123);
        Ok(())
    }

    #[test]
    fn mid_test() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // Shift left by two logical
        //                                R-type        s1    s2  (shamt) SLL
        let instructions: Vec<u32> = vec![0b000000_00000_10001_10010_00010_000000];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[0b10001] = 0b1010;
        datapath.registers.gpr[0b10010] = 0;

        datapath.execute_instruction();
        assert_eq!(datapath.registers.gpr[0b10010], 0b101000);
        Ok(())
    }
}

pub mod slt {
    use super::*;

    #[test]
    fn easy_rs_less_than_rt_test() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s2 = $s0 < $s1
        //                                  R-type  s0    s1    s2  (shamt)  SLT
        let instructions: Vec<u32> = vec![0b000000_10000_10001_10010_00000_101010];
        datapath.initialize(instructions)?;

        datapath.registers[GpRegisterType::S0] = 1;
        datapath.registers[GpRegisterType::S1] = 123;

        datapath.execute_instruction();

        assert_eq!(datapath.registers[GpRegisterType::S2], 1);
        Ok(())
    }

    #[test]
    fn easy_rs_greater_than_rt_test() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s2 = $s0 < $s1
        //                                  R-type  s0    s1    s2  (shamt)  SLT
        let instructions: Vec<u32> = vec![0b000000_10000_10001_10010_00000_101010];
        datapath.initialize(instructions)?;

        datapath.registers[GpRegisterType::S0] = 124;
        datapath.registers[GpRegisterType::S1] = 123;

        datapath.execute_instruction();

        assert_eq!(datapath.registers[GpRegisterType::S2], 0);
        Ok(())
    }

    #[test]
    fn easy_signed_test() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s2 = $s0 < $s1
        //                                  R-type  s0    s1    s2  (shamt)  SLT
        let instructions: Vec<u32> = vec![0b000000_10000_10001_10010_00000_101010];
        datapath.initialize(instructions)?;

        datapath.registers[GpRegisterType::S0] = -124_i64 as u64;
        datapath.registers[GpRegisterType::S1] = 123;

        datapath.execute_instruction();

        assert_eq!(datapath.registers[GpRegisterType::S2], 1);
        Ok(())
    }
}

pub mod sltu {
    use super::*;

    #[test]
    fn easy_rs_less_than_rt_test() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s2 = $s0 < $s1
        //                                  R-type  s0    s1    s2  (shamt)  SLTU
        let instructions: Vec<u32> = vec![0b000000_10000_10001_10010_00000_101011];
        datapath.initialize(instructions)?;

        datapath.registers[GpRegisterType::S0] = 1;
        datapath.registers[GpRegisterType::S1] = 123;

        datapath.execute_instruction();

        assert_eq!(datapath.registers[GpRegisterType::S2], 1);
        Ok(())
    }

    #[test]
    fn easy_rs_greater_than_rt_test() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s2 = $s0 < $s1
        //                                  R-type  s0    s1    s2  (shamt)  SLTU
        let instructions: Vec<u32> = vec![0b000000_10000_10001_10010_00000_101011];
        datapath.initialize(instructions)?;

        datapath.registers[GpRegisterType::S0] = 124;
        datapath.registers[GpRegisterType::S1] = 123;

        datapath.execute_instruction();

        assert_eq!(datapath.registers[GpRegisterType::S2], 0);
        Ok(())
    }

    #[test]
    fn easy_signed_test() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s2 = $s0 < $s1
        //                                  R-type  s0    s1    s2  (shamt)  SLTU
        let instructions: Vec<u32> = vec![0b000000_10000_10001_10010_00000_101011];
        datapath.initialize(instructions)?;

        datapath.registers[GpRegisterType::S0] = -124_i64 as u64;
        datapath.registers[GpRegisterType::S1] = 123;

        datapath.execute_instruction();

        assert_eq!(datapath.registers[GpRegisterType::S2], 0);
        Ok(())
    }
}

pub mod andi {
    use super::*;
    #[test]
    fn and_immediate_with_zero() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s0 = $zero & 12345
        //                                  andi    $zero  $s0   12345
        let instructions: Vec<u32> = vec![0b001100_00000_10000_0011000000111001];
        datapath.initialize(instructions)?;

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[16], 0); // $s0
        Ok(())
    }

    #[test]
    fn andi_immediate_with_value() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 & 12345
        //                                  andi     $t0   $s0   12345
        let instructions: Vec<u32> = vec![0b001100_01000_10000_0011000000111001];
        datapath.initialize(instructions)?;

        // In binary: 00111010 11011110 01101000 10110001
        datapath.registers.gpr[8] = 987654321; // $t0

        datapath.execute_instruction();

        // The result should be as follows:
        //         $t0:  00111010 11011110 01101000 10110001
        // AND  12,345:                    00110000 00111001
        // =================================================
        //       8,241:  00000000 00000000 00100000 00110001

        assert_eq!(datapath.registers.gpr[16], 0x2031); // $s0
        Ok(())
    }
}

pub mod addi_addiu {
    use super::*;
    #[test]
    fn addi_simple_test() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x4
        //                                  addi    $t0   $s0          4
        let instructions: Vec<u32> = vec![0b001000_01000_10000_0000000000000100];
        datapath.initialize(instructions)?;
        datapath.registers[GpRegisterType::T0] = 1;
        datapath.registers[GpRegisterType::S0] = 123;
        datapath.execute_instruction();

        assert_eq!(datapath.registers[GpRegisterType::S0], 5);
        Ok(())
    }

    #[test]
    // NOTE: This test falls under our initial project design that there are no
    // handled exceptions. Therefore, we would expect to see an updated value in
    // register S0, rather than having the register unmodified per the MIPS64v6
    // specification.
    fn addi_overflow_test() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x4
        //                                  addi    $t0   $s0          4
        let instructions: Vec<u32> = vec![0b001000_01000_10000_0000000000000100];
        datapath.initialize(instructions)?;
        datapath.registers[GpRegisterType::T0] = 0xffffffff;
        datapath.registers[GpRegisterType::S0] = 123;
        datapath.execute_instruction();

        // If there is an overflow on addi, $s0 should not change.
        assert_eq!(datapath.registers[GpRegisterType::S0], 3);
        Ok(())
    }

    #[test]
    fn addi_sign_extend_test() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x1
        //                                  addi    $t0   $s0          1
        let instructions: Vec<u32> = vec![0b001000_01000_10000_0000000000000001];
        datapath.initialize(instructions)?;
        datapath.registers[GpRegisterType::T0] = 0xfffffff1;
        datapath.execute_instruction();

        assert_eq!(datapath.registers[GpRegisterType::S0], 0xfffffffffffffff2);
        Ok(())
    }

    #[test]
    fn addi_sign_extend_test2() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x1
        //                                  addi    $t0   $s0          1
        let instructions: Vec<u32> = vec![0b001000_01000_10000_0000000000000001];
        datapath.initialize(instructions)?;
        datapath.registers[GpRegisterType::T0] = 0xfffffffe;
        datapath.execute_instruction();

        assert_eq!(datapath.registers[GpRegisterType::S0], 0xffffffffffffffff);
        Ok(())
    }

    #[test]
    fn addiu_simple_test() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x4
        //                                  addiu    $t0   $s0          4
        let instructions: Vec<u32> = vec![0b001001_01000_10000_0000000000000100];
        datapath.initialize(instructions)?;
        datapath.registers[GpRegisterType::T0] = 1;
        datapath.registers[GpRegisterType::S0] = 123;
        datapath.execute_instruction();

        assert_eq!(datapath.registers[GpRegisterType::S0], 5);
        Ok(())
    }

    #[test]
    fn addiu_overflow_test() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x4
        //                                  addiu    $t0   $s0          4
        let instructions: Vec<u32> = vec![0b001001_01000_10000_0000000000000100];
        datapath.initialize(instructions)?;
        datapath.registers[GpRegisterType::T0] = 0xffffffff;
        datapath.registers[GpRegisterType::S0] = 123;
        datapath.execute_instruction();

        // For the addiu instruction, $s0 would change on overflow, it would become 3.
        assert_eq!(datapath.registers[GpRegisterType::S0], 3);
        Ok(())
    }

    #[test]
    fn addiu_sign_extend_test() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x1
        //                                  addi    $t0   $s0          1
        let instructions: Vec<u32> = vec![0b001000_01000_10000_0000000000000001];
        datapath.initialize(instructions)?;
        datapath.registers[GpRegisterType::T0] = 0xfffffff1;
        datapath.execute_instruction();

        assert_eq!(datapath.registers[GpRegisterType::S0], 0xfffffffffffffff2);
        Ok(())
    }
}

pub mod daddi_and_daddiu {
    use super::*;
    #[test]
    fn daddi_simple_test() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x4
        //                                  daddi    $t0   $s0          4
        let instructions: Vec<u32> = vec![0b011000_01000_10000_0000000000000100];
        datapath.initialize(instructions)?;
        datapath.registers[GpRegisterType::T0] = 1;
        datapath.registers[GpRegisterType::S0] = 123;
        datapath.execute_instruction();

        assert_eq!(datapath.registers[GpRegisterType::S0], 5);
        Ok(())
    }

    #[test]
    // NOTE: This test falls under our initial project design that there are no
    // handled exceptions. Therefore, we would expect to see an updated value in
    // register T1, rather than having the register unmodified per the MIPS64v6
    // specification.
    fn daddi_overflow_test() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x1
        //                                  daddi    $t0   $s0          1
        let instructions: Vec<u32> = vec![0b011000_01000_10000_0000000000000001];
        datapath.initialize(instructions)?;
        datapath.registers[GpRegisterType::T0] = 0xffffffffffffffff;
        datapath.registers[GpRegisterType::S0] = 123;
        datapath.execute_instruction();

        assert_eq!(datapath.registers[GpRegisterType::S0], 0);
        Ok(())
    }

    #[test]
    fn daddi_sign_extend_test() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x1
        //                                  daddi    $t0   $s0          1
        let instructions: Vec<u32> = vec![0b011000_01000_10000_0000000000000001];
        datapath.initialize(instructions)?;
        datapath.registers[GpRegisterType::T0] = 0xfffffffffffffff1;
        datapath.execute_instruction();

        assert_eq!(datapath.registers[GpRegisterType::S0], 0xfffffffffffffff2);
        Ok(())
    }

    #[test]
    fn daddi_sign_extend_test2() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x1
        //                                  daddi    $t0   $s0          1
        let instructions: Vec<u32> = vec![0b011000_01000_10000_0000000000000001];
        datapath.initialize(instructions)?;
        datapath.registers[GpRegisterType::T0] = 0xfffffffffffffffe;
        datapath.execute_instruction();

        assert_eq!(datapath.registers[GpRegisterType::S0], 0xffffffffffffffff);
        Ok(())
    }

    #[test]
    fn daddiu_simple_test() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x4
        //                                  daddiu    $t0   $s0          4
        let instructions: Vec<u32> = vec![0b011001_01000_10000_0000000000000100];
        datapath.initialize(instructions)?;
        datapath.registers[GpRegisterType::T0] = 1;
        datapath.registers[GpRegisterType::S0] = 123;
        datapath.execute_instruction();

        assert_eq!(datapath.registers[GpRegisterType::S0], 5);
        Ok(())
    }

    #[test]
    fn daddiu_overflow_test() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x4
        //                                  daddiu    $t0   $s0          4
        let instructions: Vec<u32> = vec![0b011001_01000_10000_0000000000000100];
        datapath.initialize(instructions)?;
        datapath.registers[GpRegisterType::T0] = 0xffffffffffffffff;
        datapath.registers[GpRegisterType::S0] = 123;
        datapath.execute_instruction();

        // if there is an overflow, $s0 should not change.
        // For the daddiu instruction, $s0 would change on overflow, it would become 3.
        assert_eq!(datapath.registers[GpRegisterType::S0], 3);
        Ok(())
    }

    #[test]
    fn daddiu_sign_extend_test() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 + 0x1
        //                                  daddiu    $t0   $s0          1
        let instructions: Vec<u32> = vec![0b011001_01000_10000_0000000000000001];
        datapath.initialize(instructions)?;
        datapath.registers[GpRegisterType::T0] = 0xfffffffffffffff1;
        datapath.execute_instruction();

        assert_eq!(datapath.registers[GpRegisterType::S0], 0xfffffffffffffff2);
        Ok(())
    }
}

pub mod ori {
    use super::*;
    #[test]
    fn or_immediate_with_zero() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s0 = $zero | 12345
        //                                  ori    $zero  $s0   12345
        let instructions: Vec<u32> = vec![0b001101_00000_10000_0011000000111001];
        datapath.initialize(instructions)?;

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[16], 12345); // $s0
        Ok(())
    }

    #[test]
    fn or_immediate_with_value() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // $s0 = $t0 | 12345
        //                                  ori     $t0   $s0   12345
        let instructions: Vec<u32> = vec![0b001101_01000_10000_0011000000111001];
        datapath.initialize(instructions)?;

        // In binary: 00111010 11011110 01101000 10110001
        datapath.registers.gpr[8] = 987654321; // $t0

        datapath.execute_instruction();

        // The result should be as follows:
        //         $t0:  00111010 11011110 01101000 10110001
        // OR   12,345:                    00110000 00111001
        // =================================================
        // 987,658,425:  00111010 11011110 01111000 10111001

        assert_eq!(datapath.registers.gpr[16], 987658425); // $s0
        Ok(())
    }
}

pub mod dadd_daddu {
    use super::*;

    #[test]
    fn dadd_register_to_itself() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // dadd rd, rs, rt
        // dadd $v0, $t5, $t5
        // GPR[2] <- GPR[13] + GPR[13]
        //                                 SPECIAL rs    rt    rd    0     DADD
        //                                         13    13    2
        let instructions: Vec<u32> = vec![0b000000_01101_01101_00010_00000_101100];
        datapath.initialize(instructions)?;

        // Assume register $t5 contains 969,093,589,304, which is an integer
        // that takes up 39 bits.
        datapath.registers.gpr[13] = 969_093_589_304; // $t5

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[2], 1_938_187_178_608); // $v0
        Ok(())
    }

    // NOTE: This test falls under our initial project design that there are no
    // handled exceptions. Therefore, we would expect to see an updated value in
    // register $v0, rather than having the register unmodified per the MIPS64v6
    // specification.
    #[test]
    fn dadd_positive_overflow() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // dadd rd, rs, rt
        // dadd $v0, $t5, $t5
        // GPR[2] <- GPR[13] + GPR[13]
        //                                 SPECIAL rs    rt    rd    0     DADD
        //                                         13    13    2
        let instructions: Vec<u32> = vec![0b000000_01101_01101_00010_00000_101100];
        datapath.initialize(instructions)?;

        // Assume register $t5 contains 18,134,889,837,812,767,690, which is an integer
        // that takes up 64 bits.
        datapath.registers.gpr[13] = 18_134_889_837_812_767_690; // $t5

        datapath.execute_instruction();

        // The result is truncated to give a 64-bit value.
        assert_eq!(datapath.registers.gpr[2], 17_823_035_601_915_983_764); // $v0
        Ok(())
    }

    #[test]
    fn daddu_positive_result() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // daddu rd, rs, rt
        // daddu $s2, $s0, $s1
        // GPR[18] <- GPR[16] + GPR[17]
        //                                 SPECIAL rs    rt    rd    0     DADDU
        //                                         16    17    18
        let instructions: Vec<u32> = vec![0b000000_10000_10001_10010_00000_101101];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[16] = 1_069_193_590_294; // $s0
        datapath.registers.gpr[17] = 34_359_738_368; // $s1

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[18], 1_103_553_328_662); // $s2
        Ok(())
    }
}

pub mod dsub_dsubu {
    use super::*;

    #[test]
    fn dsub_registers_positive_result() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // dsub rd, rs, rt
        // dsub $s5, $s4, $s3
        // GPR[rd] <- GPR[rs] - GPR[rt]
        // GPR[$s5] <- GPR[$s4] - GPR[$s3]
        // GPR[19] <- GPR[18] - GPR[17]
        //                                 SPECIAL rs    rt    rd    0     funct
        //                                         $s4   $s3   $s5         DSUB
        //                                         18    17    19
        let instructions: Vec<u32> = vec![0b000000_10010_10001_10011_00000_101110];
        datapath.initialize(instructions)?;

        // Assume registers $s3 and $s4 contain numbers larger than 32 bits,
        // but smaller than 64 bits.
        datapath.registers.gpr[18] = 4_833_323_886_298_794; // $s4
        datapath.registers.gpr[17] = 163_643_849_115_304; // $s3

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[19], 4_669_680_037_183_490); // $s5
        Ok(())
    }

    // NOTE: This test falls under our initial project design that there are no
    // handled exceptions. Therefore, we would expect to see an updated value in
    // register $s5, rather than having the register unmodified per the MIPS64v6
    // specification.
    #[test]
    fn dsub_negative_integer_underflow() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // dsub rd, rs, rt
        // dsub $s5, $s4, $s3
        // GPR[rd] <- GPR[rs] - GPR[rt]
        // GPR[$s5] <- GPR[$s4] - GPR[$s3]
        // GPR[19] <- GPR[18] - GPR[17]
        //                                 SPECIAL rs    rt    rd    0     funct
        //                                         $s4   $s3   $s5         DSUB
        //                                         18    17    19
        let instructions: Vec<u32> = vec![0b000000_10010_10001_10011_00000_101110];
        datapath.initialize(instructions)?;

        // Assume registers $s4 is the minimum possible integer and $s4 is 1.
        datapath.registers.gpr[18] = 0; // $s4
        datapath.registers.gpr[17] = 1; // $s3

        datapath.execute_instruction();

        // Given a negative integer overflow, this should become the maximum possible unsigned integer.
        assert_eq!(datapath.registers.gpr[19], 0xffff_ffff_ffff_ffff); // $s5
        Ok(())
    }

    #[test]
    fn dsubu_positive_result() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // dsubu rd, rs, rt
        // dsubu $s2, $s0, $s1
        // GPR[18] <- GPR[16] - GPR[17]
        //                                 SPECIAL rs    rt    rd    0     DSUBU
        //                                         16    17    18
        let instructions: Vec<u32> = vec![0b000000_10000_10001_10010_00000_101111];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[16] = 92_975_612_771_919; // $s0
        datapath.registers.gpr[17] = 13_810_775_572_047; // $s1

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[18], 79_164_837_199_872); // $s2
        Ok(())
    }
}

pub mod dmul_dmulu {
    use super::*;

    #[test]
    fn dmul_positive_result() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // dmul rd, rs, rt
        // dmul $a0, $t8, $t9
        // dmul 4, 24, 25
        // GPR[rd] <- lo_doubleword(multiply.signed(GPR[rs] * GPR[rt]))
        //                                 opcode  rs    rt    rd          funct
        //                                 SPECIAL $t8   $t9   $a0   DMUL  SOP34
        //                                         24    25    4
        let instructions: Vec<u32> = vec![0b000000_11000_11001_00100_00010_011100];
        datapath.initialize(instructions)?;

        // Assume register $t8 contains a number larger than 32 bits,
        // but smaller than 64 bits.
        datapath.registers.gpr[24] = 5_861_036_283_017; // $t8
        datapath.registers.gpr[25] = 5; // $t9

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[4], 29_305_181_415_085); // $a0
        Ok(())
    }

    #[test]
    fn dmul_negative_result() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // dmul rd, rs, rt
        // dmul $s7, $t7, $t6
        // dmul 23, 15, 14
        // GPR[rd] <- lo_doubleword(multiply.signed(GPR[rs] * GPR[rt]))
        //                                 opcode  rs    rt    rd          funct
        //                                 SPECIAL $t7   $t6   $s7   DMUL  SOP34
        //                                         15    14    23
        let instructions: Vec<u32> = vec![0b000000_01111_01110_10111_00010_011100];
        datapath.initialize(instructions)?;

        // Assume register $t7 contains a number larger than 32 bits,
        // but smaller than 64 bits.
        datapath.registers.gpr[15] = 363_251_152_978_005; // $t7
        datapath.registers.gpr[14] = -19_i64 as u64; // $t6

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[23] as i64, -6_901_771_906_582_095); // $s7
        Ok(())
    }

    #[test]
    fn dmul_result_truncate() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // dmul rd, rs, rt
        // dmul $s2, $s4, $s3
        // dmul 18, 20, 19
        // GPR[rd] <- lo_doubleword(multiply.signed(GPR[rs] * GPR[rt]))
        //                                 opcode  rs    rt    rd          funct
        //                                 SPECIAL $s4   $s3   $s2   DMUL  SOP34
        //                                         20    19    18
        let instructions: Vec<u32> = vec![0b000000_10100_10011_10010_00010_011100];
        datapath.initialize(instructions)?;

        // Assume registers $s4 and $s3 contain numbers larger than 32 bits,
        // but smaller than 64 bits.
        datapath.registers.gpr[20] = 191_893_548_893_556_856; // $s4
        datapath.registers.gpr[19] = 2_799_316_838_897; // $s3

        datapath.execute_instruction();

        // The result, 537,170,842,693,438,490,068,661,827,832, is too large for
        // a 64-bit integer.
        // (110 11000111 10110001 01001110 10000100 [00110100 01101011 00001011 00010110 11011010 00010011 11111000 11111000])
        // The result should instead truncate to the lower 64 bits.
        assert_eq!(datapath.registers.gpr[18], 3_777_124_905_256_220_920); // $s2
        Ok(())
    }

    #[test]
    fn dmulu_positive_result() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // dmulu rd, rs, rt
        // dmulu $s2, $s0, $s1
        // dmulu 18, 16, 17
        // GPR[rd] <- lo_doubleword(multiply.unsigned(GPR[rs] * GPR[rt]))
        //                                 opcode  rs    rt    rd          funct
        //                                 SPECIAL $s0   $s1   $s2   DMULU SOP35
        //                                         16    17    18
        let instructions: Vec<u32> = vec![0b000000_10000_10001_10010_00010_011101];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[16] = 17_592_186_044_416; // $s0
        datapath.registers.gpr[17] = 1_000; // $s1

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[18], 17_592_186_044_416_000); // $s2
        Ok(())
    }
}

pub mod ddiv_ddivu {
    use super::*;

    #[test]
    fn ddiv_positive_result() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // ddiv rd, rs, rt
        // ddiv $s0, $s1, $s2
        // ddiv 16, 17, 18
        // GPR[rd] <- divide.signed(GPR[rs], GPR[rt])
        //                                 opcode  rs    rt    rd          funct
        //                                 SPECIAL $s1   $s2   $s0   DDIV  SOP36
        //                                         17    18    16
        let instructions: Vec<u32> = vec![0b000000_10001_10010_10000_00010_011110];

        datapath.initialize(instructions)?;

        // Assume register $s1 contains a number larger than 32 bits,
        // but smaller than 64 bits.
        datapath.registers.gpr[17] = 1_284_064_531_192; // $s1
        datapath.registers.gpr[18] = 7; // $s2

        datapath.execute_instruction();

        // While the actual result is 183,437,790,170.285714....
        // the decimal portion is truncated.
        assert_eq!(datapath.registers.gpr[16], 183_437_790_170); // $s0
        Ok(())
    }

    #[test]
    fn ddiv_negative_result() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // ddiv rd, rs, rt
        // ddiv $a3, $a2, $a1
        // ddiv 7, 6, 5
        // GPR[rd] <- divide.signed(GPR[rs], GPR[rt])
        //                                 opcode  rs    rt    rd          funct
        //                                 SPECIAL $a2   $a1   $a3   DDIV  SOP36
        //                                         6     5     7
        let instructions: Vec<u32> = vec![0b000000_00110_00101_00111_00010_011110];

        datapath.initialize(instructions)?;

        // Assume register $a2 contains a number larger than 32 bits,
        // but smaller than 64 bits.
        datapath.registers.gpr[6] = -6_245_352_518_120_328_878_i64 as u64; // $a2
        datapath.registers.gpr[5] = 123; // $a1

        datapath.execute_instruction();

        // While the actual result is -50,775,223,724,555,519.333333....
        // the decimal portion is truncated.
        assert_eq!(datapath.registers.gpr[7] as i64, -50_775_223_724_555_519); // $a3
        Ok(())
    }

    #[test]
    fn ddivu_positive_result() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // ddivu rd, rs, rt
        // ddivu $s2, $s0, $s1
        // ddivu 18, 16, 17
        // GPR[rd] <- divide.unsigned(GPR[rs], GPR[rt])
        //                                 opcode  rs    rt    rd          funct
        //                                 SPECIAL $s0   $s1   $s2   DDIVU SOP37
        //                                         16    17    18
        let instructions: Vec<u32> = vec![0b000000_10000_10001_10010_00010_011111];

        datapath.initialize(instructions)?;

        datapath.registers.gpr[16] = 10_213_202_487_240; // $s0
        datapath.registers.gpr[17] = 11; // $s1

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[18], 928_472_953_385); // $s2
        Ok(())
    }
}

pub mod dahi_dati {
    use super::*;

    #[test]
    fn dahi_basic_add() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // dahi rs, immediate
        // dahi $a0, 1
        // GPR[rs] <- GPR[rs] + sign_extend(immediate << 32)
        // GPR[4] <- GPR[4] + sign_extend(1 << 32)
        //                                  op     rs    rt     immediate
        //                                  REGIMM $a0   DAHI   1
        let instructions: Vec<u32> = vec![0b000001_00100_00110_0000000000000001];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[4] = 0xABCD; // $a0

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[4], 0x0000_0001_0000_ABCD);

        Ok(())
    }

    #[test]
    fn dati_basic_add() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // dati rs, immediate
        // dati $a1, 1
        // GPR[rs] <- GPR[rs] + sign_extend(immediate << 48)
        // GPR[5] <- GPR[5] + sign_extend(1 << 48)
        //                                  op     rs    rt     immediate
        //                                  REGIMM $a1   DATI   1
        let instructions: Vec<u32> = vec![0b000001_00101_11110_0000000000000001];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[5] = 0xABCD; // $a1

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[5], 0x0001_0000_0000_ABCD);

        Ok(())
    }
}

pub mod load_word {
    use super::*;
    #[test]
    fn lw_zero_offset_test() -> Result<(), String> {
        // for this test the lw instruction will load itself from
        // memory
        let mut datapath = MipsDatapath::default();

        //                                  lw     $t0   $s0      offset = 0
        let instructions: Vec<u32> = vec![0b100011_01000_10000_0000000000000000];
        datapath.initialize(instructions.clone())?;
        datapath.execute_instruction();
        assert_eq!(datapath.registers.gpr[16], instructions[0] as u64);
        Ok(())
    }

    #[test]
    fn lw_offset_at_4_test() -> Result<(), String> {
        // For this test the lw instruction will load 0x4 from memory
        // by using the offset address plus zero
        let mut datapath = MipsDatapath::default();

        //                                  lw     $t0   $s0      offset = 4
        let instructions: Vec<u32> = vec![0b100011_01000_10000_0000000000000100];
        datapath.initialize(instructions)?;

        // place data at address
        datapath.memory.store_word(0b100, 0x10000)?;

        datapath.registers.gpr[8] = 0;
        datapath.execute_instruction();
        assert_eq!(datapath.registers.gpr[16], 0x10000);
        Ok(())
    }

    #[test]
    fn lw_gpr_8_at_4_offset_at_0_test() -> Result<(), String> {
        // for this test the lw instruction will load 0x4 from memory
        // by using (offset = 0) + (gpr[8] = 4)
        let mut datapath = MipsDatapath::default();

        //                                  lw     $t0   $s0      offset = 0
        let instructions: Vec<u32> = vec![0b100011_01000_10000_0000000000000000];
        datapath.initialize(instructions)?;

        // place data at address
        datapath.memory.store_word(0b100, 0x10000)?;

        datapath.registers.gpr[8] = 4;
        datapath.execute_instruction();
        assert_eq!(datapath.registers.gpr[16], 0x10000);
        Ok(())
    }

    #[test]
    fn lw_gpr_8_at_4_offset_at_4_test() -> Result<(), String> {
        // for this test the lw instruction will load 0x8 from memory
        // by adding the offset to gpr[8]
        let mut datapath = MipsDatapath::default();

        //                                  lw     $t0   $s0      offset = 0
        let instructions: Vec<u32> = vec![0b100011_01000_10000_0000000000000100];
        datapath.initialize(instructions)?;

        // place data at address
        datapath.memory.store_word(0b1000, 0x10000)?;

        datapath.registers.gpr[8] = 4;
        datapath.execute_instruction();
        assert_eq!(datapath.registers.gpr[16], 0x10000);
        Ok(())
    }

    #[test]
    fn lw_gpr_8_at_12_offset_at_neg_4_test() -> Result<(), String> {
        // for this test the lw instruction will load 0x8 from memory
        // by adding the offset to gpr[8]
        let mut datapath = MipsDatapath::default();

        //                                  lw     $t0   $s0      offset = 0
        let instructions: Vec<u32> = vec![0b100011_01000_10000_1111111111111100];
        datapath.initialize(instructions)?;

        // place data at address
        datapath.memory.store_word(0b1000, 0x10000)?;

        datapath.registers.gpr[8] = 12;
        datapath.execute_instruction();
        assert_eq!(datapath.registers.gpr[16], 0x10000);
        Ok(())
    }
}

pub mod load_upper_imm {
    use super::*;

    #[test]
    fn basic_load_upper_imm_test() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        //                                  lui    $t0   $s0      offset = 42
        let instructions: Vec<u32> = vec![0b001111_01000_10000_0010101010101010];
        datapath.initialize(instructions)?;
        datapath.execute_instruction();

        let t = datapath.registers[GpRegisterType::S0];
        assert_eq!(t, 0x2aaa_0000);
        Ok(())
    }

    #[test]
    fn sign_extend_load_upper_imm_test() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        //                                  lui    $t0   $s0      offset = 42
        let instructions: Vec<u32> = vec![0b001111_01000_10000_1010101010101010];
        datapath.initialize(instructions)?;
        datapath.execute_instruction();

        let t = datapath.registers[GpRegisterType::S0];
        assert_eq!(t, 0xffff_ffff_aaaa_0000);
        Ok(())
    }
}
pub mod store_word {
    use super::*;
    #[test]
    fn sw_zero_offset_test() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        //                                  sw     $t0   $s0      offset = 0
        let instructions: Vec<u32> = vec![0b101011_01000_10000_0000000000000000];
        datapath.initialize(instructions)?;
        datapath.execute_instruction();

        let t = datapath.memory.load_word(0)?;
        assert_eq!(t, 0);
        Ok(())
    }

    #[test]
    fn sw_offset_at_4_test() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        //                                  sw     $t0   $s0      offset = 4
        let instructions: Vec<u32> = vec![0b101011_01000_10000_0000000000000100];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[8] = 0;
        datapath.registers.gpr[16] = 0xff;
        datapath.execute_instruction();

        let t = datapath.memory.load_word(4)?;
        assert_eq!(t, 0xff);
        Ok(())
    }

    #[test]
    fn lw_gpr_8_at_4_offset_at_4_test() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        //                                  sw     $t0   $s0      offset = 4
        let instructions: Vec<u32> = vec![0b101011_01000_10000_0000000000000100];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[8] = 4;
        datapath.registers.gpr[16] = 0xff;
        datapath.execute_instruction();

        let t = datapath.memory.load_word(8)?;
        assert_eq!(t, 0xff);
        Ok(())
    }

    #[test]
    fn lw_gpr_8_at_4_offset_at_neg_4_test() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        //                                  sw     $t0   $s0      offset = -4
        let instructions: Vec<u32> = vec![0b101011_01000_10000_1111111111111100];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[8] = 12;
        datapath.registers.gpr[16] = 0xff;
        datapath.execute_instruction();

        let t = datapath.memory.load_word(8)?;
        assert_eq!(t, 0xff);
        Ok(())
    }
}

pub mod coprocessor {
    use crate::emulation_core::datapath::Datapath;
    use crate::emulation_core::mips::datapath::MipsDatapath;

    #[test]
    pub fn add_float_single_precision() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // add.s fd, fs, ft
        // add.s $f2, $f1, $f0
        // FPR[2] = FPR[1] + FPR[0]
        //                                  COP1   fmt   ft    fs    fd    function
        //                                         s     $f0   $f1   $f2   ADD
        let instructions: Vec<u32> = vec![0b010001_10000_00000_00001_00010_000000];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[0] = f32::to_bits(0.25f32) as u64;
        datapath.coprocessor.fpr[1] = f32::to_bits(0.5f32) as u64;

        datapath.execute_instruction();

        // The result should be 0.75, represented in a 32-bit value as per the
        // IEEE 754 single-precision floating-point specification.
        assert_eq!(f32::from_bits(datapath.coprocessor.fpr[2] as u32), 0.75);
        Ok(())
    }

    #[test]
    pub fn add_float_double_precision() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // add.d fd, fs, ft
        // add.d $f2, $f1, $f0
        // FPR[2] = FPR[1] + FPR[0]
        //                                  COP1   fmt   ft    fs    fd    function
        //                                         d     $f0   $f1   $f2   ADD
        let instructions: Vec<u32> = vec![0b010001_10001_00000_00001_00010_000000];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[0] = f64::to_bits(123.125);
        datapath.coprocessor.fpr[1] = f64::to_bits(0.5);

        datapath.execute_instruction();

        // The result should be 123.625, represented in a 64-bit value as per the
        // IEEE 754 double-precision floating-point specification.
        assert_eq!(f64::from_bits(datapath.coprocessor.fpr[2]), 123.625);
        Ok(())
    }

    #[test]
    pub fn sub_float_single_precision() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // sub.s fd, fs, ft
        // sub.s $f2, $f1, $f0
        // FPR[fd] = FPR[fs] - FPR[ft]
        // FPR[2] = FPR[1] - FPR[0]
        //                                  COP1   fmt   ft    fs    fd    function
        //                                         s     $f0   $f1   $f2   SUB
        let instructions: Vec<u32> = vec![0b010001_10000_00000_00001_00010_000001];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[0] = f32::to_bits(5.625f32) as u64;
        datapath.coprocessor.fpr[1] = f32::to_bits(3.125f32) as u64;

        datapath.execute_instruction();

        assert_eq!(f32::from_bits(datapath.coprocessor.fpr[2] as u32), -2.5);
        Ok(())
    }

    #[test]
    pub fn sub_float_double_precision() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // sub.d fd, fs, ft
        // sub.d $f2, $f1, $f0
        // FPR[fd] = FPR[fs] - FPR[ft]
        // FPR[2] = FPR[1] - FPR[0]
        //                                  COP1   fmt   ft    fs    fd    function
        //                                         d     $f0   $f1   $f2   SUB
        let instructions: Vec<u32> = vec![0b010001_10001_00000_00001_00010_000001];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[0] = f64::to_bits(438.125);
        datapath.coprocessor.fpr[1] = f64::to_bits(98765.5);

        datapath.execute_instruction();

        assert_eq!(f64::from_bits(datapath.coprocessor.fpr[2]), 98327.375);
        Ok(())
    }

    #[test]
    pub fn mul_float_single_precision() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // mul.s fd, fs, ft
        // mul.s $f9, $f5, $f4
        // FPR[fd] = FPR[fs] * FPR[ft]
        // FPR[9] = FPR[5] * FPR[4]
        //                                  COP1   fmt   ft    fs    fd    function
        //                                         s     $f4   $f5   $f9   MUL
        let instructions: Vec<u32> = vec![0b010001_10000_00100_00101_01001_000010];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[5] = f32::to_bits(24.5f32) as u64;
        datapath.coprocessor.fpr[4] = f32::to_bits(0.5f32) as u64;

        datapath.execute_instruction();

        assert_eq!(f32::from_bits(datapath.coprocessor.fpr[9] as u32), 12.25f32);
        Ok(())
    }

    #[test]
    pub fn mul_float_double_precision() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // mul.d fd, fs, ft
        // mul.d $f4, $f6, $f9
        // FPR[fd] = FPR[fs] * FPR[ft]
        // FPR[4] = FPR[6] * FPR[9]
        //                                  COP1   fmt   ft    fs    fd    function
        //                                         d     $f9   $f6   $f4   MUL
        let instructions: Vec<u32> = vec![0b010001_10001_01001_00110_00100_000010];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[6] = f64::to_bits(-150.0625);
        datapath.coprocessor.fpr[9] = f64::to_bits(9.5);

        datapath.execute_instruction();

        assert_eq!(f64::from_bits(datapath.coprocessor.fpr[4]), -1425.59375);
        Ok(())
    }

    #[test]
    pub fn div_float_single_precision() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // div.s fd, fs, ft
        // div.s $f15, $f16, $f17
        // FPR[fd] = FPR[fs] / FPR[ft]
        // FPR[15] = FPR[16] / FPR[17]
        //                                  COP1   fmt   ft    fs    fd    function
        //                                         s     $f17  $f16  $f15  DIV
        let instructions: Vec<u32> = vec![0b010001_10000_10001_10000_01111_000011];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[16] = f32::to_bits(901f32) as u64;
        datapath.coprocessor.fpr[17] = f32::to_bits(2f32) as u64;

        datapath.execute_instruction();

        assert_eq!(
            f32::from_bits(datapath.coprocessor.fpr[15] as u32),
            450.5f32
        );
        Ok(())
    }

    #[test]
    pub fn div_float_double_precision() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // div.d fd, fs, ft
        // div.d $f1, $f10, $f20
        // FPR[fd] = FPR[fs] / FPR[ft]
        // FPR[1] = FPR[10] / FPR[20]
        //                                  COP1   fmt   ft    fs    fd    function
        //                                         d     $f20  $f10  $f1   DIV
        let instructions: Vec<u32> = vec![0b010001_10001_10100_01010_00001_000011];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[10] = f64::to_bits(95405.375);
        datapath.coprocessor.fpr[20] = f64::to_bits(2.0);

        datapath.execute_instruction();

        assert_eq!(f64::from_bits(datapath.coprocessor.fpr[1]), 47702.6875);
        Ok(())
    }

    #[test]
    pub fn swc1_basic_store_no_offset() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // swc1 ft, offset(base)
        // swc1 $f3, 0($s1)
        // memory[GPR[base] + offset] <- FPR[ft]
        // memory[GPR[17] + 0] <- FPR[3]
        //                                  SWC1   base  ft    offset
        //                                         $s1   $f3   0
        let instructions: Vec<u32> = vec![0b111001_10001_00011_0000000000000000];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[17] = 1028; // $s1
        datapath.coprocessor.fpr[3] = f32::to_bits(1.0625f32) as u64;

        datapath.execute_instruction();

        // The single-precision float 1.0625 should be stored at address 1028.
        assert_eq!(
            f32::from_bits(datapath.memory.load_word(1028).unwrap()),
            1.0625f32
        );
        Ok(())
    }

    #[test]
    pub fn swc1_basic_store_with_offset() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // swc1 ft, offset(base)
        // swc1 $f5, 32($s0)
        // memory[GPR[base] + offset] <- FPR[ft]
        // memory[GPR[16] + 32] <- FPR[5]
        //                                  SWC1   base  ft    offset
        //                                         $s0   $f5   32
        let instructions: Vec<u32> = vec![0b111001_10000_00101_0000000000100000];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[16] = 2000; // $s0
        datapath.coprocessor.fpr[5] = f32::to_bits(3.5f32) as u64;

        datapath.execute_instruction();

        // The single-precision float 3.5 should be stored at address 2032.
        assert_eq!(
            f32::from_bits(datapath.memory.load_word(2032).unwrap()),
            3.5f32
        );
        Ok(())
    }

    #[test]
    pub fn swc1_basic_store_64_bit_cutoff() -> Result<(), String> {
        // This test ensures that if there is 64-bit data in a floating-point
        // register, only the bottom 32 bits are stored in memory with this
        // instruction.

        let mut datapath = MipsDatapath::default();

        // swc1 ft, offset(base)
        // swc1 $f0, 0($s2)
        // memory[GPR[base] + offset] <- FPR[ft]
        // memory[GPR[18] + 0] <- FPR[0]
        //                                  SWC1   base  ft    offset
        //                                         $s2   $f0   0
        let instructions: Vec<u32> = vec![0b111001_10010_00000_0000000000000000];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[18] = 1000; // $s2
        datapath.coprocessor.fpr[0] = f64::to_bits(9853114.625);

        datapath.execute_instruction();

        // The double-precision float 9853114.625 is represented in hexadecimal as
        // 4162 CB17 5400 0000. When storing the 32-bit word, the bottom 32 bits
        // should be stored, in this case meaning 5400 0000 in hexadecimal.
        assert_eq!(datapath.memory.load_word(1000).unwrap(), 0x5400_0000);
        Ok(())
    }

    #[test]
    fn lwc1_basic_load_no_offset() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // lwc1 ft, offset(base)
        // lwc1 $f10, 0($t0)
        // FPR[ft] <- memory[GPR[base] + offset]
        // FPR[10] <- memory[GPR[8] + 0]
        //                                  LWC1   base  ft    offset
        //                                         $t0   $f10  0
        let instructions: Vec<u32> = vec![0b110001_01000_01010_0000000000000000];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[8] = 500; // $t0

        // Data is put into memory this way (rather than using load_word()) to
        // demonstrate no reliance on API calls.
        let data = f32::to_bits(413.125f32).to_be_bytes();
        for (i, byte) in data.iter().enumerate() {
            datapath.memory.memory[500 + i] = *byte;
        }

        datapath.execute_instruction();

        assert_eq!(
            f32::from_bits(datapath.coprocessor.fpr[10] as u32),
            413.125f32
        );
        Ok(())
    }

    #[test]
    fn lwc1_basic_load_with_offset() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // lwc1 ft, offset(base)
        // lwc1 $f11, 200($t1)
        // FPR[ft] <- memory[GPR[base] + offset]
        // FPR[11] <- memory[GPR[9] + 200]
        //                                  LWC1   base  ft    offset
        //                                         $t1   $f11  200
        let instructions: Vec<u32> = vec![0b110001_01001_01011_0000000011001000];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[9] = 1000; // $t1

        // Data is put into memory this way (rather than using load_word()) to
        // demonstrate no reliance on API calls.
        let data = f32::to_bits(6.1875f32).to_be_bytes();
        for (i, byte) in data.iter().enumerate() {
            datapath.memory.memory[1200 + i] = *byte;
        }

        datapath.execute_instruction();

        assert_eq!(
            f32::from_bits(datapath.coprocessor.fpr[11] as u32),
            6.1875f32
        );
        Ok(())
    }

    #[test]
    fn c_eq_s_should_be_true() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // c.eq.s fs, ft
        // c.eq.s $f1, $f2
        // CC[0] <- FPR[fs] == FPR[ft]
        // CC[0] <- FPR[1] == FPR[2]
        //                                  COP1   fmt   ft    fs    cc     __cond
        //                                         s     $f2   $f1   0        EQ
        let instructions: Vec<u32> = vec![0b010001_10000_00010_00001_000_00_110010];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[1] = f32::to_bits(15.5f32) as u64;
        datapath.coprocessor.fpr[2] = f32::to_bits(15.5f32) as u64;

        datapath.execute_instruction();

        assert_eq!(datapath.coprocessor.condition_code, 1);

        Ok(())
    }

    #[test]
    fn c_eq_s_should_be_false() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // c.eq.s fs, ft
        // c.eq.s $f14, $f3
        // CC[0] <- FPR[fs] == FPR[ft]
        // CC[0] <- FPR[14] == FPR[3]
        //                                  COP1   fmt   ft    fs    cc     __cond
        //                                         s     $f3   $f14  0        EQ
        let instructions: Vec<u32> = vec![0b010001_10000_00011_01110_000_00_110010];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[14] = f32::to_bits(20.125f32) as u64;
        datapath.coprocessor.fpr[3] = f32::to_bits(100f32) as u64;

        datapath.execute_instruction();

        assert_eq!(datapath.coprocessor.condition_code, 0);

        Ok(())
    }

    #[test]
    fn c_eq_d_should_be_true() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // c.eq.d fs, ft
        // c.eq.d $f5, $f9
        // CC[0] <- FPR[fs] == FPR[ft]
        // CC[0] <- FPR[5] == FPR[9]
        //                                  COP1   fmt   ft    fs    cc     __cond
        //                                         d     $f9   $f5   0        EQ
        let instructions: Vec<u32> = vec![0b010001_10001_01001_00101_000_00_110010];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[5] = f64::to_bits(12951.625);
        datapath.coprocessor.fpr[9] = f64::to_bits(12951.625);

        datapath.execute_instruction();

        assert_eq!(datapath.coprocessor.condition_code, 1);

        Ok(())
    }

    #[test]
    fn c_eq_d_should_be_false() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // c.eq.d fs, ft
        // c.eq.d $f15, $f19
        // CC[0] <- FPR[fs] == FPR[ft]
        // CC[0] <- FPR[15] == FPR[19]
        //                                  COP1   fmt   ft    fs    cc     __cond
        //                                         d     $f19  $f15  0        EQ
        let instructions: Vec<u32> = vec![0b010001_10001_10011_01111_000_00_110010];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[15] = f64::to_bits(6016.25);
        datapath.coprocessor.fpr[19] = f64::to_bits(820.43);

        datapath.execute_instruction();

        assert_eq!(datapath.coprocessor.condition_code, 0);

        Ok(())
    }

    #[test]
    fn c_lt_s_should_be_true() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // c.lt.s fs, ft
        // c.lt.s $f19, $f0
        // CC[0] <- FPR[fs] < FPR[ft]
        // CC[0] <- FPR[19] < FPR[0]
        //                                  COP1   fmt   ft    fs    cc     __cond
        //                                         s     $f0   $f19  0        LT
        let instructions: Vec<u32> = vec![0b010001_10000_00000_10011_000_00_111100];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[19] = f32::to_bits(2.875f32) as u64;
        datapath.coprocessor.fpr[0] = f32::to_bits(70.6f32) as u64;

        datapath.execute_instruction();

        assert_eq!(datapath.coprocessor.condition_code, 1);

        Ok(())
    }

    #[test]
    fn c_lt_s_should_be_false() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // c.lt.s fs, ft
        // c.lt.s $f30, $f31
        // CC[0] <- FPR[fs] < FPR[ft]
        // CC[0] <- FPR[30] < FPR[31]
        //                                  COP1   fmt   ft    fs    cc     __cond
        //                                         s     $f31  $f30  0        LT
        let instructions: Vec<u32> = vec![0b010001_10000_11111_11110_000_00_111100];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[30] = f32::to_bits(90.7f32) as u64;
        datapath.coprocessor.fpr[31] = f32::to_bits(-87.44f32) as u64;

        datapath.execute_instruction();

        assert_eq!(datapath.coprocessor.condition_code, 0);

        Ok(())
    }

    #[test]
    fn c_lt_d_should_be_true() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // c.lt.d fs, ft
        // c.lt.d $f12, $f29
        // CC[0] <- FPR[fs] < FPR[ft]
        // CC[0] <- FPR[12] < FPR[29]
        //                                  COP1   fmt   ft    fs    cc     __cond
        //                                         d     $f29  $f12  0        LT
        let instructions: Vec<u32> = vec![0b010001_10001_11101_01100_000_00_111100];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[12] = f64::to_bits(4.0);
        datapath.coprocessor.fpr[29] = f64::to_bits(30000.6);

        datapath.execute_instruction();

        assert_eq!(datapath.coprocessor.condition_code, 1);

        Ok(())
    }

    #[test]
    fn c_lt_d_should_be_false() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // c.lt.d fs, ft
        // c.lt.d $f4, $f5
        // CC[0] <- FPR[fs] < FPR[ft]
        // CC[0] <- FPR[4] < FPR[5]
        //                                  COP1   fmt   ft    fs    cc     __cond
        //                                         d     $f5   $f4  0        LT
        let instructions: Vec<u32> = vec![0b010001_10001_00101_00100_000_00_111100];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[4] = f64::to_bits(413.420);
        datapath.coprocessor.fpr[5] = f64::to_bits(-6600.9);

        datapath.execute_instruction();

        assert_eq!(datapath.coprocessor.condition_code, 0);

        Ok(())
    }

    #[test]
    fn c_le_s_should_be_true_less() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // c.le.s fs, ft
        // c.le.s $f0, $f1
        // CC[0] <- FPR[fs] <= FPR[ft]
        // CC[0] <- FPR[0] <= FPR[1]
        //                                  COP1   fmt   ft    fs    cc     __cond
        //                                         s     $f1   $f0   0        LE
        let instructions: Vec<u32> = vec![0b010001_10000_00001_00000_000_00_111110];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[0] = f32::to_bits(171.937f32) as u64;
        datapath.coprocessor.fpr[1] = f32::to_bits(9930.829f32) as u64;

        datapath.execute_instruction();

        assert_eq!(datapath.coprocessor.condition_code, 1);

        Ok(())
    }

    #[test]
    fn c_le_s_should_be_true_equal() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // c.le.s fs, ft
        // c.le.s $f2, $f3
        // CC[0] <- FPR[fs] <= FPR[ft]
        // CC[0] <- FPR[2] <= FPR[3]
        //                                  COP1   fmt   ft    fs    cc     __cond
        //                                         s     $f3   $f2   0        LE
        let instructions: Vec<u32> = vec![0b010001_10000_00011_00010_000_00_111110];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[2] = f32::to_bits(6.5f32) as u64;
        datapath.coprocessor.fpr[3] = f32::to_bits(6.5f32) as u64;

        datapath.execute_instruction();

        assert_eq!(datapath.coprocessor.condition_code, 1);

        Ok(())
    }

    #[test]
    fn c_le_s_should_be_false() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // c.le.s fs, ft
        // c.le.s $f4, $f5
        // CC[0] <- FPR[fs] <= FPR[ft]
        // CC[0] <- FPR[4] <= FPR[5]
        //                                  COP1   fmt   ft    fs    cc     __cond
        //                                         s     $f5   $f4   0        LE
        let instructions: Vec<u32> = vec![0b010001_10000_00101_00100_000_00_111110];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[4] = f32::to_bits(5742.006f32) as u64;
        datapath.coprocessor.fpr[5] = f32::to_bits(1336.568f32) as u64;

        datapath.execute_instruction();

        assert_eq!(datapath.coprocessor.condition_code, 0);

        Ok(())
    }

    #[test]
    fn c_le_d_should_be_true_less() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // c.le.d fs, ft
        // c.le.d $f6, $f7
        // CC[0] <- FPR[fs] <= FPR[ft]
        // CC[0] <- FPR[6] <= FPR[7]
        //                                  COP1   fmt   ft    fs    cc     __cond
        //                                         d     $f7   $f6   0        LE
        let instructions: Vec<u32> = vec![0b010001_10001_00111_00110_000_00_111110];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[6] = f64::to_bits(3483.70216);
        datapath.coprocessor.fpr[7] = f64::to_bits(7201.56625);

        datapath.execute_instruction();

        assert_eq!(datapath.coprocessor.condition_code, 1);

        Ok(())
    }

    #[test]
    fn c_le_d_should_be_true_equal() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // c.le.d fs, ft
        // c.le.d $f8, $f9
        // CC[0] <- FPR[fs] <= FPR[ft]
        // CC[0] <- FPR[8] <= FPR[9]
        //                                  COP1   fmt   ft    fs    cc     __cond
        //                                         d     $f9   $f8   0        LE
        let instructions: Vec<u32> = vec![0b010001_10001_01001_01000_000_00_111110];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[8] = f64::to_bits(77.4009);
        datapath.coprocessor.fpr[9] = f64::to_bits(77.4009);

        datapath.execute_instruction();

        assert_eq!(datapath.coprocessor.condition_code, 1);

        Ok(())
    }

    #[test]
    fn c_le_d_should_be_false() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // c.le.d fs, ft
        // c.le.d $f10, $f11
        // CC[0] <- FPR[fs] <= FPR[ft]
        // CC[0] <- FPR[10] <= FPR[11]
        //                                  COP1   fmt   ft    fs    cc     __cond
        //                                         d     $f11  $f10  0        LE
        let instructions: Vec<u32> = vec![0b010001_10001_01011_01010_000_00_111110];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[10] = f64::to_bits(9190.43309);
        datapath.coprocessor.fpr[11] = f64::to_bits(2869.57622);

        datapath.execute_instruction();

        assert_eq!(datapath.coprocessor.condition_code, 0);

        Ok(())
    }

    #[test]
    fn c_ngt_s_should_be_true() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // c.ngt.s fs, ft
        // c.ngt.s $f12, $f13
        // CC[0] <- !(FPR[fs] > FPR[ft])
        // CC[0] <- !(FPR[12] > FPR[13])
        //                                  COP1   fmt   ft    fs    cc     __cond
        //                                         s     $f13  $f12  0        NGT
        let instructions: Vec<u32> = vec![0b010001_10000_01101_01100_000_00_111111];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[12] = f32::to_bits(2469.465f32) as u64;
        datapath.coprocessor.fpr[13] = f32::to_bits(3505.57f32) as u64;

        datapath.execute_instruction();

        assert_eq!(datapath.coprocessor.condition_code, 1);

        Ok(())
    }

    #[test]
    fn c_ngt_s_should_be_false() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // c.ngt.s fs, ft
        // c.ngt.s $f14, $f15
        // CC[0] <- !(FPR[fs] > FPR[ft])
        // CC[0] <- !(FPR[14] > FPR[15])
        //                                  COP1   fmt   ft    fs    cc     __cond
        //                                         s     $f15  $f14  0        NGT
        let instructions: Vec<u32> = vec![0b010001_10000_01111_01110_000_00_111111];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[14] = f32::to_bits(7099.472f32) as u64;
        datapath.coprocessor.fpr[15] = f32::to_bits(87.198f32) as u64;

        datapath.execute_instruction();

        assert_eq!(datapath.coprocessor.condition_code, 0);

        Ok(())
    }

    #[test]
    fn c_ngt_d_should_be_true() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // c.ngt.d fs, ft
        // c.ngt.d $f16, $f17
        // CC[0] <- !(FPR[fs] > FPR[ft])
        // CC[0] <- !(FPR[16] > FPR[17])
        //                                  COP1   fmt   ft    fs    cc     __cond
        //                                         d     $f17  $f16  0        NGT
        let instructions: Vec<u32> = vec![0b010001_10001_10001_10000_000_00_111111];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[16] = f64::to_bits(7726.4794015);
        datapath.coprocessor.fpr[17] = f64::to_bits(9345.7753943);

        datapath.execute_instruction();

        assert_eq!(datapath.coprocessor.condition_code, 1);

        Ok(())
    }

    #[test]
    fn c_ngt_d_should_be_false() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // c.ngt.d fs, ft
        // c.ngt.d $f18, $f19
        // CC[0] <- !(FPR[fs] > FPR[ft])
        // CC[0] <- !(FPR[18] > FPR[19])
        //                                  COP1   fmt   ft    fs    cc     __cond
        //                                         d     $f19  $f18  0        NGT
        let instructions: Vec<u32> = vec![0b010001_10001_10011_10010_000_00_111111];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[18] = f64::to_bits(4688.2854359);
        datapath.coprocessor.fpr[19] = f64::to_bits(819.7956308);

        datapath.execute_instruction();

        assert_eq!(datapath.coprocessor.condition_code, 0);

        Ok(())
    }

    #[test]
    fn c_nge_s_should_be_true() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // c.nge.s fs, ft
        // c.nge.s $f20, $f21
        // CC[0] <- !(FPR[fs] >= FPR[ft])
        // CC[0] <- !(FPR[20] >= FPR[21])
        //                                  COP1   fmt   ft    fs    cc     __cond
        //                                         s     $f21  $f20  0        NGE
        let instructions: Vec<u32> = vec![0b010001_10000_10101_10100_000_00_111101];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[20] = f32::to_bits(3090.244f32) as u64;
        datapath.coprocessor.fpr[21] = f32::to_bits(7396.444f32) as u64;

        datapath.execute_instruction();

        assert_eq!(datapath.coprocessor.condition_code, 1);

        Ok(())
    }

    #[test]
    fn c_nge_s_should_be_false() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // c.nge.s fs, ft
        // c.nge.s $f22, $f23
        // CC[0] <- !(FPR[fs] >= FPR[ft])
        // CC[0] <- !(FPR[22] >= FPR[23])
        //                                  COP1   fmt   ft    fs    cc     __cond
        //                                         s     $f23  $f22  0        NGE
        let instructions: Vec<u32> = vec![0b010001_10000_10111_10110_000_00_111101];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[22] = f32::to_bits(6269.823f32) as u64;
        datapath.coprocessor.fpr[23] = f32::to_bits(3089.393f32) as u64;

        datapath.execute_instruction();

        assert_eq!(datapath.coprocessor.condition_code, 0);

        Ok(())
    }

    #[test]
    fn c_nge_d_should_be_true() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // c.nge.d fs, ft
        // c.nge.d $f24, $f25
        // CC[0] <- !(FPR[fs] >= FPR[ft])
        // CC[0] <- !(FPR[24] >= FPR[25])
        //                                  COP1   fmt   ft    fs    cc     __cond
        //                                         d     $f25  $f24  0        NGE
        let instructions: Vec<u32> = vec![0b010001_10001_11001_11000_000_00_111101];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[24] = f64::to_bits(819.7956308);
        datapath.coprocessor.fpr[25] = f64::to_bits(4688.2854359);

        datapath.execute_instruction();

        assert_eq!(datapath.coprocessor.condition_code, 1);

        Ok(())
    }

    #[test]
    fn c_nge_d_should_be_false() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // c.nge.d fs, ft
        // c.nge.d $f26, $f27
        // CC[0] <- !(FPR[fs] >= FPR[ft])
        // CC[0] <- !(FPR[26] >= FPR[27])
        //                                  COP1   fmt   ft    fs    cc     __cond
        //                                         d     $f27  $f26  0        NGE
        let instructions: Vec<u32> = vec![0b010001_10001_11011_11010_000_00_111101];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[26] = f64::to_bits(9776.3465875);
        datapath.coprocessor.fpr[27] = f64::to_bits(1549.8268716);

        datapath.execute_instruction();

        assert_eq!(datapath.coprocessor.condition_code, 0);

        Ok(())
    }

    #[test]
    fn mtc1_basic_move() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // mtc1 rt, fs
        // mtc1 $s0, $f0
        // FPR[fs] <- GPR[rt]
        // FPR[0] <- GPR[16]
        //                                  COP1   sub   rt    fs    0
        //                                         MT    $s0   $f0
        let instructions: Vec<u32> = vec![0b010001_00100_10000_00000_00000000000];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[16] = 25;

        datapath.execute_instruction();

        assert_eq!(datapath.coprocessor.fpr[0], 25);

        Ok(())
    }

    #[test]
    fn mtc1_truncate() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // mtc1 rt, fs
        // mtc1 $s1, $f1
        // FPR[fs] <- GPR[rt]
        // FPR[1] <- GPR[17]
        //                                  COP1   sub   rt    fs    0
        //                                         MT    $s1   $f1
        let instructions: Vec<u32> = vec![0b010001_00100_10001_00001_00000000000];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[17] = 0x1234_5678_ABCD_BEEF;

        datapath.execute_instruction();

        assert_eq!(datapath.coprocessor.fpr[1], 0xABCD_BEEF);

        Ok(())
    }

    #[test]
    fn dmtc1_basic_move() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // dmtc1 rt, fs
        // dmtc1 $t0, $f30
        // FPR[fs] <- GPR[rt]
        // FPR[30] <- GPR[8]
        //                                  COP1   sub   rt    fs    0
        //                                         DMT   $t0   $f30
        let instructions: Vec<u32> = vec![0b010001_00101_01000_11110_00000000000];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[8] = 0xDEAD_BEEF_FEED_DEED;

        datapath.execute_instruction();

        assert_eq!(datapath.coprocessor.fpr[30], 0xDEAD_BEEF_FEED_DEED);

        Ok(())
    }

    #[test]
    fn mfc1_basic_move() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // mfc1 rt, fs
        // mfc1 $s5, $f18
        // GPR[rt] <- FPR[fs]
        // GPR[21] <- FPR[18]
        //                                  COP1   sub   rt    fs    0
        //                                         MF    $s5   $f18
        let instructions: Vec<u32> = vec![0b010001_00000_10101_10010_00000000000];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[18] = 123;

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[21], 123);

        Ok(())
    }

    #[test]
    fn mfc1_truncate() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // mfc1 rt, fs
        // mfc1 $s6, $f19
        // GPR[rt] <- FPR[fs]
        // GPR[22] <- FPR[19]
        //                                  COP1   sub   rt    fs    0
        //                                         MF    $s6   $f19
        let instructions: Vec<u32> = vec![0b010001_00000_10110_10011_00000000000];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[19] = 0xABBA_BABB_3ABA_4444;

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[22], 0x3ABA_4444);

        Ok(())
    }

    #[test]
    fn mfc1_sign_extend() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // mfc1 rt, fs
        // mfc1 $s7, $f20
        // GPR[rt] <- FPR[fs]
        // GPR[23] <- FPR[20]
        //                                  COP1   sub   rt    fs    0
        //                                         MF    $s7   $f20
        let instructions: Vec<u32> = vec![0b010001_00000_10111_10100_00000000000];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[20] = 0xBADA_BEEF_BADA_B00E;

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[23], 0xFFFF_FFFF_BADA_B00E);

        Ok(())
    }

    #[test]
    fn dmfc1_basic_move() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // dmfc1 rt, fs
        // dmfc1 $t8, $f21
        // GPR[rt] <- FPR[fs]
        // GPR[24] <- FPR[21]
        //                                  COP1   sub   rt    fs    0
        //                                         DMF   $t8   $f21
        let instructions: Vec<u32> = vec![0b010001_00001_11000_10101_00000000000];
        datapath.initialize(instructions)?;

        datapath.coprocessor.fpr[21] = 0xADDA_DADD_1BAA_CAFE;

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[24], 0xADDA_DADD_1BAA_CAFE);

        Ok(())
    }
}

pub mod jump_tests {
    use super::*;
    #[test]
    fn jump_test_basic() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        //                                  J
        let instructions: Vec<u32> = vec![0b000010_00_00000000_00000000_00000010];
        datapath.initialize(instructions)?;
        datapath.execute_instruction();

        assert_eq!(datapath.registers.pc, 8);
        Ok(())
    }

    #[test]
    fn jump_test_mid() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        //                                  J
        let instructions: Vec<u32> = vec![0x0800_0fff];
        datapath.initialize(instructions)?;
        datapath.execute_instruction();

        assert_eq!(datapath.registers.pc, 0x3ffc);
        Ok(())
    }

    #[test]
    fn jump_test_hard() -> Result<(), String> {
        // Jump to address 0xfff_fffc
        let mut datapath = MipsDatapath::default();

        //                                  J             low_26
        let instructions: Vec<u32> = vec![0x0800_0000 | 0x03ff_ffff];
        datapath.initialize(instructions)?;
        datapath.execute_instruction();

        assert_eq!(datapath.registers.pc, 0x0fff_fffc);
        Ok(())
    }
}

pub mod jump_and_link_tests {
    use super::*;
    #[test]
    fn test_basic() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();
        let old_pc = datapath.registers.pc;

        //                                  J
        let instructions: Vec<u32> = vec![0b000011_00_00000000_00000000_00000010];
        datapath.initialize(instructions)?;
        datapath.execute_instruction();

        assert_eq!(datapath.registers.pc, 8);
        assert_eq!(datapath.registers.gpr[31], old_pc + 4);
        Ok(())
    }

    #[test]
    fn test_mid() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();
        let old_pc = datapath.registers.pc;

        //                                  J
        let instructions: Vec<u32> = vec![0x0c00_0fff];
        datapath.initialize(instructions)?;
        datapath.execute_instruction();

        assert_eq!(datapath.registers.pc, 0x3ffc);
        assert_eq!(datapath.registers.gpr[31], old_pc + 4);
        Ok(())
    }

    #[test]
    fn test_hard() -> Result<(), String> {
        // Jump to address 0xfff_fffc
        let mut datapath = MipsDatapath::default();
        let old_pc = datapath.registers.pc;

        //                                  J             low_26
        let instructions: Vec<u32> = vec![0x0c00_0000 | 0x03ff_ffff];
        datapath.initialize(instructions)?;
        datapath.execute_instruction();

        assert_eq!(datapath.registers.pc, 0x0fff_fffc);
        assert_eq!(datapath.registers.gpr[31], old_pc + 4);
        Ok(())
    }
}

pub mod jr_and_jalr_tests {
    use super::*;
    #[test]
    fn test_basic_jr() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // JR $r8
        //                                  Special $r8  $zero $zero        JALR
        let instructions: Vec<u32> = vec![0b000000_01000_00000_00000_00000_001001];
        datapath.initialize(instructions)?;
        datapath.registers.gpr[0b01000] = 24;
        datapath.execute_instruction();

        assert_eq!(datapath.registers.pc, 24);
        assert_eq!(datapath.registers.gpr[8], 24);
        Ok(())
    }

    #[test]
    fn test_basic_jalr() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        // JALR $r8
        //                                     Special  $r8  $zero $ra          JALR
        let instructions: Vec<u32> = vec![0, 0, 0b000000_01000_00000_11111_00000_001001];
        datapath.initialize(instructions)?;
        datapath.registers.pc = 8;
        let initial_pc = datapath.registers.pc;
        datapath.registers.gpr[0b01000] = 24;
        datapath.execute_instruction();

        assert_eq!(datapath.registers.pc, 24);
        assert_eq!(datapath.registers.gpr[31], initial_pc + 4);
        Ok(())
    }
}

pub mod beq_tests {
    use super::*;
    #[test]
    fn beq_test_basic_registers_are_equal() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        //                                  beq
        let instructions: Vec<u32> = vec![0b000100_01000_10000_0000000000000001];
        datapath.initialize(instructions)?;

        let initial_pc = datapath.registers.pc;
        datapath.execute_instruction();
        let expt_result = (0b0000_0000_0000_0001 << 2) + initial_pc + 4;
        assert_eq!(datapath.registers.pc, expt_result);
        Ok(())
    }

    #[test]
    fn beq_test_basic_register_are_not_equal() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        //                                  beq
        let instructions: Vec<u32> = vec![
            0b000100_01000_10000_0000000000000001,
            0b000100_01000_10000_0000000000000001,
        ];
        datapath.initialize(instructions)?;

        datapath.registers.gpr[0b01000] = 1234;
        datapath.registers.gpr[0b10000] = 4321;

        datapath.execute_instruction();
        assert_eq!(datapath.registers.pc, 4);

        datapath.execute_instruction();
        assert_eq!(datapath.registers.pc, 8);
        Ok(())
    }

    #[test]
    fn beq_test_basic_branch_backwards() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        let instructions: Vec<u32> = vec![
            0b000100_01000_10000_0000000000000011, // 0x00, Branch to 0x10
            0,                                     // 0x04
            0,                                     // 0x08
            0,                                     // 0x0c
            0b000100_01000_10000_1111111111111011, // 0x10, Branch to 0x00
        ];
        datapath.initialize(instructions)?;
        datapath.registers.gpr[0b01000] = 1234;
        datapath.registers.gpr[0b10000] = 1234;

        let initial_pc = datapath.registers.pc;
        let offset = 0b0000_0000_0000_0011;
        // 0x10, aka 16
        let expt_result = ((offset as i16 as i64 as u64) << 2)
            .wrapping_add(initial_pc)
            .wrapping_add(4);

        datapath.execute_instruction(); // branch to address 16 from address 0
        assert_eq!(datapath.registers.pc, expt_result);
        assert_eq!(datapath.registers.gpr[0b01000], 1234);
        assert_eq!(datapath.registers.gpr[0b10000], 1234);

        let initial_pc = datapath.registers.pc;
        let offset = 0b1111_1111_1111_1011;
        // 0x00
        let expt_result = ((offset as i16 as i64 as u64) << 2)
            .wrapping_add(initial_pc)
            .wrapping_add(4);

        datapath.execute_instruction();
        assert_eq!(datapath.registers.pc, expt_result);

        // Some loop stuff:
        datapath.execute_instruction(); // Branch to 0x10
        datapath.execute_instruction(); // Branch to 0x00
        datapath.execute_instruction(); // Branch to 0x10
        datapath.execute_instruction(); // Branch to 0x00
        datapath.execute_instruction(); // Branch to 0x10
        datapath.execute_instruction(); // Branch to 0x00
        assert_eq!(datapath.registers.pc, expt_result);
        Ok(())
    }
}

pub mod bne_tests {
    use super::*;
    #[test]
    fn bne_test_basic_registers_are_equal() -> Result<(), String> {
        // There should be no branching, the rs and rt are equal

        let mut datapath = MipsDatapath::default();
        //                                  bne                         1 word
        let instructions: Vec<u32> = vec![0b000101_01000_10000_0000000000000001];
        datapath.registers.gpr[0b01000] = 1234;
        datapath.registers.gpr[0b10000] = 1234;
        datapath.initialize(instructions)?;
        datapath.execute_instruction();
        let expt_result = 4; // PC + 4, PC starts at 0 with the bne instruction at address 0, no branch acures
        assert_eq!(datapath.registers.pc, expt_result);

        Ok(())
    }

    #[test]
    fn bne_test_loop() -> Result<(), String> {
        // This test starts with Branching from 0x0 to 0x8.
        // then from 0x8, at branch to 0x20.
        // then from 0x20 back to 0x8.
        // then from 0x8 to 0x20
        // backcally we have a loop of branching forever

        let mut datapath = MipsDatapath::default();
        let instructions: Vec<u32> = vec![
            0b000101_01000_10000_0000000000000001, // 0x00, Branch to 0x8
            0,                                     // 0x04
            0b000101_01000_10000_0000000000000101, // 0x08, Branch to 0x20
            0,                                     // 0x0c
            0,                                     // 0x10
            0,                                     // 0x14
            0,                                     // 0x18
            0,                                     // 0x1c
            0b000101_01000_10000_1111111111111001, // 0x20, bne r8, r16, -24, (branch -28 relative to next addres), branch to 0x08
        ];
        datapath.initialize(instructions)?;
        datapath.registers.gpr[0b01000] = 1234;
        datapath.registers.gpr[0b10000] = 4321;

        // test beq going from pc = 0 to next_pc + 4, 0x0 to 0x8
        datapath.execute_instruction();
        assert_eq!(datapath.registers.pc, 8);

        // Branch from 0x8 to 0x20, aka from 8 to 32, branch by 24
        let initial_pc = datapath.registers.pc;
        datapath.execute_instruction();
        let expt_result = (0b0000000000000101 << 2) + initial_pc + 4; // 32
        assert_eq!(datapath.registers.pc, expt_result);

        // Branch back to 0x8 from 0x20, aka 32 to 8
        // The next_pc after 32 it 36, thus our branch offset will be 8 - 36 = -28
        //
        // destination_addr = SOME_LABEL
        // Branch offset = (destination_addr - next_pc)
        let initial_pc = datapath.registers.pc;
        let offset = 0b1111_1111_1111_1001; // -28
                                            // 0x8
        let expt_result = ((offset as i16 as i64 as u64) << 2)
            .wrapping_add(initial_pc)
            .wrapping_add(4);

        datapath.execute_instruction(); // branch to 0x08
        assert_eq!(datapath.registers.pc as i64, expt_result as i64);

        // loop around a few times
        datapath.execute_instruction(); // branch to 0x20
        datapath.execute_instruction(); // branch to 0x08
        datapath.execute_instruction(); // branch to 0x20
        datapath.execute_instruction(); // branch to 0x08
        assert_eq!(datapath.registers.pc as i64, expt_result as i64);

        Ok(())
    }
}

pub mod syscall {
    use super::*;

    #[test]
    fn halts_on_syscall() -> Result<(), String> {
        let mut datapath = MipsDatapath::default();

        assert!(datapath.is_halted());

        // This program doubles the value in $t1 and stops.

        let instructions: Vec<u32> = vec![
            // $t1 = $t1 + $t1
            // SPECIAL t1   t1    t1  (shamt)  ADD
            0b000000_01001_01001_01001_00000_100000,
            // syscall
            // SPECIAL     (code)        SYSCALL
            0b000000_00000000000000000000_001100,
        ];
        datapath.initialize(instructions)?;
        assert!(!datapath.is_halted());

        datapath.registers.gpr[9] = 5; // $t1

        // Execute 2 instructions.
        for _ in 0..2 {
            datapath.execute_instruction();
        }

        assert_eq!(datapath.registers.gpr[9], 10); // $t1
        assert!(datapath.is_halted());
        Ok(())
    }
}
