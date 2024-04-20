use std::collections::HashMap;

use crate::emulation_core::riscv::instruction::RiscInstruction;

// *** Test negative number instructions ***

#[test]
fn test_instruction_negative_16() {
    let instruction: u32 = 0xff010113;

    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    assert!(
        match RiscInstruction::get_string_version(instruction, labels.clone()) {
            Ok(string) => string.contains("addi x2, x2, -16"),
            _ => false,
        }
    );
}

#[test]
fn test_instruction_negative_2() {
    let instruction: u32 = 0xffe50513;

    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    assert!(
        match RiscInstruction::get_string_version(instruction, labels.clone()) {
            Ok(string) => string.contains("addi x10, x10, -2"),
            _ => false,
        }
    );
}

#[test]
fn test_jal_instruction() {
    let instruction: u32 = 0x00008067;

    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    assert!(
        match RiscInstruction::get_string_version(instruction, labels.clone()) {
            Ok(string) => string.contains("jalr x0, x1, 0"),
            _ => false,
        }
    );
}

// ** Test all other instructions **

#[test]
fn err_on_empty_instruction() {
    let instruction: u32 = 0b00000000000000000000000000000000;

    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    assert!(
        match RiscInstruction::get_string_version(instruction, labels.clone()) {
            Err(e) => e.contains("not supported"),
            _ => false,
        }
    );
}

