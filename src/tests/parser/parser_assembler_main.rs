#[cfg(test)]
mod parser_main_function_tests {
    use crate::emulation_core::architectures::AvailableDatapaths;
    use crate::parser::parser_assembler_main::*;

    #[test]
    fn parser_takes_string_and_returns_vec_of_instructions() {
        let results = parser(
            "lw $t1, 512($t1)\nadd $t1, $s6, $t2\naddi $t1, $t2, 43690".to_string(),
            AvailableDatapaths::MIPS,
        );

        assert_eq!(
            results.0.instructions[0].binary,
            0b10001101001010010000001000000000
        );
        assert_eq!(
            results.0.instructions[1].binary,
            0b00000010110010100100100000100000
        );
        assert_eq!(
            results.0.instructions[2].binary,
            0b00100001010010011010101010101010
        );
    }
}

mod read_riscv_instructions_tests {

    use crate::tests::parser::parser_assembler_main::helper_functions::instruction_parser_riscv;

    // RV32I Instructions

    #[test]
    fn read_instructions_add() {
        let file_string = "add ra, t5, s0".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000000100011110000000010110011
        );
    }

    #[test]
    fn read_instructions_sub() {
        let file_string = "sub x1, x2, x3".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000000001100010000000010110011
        );
    }

    #[test]
    fn read_instructions_sll() {
        let file_string = "sll x4, x5, x6".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000000011000101001001000110011
        );
    }

    #[test]
    fn read_instructions_slt() {
        let file_string = "slt x7, x8, x9".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000000100101000010001110110011
        );
    }

    #[test]
    fn read_instructions_sltu() {
        let file_string = "sltu x10, x11, x12".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000000110001011011010100110011
        );
    }

    #[test]
    fn read_instructions_xor() {
        let file_string = "xor x13, x14, x15".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000000111101110100011010110011
        );
    }

    #[test]
    fn read_instructions_srl() {
        let file_string = "srl x16, x17, x18".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001001010001101100000110011
        );
    }

    #[test]
    fn read_instructions_sra() {
        let file_string = "sra x19, x20, x21".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000001010110100101100110110011
        );
    }

    #[test]
    fn read_instructions_or() {
        let file_string = "or x22, x23, x24".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001100010111110101100110011
        );
    }

    #[test]
    fn read_instructions_and() {
        let file_string = "and x25, x26, x27".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001101111010111110010110011
        );
    }

    #[test]
    fn read_instructions_addi() {
        let file_string = "addi x28, x29, x30".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000000000011101000111000010011
        );
    }

    #[test]
    fn read_instructions_slti() {
        let file_string = "slti x31, t0, 150".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00001001011000101010111110010011
        );
    }

    #[test]
    fn read_instructions_sltiu() {
        let file_string = "sltiu t1, t2, 241".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00001111000100111011001100010011
        );
    }

    #[test]
    fn read_instructions_xori() {
        let file_string = "xori t3, t4, 440".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00011011100011101100111000010011
        );
    }

    #[test]
    fn read_instructions_ori() {
        let file_string = "ori t5, t6, 621".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00100110110111111110111100010011
        );
    }

    #[test]
    fn read_instructions_andi() {
        let file_string = "andi ra, sp, 1024".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000000000000010111000010010011
        );
    }

    #[test]
    fn read_instructions_slli() {
        let file_string = "slli gp, tp, 5".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000000010100100001000110010011
        );
    }

    #[test]
    fn read_instructions_srli() {
        let file_string = "srli s0, s1, 10".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000000101001001101010000010011
        );
    }

    #[test]
    fn read_instructions_srai() {
        let file_string = "srai a0, a1, 6".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000000011001011101010100010011
        );
    }

    #[test]
    fn read_instructions_lb() {
        let file_string = "lb a2, 150(a3)".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00001001011001101000011000000011
        );
    }

    #[test]
    fn read_instructions_lh() {
        let file_string = "lh a4, 220(a5)".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00001101110001111001011100000011
        );
    }

    #[test]
    fn read_instructions_lw() {
        let file_string = "lw a6, 32(a7)".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000010000010001010100000000011
        );
    }

    #[test]
    fn read_instructions_lbu() {
        let file_string = "lbu s2, 128(s3)".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00001000000010011100100100000011
        );
    }

    #[test]
    fn read_instructions_lhu() {
        let file_string = "lhu s4, 256(s5)".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00010000000010101101101000000011
        );
    }

    #[test]
    fn read_instructions_sb() {
        let file_string = "sb s6, 512(s7)".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00100001011010111000000000100011
        );
    }

    #[test]
    fn read_instructions_sh() {
        let file_string = "sh s8, 1024(s9)".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000001100011001001000000100011
        );
    }

    #[test]
    fn read_instructions_sw() {
        let file_string = "sw s10, 2044(s11)".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01111111101011011010111000100011
        );
    }

    #[test]
    fn read_instructions_jal() {
        let file_string = "main:\njal x1, L1\nret\nL1:\nadd x1, x2, x3".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000000000000000010000011101111
        );
    }

    #[test]
    fn read_instructions_jalr() {
        let file_string = "jalr x2, x3, 128".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00001000000000011000000101100111
        );
    }

    #[test]
    fn read_instructions_beq() {
        let file_string = "main:\nbeq x1, x2, L1\nret\nL1:\nadd x1, x2, x3".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000000001000010000000011100011
        );
    }

    #[test]
    fn read_instructions_bne() {
        let file_string = "main:\nbne x1, x2, L1\nret\nL1:\nadd x1, x2, x3".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000000001000010001000011100011
        );
    }

    #[test]
    fn read_instructions_blt() {
        let file_string = "main:\nblt x1, x2, L1\nret\nL1:\nadd x1, x2, x3".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000000001000010100000011100011
        );
    }

    #[test]
    fn read_instructions_bge() {
        let file_string = "main:\nbge x1, x2, L1\nret\nL1:\nadd x1, x2, x3".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000000001000010101000011100011
        );
    }

    #[test]
    fn read_instructions_bltu() {
        let file_string = "main:\nbltu x1, x2, L1\nret\nL1:\nadd x1, x2, x3".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000000001000010110000011100011
        );
    }

    #[test]
    fn read_instructions_bgeu() {
        let file_string = "main:\nbgeu x1, x2, L1\nret\nL1:\nadd x1, x2, x3".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000000001000010111000011100011
        );
    }

    #[test]
    fn read_instructions_ecall() {
        let file_string = "ecall".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000000000000000000000001110011
        );
    }

    #[test]
    fn read_instructions_ebreak() {
        let file_string = "ebreak".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000000000100000000000001110011
        );
    }

    #[test]
    fn read_instructions_lui() {
        let file_string = "lui x16, 4096".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001000000000000100000110111
        );
    }

    #[test]
    fn read_instructions_auipc() {
        let file_string = "auipc x17, 5024".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001001110100000100010010111
        );
    }

    // RV64I Tests

    #[test]
    fn read_instructions_addiw() {
        let file_string = "addiw x18, x19, 50".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000011001010011000100100011011
        );
    }

    #[test]
    fn read_instructions_slliw() {
        let file_string = "slliw x20, x21, 16".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001000010101001101000011011
        );
    }

    #[test]
    fn read_instructions_srliw() {
        let file_string = "srliw x22, x23, 20".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001010010111101101100011011
        );
    }

    #[test]
    fn read_instructions_sraiw() {
        let file_string = "sraiw x24, x25, 10".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000000101011001101110000011011
        );
    }

    #[test]
    fn read_instructions_addw() {
        let file_string = "addw x26, x27, x28".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001110011011000110100111011
        );
    }

    #[test]
    fn read_instructions_subw() {
        let file_string = "subw x29, x30, x31".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000001111111110000111010111011
        );
    }

    #[test]
    fn read_instructions_sllw() {
        let file_string = "sllw ra, sp, gp".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000000001100010001000010111011
        );
    }

    #[test]
    fn read_instructions_srlw() {
        let file_string = "srlw tp, t0, t1".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000000011000101101001000111011
        );
    }

    #[test]
    fn read_instructions_sraw() {
        let file_string = "sraw t2, fp, s1".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000000100101000101001110111011
        );
    }

    #[test]
    fn read_instructions_lwu() {
        let file_string = "lwu a0, 50(a1)".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000011001001011110010100000011
        );
    }

    #[test]
    fn read_instructions_ld() {
        let file_string = "ld a2, 50(a3)".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000011001001101011011000000011
        );
    }

    #[test]
    fn read_instructions_sd() {
        let file_string = "sd a4, 50(a5)".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000010111001111011100100100011
        );
    }

    // Start of RV32M

    #[test]
    fn read_instructions_mul() {
        let file_string = "mul a6, a7, s2".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000011001010001000100000110011
        );
    }

    #[test]
    fn read_instructions_mulh() {
        let file_string = "mulh s3, s4, s5".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000011010110100001100110110011
        );
    }

    #[test]
    fn read_instructions_mulhsu() {
        let file_string = "mulhsu s6, s7, s8".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000011100010111010101100110011
        );
    }

    #[test]
    fn read_instructions_mulhu() {
        let file_string = "mulhu s9, s10, s11".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000011101111010011110010110011
        );
    }

    #[test]
    fn read_instructions_div() {
        let file_string = "div t3, t4, t5".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000011111011101100111000110011
        );
    }

    #[test]
    fn read_instructions_divu() {
        let file_string = "divu t6, x1, x2".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000010001000001101111110110011
        );
    }

    #[test]
    fn read_instructions_rem() {
        let file_string = "rem x3, x4, x5".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000010010100100110000110110011
        );
    }

    #[test]
    fn read_instructions_remu() {
        let file_string = "remu x6, x7, x8".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000010100000111111001100110011
        );
    }

    // RV64M

    #[test]
    fn read_instructions_mulw() {
        let file_string = "mulw x9, x10, x11".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000010101101010000010010111011
        );
    }

    #[test]
    fn read_instructions_divw() {
        let file_string = "divw x12, x13, x14".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000010111001101100011000111011
        );
    }

    #[test]
    fn read_instructions_divuw() {
        let file_string = "divuw x15, x16, x17".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000011000110000101011110111011
        );
    }

    #[test]
    fn read_instructions_remw() {
        let file_string = "remw x18, x19, x20".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000011010010011110100100111011
        );
    }

    #[test]
    fn read_instructions_remuw() {
        let file_string = "remuw x21, x22, x23".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000011011110110111101010111011
        );
    }

    // Start of RV32F

    #[test]
    fn read_instructions_fmadds() {
        let file_string = "fmadd.s f1, f2, f3, f4".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00100000001100010111000011000011
        );
    }

    #[test]
    fn read_instructions_fmsubs() {
        let file_string = "fmsub.s f5, f6, f7, f8".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000000011100110111001011000111
        );
    }

    #[test]
    fn read_instructions_fnmsubs() {
        let file_string = "fnmsub.s f9, f10, f11, f12".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01100000101101010111010011001011
        );
    }

    #[test]
    fn read_instructions_fnmadds() {
        let file_string = "fnmadd.s f13, f14, f15, f16".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b10000000111101110111011011001111
        );
    }

    #[test]
    fn read_instructions_fadds() {
        let file_string = "fadd.s f17, f18, f19".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001001110010111100011010011
        );
    }

    #[test]
    fn read_instructions_fsubs() {
        let file_string = "fsub.s f20, f21, f22".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00001001011010101111101001010011
        );
    }

    #[test]
    fn read_instructions_fmuls() {
        let file_string = "fmul.s f23, f24, f25".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00010001100111000111101111010011
        );
    }

    #[test]
    fn read_instructions_fdivs() {
        let file_string = "fdiv.s f26, f27, f28".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00011001110011011111110101010011
        );
    }

    #[test]
    fn read_instructions_fsqrts() {
        let file_string = "fsqrt.s f29, f30".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01011000000011110111111011010011
        );
    }

    #[test]
    fn read_instructions_fsgnjs() {
        let file_string = "fsgnj.s f31, ft1, ft2".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00100000001000001000111111010011
        );
    }

    #[test]
    fn read_instructions_fsgnjns() {
        let file_string = "fsgnjn.s ft3, ft4, ft5".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00100000010100100001000111010011
        );
    }

    #[test]
    fn read_instructions_fsgnjxs() {
        let file_string = "fsgnjx.s f6, f7, f8".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00100000100000111010001101010011
        );
    }

    #[test]
    fn read_instructions_fmins() {
        let file_string = "fmin.s fs1, fa0, fa1".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00101000101101010000010011010011
        );
    }

    #[test]
    fn read_instructions_fmaxs() {
        let file_string = "fmax.s fa2, fa3, fa4".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00101000111001101001011001010011
        );
    }

    #[test]
    fn read_instructions_fcvtws() {
        let file_string = "fcvt.w.s x1, fa5".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b11000000000001111111000011010011
        );
    }

    #[test]
    fn read_instructions_fcvtwus() {
        let file_string = "fcvt.wu.s x2, fa6".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b11000000000110000111000101010011
        );
    }

    #[test]
    fn read_instructions_fmvxw() {
        let file_string = "fmv.x.w x3, fa7".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b11100000000010001000000111010011
        );
    }

    #[test]
    fn read_instructions_feqs() {
        let file_string = "feq.s x4, fs2, fs3".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b10100001001110010010001001010011
        );
    }

    #[test]
    fn read_instructions_flts() {
        let file_string = "flt.s x5, fs4, fs5".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b10100001010110100001001011010011
        );
    }

    #[test]
    fn read_instructions_fles() {
        let file_string = "fle.s x6, fs6, fs7".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b10100001011110110000001101010011
        );
    }

    #[test]
    fn read_instructions_fclasss() {
        let file_string = "fclass.s x7, fs8".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b11100000000011000001001111010011
        );
    }

    #[test]
    fn read_instructions_fcvtsw() {
        let file_string = "fcvt.s.w fs9, x8".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b11010000000001000111110011010011
        );
    }

    #[test]
    fn read_instructions_fcvtswu() {
        let file_string = "fcvt.s.wu fs10, x9".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b11010000000101001111110101010011
        );
    }

    #[test]
    fn read_instructions_fmvwx() {
        let file_string = "fmv.w.x fs11, x10".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b11110000000001010000110111010011
        );
    }

    #[test]
    fn read_instructions_fmaddd() {
        let file_string = "fmadd.d ft8, ft9, ft10, ft11".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b11111011111011101111111001000011
        );
    }

    #[test]
    fn read_instructions_fmsubd() {
        let file_string = "fmsub.d f1, f2, f3, f4".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00100010001100010111000011000111
        );
    }

    #[test]
    fn read_instructions_fnmaddd() {
        let file_string = "fnmsub.d f5, f6, f7, f8".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000010011100110111001011001011
        );
    }

    #[test]
    fn read_instructions_fnmsubd() {
        let file_string = "fnmadd.d f9, f10, f11, f12".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01100010101101010111010011001111
        );
    }

    #[test]
    fn read_instructions_faddd() {
        let file_string = "fadd.d f13, f14, f15".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000010111101110111011011010011
        );
    }

    #[test]
    fn read_instructions_fsubd() {
        let file_string = "fsub.d f16, f17, f18".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00001011001010001111100001010011
        );
    }

    #[test]
    fn read_instructions_fmuld() {
        let file_string = "fmul.d f19, f20, f21".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00010011010110100111100111010011
        );
    }

    #[test]
    fn read_instructions_fdivd() {
        let file_string = "fdiv.d f22, f23, f24".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00011011100010111111101101010011
        );
    }

    #[test]
    fn read_instructions_fsqrtd() {
        let file_string = "fsqrt.d f25, f26".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01011010000011010111110011010011
        );
    }

    #[test]
    fn read_instructions_fsgnjd() {
        let file_string = "fsgnj.d f27, f28, f29".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00100011110111100000110111010011
        );
    }

    #[test]
    fn read_instructions_fsgnjnd() {
        let file_string = "fsgnjn.d f30, f31, f1".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00100010000111111001111101010011
        );
    }

    #[test]
    fn read_instructions_fsgnjxd() {
        let file_string = "fsgnjx.d f2, f3, f4".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00100010010000011010000101010011
        );
    }

    #[test]
    fn read_instructions_fmind() {
        let file_string = "fmin.d f5, f6, f7".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00101010011100110000001011010011
        );
    }

    #[test]
    fn read_instructions_fmaxd() {
        let file_string = "fmax.d f8, f9, f10".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00101010101001001001010001010011
        );
    }

    #[test]
    fn read_instructions_fcvtsd() {
        let file_string = "fcvt.s.d f11, f12".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000000000101100111010111010011
        );
    }

    #[test]
    fn read_instructions_fcvtds() {
        let file_string = "fcvt.d.s f13, f14".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000010000001110111011011010011
        );
    }

    #[test]
    fn read_instructions_feqd() {
        let file_string = "feq.d x11, f15, f16".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b10100011000001111010010111010011
        );
    }

    #[test]
    fn read_instructions_fltd() {
        let file_string = "flt.d x12, f17, f18".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b10100011001010001001011001010011
        );
    }

    #[test]
    fn read_instructions_fled() {
        let file_string = "fle.d x13, f19, f20".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b10100011010010011000011011010011
        );
    }

    #[test]
    fn read_instructions_fclassd() {
        let file_string = "fclass.d x14, f21".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b11100010000010101001011101010011
        );
    }

    #[test]
    fn read_instructions_fcvtwd() {
        let file_string = "fcvt.w.d x15, f22".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b11000010000010110111011111010011
        );
    }

    #[test]
    fn read_instructions_fcvtwud() {
        let file_string = "fcvt.wu.d x16, f23".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b11000010000110111111100001010011
        );
    }

    #[test]
    fn read_instructions_fcvtdw() {
        let file_string = "fcvt.d.w f24, x17".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b11010010000010001111110001010011
        );
    }

    #[test]
    fn read_instructions_fcvtdwu() {
        let file_string = "fcvt.d.wu f25, x18".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b11010010000110010111110011010011
        );
    }

    #[test]
    fn read_instructions_flw() {
        let file_string = "flw f26, 128(x19)".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00001000000010011010110100000111
        );
    }

    #[test]
    fn read_instructions_fsw() {
        let file_string = "fsw f27, 256(x20)".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00010001101110100010000000100111
        );
    }

    #[test]
    fn read_instructions_fld() {
        let file_string = "fld f28, 512(x21)".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00100000000010101011111000000111
        );
    }

    #[test]
    fn read_instructions_fsd() {
        let file_string = "fsd f29, 1024(x22)".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000001110110110011000000100111
        );
    }

    #[test]
    fn read_instructions_fcvtls() {
        let file_string = "fcvt.l.s x23, f30".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b11000000001011110111101111010011
        );
    }

    #[test]
    fn read_instructions_fcvtlus() {
        let file_string = "fcvt.lu.s x24, f31".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b11000000001111111111110001010011
        );
    }

    #[test]
    fn read_instructions_fcvtsl() {
        let file_string = "fcvt.s.l f1, x25".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b11010000001011001111000011010011
        );
    }

    #[test]
    fn read_instructions_fcvtslu() {
        let file_string = "fcvt.s.lu f2, x26".to_string();

        let instruction_list = instruction_parser_riscv(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b11010000001111010111000101010011
        );
    }
}

