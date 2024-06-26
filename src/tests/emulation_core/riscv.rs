#![allow(clippy::unusual_byte_groupings)]

use crate::emulation_core::datapath::Datapath;
use crate::emulation_core::riscv::datapath::RiscDatapath;
use crate::emulation_core::riscv::registers::RiscGpRegisterType;

pub mod api {
    use super::*;
    use crate::{
        emulation_core::architectures::AvailableDatapaths, parser::parser_assembler_main::parser,
    };

    #[test]
    fn reset_datapath() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // Add instruction into emulation core memory.
        let instruction = String::from("ori s0, zero, 5");
        let (_, instruction_bits, _labels) = parser(instruction, AvailableDatapaths::RISCV);
        datapath.initialize(0, instruction_bits)?;

        datapath.execute_instruction();

        // Datapath should now have some data in it.
        assert_ne!(datapath.registers.gpr[8], 0); // $s0
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
        let mut datapath = RiscDatapath::default();

        // $t1 = $t1 + $t1
        let instructions: Vec<u32> = vec![0b0000000_00110_00110_000_00110_0110011];
        datapath.initialize(0, instructions)?;

        // Assume the register $t1 has the value 5.
        datapath.registers[RiscGpRegisterType::X6] = 5;

        datapath.execute_instruction();

        // After the operation is finished, the register should be 10.
        assert_eq!(datapath.registers[RiscGpRegisterType::X6], 10);
        Ok(())
    }

    #[test]
    fn add_register_to_another() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // $s2 = $s0 + $s1
        let instructions: Vec<u32> = vec![0b0000000_01001_01000_000_10010_0110011];
        datapath.initialize(0, instructions)?;

        datapath.registers.gpr[8] = 15; // $s0
        datapath.registers.gpr[9] = 40; // $s1

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
        let mut datapath = RiscDatapath::default();

        // $zero = $t3 + $t3
        let instructions: Vec<u32> = vec![0b0000000_11100_11100_000_00000_0110011];
        datapath.initialize(0, instructions)?;

        datapath.registers.gpr[28] = 1234; // $t3

        datapath.execute_instruction();

        // $zero should still contain 0.
        assert_eq!(datapath.registers.gpr[0], 0);
        Ok(())
    }
}

pub mod sub {
    use super::*;

    #[test]
    fn sub_positive_result() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // $s2 = $s3 - $s2
        let instructions: Vec<u32> = vec![0b0100000_10010_10011_000_10010_0110011];
        datapath.initialize(0, instructions)?;

        datapath.registers.gpr[19] = 7; // $s3
        datapath.registers.gpr[18] = 3; // $s2

        datapath.execute_instruction();

        // Register $s2 should contain 4, as 7 - 3 = 4.
        assert_eq!(datapath.registers.gpr[18], 4);
        Ok(())
    }

    #[test]
    fn sub_negative_result() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // $s2 = $s3 - $s2
        let instructions: Vec<u32> = vec![0b0100000_10010_10011_000_10010_0110011];
        datapath.initialize(0, instructions)?;

        datapath.registers.gpr[19] = 3; // $s3
        datapath.registers.gpr[18] = 7; // $s2

        datapath.execute_instruction();

        // Register $s2 should contain 4, as 3 - 7 = -4.
        assert_eq!(datapath.registers.gpr[18] as i64, -4);
        Ok(())
    }
}

pub mod or {
    use super::*;

    #[test]
    fn or_register_to_itself() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // $t1 = $t1 | $t1
        let instructions: Vec<u32> = vec![0b0000000_00110_00110_110_00110_0110011];
        datapath.initialize(0, instructions)?;

        // Assume the register $t1 has the value 5.
        datapath.registers[RiscGpRegisterType::X6] = 0x5;