#[test]
fn test_instructions() {
    let instructions: Vec<u32> = vec![
        0b10010001101000101000010110111,
        0b1000000000000000100010111,
        0b101000001000000110010011,
        0b10100010010001000010011,
        0b1111111100011100001010010011,
        0b1111000000100110001100010011,
        0b111100101111001110010011,
        0b1000110001010000010011,
        0b00000000001100111101010010010011,
        0b1000000000101000101010100010011,
        0b10000011000010110110011,
        0b1000000011000101000011000110011,
        0b100000111001011010110011,
        0b101001001010011100110011,
        0b110001011011011110110011,
        0b111001101110100000110011,
        0b1000001111111100010110011,
        0b1110011,
        0b100000000000001110011,
        0b1000000000000001110011,
        0b10000001000000000000001110011,
        0b110000001000000000000001110011,
        0b10000010100000000000001110011,
        0b1000110000000011,
        0b10000010001110010000011,
        0b100000011010110100000011,
        0b110000100100110110000011,
        0b1000000101101111000000011,
        0b1110100110000101000100011,
        0b1111000111001110000100011,
        0b1111101000010111000100011,
        // 0b10011101111,
        0b1001000010101100111,
        // 0b1100000010111100011,
        // 0b1110001011011100011,
        // 0b10000100011111100011,
        // 0b10010101100011100011,
        // 0b10100110100111100011,
        // 0b10110111101011100011,
        0b101011000000101110011011,
        0b1011001001110010011011,
        0b1111010101110100011011,
        0b1110011011000110110111011,
        0b1000001110111100000111000111011,
        0b1111011101001111010111011,
        0b1111111110101111100111011,
        0b1000000000111111101111110111011,
        0b10110000010000011,
        0b100000011011000100000011,
        0b1100100011100000100011,
        0b10011000101000001000110011,
        0b10011100110001001010110011,
        0b10100000111010001100110011,
        0b10100101000011001110110011,
        0b10101001001100010000110011,
        0b10101101010101010010110011,
        0b10110001011110010100110011,
        0b10110101100111010110110011,
        0b10111001101000011000111011,
        0b10111101110100011010111011,
        0b11000001111101011100111011,
        0b11000110000110011110111011,
        0b11001010001111100000111011,
        0b100000001100010111000011000011,
        0b101000010000011111000101000111,
        0b110000010100100111000111001011,
        0b111000011000101111001001001111,
        0b11100110111001011010011,
        0b1000100000111111001101010011,
        0b10000100101000111001111010011,
        0b11000101001001111010001010011,
        0b1011000000001010111010011010011,
        0b100000110001011000010101010011,
        0b100000110101100001010111010011,
        0b100000111001101010011001010011,
        0b101000111101110000011011010011,
        0b101001000001111001011101010011,
        0b11000000000000001111000011010011,
        0b11000000000100010111000101010011,
        0b11100000000000011000000111010011,
        0b10100000010100100010001001010011,
        0b10100000011000101001001011010011,
        0b10100000011100110000001101010011,
        0b11100000000000111001001111010011,
        0b11010000000001000111000011010011,
        0b11010000000101001111000101010011,
        0b11110000000001010000000111010011,
        0b100010001100010111000011000011,
        0b101010010000011111000101000111,
        0b110010010100100111000111001111,
        0b111010011000101111001001001011,
        0b10011100110111001011010011,
        0b1010100000111111001101010011,
        0b10010100101000111001111010011,
        0b11010101001001111010001010011,
        0b1011010000001010111010011010011,
        0b100010110001011000010101010011,
        0b100010110101100001010111010011,
        0b100010111001101010011001010011,
        0b101010111101110000011011010011,
        0b101011000001111001011101010011,
        0b1000000000110000111011111010011,
        0b1000010000010001111100001010011,
        0b10100011001010001010100101010011,
        0b10100011001110010001100111010011,
        0b10100011010010011000101001010011,
        0b11100010000010100001101011010011,
        0b11000010000010101111101101010011,
        0b11000010000110110111101111010011,
        0b11010010000010111111100011010011,
        0b11010010000111000111100101010011,
        0b1010100110000111,
        0b1010000010010001000100111,
        0b100000011011101010000111,
        0b1011000100011011000100111,
        0b11000000001000001111110001010011,
        0b11000000001100010111110011010011,
        0b11010000001011001111000111010011,
        0b11010000001111010111001001010011,
    ];

    let labels: HashMap<String, usize> = HashMap::new();

    let expected_instructions = vec![
        "lui x1, 74565",
        "auipc x2, 4096",
        "addi x3, x1, 10",
        "slti x4, x2, 5",
        "xori x5, x3, 255",
        "ori x6, x4, 240",
        "andi x7, x5, 15",
        "slli x8, x6, 2",
        "srli x9, x7, 3",
        "srai x10, x8, 1",
        "add x11, x3, x4",
        "sub x12, x5, x6",
        "sll x13, x7, x8",
        "slt x14, x9, x10",
        "sltu x15, x11, x12",
        "or x16, x13, x14",
        "and x17, x15, x16",
        "ecall",
        "ebreak",
        "uret",
        "sret",
        "mret",
        "wfi",
        "lb x24, 0(x1)",
        "lh x25, 4(x2)",
        "lw x26, 8(x3)",
        "lbu x27, 12(x4)",
        "lhu x28, 16(x5)",
        "sb x29, 20(x6)",
        "sh x30, 24(x7)",
        "sw x31, 28(x8)",
        // "jal x9, 0x100",
        "jalr x10, x9, 0",
        // "beq x11, x12, 0x100",
        // "bne x13, x14, 0x101",
        // "blt x15, x16, 0x102",
        // "bge x17, x18, 0x103",
        // "bltu x19, x20, 0x104",
        // "bgeu x21, x22, 0x105",
        "addiw x23, x24, 10",
        "slliw x25, x25, 2",
        "srliw x26, x26, 3",
        "addw x27, x27, x28",
        "subw x28, x28, x29",
        "sllw x29, x29, x30",
        "srlw x30, x30, x31",
        "sraw x31, x31, x1",
        "lwu x1, 0(x2)",
        "ld x2, 8(x3)",
        "sd x3, 16(x4)",
        "mul x4, x5, x6",
        "mulh x5, x6, x7",
        "mulhsu x6, x7, x8",
        "mulhu x7, x8, x9",
        "div x8, x9, x10",
        "divu x9, x10, x11",
        "rem x10, x11, x12",
        "remu x11, x12, x13",
        "mulw x12, x13, x14",
        "divw x13, x14, x15",
        "divuw x14, x15, x16",
        "remw x15, x16, x17",
        "remuw x16, x17, x18",
        "fmadd.s f1, f2, f3, f4",
        "fmsub.s f2, f3, f4, f5",
        "fnmsub.s f3, f4, f5, f6",
        "fnmadd.s f4, f5, f6, f7",
        "fadd.s f5, f6, f7",
        "fsub.s f6, f7, f8",
        "fmul.s f7, f8, f9",
        "fdiv.s f8, f9, f10",
        "fsqrt.s f9, f10",
        "fsgnj.s f10, f11, f12",
        "fsgnjn.s f11, f12, f13",
        "fsgnjx.s f12, f13, f14",
        "fmin.s f13, f14, f15",
        "fmax.s f14, f15, f16",
        "fcvt.w.s x1, f1",
        "fcvt.wu.s x2, f2",
        "fmv.x.w x3, f3",
        "feq.s x4, f4, f5",
        "flt.s x5, f5, f6",
        "fle.s x6, f6, f7",
        "fclass.s x7, f7",
        "fcvt.s.w f1, x8",
        "fcvt.s.wu f2, x9",
        "fmv.w.x f3, x10",
        "fmadd.d f1, f2, f3, f4",
        "fmsub.d f2, f3, f4, f5",
        "fnmadd.d f3, f4, f5, f6",
        "fnmsub.d f4, f5, f6, f7",
        "fadd.d f5, f6, f7",
        "fsub.d f6, f7, f8",
        "fmul.d f7, f8, f9",
        "fdiv.d f8, f9, f10",
        "fsqrt.d f9, f10",
        "fsgnj.d f10, f11, f12",
        "fsgnjn.d f11, f12, f13",
        "fsgnjx.d f12, f13, f14",
        "fmin.d f13, f14, f15",
        "fmax.d f14, f15, f16",
        "fcvt.s.d f15, f16",
        "fcvt.d.s f16, f17",
        "feq.d x18, f17, f18",
        "flt.d x19, f18, f19",
        "fle.d x20, f19, f20",
        "fclass.d x21, f20",
        "fcvt.w.d x22, f21",
        "fcvt.wu.d x23, f22",
        "fcvt.d.w f17, x23",
        "fcvt.d.wu f18, x24",
        "flw f19, 0(x1)",
        "fsw f20, 4(x2)",
        "fld f21, 8(x3)",
        "fsd f22, 12(x4)",
        "fcvt.l.s x24, f1",
        "fcvt.lu.s x25, f2",
        "fcvt.s.l f3, x25",
        "fcvt.s.lu f4, x26",
    ];

    for (instruction, expected) in instructions.iter().zip(expected_instructions.iter()) {
        let instr_str = match RiscInstruction::get_string_version(*instruction, labels.clone()) {
            Ok(string) => string,
            Err(e) => panic!(
                "Error for instruction {} that was expected {}: {}",
                *instruction, *expected, e
            ),
        };
        assert!(
            instr_str.contains(expected),
            "Instruction {} does not match expected: {}",
            instr_str,
            expected
        );
    }
}
