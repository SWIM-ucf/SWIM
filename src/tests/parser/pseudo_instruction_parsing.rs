use crate::parser::assembling::assemble_data_binary;
use crate::parser::parser_assembler_main::parser;
use crate::parser::parser_structs_and_enums::instruction_tokenization::TokenType::Operator;
use crate::parser::parser_structs_and_enums::instruction_tokenization::{
    Instruction, ProgramInfo, Token,
};
use crate::parser::parsing::{create_label_map, separate_data_and_text, tokenize_program};
use crate::parser::pseudo_instruction_parsing::{
    complete_lw_sw_pseudo_instructions, expand_pseudo_instructions_and_assign_instruction_numbers,
};
use std::collections::HashMap;

#[test]
fn expand_pseudo_instructions_and_assign_instruction_number_adds_syscall_if_it_is_missing() {
    let result = parser("addi $t1, $t2, 100\nsw $t1, label".to_string())
        .0
        .updated_monaco_string;

    let correct_result = "addi $t1, $t2, 100\nsw $t1, label\nsyscall\n".to_string();
    assert_eq!(result, correct_result);
}

#[test]
fn expand_pseudo_instructions_and_assign_instruction_number_adds_syscall_at_beginning_if_no_instruction(
) {
    let result = parser(".data\nword .word 100\nother .byte 'a','a'\n".to_string())
        .0
        .updated_monaco_string;

    let correct_result = ".text\nsyscall\n.data\nword .word 100\nother .byte 'a','a'\n".to_string();

    assert_eq!(result, correct_result);
}

#[test]
fn expand_pseudo_instructions_and_assign_instruction_number_adds_syscall_after_first_instance_of_text(
) {
    let result = parser(".data\nword .word 100\n.text\n.data\nother .byte 'a','a'\n.text\n.data\nfinal: .space 10\n".to_string()).0.updated_monaco_string;

    let correct_result = ".data\nword .word 100\n.text\nsyscall\n.data\nother .byte 'a','a'\n.text\n.data\nfinal: .space 10\n".to_string();

    assert_eq!(result, correct_result);
}

#[test]
fn expand_pseudo_instructions_and_assign_instruction_number_does_not_add_syscall_if_it_is_present()
{
    let result = parser("addi $t1, $t2, 100\nsw $t1, label\nsyscall\n".to_string())
        .0
        .updated_monaco_string;

    let correct_result: String = "addi $t1, $t2, 100\nsw $t1, label\nsyscall\n".to_string();

    assert_eq!(result, correct_result);
}

#[test]
fn expand_pseudo_instructions_and_assign_instruction_number_adds_syscall_at_proper_spot_with_data_after(
) {
    let result = parser("addi $t1, $t2, 100\nsw $t1, label\n.data\n word: .word 100\n".to_string())
        .0
        .updated_monaco_string;

    let correct_result =
        "addi $t1, $t2, 100\nsw $t1, label\nsyscall\n.data\n word: .word 100\n".to_string();

    assert_eq!(result, correct_result);
}

#[test]
fn add_syscall_to_program_info() {
    let result = parser(".text\naddi $t1, $t2, $t3\nsyscall\n.data\n".to_string())
        .0
        .instructions;

    for instr in result {
        println!("{}", instr.operator.token_name);
    }
}