mod read_mips_instructions_tests {
    use crate::parser::parser_structs_and_enums::ErrorType::JALRRDRegisterZero;
    use crate::tests::parser::parser_assembler_main::helper_functions::instruction_parser_mips;

    #[test]
    fn read_instructions_add() {
        let file_string = "add $t1, $s6, $t2".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000010110010100100100000100000
        );
    }

    #[test]
    fn read_instructions_addu() {
        let file_string = "addu $t1, $t2, $t3".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001010010110100100000100001
        );
    }

    #[test]
    fn read_instructions_sub() {
        let file_string = "sub $t1, $s6, $t2".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000010110010100100100000100010
        );
    }

    #[test]
    fn read_instructions_mul() {
        let file_string = "mul $t1, $s6, $t2".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000010110010100100100010011000
        );
    }

    #[test]
    fn read_instructions_div() {
        let file_string = "div $t1, $t1, $s6".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001001101100100100010011010
        );
    }

    #[test]
    fn read_instructions_lw() {
        let file_string = "lw $t1, 512($t1)".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b10001101001010010000001000000000
        );
    }

    #[test]
    fn read_instructions_sw() {
        let file_string = "sw $t1, 512($t1)".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b10101101001010010000001000000000
        );
    }

    #[test]
    fn read_instructions_lui() {
        let file_string = "lui $t1, 43690".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00111100000010011010101010101010
        );
    }

    #[test]
    fn read_instructions_aui() {
        let file_string = "aui $t1, $t1, 43690".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00111101001010011010101010101010
        );
    }

    #[test]
    fn read_instructions_addi() {
        let file_string = "addi $t1, $t2, 43690".to_string();

        let instruction_list = instruction_parser_mips(file_string);
        assert_eq!(
            instruction_list[0].binary,
            0b00100001010010011010101010101010
        );
    }

    #[test]
    fn read_instructions_recognizes_addiu() {
        let instruction_list = instruction_parser_mips("addiu $t1, $t2, 0x64".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b00100101010010010000000001100100
        )
    }

    #[test]
    fn read_instructions_and() {
        let file_string = "and $t1, $s6, $t2".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000010110010100100100000100100
        );
    }

    #[test]
    fn read_instructions_or() {
        let file_string = "or $t1, $s6, $t2".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000010110010100100100000100101
        );
    }

    #[test]
    fn read_instructions_ori() {
        let file_string = "ori $t1, $t2, 43690".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00110101010010011010101010101010
        );
    }

    #[test]
    fn read_instructions_andi() {
        let file_string = "andi $t1, $t2, 43690".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00110001010010011010101010101010
        );
    }

    #[test]
    fn read_instructions_dadd() {
        let file_string = "dadd $t1, $t2, $s6".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001010101100100100000101100
        );
    }

    #[test]
    fn read_instructions_dsub() {
        let file_string = "dsub $t1, $t2, $s6".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001010101100100100000101110
        );
    }

    #[test]
    fn read_instructions_dmul() {
        let file_string = "dmul $t1, $t2, $s6".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001010101100100100010011100
        );
    }

    #[test]
    fn read_instructions_ddiv() {
        let file_string = "ddiv $t1, $t1, $t2".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001001010100100100010011110
        );
    }

    #[test]
    fn read_instructions_add_s() {
        let file_string = "add.s $f9, $f10, $f22".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000110000101100101001001000000
        );
    }

    #[test]
    fn read_instructions_add_d() {
        let file_string = "add.d $f9, $f10, $f22".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000110001101100101001001000000
        );
    }

    #[test]
    fn read_instructions_sub_s() {
        let file_string = "sub.s $f9, $f10, $f22".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000110000101100101001001000001
        );
    }

    #[test]
    fn read_instructions_sub_d() {
        let file_string = "sub.d $f9, $f10, $f22".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000110001101100101001001000001
        );
    }

    #[test]
    fn read_instructions_mul_s() {
        let file_string = "mul.s $f9, $f10, $f22".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000110000101100101001001000010
        );
    }

    #[test]
    fn read_instructions_mul_d() {
        let file_string = "mul.d $f9, $f10, $f22".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000110001101100101001001000010
        );
    }

    #[test]
    fn read_instructions_div_s() {
        let file_string = "div.s $f9, $f10, $f22".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000110000101100101001001000011
        );
    }

    #[test]
    fn read_instructions_div_d() {
        let file_string = "div.d $f9, $f10, $f22".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000110001101100101001001000011
        );
    }

    #[test]
    fn read_instructions_dahi() {
        let file_string = "dahi $t1, 43690".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000101001001101010101010101010
        );
    }

    #[test]
    fn read_instructions_dati() {
        let file_string = "dati $t1, 43690".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000101001111101010101010101010
        );
    }

    #[test]
    fn read_instructions_daddi() {
        let file_string = "daddi $t1, $t2, 43690".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01100001010010011010101010101010
        );
    }

    #[test]
    fn read_instructions_daddiu() {
        let file_string = "daddiu $t1, $t2, 43690".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01100101010010011010101010101010
        );
    }

    #[test]
    fn read_instructions_daddu() {
        let file_string = "daddu $t1, $t2, $t3".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001010010110100100000101101
        );
    }

    #[test]
    fn read_instructions_dsubu() {
        let file_string = "dsubu $t1, $t2, $t3".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001010010110100100000101111
        );
    }

    #[test]
    fn read_instructions_dmulu() {
        let file_string = "dmulu $t1, $t2, $t3".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001010010110100100010011101
        );
    }

    #[test]
    fn read_instructions_ddivu() {
        let file_string = "ddivu $t1, $t1, $t2".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001001010100100100010011111
        );
    }

    #[test]
    fn read_instructions_slt() {
        let file_string = "slt $t1, $t2, $s6".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001010101100100100000101010
        );
    }

    #[test]
    fn read_instructions_sltu() {
        let file_string = "sltu $t1, $t2, $s6".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001010101100100100000101011
        );
    }

    #[test]
    fn read_instructions_swc1() {
        let file_string = "swc1 $f9, 43690($t2)".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b11100101010010011010101010101010
        );
    }

    #[test]
    fn read_instructions_lwc1() {
        let file_string = "lwc1 $f9, 43690($t2)".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b11000101010010011010101010101010
        );
    }

    #[test]
    fn read_instructions_mtc1() {
        let file_string = "mtc1 $t1, $f22".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000100100010011011000000000000
        );
    }

    #[test]
    fn read_instructions_dmtc1() {
        let file_string = "dmtc1 $t1, $f22".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000100101010011011000000000000
        );
    }

    #[test]
    fn read_instructions_mfc1() {
        let file_string = "mfc1 $t1, $f22".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000100000010011011000000000000
        );
    }

    #[test]
    fn read_instructions_dmfc1() {
        let file_string = "dmfc1 $t1, $f22".to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000100001010011011000000000000
        );
    }

    #[test]
    fn read_instructions_j() {
        let file_string =
            "Add $t1, $t2, $t3\nAddress: add $t1, #t2, $t3\nlw $t1, 400($t2)\nj Address"
                .to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[3].binary,
            0b00001000000000000000000000000001
        )
    }

    #[test]
    fn read_instructions_jal() {
        let file_string =
            "Add $t1, $t2, $t3\nAddress: add $t1, #t2, $t3\nlw $t1, 400($t2)\njal Address"
                .to_string();

        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[3].binary,
            0b00001100000000000000000000000001
        )
    }

    #[test]
    fn read_instructions_beq() {
        let file_string = "Add $t1, $t2, $t3\nAddress: add $t1, #t2, $t3\nlw $t1, 400($t2)\nbeq $t1, $t2, address".to_string();
        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[3].binary,
            0b00010001001010101111111111111101
        )
    }

    #[test]
    fn read_instructions_bne() {
        let file_string = "Add $t1, $t2, $t3\nAddress: add $t1, #t2, $t3\nlw $t1, 400($t2)\nbne $t1, $t2, address".to_string();
        let instruction_list = instruction_parser_mips(file_string);

        assert_eq!(
            instruction_list[3].binary,
            0b00010101001010101111111111111101
        )
    }

    #[test]
    fn read_instructions_c_eq_s() {
        let instruction_list = instruction_parser_mips("c.eq.s $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110000101100100100000110010
        )
    }

    #[test]
    fn read_instructions_c_eq_d() {
        let instruction_list = instruction_parser_mips("c.eq.d $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110001101100100100000110010
        )
    }

    #[test]
    fn read_instructions_c_lt_s() {
        let instruction_list = instruction_parser_mips("c.lt.s $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110000101100100100000111100
        )
    }

    #[test]
    fn read_instructions_c_lt_d() {
        let instruction_list = instruction_parser_mips("c.lt.d $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110001101100100100000111100
        )
    }

    #[test]
    fn read_instructions_c_le_s() {
        let instruction_list = instruction_parser_mips("c.le.s $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110000101100100100000111110
        )
    }

    #[test]
    fn read_instructions_c_le_d() {
        let instruction_list = instruction_parser_mips("c.le.d $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110001101100100100000111110
        )
    }

    #[test]
    fn read_instructions_c_ngt_s() {
        let instruction_list = instruction_parser_mips("c.ngt.s $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110000101100100100000111111
        )
    }

    #[test]
    fn read_instructions_c_ngt_d() {
        let instruction_list = instruction_parser_mips("c.ngt.d $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110001101100100100000111111
        )
    }

    #[test]
    fn read_instructions_c_nge_s() {
        let instruction_list = instruction_parser_mips("c.nge.s $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110000101100100100000111101
        )
    }

    #[test]
    fn read_instructions_c_nge_d() {
        let instruction_list = instruction_parser_mips("c.nge.d $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110001101100100100000111101
        )
    }

    #[test]
    fn read_instruction_bc1t() {
        let instruction_list =
            instruction_parser_mips("instruction: add $t1, $t2, $t3\nbc1t instruction".to_string());

        assert_eq!(
            instruction_list[1].binary,
            0b01000101000000011111111111111110
        );
    }

    #[test]
    fn read_instruction_bc1f() {
        let instruction_list =
            instruction_parser_mips("instruction: add $t1, $t2, $t3\nbc1f instruction".to_string());

        assert_eq!(
            instruction_list[1].binary,
            0b01000101000000001111111111111110
        );
    }

    #[test]
    fn read_instruction_jalr_with_rd() {
        let instruction_list = instruction_parser_mips("jalr $t1, $t2".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b00000001010000000100100000001001
        );
    }

    #[test]
    fn read_instruction_jalr_without_rd() {
        let instruction_list = instruction_parser_mips("jalr $t2".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b00000001010000001111100000001001
        );
    }

    #[test]
    fn read_instruction_jalr_creates_error_with_rd_equal_0() {
        let instruction_list = instruction_parser_mips("jalr $zero, $t2".to_string());

        assert_eq!(instruction_list[0].errors[0].error_name, JALRRDRegisterZero);
    }

    #[test]
    fn read_instructions_recognizes_b() {
        let instruction_list =
            instruction_parser_mips(".text\njump: addi $t1, $t2, 100\nb jump".to_string());

        assert_eq!(
            instruction_list[1].binary,
            0b00010000000000001111111111111110
        );
    }

    #[test]
    fn read_instructions_recognizes_jr() {
        let instruction_list = instruction_parser_mips(".text\njump: jr $zero\nb jump".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b00000000000000000000000000001001
        );
    }

    #[test]
    fn read_instructions_recognizes_jr_ra() {
        let instruction_list = instruction_parser_mips(".text\njump: jr $ra\nb jump".to_string());

        // Page 249 in the MIPS64 release 6 manual
        // https://s3-eu-west-1.amazonaws.com/downloads-mips/documents/MIPS_Architecture_MIPS64_InstructionSet_%20AFP_P_MD00087_06.05.pdf
        assert_eq!(
            instruction_list[0].binary,
            0b0000_0011_1110_0000_0000_0000_0000_1001
        );
    }

    #[test]
    fn read_instructions_recognizes_sll() {
        let instruction_list = instruction_parser_mips(".text\nsll $t1, $t2, 5".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b00000000000010100100100101000000
        );
    }

    #[test]
    fn read_instructions_recognizes_nop() {
        let instruction_list = instruction_parser_mips(".text\nnop".to_string());

        assert_eq!(instruction_list[0].binary, 0);
    }
}