        datapath.execute_instruction();
        assert_eq!(datapath.registers[RiscGpRegisterType::X6], 0x5);
        Ok(())
    }

    #[test]
    fn or_register_to_another() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // $s2 = $s0 | $s1
        let instructions: Vec<u32> = vec![0b0000000_01001_01000_110_10010_0110011];
        datapath.initialize(0, instructions)?;

        datapath.registers.gpr[8] = 0x1234; // $s0
        datapath.registers.gpr[9] = 0x4321; // $s1

        datapath.execute_instruction();

        // Register $s2 should contain 55.
        let result = datapath.registers.gpr[18];
        assert_eq!(result, 0x5335);
        Ok(())
    }

    #[test]
    // This test attempts to write to register $zero. The datapath should
    // not overwrite this register, and remain with a value of 0.
    fn or_to_register_zero() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // $zero = $t3 | $t3
        let instructions: Vec<u32> = vec![0b0000000_11100_11100_110_00000_0110011];
        datapath.initialize(0, instructions)?;

        datapath.registers.gpr[28] = 1234; // $t3

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
        let mut datapath = RiscDatapath::default();

        // $t1 = $t1 & $t1
        let instructions: Vec<u32> = vec![0b0000000_00110_00110_111_00110_0110011];
        datapath.initialize(0, instructions)?;

        // Assume the register $t1 has the value 5.
        datapath.registers[RiscGpRegisterType::X6] = 0x5;

        datapath.execute_instruction();
        assert_eq!(datapath.registers[RiscGpRegisterType::X6], 0x5);
        Ok(())
    }

    #[test]
    fn and_register_to_another() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // $s2 = $s0 & $s1
        let instructions: Vec<u32> = vec![0b0000000_01001_01000_111_10010_0110011];
        datapath.initialize(0, instructions)?;

        datapath.registers.gpr[8] = 0x1234; // $s0
        datapath.registers.gpr[9] = 0x4321; // $s1

        datapath.execute_instruction();

        // Register $s2 should contain 55.
        let result = datapath.registers.gpr[18];
        assert_eq!(result, 0x0220);
        Ok(())
    }

    #[test]
    // This test attempts to write to register $zero. The datapath should
    // not overwrite this register, and remain with a value of 0.
    fn and_to_register_zero() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // $zero = $t3 & $t3
        let instructions: Vec<u32> = vec![0b0000000_11100_11100_111_00000_0110011];
        datapath.initialize(0, instructions)?;

        datapath.registers.gpr[28] = 1234; // $t3

        datapath.execute_instruction();

        // $zero should still contain 0.
        assert_eq!(datapath.registers.gpr[0], 0);
        Ok(())
    }
}

pub mod andi {
    use super::*;
    #[test]
    fn and_immediate_with_zero() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // $s0 = $zero & 1234
        let instructions: Vec<u32> = vec![0b010011010010_00000_111_01000_0010011];
        datapath.initialize(0, instructions)?;

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[8], 0); // $s0
        Ok(())
    }

    #[test]
    fn andi_immediate_with_value() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // $s0 = $t0 & 1234
        let instructions: Vec<u32> = vec![0b010011010010_00101_111_01000_0010011];
        datapath.initialize(0, instructions)?;

        // In binary: 00111010 11011110 01101000 10110001
        datapath.registers.gpr[5] = 987654321; // $t0

        datapath.execute_instruction();

        // The result should be as follows:
        //         $t0:  00111010 11011110 01101000 10110001
        // AND   1,234:                    00000100 11010010
        // =================================================
        //         144:  00000000 00000000 00000000 10010000

        assert_eq!(datapath.registers.gpr[8], 0x90); // $s0
        Ok(())
    }
}

pub mod ori {
    use super::*;
    #[test]
    fn or_immediate_with_zero() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // $s0 = $zero | 1234
        let instructions: Vec<u32> = vec![0b010011010010_00000_110_01000_0010011];
        datapath.initialize(0, instructions)?;

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[8], 1234); // $s0
        Ok(())
    }

    #[test]
    fn or_immediate_with_value() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // $s0 = $t0 | 1234
        let instructions: Vec<u32> = vec![0b010011010010_00101_110_01000_0010011];
        datapath.initialize(0, instructions)?;

        // In binary: 00111010 11011110 01101000 10110001
        datapath.registers.gpr[5] = 987654321; // $t0

        datapath.execute_instruction();

        // The result should be as follows:
        //         $t0:  00111010 11011110 01101000 10110001
        //  OR   1,234:                    00000100 11010010
        // =================================================
        //               00111010 11011110 01101100 11110011

        assert_eq!(datapath.registers.gpr[8], 0x3ade6cf3); // $s0
        Ok(())
    }
}