#[test]
fn expand_pseudo_instructions_and_assign_instruction_numbers_works_subi() {
    let mut program_info = ProgramInfo::default();

    let file_string = "subi $t1, $t2, 100\nsw $t1, label".to_string();

    let mut monaco_line_info_vec = tokenize_program(file_string);
    (program_info.instructions, program_info.data) =
        separate_data_and_text(monaco_line_info_vec.clone());
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "ori".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$at".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$zero".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "100".to_string(),
                    start_end_columns: (15, 18),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 0,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
    assert_eq!(
        program_info.instructions[1],
        Instruction {
            operator: Token {
                token_name: "sub".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (5, 8),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t2".to_string(),
                    start_end_columns: (10, 13),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$at".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 1,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
}

#[test]
fn expand_pseudo_instructions_and_assign_instruction_numbers_works_muli() {
    let mut program_info = ProgramInfo::default();

    let file_string = "muli $t1, $t2, 100\nsw $t1, label".to_string();

    let mut monaco_line_info_vec = tokenize_program(file_string);
    (program_info.instructions, program_info.data) =
        separate_data_and_text(monaco_line_info_vec.clone());
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "ori".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$at".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$zero".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "100".to_string(),
                    start_end_columns: (15, 18),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 0,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
    assert_eq!(
        program_info.instructions[1],
        Instruction {
            operator: Token {
                token_name: "mul".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (5, 8),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t2".to_string(),
                    start_end_columns: (10, 13),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$at".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 1,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
}

#[test]
fn expand_pseudo_instructions_and_assign_instruction_numbers_works_divi() {
    let mut program_info = ProgramInfo::default();

    let file_string = "divi $t1, $t1, 100\nsw $t1, label".to_string();

    let mut monaco_line_info_vec = tokenize_program(file_string);
    (program_info.instructions, program_info.data) =
        separate_data_and_text(monaco_line_info_vec.clone());
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "ori".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$at".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$zero".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "100".to_string(),
                    start_end_columns: (15, 18),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 0,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
    assert_eq!(
        program_info.instructions[1],
        Instruction {
            operator: Token {
                token_name: "div".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (5, 8),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (10, 13),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$at".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 1,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
}

#[test]
fn expand_pseudo_instructions_and_assign_instruction_numbers_works_dsubi() {
    let mut program_info = ProgramInfo::default();

    let file_string = "dsubi $t1, $t2, 100\nsw $t1, label".to_string();

    let mut monaco_line_info_vec = tokenize_program(file_string);
    (program_info.instructions, program_info.data) =
        separate_data_and_text(monaco_line_info_vec.clone());
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "ori".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$at".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$zero".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "100".to_string(),
                    start_end_columns: (16, 19),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 0,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
    assert_eq!(
        program_info.instructions[1],
        Instruction {
            operator: Token {
                token_name: "dsub".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (6, 9),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t2".to_string(),
                    start_end_columns: (11, 14),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$at".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 1,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
}

#[test]
fn expand_pseudo_instructions_and_assign_instruction_numbers_works_dsubiu() {
    let mut program_info = ProgramInfo::default();

    let file_string = "dsubiu $t1, $t2, 100\nsw $t1, label".to_string();

    let mut monaco_line_info_vec = tokenize_program(file_string);
    (program_info.instructions, program_info.data) =
        separate_data_and_text(monaco_line_info_vec.clone());
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "ori".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$at".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$zero".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "100".to_string(),
                    start_end_columns: (17, 20),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 0,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
    assert_eq!(
        program_info.instructions[1],
        Instruction {
            operator: Token {
                token_name: "dsubu".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (7, 10),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t2".to_string(),
                    start_end_columns: (12, 15),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$at".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 1,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
}

#[test]
fn expand_pseudo_instructions_and_assign_instruction_numbers_works_dmuli() {
    let mut program_info = ProgramInfo::default();

    let file_string = "dmuli $t1, $t2, 100\nsw $t1, label".to_string();

    let mut monaco_line_info_vec = tokenize_program(file_string);
    (program_info.instructions, program_info.data) =
        separate_data_and_text(monaco_line_info_vec.clone());
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "ori".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$at".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$zero".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "100".to_string(),
                    start_end_columns: (16, 19),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 0,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
    assert_eq!(
        program_info.instructions[1],
        Instruction {
            operator: Token {
                token_name: "dmul".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (6, 9),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t2".to_string(),
                    start_end_columns: (11, 14),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$at".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 1,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
}

#[test]
fn expand_pseudo_instructions_and_assign_instruction_numbers_works_dmuliu() {
    let mut program_info = ProgramInfo::default();

    let file_string = "dmuliu $t1, $t2, 100\nsw $t1, label".to_string();

    let mut monaco_line_info_vec = tokenize_program(file_string);
    (program_info.instructions, program_info.data) =
        separate_data_and_text(monaco_line_info_vec.clone());
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "ori".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$at".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$zero".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "100".to_string(),
                    start_end_columns: (17, 20),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 0,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
    assert_eq!(
        program_info.instructions[1],
        Instruction {
            operator: Token {
                token_name: "dmulu".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (7, 10),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t2".to_string(),
                    start_end_columns: (12, 15),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$at".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 1,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
}

#[test]
fn expand_pseudo_instructions_and_assign_instruction_numbers_works_ddivi() {
    let mut program_info = ProgramInfo::default();

    let file_string = "ddivi $t1, $t1, 100\nsw $t1, label".to_string();

    let mut monaco_line_info_vec = tokenize_program(file_string);
    (program_info.instructions, program_info.data) =
        separate_data_and_text(monaco_line_info_vec.clone());
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "ori".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$at".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$zero".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "100".to_string(),
                    start_end_columns: (16, 19),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 0,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
    assert_eq!(
        program_info.instructions[1],
        Instruction {
            operator: Token {
                token_name: "ddiv".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (6, 9),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (11, 14),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$at".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 1,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
}

#[test]
fn expand_pseudo_instructions_and_assign_instruction_numbers_works_ddiviu() {
    let mut program_info = ProgramInfo::default();

    let file_string = "ddiviu $t1, 100\nsw $t1, label".to_string();

    let mut monaco_line_info_vec = tokenize_program(file_string);
    (program_info.instructions, program_info.data) =
        separate_data_and_text(monaco_line_info_vec.clone());
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "ori".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$at".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$zero".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "100".to_string(),
                    start_end_columns: (12, 15),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 0,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
    assert_eq!(
        program_info.instructions[1],
        Instruction {
            operator: Token {
                token_name: "ddivu".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (7, 10),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$at".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 1,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
}

#[test]
fn expand_pseudo_instructions_and_assign_instruction_numbers_works_sgt() {
    let mut program_info = ProgramInfo::default();

    let file_string = "sgt $t1, $t2, $t3\nsw $t1, label".to_string();

    let mut monaco_line_info_vec = tokenize_program(file_string);
    (program_info.instructions, program_info.data) =
        separate_data_and_text(monaco_line_info_vec.clone());
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "slt".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (4, 7),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t3".to_string(),
                    start_end_columns: (14, 17),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t2".to_string(),
                    start_end_columns: (9, 12),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 0,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
}

#[test]
fn expand_pseudo_instructions_and_assign_instruction_numbers_works_sgtu() {
    let mut program_info = ProgramInfo::default();

    let file_string = "sgtu $t1, $t2, $t3\nsw $t1, label".to_string();

    let mut monaco_line_info_vec = tokenize_program(file_string);
    (program_info.instructions, program_info.data) =
        separate_data_and_text(monaco_line_info_vec.clone());
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "sltu".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (5, 8),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t3".to_string(),
                    start_end_columns: (15, 18),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t2".to_string(),
                    start_end_columns: (10, 13),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 0,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
}

#[test]
fn expand_pseudo_instructions_and_assign_instruction_numbers_works_seq() {
    let mut program_info = ProgramInfo::default();

    let file_string = "seq $t1, $t2, $t3\nsw $t1, label".to_string();

    let mut monaco_line_info_vec = tokenize_program(file_string);
    (program_info.instructions, program_info.data) =
        separate_data_and_text(monaco_line_info_vec.clone());
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "sub".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (4, 7),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t2".to_string(),
                    start_end_columns: (9, 12),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t3".to_string(),
                    start_end_columns: (14, 17),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 0,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
    assert_eq!(
        program_info.instructions[1],
        Instruction {
            operator: Token {
                token_name: "ori".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$at".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$zero".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "1".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 1,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
    assert_eq!(
        program_info.instructions[2],
        Instruction {
            operator: Token {
                token_name: "sltu".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (4, 7),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (4, 7),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$at".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 2,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
}

#[test]
fn expand_pseudo_instructions_and_assign_instruction_numbers_works_sne() {
    let mut program_info = ProgramInfo::default();

    let file_string = "sne $t1, $t2, $t3\nsw $t1, label".to_string();

    let mut monaco_line_info_vec = tokenize_program(file_string);
    (program_info.instructions, program_info.data) =
        separate_data_and_text(monaco_line_info_vec.clone());
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "sub".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (4, 7),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t2".to_string(),
                    start_end_columns: (9, 12),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t3".to_string(),
                    start_end_columns: (14, 17),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 0,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
    assert_eq!(
        program_info.instructions[1],
        Instruction {
            operator: Token {
                token_name: "sltu".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (4, 7),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$zero".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (4, 7),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 1,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
}

#[test]
fn expand_pseudo_instructions_and_assign_instruction_numbers_works_sle() {
    let mut program_info = ProgramInfo::default();

    let file_string = "sle $t1, $t2, $t3\nsw $t1, label".to_string();

    let mut monaco_line_info_vec = tokenize_program(file_string);
    (program_info.instructions, program_info.data) =
        separate_data_and_text(monaco_line_info_vec.clone());
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "slt".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (4, 7),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t3".to_string(),
                    start_end_columns: (14, 17),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t2".to_string(),
                    start_end_columns: (9, 12),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 0,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
    assert_eq!(
        program_info.instructions[1],
        Instruction {
            operator: Token {
                token_name: "addi".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (4, 7),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (4, 7),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "1".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 1,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );

    assert_eq!(
        program_info.instructions[2],
        Instruction {
            operator: Token {
                token_name: "andi".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (4, 7),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (4, 7),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "1".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 2,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
}

#[test]
fn expand_pseudo_instructions_and_assign_instruction_numbers_works_sleu() {
    let mut program_info = ProgramInfo::default();

    let file_string = "sleu $t1, $t2, $t3\nsw $t1, label".to_string();

    let mut monaco_line_info_vec = tokenize_program(file_string);
    (program_info.instructions, program_info.data) =
        separate_data_and_text(monaco_line_info_vec.clone());
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "sltu".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (5, 8),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t3".to_string(),
                    start_end_columns: (15, 18),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t2".to_string(),
                    start_end_columns: (10, 13),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 0,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
    assert_eq!(
        program_info.instructions[1],
        Instruction {
            operator: Token {
                token_name: "addi".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (5, 8),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (5, 8),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "1".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 1,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );

    assert_eq!(
        program_info.instructions[2],
        Instruction {
            operator: Token {
                token_name: "andi".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (5, 8),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (5, 8),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "1".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 2,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
}

#[test]
fn expand_pseudo_instructions_and_assign_instruction_numbers_works_sge() {
    let mut program_info = ProgramInfo::default();

    let file_string = "sge $t1, $t2, $t3\nsw $t1, label".to_string();

    let mut monaco_line_info_vec = tokenize_program(file_string);
    (program_info.instructions, program_info.data) =
        separate_data_and_text(monaco_line_info_vec.clone());
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "slt".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (4, 7),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t2".to_string(),
                    start_end_columns: (9, 12),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t3".to_string(),
                    start_end_columns: (14, 17),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 0,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
    assert_eq!(
        program_info.instructions[1],
        Instruction {
            operator: Token {
                token_name: "addi".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (4, 7),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (4, 7),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "1".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 1,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );

    assert_eq!(
        program_info.instructions[2],
        Instruction {
            operator: Token {
                token_name: "andi".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (4, 7),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (4, 7),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "1".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 2,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
}

#[test]
fn expand_pseudo_instructions_and_assign_instruction_numbers_works_sgeu() {
    let mut program_info = ProgramInfo::default();
    let file_string = "sgeu $t1, $t2, $t3\nsw $t1, label".to_string();
    let mut monaco_line_info_vec = tokenize_program(file_string);
    (program_info.instructions, program_info.data) =
        separate_data_and_text(monaco_line_info_vec.clone());
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut monaco_line_info_vec,
    );

    let mut correct_program_info = ProgramInfo::default();
    let correct_string =
        "sltu $t1, $t2, $t3\naddi $t1, $t1, 1\nandi $t1, $t1, 1\nsw $t1, label".to_string();
    let mut monaco_line_info_vec = tokenize_program(correct_string);
    (correct_program_info.instructions, correct_program_info.data) =
        separate_data_and_text(monaco_line_info_vec.clone());
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut correct_program_info.instructions,
        &program_info.data,
        &mut monaco_line_info_vec,
    );

    //    assert_eq!(correct_program_info.instructions, program_info.instructions);

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "sltu".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (5, 8),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t2".to_string(),
                    start_end_columns: (10, 13),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t3".to_string(),
                    start_end_columns: (15, 18),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 0,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
    assert_eq!(
        program_info.instructions[1],
        Instruction {
            operator: Token {
                token_name: "addi".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (5, 8),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (5, 8),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "1".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 1,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );

    assert_eq!(
        program_info.instructions[2],
        Instruction {
            operator: Token {
                token_name: "andi".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (5, 8),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (5, 8),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "1".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 2,
            line_number: 0,
            errors: vec![],
            label: None,
        }
    );
}

#[test]
fn complete_lw_sw_pseudo_instructions_works_multiple_using_same_label() {
    let mut program_info = ProgramInfo::default();

    let file_string = ".data\nlabel: .word 100\n.text\nlw $t1, label\nlw $t2, label".to_string();

    let monaco_line_info_vec = tokenize_program(file_string);
    program_info.monaco_line_info = monaco_line_info_vec;
    (program_info.instructions, program_info.data) =
        separate_data_and_text(program_info.monaco_line_info.clone());
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut program_info.monaco_line_info,
    );
    let _ = assemble_data_binary(&mut program_info.data);
    let labels: HashMap<String, usize> =
        create_label_map(&mut program_info.instructions, &mut program_info.data);

    complete_lw_sw_pseudo_instructions(
        &mut program_info.instructions,
        &labels,
        &mut program_info.monaco_line_info,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "lui".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$at".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "0".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 0,
            line_number: 3,
            errors: vec![],
            label: None,
        }
    );
    assert_eq!(
        program_info.instructions[1],
        Instruction {
            operator: Token {
                token_name: "lw".to_string(),
                start_end_columns: (0, 2),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (3, 6),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "20($at)".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 1,
            line_number: 3,
            errors: vec![],
            label: None,
        }
    );
    assert_eq!(
        program_info.instructions[2],
        Instruction {
            operator: Token {
                token_name: "lui".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$at".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "0".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 2,
            line_number: 4,
            errors: vec![],
            label: None,
        }
    );
    assert_eq!(
        program_info.instructions[3],
        Instruction {
            operator: Token {
                token_name: "lw".to_string(),
                start_end_columns: (0, 2),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t2".to_string(),
                    start_end_columns: (3, 6),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "20($at)".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 3,
            line_number: 4,
            errors: vec![],
            label: None,
        }
    );
}

#[test]
fn complete_lw_sw_pseudo_instructions_works() {
    let mut program_info = ProgramInfo::default();

    let file_string = ".data\nlabel: .word 100\n.text\nlw $t1, label\nsw $t1, label".to_string();

    program_info.monaco_line_info = tokenize_program(file_string);
    (program_info.instructions, program_info.data) =
        separate_data_and_text(program_info.monaco_line_info.clone());
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut program_info.monaco_line_info,
    );
    let _vec_of_data = assemble_data_binary(&mut program_info.data);
    let labels: HashMap<String, usize> =
        create_label_map(&mut program_info.instructions, &mut program_info.data);

    complete_lw_sw_pseudo_instructions(
        &mut program_info.instructions,
        &labels,
        &mut program_info.monaco_line_info,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "lui".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$at".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "0".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 0,
            line_number: 3,
            errors: vec![],
            label: None,
        }
    );
    assert_eq!(
        program_info.instructions[1],
        Instruction {
            operator: Token {
                token_name: "lw".to_string(),
                start_end_columns: (0, 2),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (3, 6),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "20($at)".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 1,
            line_number: 3,
            errors: vec![],
            label: None,
        }
    );
    assert_eq!(
        program_info.instructions[2],
        Instruction {
            operator: Token {
                token_name: "lui".to_string(),
                start_end_columns: (0, 0),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$at".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "0".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 2,
            line_number: 4,
            errors: vec![],
            label: None,
        }
    );
    assert_eq!(
        program_info.instructions[3],
        Instruction {
            operator: Token {
                token_name: "sw".to_string(),
                start_end_columns: (0, 2),
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    start_end_columns: (3, 6),
                    token_type: Default::default(),
                },
                Token {
                    token_name: "20($at)".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                }
            ],
            binary: 0,
            instruction_number: 3,
            line_number: 4,
            errors: vec![],
            label: None,
        }
    );
}

#[test]
fn complete_lw_sw_pseudo_instructions_doesnt_break_with_empty_instruction_list() {
    let mut program_info = ProgramInfo::default();

    let file_string = ".data\nlabel: .word 100\n.text\nlw $t1, label\nsw $t1, label".to_string();

    program_info.monaco_line_info = tokenize_program(file_string);
    (program_info.instructions, program_info.data) =
        separate_data_and_text(program_info.monaco_line_info.clone());
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut program_info.monaco_line_info,
    );
    let _ = assemble_data_binary(&mut program_info.data);
    let labels: HashMap<String, usize> =
        create_label_map(&mut program_info.instructions, &mut program_info.data);

    complete_lw_sw_pseudo_instructions(
        &mut program_info.instructions,
        &labels,
        &mut program_info.monaco_line_info,
    );
}

#[test]
fn expanded_pseudo_instructions_are_added_into_updated_monaco_string() {
    let result = parser(
        ".text\nli $t1, 100\nseq $t1, $t2, $t3\nsne $t1, $t2, $t3\nsle $t1, $t2, $t3\nsleu $t1, $t2, $t3\nsgt $t1, $t2, $t3\nsgtu $t1, $t2, $t3\nsge $t1, $t2, $t3\nsgeu $t1, $t2, $t3\nsubi $t1, $t2, 100\ndsubi $t1, $t2, 100\ndsubiu $t1, $t2, 100\nmuli $t1, $t2, 100\ndmuli $t1, $t2, 100\ndmuliu $t1, $t2, 100\ndivi $t1, 100\nddivi $t1, 100\nddiviu $t1, 100\nlw $t1, memory\n.data\nmemory: .word 200"
            .to_string(),
    )
    .0;
    for line in result.monaco_line_info {
        println!("{}", line.updated_monaco_string);
    }

    //println!("{}", result.updated_monaco_string);
}
