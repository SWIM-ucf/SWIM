use std::collections::HashMap;

use crate::emulation_core::riscv::instruction::RiscInstruction;

// *** Test Fibonacci program instructions ***

#[test]
fn test_instruction_1() {
    let instruction: u32 = 0x00112023;
    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    assert!(
        match RiscInstruction::get_string_version(instruction, labels.clone()) {
            Ok(string) => string.contains("sw x1, 0(x2)"),
            _ => false,
        }
    );
}

#[test]
fn test_instruction_2() {
    let instruction: u32 = 0x00a12223;

    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    assert!(
        match RiscInstruction::get_string_version(instruction, labels.clone()) {
            Ok(string) => string.contains("sw x10, 4(x2)"),
            _ => false,
        }
    );
}

#[test]
fn test_instruction_3() {
    let instruction: u32 = 0x00006293;

    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    assert!(
        match RiscInstruction::get_string_version(instruction, labels.clone()) {
            Ok(string) => string.contains("ori x5, x0, 0"),
            _ => false,
        }
    );
}

#[test]
fn test_instruction_4() {
    let instruction: u32 = 0x00106313;

    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    assert!(
        match RiscInstruction::get_string_version(instruction, labels.clone()) {
            Ok(string) => string.contains("ori x6, x0, 1"),
            _ => false,
        }
    );
}

// #[test]
// fn test_instruction_5() {
//     let instruction: u32 = 0x01528563;

//     let labels: HashMap::<String, usize> = HashMap::<String, usize>::new();

//     assert!(match RiscInstruction::get_string_version(instruction, labels.clone()) {
//         Ok(string) => string.contains("beq x10, x5, RET_0"),
//         _ => false,
//     });
// }

// #[test]
// fn test_instruction_6() {
//     let instruction: u32 = 0x01830563;

//     let labels: HashMap::<String, usize> = HashMap::<String, usize>::new();

//     assert!(match RiscInstruction::get_string_version(instruction, labels.clone()) {
//         Ok(string) => string.contains("beq x10, x6, RET_1"),
//         _ => false,
//     });
// }

#[test]
fn test_instruction_7() {
    let instruction: u32 = 0xfff50513;

    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    assert!(
        match RiscInstruction::get_string_version(instruction, labels.clone()) {
            Ok(string) => string.contains("addi x10, x10, -1"),
            _ => false,
        }
    );
}

#[test]
fn test_instruction_8() {
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
fn test_instruction_9() {
    let instruction: u32 = 0x000280e7;

    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    assert!(
        match RiscInstruction::get_string_version(instruction, labels.clone()) {
            Ok(string) => string.contains("jalr x1, x5, 0"),
            _ => false,
        }
    );
}

#[test]
fn test_instruction_10() {
    let instruction: u32 = 0x00b12423;

    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    assert!(
        match RiscInstruction::get_string_version(instruction, labels.clone()) {
            Ok(string) => string.contains("sw x11, 8(x2)"),
            _ => false,
        }
    );
}

#[test]
fn test_instruction_11() {
    let instruction: u32 = 0x00412503;

    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    assert!(
        match RiscInstruction::get_string_version(instruction, labels.clone()) {
            Ok(string) => string.contains("lw x10, 4(x2)"),
            _ => false,
        }
    );
}

#[test]
fn test_instruction_12() {
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
fn test_instruction_15() {
    let instruction: u32 = 0x00b12623;

    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    assert!(
        match RiscInstruction::get_string_version(instruction, labels.clone()) {
            Ok(string) => string.contains("sw x11, 12(x2)"),
            _ => false,
        }
    );
}

#[test]
fn test_instruction_16() {
    let instruction: u32 = 0x00812283;

    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    assert!(
        match RiscInstruction::get_string_version(instruction, labels.clone()) {
            Ok(string) => string.contains("lw x5, 8(x2)"),
            _ => false,
        }
    );
}

#[test]
fn test_instruction_17() {
    let instruction: u32 = 0x00c12303;

    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    assert!(
        match RiscInstruction::get_string_version(instruction, labels.clone()) {
            Ok(string) => string.contains("lw x6, 12(x2)"),
            _ => false,
        }
    );
}

#[test]
fn test_instruction_18() {
    let instruction: u32 = 0x006285b3;

    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    assert!(
        match RiscInstruction::get_string_version(instruction, labels.clone()) {
            Ok(string) => string.contains("add x11, x5, x6"),
            _ => false,
        }
    );
}

#[test]
fn test_instruction_19() {
    let instruction: u32 = 0x00012083;

    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    assert!(
        match RiscInstruction::get_string_version(instruction, labels.clone()) {
            Ok(string) => string.contains("lw x1, 0(x2)"),
            _ => false,
        }
    );
}

#[test]
fn test_instruction_20() {
    let instruction: u32 = 0x01010113;

    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    assert!(
        match RiscInstruction::get_string_version(instruction, labels.clone()) {
            Ok(string) => string.contains("addi x2, x2, 16"),
            _ => false,
        }
    );
}

#[test]
fn test_instruction_21() {
    let instruction: u32 = 0x00008067;

    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    assert!(
        match RiscInstruction::get_string_version(instruction, labels.clone()) {
            Ok(string) => string.contains("jalr x0, x1, 0"),
            _ => false,
        }
    );
}

#[test]
fn test_instruction_23() {
    let instruction: u32 = 0x00006593;

    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    assert!(
        match RiscInstruction::get_string_version(instruction, labels.clone()) {
            Ok(string) => string.contains("ori x11, x0, 0"),
            _ => false,
        }
    );
}

#[test]
fn test_instruction_27() {
    let instruction: u32 = 0x00106593;

    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    assert!(
        match RiscInstruction::get_string_version(instruction, labels.clone()) {
            Ok(string) => string.contains("ori x11, x0, 1"),
            _ => false,
        }
    );
}

#[test]
fn test_instruction_30() {
    let instruction: u32 = 0x00506513;

    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    assert!(
        match RiscInstruction::get_string_version(instruction, labels.clone()) {
            Ok(string) => string.contains("ori x10, x0, 5"),
            _ => false,
        }
    );
}

#[test]
fn test_instruction_31() {
    let instruction: u32 = 0x00106513;

    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    assert!(
        match RiscInstruction::get_string_version(instruction, labels.clone()) {
            Ok(string) => string.contains("ori x10, x0, 1"),
            _ => false,
        }
    );
}

#[test]
fn test_instruction_32() {
    let instruction: u32 = 0x00000073;

    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    assert!(
        match RiscInstruction::get_string_version(instruction, labels.clone()) {
            Ok(string) => string.contains("ecall"),
            _ => false,
        }
    );
}

#[test]
fn test_instruction_33() {
    let instruction: u32 = 0x00006513;

    let labels: HashMap<String, usize> = HashMap::<String, usize>::new();

    assert!(
        match RiscInstruction::get_string_version(instruction, labels.clone()) {
            Ok(string) => string.contains("ori x10, x0, 0"),
            _ => false,
        }
    );
}

// ** Test other instructions **

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