// Shift Left
pub mod sll {
    use super::*;
    #[test]
    fn easy_test() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        let instructions: Vec<u32> = vec![0b0000000_10010_10001_001_10011_0110011];
        datapath.initialize(0, instructions)?;

        datapath.registers.gpr[0b10001] = 0b101;
        datapath.registers.gpr[0b10010] = 0b1;

        datapath.execute_instruction();
        assert_eq!(datapath.registers.gpr[0b10011], 0b1010);
        Ok(())
    }

    #[test]
    fn harder_test() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // Shift left by two logical
        let instructions: Vec<u32> = vec![0b0000000_10010_10001_001_10011_0110011];
        datapath.initialize(0, instructions)?;

        datapath.registers.gpr[0b10001] = 60;
        datapath.registers.gpr[0b10010] = 3;

        datapath.execute_instruction();
        println!("hmm {:#02x}", datapath.registers.gpr[0b10010]);
        assert_eq!(datapath.registers.gpr[0b10011], 480);
        Ok(())
    }
}

// Shift Right
pub mod sr {
    use super::*;
    #[test]
    fn srl_test() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        let instructions: Vec<u32> = vec![0b0000000_10010_10001_101_10011_0110011];
        datapath.initialize(0, instructions)?;

        datapath.registers.gpr[0b10001] = 360;
        datapath.registers.gpr[0b10010] = 1;

        datapath.execute_instruction();
        assert_eq!(datapath.registers.gpr[0b10011], 180);
        Ok(())
    }

    #[test]
    fn sra_test() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // Shift left by two logical
        let instructions: Vec<u32> = vec![0b0100000_10010_10001_101_10011_0110011];
        datapath.initialize(0, instructions)?;

        datapath.registers.gpr[0b10001] = 0xf00f_0ff0_f0f0_0f0f;
        datapath.registers.gpr[0b10010] = 4;

        datapath.execute_instruction();
        println!("hmm {:#02x}", datapath.registers.gpr[0b10011]);
        assert_eq!(datapath.registers.gpr[0b10011], 0xff00_f0ff_0f0f_00f0);
        Ok(())
    }
}

pub mod slt {
    use super::*;

    #[test]
    fn easy_rs_less_than_rt_test() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // $s2 = $s0 < $s1
        let instructions: Vec<u32> = vec![0b0000000_01001_01000_010_10010_0110011];
        datapath.initialize(0, instructions)?;

        datapath.registers[RiscGpRegisterType::X8] = 1;
        datapath.registers[RiscGpRegisterType::X9] = 123;

        datapath.execute_instruction();

        assert_eq!(datapath.registers[RiscGpRegisterType::X18], 1);
        Ok(())
    }

    #[test]
    fn easy_rs_greater_than_rt_test() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // $s2 = $s0 < $s1
        let instructions: Vec<u32> = vec![0b0000000_01001_01000_010_10010_0110011];
        datapath.initialize(0, instructions)?;

        datapath.registers[RiscGpRegisterType::X8] = 124;
        datapath.registers[RiscGpRegisterType::X9] = 123;

        datapath.execute_instruction();

        assert_eq!(datapath.registers[RiscGpRegisterType::X18], 0);
        Ok(())
    }

    #[test]
    fn easy_signed_test() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // $s2 = $s0 < $s1
        let instructions: Vec<u32> = vec![0b0000000_01001_01000_010_10010_0110011];
        datapath.initialize(0, instructions)?;

        datapath.registers[RiscGpRegisterType::X8] = -124_i64 as u64;
        datapath.registers[RiscGpRegisterType::X9] = 123;

        datapath.execute_instruction();

        assert_eq!(datapath.registers[RiscGpRegisterType::X18], 1);
        Ok(())
    }
}

pub mod sltu {
    use super::*;

    #[test]
    fn easy_rs_less_than_rt_test() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // $s2 = $s0 < $s1
        let instructions: Vec<u32> = vec![0b0000000_01001_01000_011_10010_0110011];
        datapath.initialize(0, instructions)?;