use crate::emulation_core::architectures::AvailableDatapaths;
use crate::parser::assembling::assemble_data_binary;
use crate::parser::parser_assembler_main::{
    create_binary_vec, parser, place_binary_in_middle_of_another, read_instructions,
};
use crate::parser::parser_structs_and_enums::ErrorType::{
    UnrecognizedInstruction, UnsupportedInstruction,
};
use crate::parser::parser_structs_and_enums::{
    ProgramInfo, SUPPORTED_INSTRUCTIONS_MIPS, UNSUPPORTED_INSTRUCTIONS_MIPS,
};
use crate::parser::parsing::{create_label_map, separate_data_and_text, tokenize_program};
use crate::parser::pseudo_instruction_parsing::{
    complete_lw_sw_pseudo_instructions, expand_pseudo_instructions_and_assign_instruction_numbers,
};
use std::collections::HashMap;

#[test]
fn place_binary_in_middle_of_another_works() {
    let result = place_binary_in_middle_of_another(0b11, 0b0, 1, 0);
    assert_eq!(result, 0b101);
}
#[test]
fn place_binary_in_middle_of_another_works_2() {
    let result = place_binary_in_middle_of_another(0b1001, 0b111, 3, 1);
    assert_eq!(result, 0b1011101);
}
#[test]
fn place_binary_in_middle_of_another_works_3() {
    let result = place_binary_in_middle_of_another(0b10100101, 0b11011, 5, 3);
    assert_eq!(result, 0b1010110110101);
}