        datapath.registers[RiscGpRegisterType::X8] = 1;
        datapath.registers[RiscGpRegisterType::X9] = 123;

        datapath.execute_instruction();

        assert_eq!(datapath.registers[RiscGpRegisterType::X18], 1);
        Ok(())
    }

    #[test]
    fn easy_rs_greater_than_rt_test() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // $s2 = $s0 < $s1
        let instructions: Vec<u32> = vec![0b0000000_01001_01000_011_10010_0110011];
        datapath.initialize(0, instructions)?;

        datapath.registers[RiscGpRegisterType::X8] = 124;
        datapath.registers[RiscGpRegisterType::X9] = 123;

        datapath.execute_instruction();

        assert_eq!(datapath.registers[RiscGpRegisterType::X18], 0);
        Ok(())
    }

    #[test]
    fn easy_signed_test() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // $s2 = $s0 < $s1
        let instructions: Vec<u32> = vec![0b0000000_01001_01000_011_10010_0110011];
        datapath.initialize(0, instructions)?;

        datapath.registers[RiscGpRegisterType::X8] = -124_i64 as u64;
        datapath.registers[RiscGpRegisterType::X9] = 123;

        datapath.execute_instruction();

        assert_eq!(datapath.registers[RiscGpRegisterType::X18], 0);
        Ok(())
    }
}

pub mod addi_addiu {
    use super::*;
    #[test]
    fn addi_simple_test() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // $s0 = $t0 + 0x4
        let instructions: Vec<u32> = vec![0b000000000100_00101_000_01000_0010011];
        datapath.initialize(0, instructions)?;
        datapath.registers[RiscGpRegisterType::X5] = 1;
        datapath.registers[RiscGpRegisterType::X8] = 123;
        datapath.execute_instruction();

        assert_eq!(datapath.registers[RiscGpRegisterType::X8], 5);
        Ok(())
    }

    #[test]
    fn addi_overflow_test() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // $s0 = $t0 + 0x4
        let instructions: Vec<u32> = vec![0b000000000100_00101_000_01000_0010011];
        datapath.initialize(0, instructions)?;
        datapath.registers[RiscGpRegisterType::X5] = 0xffffffffffffffff;
        datapath.registers[RiscGpRegisterType::X8] = 123;
        datapath.execute_instruction();

        // If there is an overflow on addi, $s0 should not change.
        assert_eq!(datapath.registers[RiscGpRegisterType::X8], 3);
        Ok(())
    }

    #[test]
    fn addi_sign_extend_test() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // $s0 = $t0 + 0x1
        let instructions: Vec<u32> = vec![0b000000000001_00101_000_01000_0010011];
        datapath.initialize(0, instructions)?;
        datapath.registers[RiscGpRegisterType::X5] = 0xfffffffffffffff1;
        datapath.execute_instruction();

        assert_eq!(
            datapath.registers[RiscGpRegisterType::X8],
            0xfffffffffffffff2
        );
        Ok(())
    }
}

pub mod load_word {
    use super::*;
    #[test]
    fn lw_zero_offset_test() -> Result<(), String> {
        // for this test the lw instruction will load itself from
        // memory
        let mut datapath = RiscDatapath::default();

        let instructions: Vec<u32> = vec![0b000000000000_01000_010_10000_0000011];
        datapath.initialize(0, instructions.clone())?;
        datapath.execute_instruction();
        assert_eq!(datapath.registers.gpr[16], instructions[0] as u64);
        Ok(())
    }