#[test]
fn place_binary_works_dahi() {
    let result = place_binary_in_middle_of_another(0b000001010011010101010101010, 0b00110, 5, 15);
    assert_eq!(result, 0b00000101001001101010101010101010);
}

mod helper_functions {
    use crate::parser::assembling::assemble_data_binary;
    use crate::parser::parser_assembler_main::{read_instructions, read_instructions_riscv};
    use crate::parser::parser_structs_and_enums::Instruction;
    use crate::parser::parsing::{create_label_map, separate_data_and_text, tokenize_program};
    use crate::parser::pseudo_instruction_parsing::{
        expand_pseudo_instructions_and_assign_instruction_numbers,
        expand_pseudo_instructions_and_assign_instruction_numbers_riscv,
    };
    use std::collections::HashMap;

    pub fn instruction_parser_mips(mut file_string: String) -> Vec<Instruction> {
        file_string = file_string.to_lowercase();

        let mut monaco_line_info_vec = tokenize_program(file_string);
        let (mut instruction_list, mut data) =
            separate_data_and_text(&mut monaco_line_info_vec.clone());
        expand_pseudo_instructions_and_assign_instruction_numbers(
            &mut instruction_list,
            &data,
            &mut monaco_line_info_vec,
        );
        assemble_data_binary(&mut data);

        let labels: HashMap<String, usize> = create_label_map(&mut instruction_list, &mut data);

        read_instructions(&mut instruction_list, &labels, &mut monaco_line_info_vec);

        instruction_list
    }