    #[test]
    fn lw_offset_at_4_test() -> Result<(), String> {
        // For this test the lw instruction will load 0x4 from memory
        // by using the offset address plus zero
        let mut datapath = RiscDatapath::default();

        let instructions: Vec<u32> = vec![0b000000000100_01000_010_10000_0000011];
        datapath.initialize(0, instructions)?;

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
        let mut datapath = RiscDatapath::default();

        let instructions: Vec<u32> = vec![0b000000000000_01000_010_10000_0000011];
        datapath.initialize(0, instructions)?;

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
        let mut datapath = RiscDatapath::default();

        let instructions: Vec<u32> = vec![0b000000000100_01000_010_10000_0000011];
        datapath.initialize(0, instructions)?;

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
        let mut datapath = RiscDatapath::default();

        let instructions: Vec<u32> = vec![0b111111111100_01000_010_10000_0000011];
        datapath.initialize(0, instructions)?;

        // place data at address
        datapath.memory.store_word(0b1000, 0x10000)?;

        datapath.registers.gpr[8] = 12;
        datapath.execute_instruction();
        assert_eq!(datapath.registers.gpr[16], 0x10000);
        Ok(())
    }
}

pub mod store_word {
    use super::*;
    #[test]
    fn sw_zero_offset_test() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        let instructions: Vec<u32> = vec![0b0000000_10000_01000_010_00000_0100011];
        datapath.initialize(0, instructions)?;
        datapath.execute_instruction();

        let t = datapath.memory.load_word(0)?;
        assert_eq!(t, 0);
        Ok(())
    }

    #[test]
    fn sw_offset_at_4_test() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        let instructions: Vec<u32> = vec![0b0000000_10000_01000_010_00100_0100011];
        datapath.initialize(0, instructions)?;

        datapath.registers.gpr[8] = 0;
        datapath.registers.gpr[16] = 0xff;
        datapath.execute_instruction();

        let t = datapath.memory.load_word(4)?;
        assert_eq!(t, 0xff);
        Ok(())
    }

    #[test]
    fn sw_gpr_8_at_4_offset_at_4_test() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        let instructions: Vec<u32> = vec![0b0000000_10000_01000_010_00100_0100011];
        datapath.initialize(0, instructions)?;

        datapath.registers.gpr[8] = 4;
        datapath.registers.gpr[16] = 0xff;
        datapath.execute_instruction();

        let t = datapath.memory.load_word(8)?;
        assert_eq!(t, 0xff);
        Ok(())
    }

    #[test]
    fn sw_gpr_8_at_4_offset_at_neg_4_test() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        let instructions: Vec<u32> = vec![0b1111111_10000_01000_010_11100_0100011];
        datapath.initialize(0, instructions)?;

        datapath.registers.gpr[8] = 12;
        datapath.registers.gpr[16] = 0xff;
        datapath.execute_instruction();

        let t = datapath.memory.load_word(8)?;
        assert_eq!(t, 0xff);
        Ok(())
    }
}

pub mod mul {
    use super::*;

    #[test]
    fn mul_positive_result() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // $s5 = $t5 * $t6
        let instructions: Vec<u32> = vec![0b0000001_11111_11110_000_10101_0110011];
        datapath.initialize(0, instructions)?;

        datapath.registers.gpr[30] = 8; // $t5
        datapath.registers.gpr[31] = 95; // $t6

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[21], 760); // $s5
        Ok(())
    }

    #[test]
    fn mul_negative_result() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // $s5 = $t5 * $t6
        let instructions: Vec<u32> = vec![0b0000001_11111_11110_000_10101_0110011];
        datapath.initialize(0, instructions)?;

        datapath.registers.gpr[30] = 5; // $t5
        datapath.registers.gpr[31] = -5_i64 as u64; // $t6

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[21] as i64, -25); // $s5
        Ok(())
    }
}

pub mod div {
    use super::*;

    #[test]
    fn div_positive_result() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // $s4 = $t6 / $t5
        let instructions: Vec<u32> = vec![0b0000001_11110_11111_100_10100_0110011];
        datapath.initialize(0, instructions)?;

        datapath.registers.gpr[31] = 20; // $t6
        datapath.registers.gpr[30] = 2; // $t5

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[20], 10); // $s5
        Ok(())
    }

    #[test]
    fn div_negative_result() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        // $s4 = $t6 / $t5
        let instructions: Vec<u32> = vec![0b0000001_11110_11111_100_10100_0110011];
        datapath.initialize(0, instructions)?;

        datapath.registers.gpr[31] = 20; // $t6
        datapath.registers.gpr[30] = -5_i64 as u64; // $t5

        datapath.execute_instruction();

        assert_eq!(datapath.registers.gpr[20] as i64, -4); // $s5
        Ok(())
    }
}