    pub fn instruction_parser_riscv(mut file_string: String) -> Vec<Instruction> {
        file_string = file_string.to_lowercase();

        let mut monaco_line_info_vec = tokenize_program(file_string);
        let (mut instruction_list, mut data) =
            separate_data_and_text(&mut monaco_line_info_vec.clone());
        expand_pseudo_instructions_and_assign_instruction_numbers_riscv(
            &mut instruction_list,
            &data,
            &mut monaco_line_info_vec,
        );
        assemble_data_binary(&mut data);

        let labels: HashMap<String, usize> = create_label_map(&mut instruction_list, &mut data);

        read_instructions_riscv(&mut instruction_list, &labels, &mut monaco_line_info_vec);

        instruction_list
    }
}

#[test]
fn create_binary_vec_works_with_data() {
    let mut program_info = ProgramInfo::default();
    let file_string =
        ".data\nlabel: .ascii \"this is a string\"\n.text\nlw $t1, label\nsyscall".to_lowercase();
    let monaco_line_info_vec = tokenize_program(file_string);
    program_info.monaco_line_info = monaco_line_info_vec;
    (program_info.instructions, program_info.data) =
        separate_data_and_text(&mut program_info.monaco_line_info.clone());
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut program_info.monaco_line_info,
    );
    let vec_of_data = assemble_data_binary(&mut program_info.data);

    let labels: HashMap<String, usize> =
        create_label_map(&mut program_info.instructions, &mut program_info.data);

    complete_lw_sw_pseudo_instructions(
        &mut program_info.instructions,
        &labels,
        &mut program_info.monaco_line_info,
    );
    read_instructions(
        &mut program_info.instructions,
        &labels,
        &mut program_info.monaco_line_info,
    );

    let (result, _) = create_binary_vec(program_info.instructions.clone(), vec_of_data);

    assert_eq!(result[3], 0b01110100011010000110100101110011);
    assert_eq!(result[4], 0b00100000011010010111001100100000);
    assert_eq!(result[5], 0b01100001001000000111001101110100);
    assert_eq!(result[6], 0b01110010011010010110111001100111);
}

#[test]
fn read_instructions_recognizes_valid_but_unsupported_instructions() {
    let program_info = parser(
        "nor $t1, $t2, $t3\ndsrav $t1, $t2, $t3\n".to_string(),
        AvailableDatapaths::MIPS,
    )
    .0;

    assert_eq!(
        program_info.instructions[0].errors[0].error_name,
        UnsupportedInstruction
    );
    assert_eq!(
        program_info.instructions[1].errors[0].error_name,
        UnsupportedInstruction
    );
}

#[test]
fn console_output_post_assembly_works_with_errors() {
    let result = parser(
        ".text\nadd $t1, $t2, 1235\n.data\nlabel: .ascii 100\n.text\nlw t1, address".to_string(),
        AvailableDatapaths::MIPS,
    )
    .0
    .console_out_post_assembly;

    assert_eq!(result, "UnrecognizedGPRegister on line 2 with token \"1235\"\nGP register is not recognized.\n\nInvalidMemorySyntax on line 6 with token \"address\"\nThe given string for memory does not match syntax of \"offset(base)\" or \"label\".\n\nImproperlyFormattedASCII on line 4 with token \"100\"\nToken recognized as ASCII does not start and or end with double quotes (\").\n\n")
}

#[test]
fn console_output_post_assembly_works_with_no_errors_present() {
    let result = parser(
        ".text\nadd $t1, $t2, $t3\n.data\nlabel: .ascii \"string\"\n.text\nlw $t1, 40($t1)"
            .to_string(),
        AvailableDatapaths::MIPS,
    )
    .0
    .console_out_post_assembly;

    assert_eq!(result, "Program assembled successfully!".to_string());
}