pub mod load_upper_imm {
    use super::*;

    #[test]
    fn basic_load_upper_imm_test() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        let instructions: Vec<u32> = vec![0b00101010101010100000_01000_0110111];
        datapath.initialize(0, instructions)?;
        datapath.execute_instruction();

        let t = datapath.registers[RiscGpRegisterType::X8];
        assert_eq!(t, 0x2aaa_0000);
        Ok(())
    }

    #[test]
    fn sign_extend_load_upper_imm_test() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        let instructions: Vec<u32> = vec![0b10101010101010100000_01000_0110111];
        datapath.initialize(0, instructions)?;
        datapath.execute_instruction();

        let t = datapath.registers[RiscGpRegisterType::X8];
        assert_eq!(t, 0xffff_ffff_aaaa_0000);
        Ok(())
    }
}

pub mod beq_tests {
    use super::*;
    #[test]
    fn beq_test_basic_registers_are_equal() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        let instructions: Vec<u32> = vec![0b000000000100_10000_000_01000_1100011];
        datapath.initialize(0, instructions)?;

        let initial_pc = datapath.registers.pc;
        datapath.execute_instruction();
        let expt_result = (0b0000_0000_0000_0100 << 2) + initial_pc;
        assert_eq!(datapath.registers.pc, expt_result);
        Ok(())
    }

    #[test]
    fn beq_test_basic_register_are_not_equal() -> Result<(), String> {
        let mut datapath = RiscDatapath::default();

        let instructions: Vec<u32> = vec![
            0b000000000100_10000_000_01000_1100011,
            0b000000000100_10000_000_01000_1100011,
        ];
        datapath.initialize(0, instructions)?;

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
        let mut datapath = RiscDatapath::default();

        let instructions: Vec<u32> = vec![
            0b000000000100_10000_000_01000_1100011, // 0x00, Branch to 0x10
            0,                                      // 0x04
            0,                                      // 0x08
            0,                                      // 0x0c
            0b000000000000_10000_000_01000_1100011, // 0x10, Branch to 0x00
        ];
        datapath.initialize(0, instructions)?;
        datapath.registers.gpr[0b01000] = 1234;
        datapath.registers.gpr[0b10000] = 1234;

        // 0x10, aka 16
        let expt_result = 0x10;

        datapath.execute_instruction(); // branch to address 16 from address 0
        assert_eq!(datapath.registers.pc, expt_result);
        assert_eq!(datapath.registers.gpr[0b01000], 1234);
        assert_eq!(datapath.registers.gpr[0b10000], 1234);

        // 0x00
        let expt_result = 0x00;

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

        let mut datapath = RiscDatapath::default();

        let instructions: Vec<u32> = vec![0b000000000100_10000_001_01000_1100011];
        datapath.registers.gpr[0b01000] = 1234;
        datapath.registers.gpr[0b10000] = 1234;
        datapath.initialize(0, instructions)?;
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

        let mut datapath = RiscDatapath::default();
        let instructions: Vec<u32> = vec![
            0b000000000010_10000_001_01000_1100011, // 0x00, Branch to 0x8
            0,                                      // 0x04
            0b000000001000_10000_001_01000_1100011, // 0x08, Branch to 0x20
            0,                                      // 0x0c
            0,                                      // 0x10
            0,                                      // 0x14
            0,                                      // 0x18
            0,                                      // 0x1c
            0b000000000010_10000_001_01000_1100011, // 0x20, branch to 0x08
        ];
        datapath.initialize(0, instructions)?;
        datapath.registers.gpr[0b01000] = 1234;
        datapath.registers.gpr[0b10000] = 4321;

        // test 0x0 to 0x8
        datapath.execute_instruction();
        assert_eq!(datapath.registers.pc, 8);

        // Branch from 0x8 to 0x20, aka from 8 to 32, branch by 24
        datapath.execute_instruction();
        let expt_result = 0b0000000000001000 << 2; // 32
        assert_eq!(datapath.registers.pc, expt_result);

        // Branch back to 0x8 from 0x20, aka 32 to 8
        let expt_result = 0x8;

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