#[test]
fn mouse_hover_holds_information_about_valid_instructions() {
    let program_info = parser(
        ".text\nori $t1, $t2, 100\nsyscall".to_string(),
        AvailableDatapaths::MIPS,
    )
    .0;

    assert_eq!(program_info.monaco_line_info[0].mouse_hover_string, "");
    assert_eq!(program_info.monaco_line_info[1].mouse_hover_string, "**Syntax:** `ori rt, rs, immediate`\n\nBitwise ors the contents of `rs` with the left zero-extended `immediate` value, and stores the result in `rt`.\n\n\n\n**Binary:** `0b00110101010010010000000001100100`");
}

#[test]
fn mouse_hover_holds_information_about_pseudo_instructions() {
    let program_info = parser(
        ".text\nlabel: subi $t1, $t2, 100\nsyscall".to_string(),
        AvailableDatapaths::MIPS,
    )
    .0;

    assert_eq!(program_info.monaco_line_info[0].mouse_hover_string, "");
    assert_eq!(program_info.monaco_line_info[1].mouse_hover_string, "`subi` is a pseudo-instruction.\n\n```\nsubi rt, rs, immediate =>\nori $at, $zero, immediate\nsub rt, rs, $at\n\n```\n\n\n\n**Binary:** `0b00110100000000010000000001100100`\n\n**Binary:** `0b00000001010000010100100000100010`");
}

#[test]
fn errors_do_not_go_into_mouse_hover() {
    let program_info = parser(
        ".text\nori $t1, $t2, $t3\nsyscall".to_string(),
        AvailableDatapaths::MIPS,
    )
    .0;

    assert_eq!(program_info.monaco_line_info[0].mouse_hover_string, "");
    assert_eq!(program_info.monaco_line_info[1].mouse_hover_string, "**Syntax:** `ori rt, rs, immediate`\n\nBitwise ors the contents of `rs` with the left zero-extended `immediate` value, and stores the result in `rt`.\n\n");
}

#[test]
fn syscall_message_and_binary_does_not_go_in_mouse_hover_if_the_syscall_was_added_by_parser() {
    let monaco_line_info = parser(
        ".text\nori $t1, $t2, 100\nlabel: subi $t1, $t2, 100\nadd $t1, $t2, $t3\n".to_string(),
        AvailableDatapaths::MIPS,
    )
    .0
    .monaco_line_info;

    assert_eq!(monaco_line_info[0].mouse_hover_string, "");
    assert_eq!(monaco_line_info[1].mouse_hover_string, "**Syntax:** `ori rt, rs, immediate`\n\nBitwise ors the contents of `rs` with the left zero-extended `immediate` value, and stores the result in `rt`.\n\n\n\n**Binary:** `0b00110101010010010000000001100100`");
    assert_eq!(monaco_line_info[2].mouse_hover_string, "`subi` is a pseudo-instruction.\n\n```\nsubi rt, rs, immediate =>\nori $at, $zero, immediate\nsub rt, rs, $at\n\n```\n\n\n\n**Binary:** `0b00110100000000010000000001100100`\n\n**Binary:** `0b00000001010000010100100000100010`");
    assert_eq!(monaco_line_info[3].mouse_hover_string, "**Syntax:** `add rd, rs, rt`\n\nAdds the 32-bit values in `rs` and `rt`, and places the result in `rd`.\n\nIn hardware implementations, the result is not placed in `rd` if adding `rs` and `rt` causes a 32-bit overflow. However, SWIM places the result in `rd` regardless since there is no exception handling.\n\n**Binary:** `0b00000001010010110100100000100000`\n\n");

    let monaco_line_info = parser(".text".to_string(), AvailableDatapaths::MIPS)
        .0
        .monaco_line_info;
    assert_eq!(monaco_line_info[0].mouse_hover_string, "\n\n");
}

#[test]
fn mouse_hover_holds_information_info_for_various_instruction_types() {
    let program_info = parser(
        ".text\nori $t1, $t2, 100\nlabel: subi $t1, $t2, 100\nadd $t1, $t2, $t3\nsyscall\n"
            .to_string(),
        AvailableDatapaths::MIPS,
    )
    .0;

    assert_eq!(program_info.monaco_line_info[0].mouse_hover_string, "");
    assert_eq!(program_info.monaco_line_info[1].mouse_hover_string, "**Syntax:** `ori rt, rs, immediate`\n\nBitwise ors the contents of `rs` with the left zero-extended `immediate` value, and stores the result in `rt`.\n\n\n\n**Binary:** `0b00110101010010010000000001100100`");
    assert_eq!(program_info.monaco_line_info[2].mouse_hover_string, "`subi` is a pseudo-instruction.\n\n```\nsubi rt, rs, immediate =>\nori $at, $zero, immediate\nsub rt, rs, $at\n\n```\n\n\n\n**Binary:** `0b00110100000000010000000001100100`\n\n**Binary:** `0b00000001010000010100100000100010`");
    assert_eq!(program_info.monaco_line_info[3].mouse_hover_string, "**Syntax:** `add rd, rs, rt`\n\nAdds the 32-bit values in `rs` and `rt`, and places the result in `rd`.\n\nIn hardware implementations, the result is not placed in `rd` if adding `rs` and `rt` causes a 32-bit overflow. However, SWIM places the result in `rd` regardless since there is no exception handling.\n\n**Binary:** `0b00000001010010110100100000100000`");
    assert_eq!(program_info.monaco_line_info[4].mouse_hover_string, "**Syntax:** `syscall`\n\nThis function is currently stubbed in SWIM. Normally, it reverts control back to the OS. SWIM uses it to effectively end the program.\n\n**Binary:** `0b00000000000000000000000000001100`");
}

#[test]
fn instructions_directives_and_registers_work_regardless_of_capitalization() {
    let result = parser(
        ".TexT\nOR $t1, $T2, $t3\nor $t1, $t2, $t3\n.DATA\nabel: .WOrD 100".to_string(),
        AvailableDatapaths::MIPS,
    );

    let correct = parser(
        ".TexT\nOR $t1, $T2, $t3\nor $t1, $t2, $t3\n.DATA\nabel: .WOrD 100".to_lowercase(),
        AvailableDatapaths::MIPS,
    );
    assert_eq!(result.1, correct.1);
    assert_eq!(
        result.0.console_out_post_assembly,
        correct.0.console_out_post_assembly
    );
    assert_eq!(
        result.0.address_to_line_number,
        correct.0.address_to_line_number
    );
    for i in 0..result.0.monaco_line_info.len() {
        assert_eq!(
            result.0.monaco_line_info[i].mouse_hover_string,
            correct.0.monaco_line_info[i].mouse_hover_string
        );
    }
}

#[test]
fn parser_assembler_works_with_empty_strings() {
    let _ = parser("".to_string(), AvailableDatapaths::MIPS);
    let _ = parser("\n".to_string(), AvailableDatapaths::MIPS);
    let _ = parser("\n\n".to_string(), AvailableDatapaths::MIPS);
}

#[test]
fn create_binary_vec_works_with_all_mod_4_options() {
    let result = parser(
        "ori $s0, $zero, 12345\nori $s0, $zero, 12345\n.data\nlab: .ascii \"h\"".to_string(),
        AvailableDatapaths::MIPS,
    )
    .1;
    assert_eq!(result, vec![873476153, 873476153, 12, 1744830464]);

    let result = parser(
        "ori $s0, $zero, 12345\nori $s0, $zero, 12345\n.data\nlab: .ascii \"ha\"".to_string(),
        AvailableDatapaths::MIPS,
    )
    .1;
    assert_eq!(
        result,
        vec![873476153, 873476153, 12, 0b01101000011000010000000000000000]
    );

    let result = parser(
        "ori $s0, $zero, 12345\nori $s0, $zero, 12345\n.data\nlab: .ascii \"han\"".to_string(),
        AvailableDatapaths::MIPS,
    )
    .1;
    assert_eq!(
        result,
        vec![873476153, 873476153, 12, 0b01101000011000010110111000000000]
    );

    let result = parser(
        "ori $s0, $zero, 12345\nori $s0, $zero, 12345\n.data\nlab: .ascii \"hank\"".to_string(),
        AvailableDatapaths::MIPS,
    )
    .1;
    assert_eq!(
        result,
        vec![873476153, 873476153, 12, 0b01101000011000010110111001101011]
    );
}

#[test]
fn no_unsupported_mips_instructions_are_recognized_by_parser() {
    for instruction in UNSUPPORTED_INSTRUCTIONS_MIPS {
        let result = parser(instruction.to_string(), AvailableDatapaths::MIPS)
            .0
            .monaco_line_info;
        assert_eq!(result[0].errors[0].error_name, UnsupportedInstruction);
    }
}

#[test]
fn supported_mips_instructions_are_recognized_by_parser() {
    for instruction in SUPPORTED_INSTRUCTIONS_MIPS {
        let result = parser(instruction.to_string(), AvailableDatapaths::MIPS)
            .0
            .monaco_line_info;
        for error in &result[0].errors {
            assert_ne!(error.error_name, UnsupportedInstruction);
            assert_ne!(error.error_name, UnrecognizedInstruction);
        }
    }
}

#[test]
fn main_and_start_labelled_instructions_change_program_info_pc_starting_point() {
    let result = parser(
        "addi $t1, $t2, 100\nsw $t1, 400($zero)".to_string(),
        AvailableDatapaths::MIPS,
    )
    .0
    .pc_starting_point;
    assert_eq!(result, 0);

    let result = parser(
        "addi $t1, $t2, 100\nsw $t1, 400($zero)\nmain: lw $t2, 320($zero)".to_string(),
        AvailableDatapaths::MIPS,
    )
    .0
    .pc_starting_point;
    assert_eq!(result, 8);

    let result = parser(
        "addi $t1, $t2, 100\nstart: sw $t1, 400($zero)\nlw $t2, 320($zero)".to_string(),
        AvailableDatapaths::MIPS,
    )
    .0
    .pc_starting_point;
    assert_eq!(result, 4);

    let result = parser(
        "addi $t1, $t2, 100\nstart: sw $t1, 400($zero)\nmain: lw $t2, 320($zero)".to_string(),
        AvailableDatapaths::MIPS,
    )
    .0
    .pc_starting_point;
    assert_eq!(result, 8);
}
