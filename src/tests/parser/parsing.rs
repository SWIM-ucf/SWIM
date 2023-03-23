use crate::parser::assembling::assemble_data_binary;
use crate::parser::parser_assembler_main::parser;
use crate::parser::parser_structs_and_enums::instruction_tokenization::ErrorType::{
    LabelAssignmentError, LabelMultipleDefinition, MissingComma,
};
use crate::parser::parser_structs_and_enums::instruction_tokenization::TokenType::{
    Label, Operator, Unknown,
};
use crate::parser::parser_structs_and_enums::instruction_tokenization::{
    Data, Error, ErrorType, Instruction, MonacoLineInfo, Token,
};
use crate::parser::parsing::create_label_map;
#[cfg(test)]
use crate::parser::parsing::{separate_data_and_text, tokenize_program};
use crate::parser::pseudo_instruction_parsing::expand_pseudo_instructions_and_assign_instruction_numbers;
use std::collections::HashMap;

#[test]
fn tokenize_program_works_basic_version() {
    let result = tokenize_program("This line\nThis second line\nHere's a third!".to_string());

    let i_0_t_0 = Token {
        token_name: "This".to_string(),
        start_end_columns: (0, 4),
        token_type: Unknown,
    };
    let i_0_t_1 = Token {
        token_name: "line".to_string(),
        start_end_columns: (5, 9),
        token_type: Unknown,
    };

    let i_1_t_0 = Token {
        token_name: "This".to_string(),
        start_end_columns: (0, 4),
        token_type: Unknown,
    };
    let i_1_t_1 = Token {
        token_name: "second".to_string(),
        start_end_columns: (5, 11),
        token_type: Unknown,
    };
    let i_1_t_2 = Token {
        token_name: "line".to_string(),
        start_end_columns: (12, 16),
        token_type: Unknown,
    };
    let i_2_t_0 = Token {
        token_name: "Here's".to_string(),
        start_end_columns: (0, 6),
        token_type: Unknown,
    };
    let i_2_t_1 = Token {
        token_name: "a".to_string(),
        start_end_columns: (7, 8),
        token_type: Unknown,
    };
    let i_2_t_2 = Token {
        token_name: "third!".to_string(),
        start_end_columns: (9, 15),
        token_type: Unknown,
    };
    let line_0 = MonacoLineInfo {
        mouse_hover_string: "".to_string(),
        line_number: 0,
        error_start_end_columns: vec![],
        tokens: vec![i_0_t_0, i_0_t_1],
        updated_monaco_string: "This line".to_string(),
        errors: vec![],
    };

    let line_1 = MonacoLineInfo {
        mouse_hover_string: "".to_string(),
        line_number: 1,
        error_start_end_columns: vec![],
        tokens: vec![i_1_t_0, i_1_t_1, i_1_t_2],
        updated_monaco_string: "This second line".to_string(),
        errors: vec![],
    };

    let line_2 = MonacoLineInfo {
        mouse_hover_string: "".to_string(),
        line_number: 2,
        error_start_end_columns: vec![],
        tokens: vec![i_2_t_0, i_2_t_1, i_2_t_2],
        updated_monaco_string: "Here's a third!".to_string(),
        errors: vec![],
    };

    let correct_result = vec![line_0, line_1, line_2];
    assert_eq!(result, correct_result);
}

#[test]
fn tokenize_program_handles_no_spaces_between_commas() {
    let result = tokenize_program("add $t1, $t2, $t3\nsub $s1,$s2,$s3\n".to_string());

    let i_0_t_0 = Token {
        token_name: "add".to_string(),
        start_end_columns: (0, 3),
        token_type: Unknown,
    };
    let i_0_t_1 = Token {
        token_name: "$t1,".to_string(),
        start_end_columns: (4, 7),
        token_type: Unknown,
    };

    let i_0_t_2 = Token {
        token_name: "$t2,".to_string(),
        start_end_columns: (9, 12),
        token_type: Unknown,
    };
    let i_0_t_3 = Token {
        token_name: "$t3".to_string(),
        start_end_columns: (14, 17),
        token_type: Unknown,
    };
    let i_1_t_0 = Token {
        token_name: "sub".to_string(),
        start_end_columns: (0, 3),
        token_type: Unknown,
    };
    let i_1_t_1 = Token {
        token_name: "$s1,".to_string(),
        start_end_columns: (4, 7),
        token_type: Unknown,
    };

    let i_1_t_2 = Token {
        token_name: "$s2,".to_string(),
        start_end_columns: (8, 11),
        token_type: Unknown,
    };
    let i_1_t_3 = Token {
        token_name: "$s3".to_string(),
        start_end_columns: (12, 15),
        token_type: Unknown,
    };

    let line_0 = MonacoLineInfo {
        mouse_hover_string: "".to_string(),
        line_number: 0,
        error_start_end_columns: vec![],
        tokens: vec![i_0_t_0, i_0_t_1, i_0_t_2, i_0_t_3],
        updated_monaco_string: "add $t1, $t2, $t3".to_string(),
        errors: vec![],
    };

    let line_1 = MonacoLineInfo {
        mouse_hover_string: "".to_string(),
        line_number: 1,
        error_start_end_columns: vec![],
        tokens: vec![i_1_t_0, i_1_t_1, i_1_t_2, i_1_t_3],
        updated_monaco_string: "sub $s1,$s2,$s3".to_string(),
        errors: vec![],
    };

    let correct_result = vec![line_0, line_1];
    assert_eq!(result, correct_result);
}

#[test]
fn tokenize_program_handles_comma_after_space() {
    let result = tokenize_program("add $t1 , $t2, $t3\n".to_string());

    let i_0_t_0 = Token {
        token_name: "add".to_string(),
        start_end_columns: (0, 3),
        token_type: Unknown,
    };
    let i_0_t_1 = Token {
        token_name: "$t1,".to_string(),
        start_end_columns: (4, 7),
        token_type: Unknown,
    };
    let i_0_t_2 = Token {
        token_name: "$t2,".to_string(),
        start_end_columns: (10, 13),
        token_type: Unknown,
    };
    let i_0_t_3 = Token {
        token_name: "$t3".to_string(),
        start_end_columns: (15, 18),
        token_type: Unknown,
    };
    let line_0 = MonacoLineInfo {
        mouse_hover_string: "".to_string(),
        line_number: 0,
        error_start_end_columns: vec![],
        tokens: vec![i_0_t_0, i_0_t_1, i_0_t_2, i_0_t_3],
        updated_monaco_string: "add $t1 , $t2, $t3".to_string(),
        errors: vec![],
    };

    let correct_result = vec![line_0];
    assert_eq!(result, correct_result);
}

#[test]
fn tokenize_program_ignores_comments() {
    let results = tokenize_program(
        "This Line\n#this line is a comment\nbut_this_isn't\nthis#has a comment in the middle\n"
            .to_string(),
    );

    let i_0_t_0 = Token {
        token_name: "This".to_string(),
        start_end_columns: (0, 4),
        token_type: Unknown,
    };
    let i_0_t_1 = Token {
        token_name: "Line".to_string(),
        start_end_columns: (5, 9),
        token_type: Unknown,
    };
    let line_0 = MonacoLineInfo {
        mouse_hover_string: "".to_string(),
        line_number: 0,
        error_start_end_columns: vec![],
        tokens: vec![i_0_t_0, i_0_t_1],
        updated_monaco_string: "This Line".to_string(),
        errors: vec![],
    };
    let line_1 = MonacoLineInfo {
        mouse_hover_string: "".to_string(),
        line_number: 1,
        error_start_end_columns: vec![],
        tokens: vec![],
        updated_monaco_string: "#this line is a comment".to_string(),
        errors: vec![],
    };
    let line_2 = MonacoLineInfo {
        mouse_hover_string: "".to_string(),
        line_number: 2,
        error_start_end_columns: vec![],
        tokens: vec![Token {
            token_name: "but_this_isn't".to_string(),
            start_end_columns: (0, 14),
            token_type: Unknown,
        }],
        updated_monaco_string: "but_this_isn't".to_string(),
        errors: vec![],
    };
    let line_3 = MonacoLineInfo {
        mouse_hover_string: "".to_string(),
        line_number: 3,
        error_start_end_columns: vec![],
        tokens: vec![Token {
            token_name: "this".to_string(),
            start_end_columns: (0, 4),
            token_type: Unknown,
        }],
        updated_monaco_string: "this#has a comment in the middle".to_string(),
        errors: vec![],
    };

    let correct_result = vec![line_0, line_1, line_2, line_3];
    assert_eq!(results, correct_result);
}

#[test]
fn tokenize_program_recognizes_comments() {
    let results = tokenize_program("Addi $t1, $t2, 300\n# this is a comment\nadd $t1, $t2, $t3\n#I'm making a note here. Huge comment".to_string());
    assert_eq!(results[0].line_number, 0);
    assert_eq!(results[2].line_number, 2)
}

#[test]
fn tokenize_program_recognizes_comments_middle_of_line() {
    let results = tokenize_program("Addi $t1, $t2, 300 # this is a comment\nadd $t1, $t2, $t3#I'm making a note here. Huge comment".to_string());

    assert_eq!(results[0].line_number, 0);
    assert_eq!(results[1].line_number, 1);
}

#[test]
fn tokenize_program_reads_ascii_properly() {
    let result = tokenize_program(".data\nlabel: .ascii \"this is a string\"".to_string());

    assert_eq!(result[1].tokens[2].token_name, "\"this is a string\"");
}

#[test]
fn separate_data_and_text_works_basic_version() {
    let lines = tokenize_program("add $t1, $t2, $t3\nlw $t1, 400($t1)\naddi $t1, 100".to_string());
    let result = separate_data_and_text(lines.clone());

    let mut instruction_0 = Instruction {
        operator: lines[0].tokens[0].clone(),
        operands: vec![
            lines[0].tokens[1].clone(),
            lines[0].tokens[2].clone(),
            lines[0].tokens[3].clone(),
        ],
        line_number: 0,
        ..Default::default()
    };
    instruction_0.operator.token_type = Operator;
    instruction_0.operands[0].token_name.pop();
    instruction_0.operands[1].token_name.pop();

    let mut instruction_1 = Instruction {
        operator: lines[1].tokens[0].clone(),
        operands: vec![lines[1].tokens[1].clone(), lines[1].tokens[2].clone()],
        line_number: 1,
        ..Default::default()
    };
    instruction_1.operator.token_type = Operator;
    instruction_1.operands[0].token_name.pop();

    let mut instruction_2 = Instruction {
        operator: lines[2].tokens[0].clone(),
        operands: vec![lines[2].tokens[1].clone(), lines[2].tokens[2].clone()],
        line_number: 2,
        ..Default::default()
    };
    instruction_2.operator.token_type = Operator;
    instruction_2.operands[0].token_name.pop();

    let correct_result = vec![instruction_0, instruction_1, instruction_2];

    assert_eq!(result.0, correct_result);
}

#[test]
fn separate_data_and_text_can_handle_empty_lines() {
    //this test realistically is only important to check that it does not panic but we might as well go a step further and
    //check that the result generated with empty lines is identical to the result without empty lines save for line number
    let mut result_1 =
        parser(".text\nori $s0, $zero, 0x1234\n\n.data\nlabel: .word 0xface".to_string())
            .0
            .monaco_line_info;
    let result_2 = parser(".text\nori $s0, $zero, 0x1234\n.data\nlabel: .word 0xface".to_string())
        .0
        .monaco_line_info;
    result_1[2].line_number = 1;
    result_1[3].line_number = 2;
    result_1[4].line_number = 3;

    assert_eq!(result_1[0], result_2[0]);
    assert_eq!(result_1[1], result_2[1]);
    assert_eq!(result_1[3], result_2[2]);
    assert_eq!(result_1[4], result_2[3]);
}

#[test]
fn separate_data_and_text_generates_error_on_missing_commas_text() {
    let lines = tokenize_program("add $t1, $t2, $t3\nlw $t1 400($t2)".to_string());
    let result = separate_data_and_text(lines);
    let correct_error = Error {
        error_name: MissingComma,
        token_causing_error: "$t1".to_string(),
        start_end_columns: (3, 6),
        message: "".to_string(),
    };
    assert_eq!(correct_error, result.0[1].errors[0]);
}

#[test]
fn separate_data_and_text_works_on_line_label() {
    let lines = tokenize_program(
        "add $t1, $t2, $t3\nLoad_from_memory: lw $t1, 400($t1)\naddi $t1, 100".to_string(),
    );
    let result = separate_data_and_text(lines.clone());

    let mut instruction_0 = Instruction {
        operator: lines[0].tokens[0].clone(),
        operands: vec![
            lines[0].tokens[1].clone(),
            lines[0].tokens[2].clone(),
            lines[0].tokens[3].clone(),
        ],
        instruction_number: 0,
        ..Default::default()
    };
    instruction_0.operator.token_type = Operator;
    instruction_0.operands[0].token_name.pop();
    instruction_0.operands[1].token_name.pop();

    let token = Token {
        token_name: "Load_from_memory".to_string(),
        start_end_columns: (0, 17),
        token_type: Label,
    };
    let mut instruction_1 = Instruction {
        operator: lines[1].tokens[1].clone(),
        operands: vec![lines[1].tokens[2].clone(), lines[1].tokens[3].clone()],
        line_number: 1,
        label: Some((token, 1)),
        ..Default::default()
    };
    instruction_1.operator.token_type = Operator;
    instruction_1.operands[0].token_name.pop();

    let mut instruction_2 = Instruction {
        operator: lines[2].tokens[0].clone(),
        operands: vec![lines[2].tokens[1].clone(), lines[2].tokens[2].clone()],
        line_number: 2,
        ..Default::default()
    };
    instruction_2.operator.token_type = Operator;
    instruction_2.operands[0].token_name.pop();

    let correct_result = vec![instruction_0, instruction_1, instruction_2];

    assert_eq!(correct_result, result.0);
}

#[test]
fn separate_data_and_text_works_off_line_label() {
    let lines = tokenize_program(
        "add $t1, $t2, $t3\nLoad_from_memory:\nlw $t1, 400($t1)\naddi $t1, 100".to_string(),
    );
    let result = separate_data_and_text(lines.clone());

    let mut instruction_0 = Instruction {
        operator: lines[0].tokens[0].clone(),
        operands: vec![
            lines[0].tokens[1].clone(),
            lines[0].tokens[2].clone(),
            lines[0].tokens[3].clone(),
        ],
        instruction_number: 0,
        ..Default::default()
    };
    instruction_0.operator.token_type = Operator;
    instruction_0.operands[0].token_name.pop();
    instruction_0.operands[1].token_name.pop();

    let token = Token {
        token_name: "Load_from_memory".to_string(),
        start_end_columns: (0, 17),
        token_type: Label,
    };
    let mut instruction_1 = Instruction {
        operator: lines[2].tokens[0].clone(),
        operands: vec![lines[2].tokens[1].clone(), lines[2].tokens[2].clone()],
        line_number: 2,
        label: Some((token, 1)),
        ..Default::default()
    };
    instruction_1.operator.token_type = Operator;
    instruction_1.operands[0].token_name.pop();

    let mut instruction_2 = Instruction {
        operator: lines[3].tokens[0].clone(),
        operands: vec![lines[3].tokens[1].clone(), lines[3].tokens[2].clone()],
        line_number: 3,
        ..Default::default()
    };
    instruction_2.operator.token_type = Operator;
    instruction_2.operands[0].token_name.pop();

    let correct_result = vec![instruction_0, instruction_1, instruction_2];

    assert_eq!(correct_result, result.0);
}

#[test]
fn separate_data_and_text_recognizes_text() {
    let lines = tokenize_program(".text\nadd $t1, $t2, $t3\nlw $t1, 400($t1)\n".to_string());
    let result = separate_data_and_text(lines.clone());

    let mut correct_result: Vec<Instruction> = vec![
        Instruction {
            operator: lines[1].tokens[0].clone(),
            operands: vec![
                lines[1].tokens[1].clone(),
                lines[1].tokens[2].clone(),
                lines[1].tokens[3].clone(),
            ],
            line_number: 1,
            ..Default::default()
        },
        Instruction {
            operator: lines[2].tokens[0].clone(),
            operands: vec![lines[2].tokens[1].clone(), lines[2].tokens[2].clone()],
            line_number: 2,
            ..Default::default()
        },
    ];
    correct_result[0].operator.token_type = Operator;
    correct_result[1].operator.token_type = Operator;
    correct_result[0].operands[0].token_name.pop();
    correct_result[0].operands[1].token_name.pop();
    correct_result[1].operands[0].token_name.pop();

    assert_eq!(result.0, correct_result);
}

#[test]
fn separate_data_and_text_recognizes_data_and_text_interspersed() {
    let lines = tokenize_program(
        ".data\nword1: .word 32\n.text\nadd $t1, $t2, $t3\n.data\nword2: .word 1,2,3\n.text\nlw $t1, 400($t1)\n".to_string());
    let result = separate_data_and_text(lines.clone());

    let mut correct_result: (Vec<Instruction>, Vec<Data>) = (
        vec![
            Instruction {
                operator: lines[3].tokens[0].clone(),
                operands: vec![
                    lines[3].tokens[1].clone(),
                    lines[3].tokens[2].clone(),
                    lines[3].tokens[3].clone(),
                ],
                line_number: 3,
                ..Default::default()
            },
            Instruction {
                operator: lines[7].tokens[0].clone(),
                operands: vec![lines[7].tokens[1].clone(), lines[7].tokens[2].clone()],
                line_number: 7,
                ..Default::default()
            },
        ],
        vec![
            Data {
                line_number: 1,
                label: lines[1].tokens[0].clone(),
                data_type: lines[1].tokens[1].clone(),
                data_entries_and_values: vec![(lines[1].tokens[2].clone(), 0)],
                ..Default::default()
            },
            Data {
                line_number: 5,
                label: lines[5].tokens[0].clone(),
                data_type: lines[5].tokens[1].clone(),
                data_entries_and_values: vec![
                    (lines[5].tokens[2].clone(), 0),
                    (lines[5].tokens[3].clone(), 0),
                    (lines[5].tokens[4].clone(), 0),
                ],
                ..Default::default()
            },
        ],
    );
    correct_result.0[0].operator.token_type = Operator;
    correct_result.0[1].operator.token_type = Operator;
    correct_result.0[0].operands[0].token_name.pop();
    correct_result.0[0].operands[1].token_name.pop();
    correct_result.0[1].operands[0].token_name.pop();
    correct_result.1[0].label.token_type = Label;
    correct_result.1[0].label.token_name.pop();
    correct_result.1[1].label.token_type = Label;
    correct_result.1[1].label.token_name.pop();
    correct_result.1[1].data_entries_and_values[0]
        .0
        .token_name
        .pop();
    correct_result.1[1].data_entries_and_values[1]
        .0
        .token_name
        .pop();

    assert_eq!(result, correct_result);
}

#[test]
fn separate_data_and_text_recognizes_ascii_data() {
    let lines = tokenize_program(
        ".data\nword: .ascii \"this is a string\"\nword2: .word 1,2,3\n.text\nadd $t1, $t2, $t3\nlw $t1, 400($t1)\n"
            .to_string(),
    );
    let result = separate_data_and_text(lines.clone());

    let mut correct_result: (Vec<Instruction>, Vec<Data>) = (
        vec![
            Instruction {
                operator: lines[4].tokens[0].clone(),
                operands: vec![
                    lines[4].tokens[1].clone(),
                    lines[4].tokens[2].clone(),
                    lines[4].tokens[3].clone(),
                ],
                line_number: 4,
                ..Default::default()
            },
            Instruction {
                operator: lines[5].tokens[0].clone(),
                operands: vec![lines[5].tokens[1].clone(), lines[5].tokens[2].clone()],
                line_number: 5,
                ..Default::default()
            },
        ],
        vec![
            Data {
                line_number: 1,
                label: lines[1].tokens[0].clone(),
                data_type: lines[1].tokens[1].clone(),
                data_entries_and_values: vec![(lines[1].tokens[2].clone(), 0)],
                ..Default::default()
            },
            Data {
                line_number: 2,
                label: lines[2].tokens[0].clone(),
                data_type: lines[2].tokens[1].clone(),
                data_entries_and_values: vec![
                    (lines[2].tokens[2].clone(), 0),
                    (lines[2].tokens[3].clone(), 0),
                    (lines[2].tokens[4].clone(), 0),
                ],
                ..Default::default()
            },
        ],
    );
    correct_result.0[0].operator.token_type = Operator;
    correct_result.0[1].operator.token_type = Operator;
    correct_result.0[0].operands[0].token_name.pop();
    correct_result.0[0].operands[1].token_name.pop();
    correct_result.0[1].operands[0].token_name.pop();
    correct_result.1[0].label.token_type = Label;
    correct_result.1[0].label.token_name.pop();
    correct_result.1[1].label.token_type = Label;
    correct_result.1[1].label.token_name.pop();
    correct_result.1[1].data_entries_and_values[0]
        .0
        .token_name
        .pop();
    correct_result.1[1].data_entries_and_values[1]
        .0
        .token_name
        .pop();

    assert_eq!(result, correct_result);
}

#[test]
fn separate_data_and_text_recognizes_data_and_text() {
    let lines = tokenize_program(
        ".data\nword1: .word 32\nword2: .word 1,2,3\n.text\nadd $t1, $t2, $t3\nlw $t1, 400($t1)\n"
            .to_string(),
    );
    let result = separate_data_and_text(lines.clone());

    let mut correct_result: (Vec<Instruction>, Vec<Data>) = (
        vec![
            Instruction {
                operator: lines[4].tokens[0].clone(),
                operands: vec![
                    lines[4].tokens[1].clone(),
                    lines[4].tokens[2].clone(),
                    lines[4].tokens[3].clone(),
                ],
                line_number: 4,
                ..Default::default()
            },
            Instruction {
                operator: lines[5].tokens[0].clone(),
                operands: vec![lines[5].tokens[1].clone(), lines[5].tokens[2].clone()],
                line_number: 5,
                ..Default::default()
            },
        ],
        vec![
            Data {
                line_number: 1,
                label: lines[1].tokens[0].clone(),
                data_type: lines[1].tokens[1].clone(),
                data_entries_and_values: vec![(lines[1].tokens[2].clone(), 0)],
                ..Default::default()
            },
            Data {
                line_number: 2,
                label: lines[2].tokens[0].clone(),
                data_type: lines[2].tokens[1].clone(),
                data_entries_and_values: vec![
                    (lines[2].tokens[2].clone(), 0),
                    (lines[2].tokens[3].clone(), 0),
                    (lines[2].tokens[4].clone(), 0),
                ],
                ..Default::default()
            },
        ],
    );
    correct_result.0[0].operator.token_type = Operator;
    correct_result.0[1].operator.token_type = Operator;
    correct_result.0[0].operands[0].token_name.pop();
    correct_result.0[0].operands[1].token_name.pop();
    correct_result.0[1].operands[0].token_name.pop();
    correct_result.1[0].label.token_type = Label;
    correct_result.1[0].label.token_name.pop();
    correct_result.1[1].label.token_type = Label;
    correct_result.1[1].label.token_name.pop();
    correct_result.1[1].data_entries_and_values[0]
        .0
        .token_name
        .pop();
    correct_result.1[1].data_entries_and_values[1]
        .0
        .token_name
        .pop();

    assert_eq!(result, correct_result);
}

#[test]
fn build_instruction_list_generates_error_on_double_label() {
    let lines =
        tokenize_program("lw $t1, 400($zero)\nLabel1:\nLabel2: add $t1, $t2, $t3\n".to_string());
    let result = separate_data_and_text(lines);
    assert_eq!(result.0[1].errors[0].error_name, LabelAssignmentError);
}

#[test]
fn build_instruction_list_generates_error_on_label_on_last_line() {
    let lines = tokenize_program("lw $t1, 400($zero)\nadd $t1, $t2, $t3\nlabel:\n".to_string());
    let result = separate_data_and_text(lines);
    assert_eq!(result.0[2].errors[0].error_name, LabelAssignmentError);
}

#[test]
fn create_label_map_generates_map_on_no_errors() {
    let mut monaco_line_info_vec = tokenize_program("add $t1, $t2, $t3\nload_from_memory: lw $t1, 400($t2)\nadd $t1, $t2, $t3\nstore_in_memory: sw $t1, 400($t2)".to_string());
    let (mut instruction_list, mut data) = separate_data_and_text(monaco_line_info_vec.clone());
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut instruction_list,
        &data,
        &mut monaco_line_info_vec,
    );

    let results: HashMap<String, usize> = create_label_map(&mut instruction_list, &mut data);

    let mut correct_map: HashMap<String, usize> = HashMap::new();
    correct_map.insert("load_from_memory".to_string(), 4);
    correct_map.insert("store_in_memory".to_string(), 12);

    assert_eq!(results, correct_map);
}

#[test]
fn create_label_map_recognizes_data_labels() {
    let mut monaco_line_info_vec = tokenize_program(".data\nlabel: .byte 'a'\nlabel2: .float 200\nlabel3: .word 200\n.text\nadd $t1, $t2, $t3\n".to_string());
    let (mut instruction_list, mut data) = separate_data_and_text(monaco_line_info_vec.clone());
    assemble_data_binary(&mut data);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut instruction_list,
        &data,
        &mut monaco_line_info_vec,
    );
    let results: HashMap<String, usize> = create_label_map(&mut instruction_list, &mut data);

    let mut correct_map: HashMap<String, usize> =
        create_label_map(&mut instruction_list, &mut data);
    correct_map.insert("label".to_string(), 8);
    correct_map.insert("label2".to_string(), 9);
    correct_map.insert("label3".to_string(), 13);

    assert_eq!(results, correct_map);
}

#[test]
fn create_label_map_recognizes_data_labels_and_text_together() {
    let mut monaco_line_info_vec = tokenize_program(".data\nlabel: .byte 'a'\nlabel2: .float 200\nlabel3: .word 200\n.text\nadd $t1, $t2, $t3\ninstruction: sub $t1, $t2, $t3\n".to_string());
    let (mut instruction_list, mut data) = separate_data_and_text(monaco_line_info_vec.clone());
    assemble_data_binary(&mut data);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut instruction_list,
        &data,
        &mut monaco_line_info_vec,
    );
    let results: HashMap<String, usize> = create_label_map(&mut instruction_list, &mut data);

    let mut correct_map: HashMap<String, usize> =
        create_label_map(&mut instruction_list, &mut data);
    correct_map.insert("instruction".to_string(), 4);
    correct_map.insert("label".to_string(), 12);
    correct_map.insert("label2".to_string(), 13);
    correct_map.insert("label3".to_string(), 17);

    assert_eq!(results, correct_map);
}

#[test]
fn create_label_map_pushes_errors_instead_of_inserting_duplicate_label_name() {
    let mut monaco_line_info_vec = tokenize_program("add $t1, $t2, $t3\nload_from_memory: lw $t1, 400($t2)\nadd $t1, $t2, $t3\nload_from_memory: lw $t2, 400($t2)".to_string());
    let (mut instruction_list, mut data) = separate_data_and_text(monaco_line_info_vec.clone());
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut instruction_list,
        &data,
        &mut monaco_line_info_vec,
    );

    let results: HashMap<String, usize> = create_label_map(&mut instruction_list, &mut data);

    let mut correct_map: HashMap<String, usize> = HashMap::new();
    correct_map.insert("load_from_memory".to_string(), 4);

    assert_eq!(results, correct_map);
    assert_eq!(
        instruction_list[3].errors[0].error_name,
        LabelMultipleDefinition
    );
}
#[test]
fn suggest_error_corrections_works_with_various_gp_registers() {
    let result = parser("add $t1, $t2, t3\nori not, ro, 100".to_string())
        .0
        .instructions;

    assert_eq!(
        result[0].errors[0].message,
        "GP register is not recognized. A valid, similar register is: $t3.\n"
    );
    assert_eq!(
        result[1].errors[0].message,
        "GP register is not recognized. A valid, similar register is: $at.\n"
    );
    assert_eq!(
        result[1].errors[1].message,
        "GP register is not recognized. A valid, similar register is: r0.\n"
    );
}

#[test]
fn suggest_error_corrections_works_with_various_fp_registers() {
    let result = parser("add.s $f1, $f2, f3\nadd.d fake, $052, 1qp".to_string())
        .0
        .instructions;

    assert_eq!(
        result[0].errors[0].message,
        "FP register is not recognized. A valid, similar register is: $f3.\n"
    );
    assert_eq!(
        result[1].errors[0].message,
        "FP register is not recognized. A valid, similar register is: $f0.\n"
    );
    assert_eq!(
        result[1].errors[1].message,
        "FP register is not recognized. A valid, similar register is: $f2.\n"
    );
    assert_eq!(
        result[1].errors[2].message,
        "FP register is not recognized. A valid, similar register is: $f0.\n"
    );
}

#[test]
fn suggest_error_corrections_works_with_labels() {
    let result =
        parser("j stable\nlabel: add $t1, $t2, $t3\ntable: sub $t1, $t2, $t3\nj lapel".to_string())
            .0
            .instructions;

    assert_eq!(
        result[0].errors[0].message,
        "A valid, similar label is: table.\n"
    );
    assert_eq!(
        result[3].errors[0].message,
        "A valid, similar label is: label.\n"
    );
}

#[test]
fn suggest_error_corrections_works_with_labels_when_no_labels_specified() {
    let result = parser("add $t1, $t2, $t3\nj stable\nlw $t1, 100($zero)\n".to_string())
        .0
        .instructions;
    assert_eq!(
        result[1].errors[0].message,
        "There is no recognized labelled memory.\n"
    );
}

#[test]
fn suggest_error_corrections_works_with_instructions() {
    let result = parser("sun $t1, $t2, $t3\nlq $t1, 100($zero)\n.c.eqd $f1, $f1, $f3".to_string())
        .0
        .instructions;

    assert_eq!(
        result[0].errors[0].message,
        "A valid, similar instruction is: sub.\n"
    );
    assert_eq!(
        result[1].errors[0].message,
        "A valid, similar instruction is: lw.\n"
    );
    assert_eq!(
        result[2].errors[0].message,
        "A valid, similar instruction is: c.eq.d.\n"
    );
}

#[test]
fn suggest_error_corrections_works_with_data_types() {
    let result = parser(
        ".data\nlabel: word 100\ntable: .bite 'c','1'\nlapel: gobbledygook \"this is a string\""
            .to_string(),
    )
    .0
    .data;

    assert_eq!(
        result[0].errors[0].message,
        "A valid, similar data type is: .word.\n"
    );
    assert_eq!(
        result[1].errors[0].message,
        "A valid, similar data type is: .byte.\n"
    );
    assert_eq!(
        result[2].errors[0].message,
        "A valid, similar data type is: .double.\n"
    );
}

#[test]
fn suggest_error_suggestions_associates_error_with_monaco_line_info() {
    let lines =
        parser("ori $t1, 100, $t2\nlw $f1, 400($zero)\n.data\nword .wod \"a\"\n".to_string())
            .0
            .monaco_line_info;

    let actual = Error {
        error_name: ErrorType::UnrecognizedGPRegister,
        token_causing_error: "100".to_string(),
        start_end_columns: (9, 12),
        message: "GP register is not recognized. A valid, similar register is: $v0.\n".to_string(),
    };
    assert_eq!(lines[0].errors[0], actual);

    let actual = Error {
        error_name: ErrorType::NonIntImmediate,
        token_causing_error: "$t2".to_string(),
        start_end_columns: (14, 17),
        message: "The given string cannot be recognized as an integer.\n".to_string(),
    };
    assert_eq!(lines[0].errors[1], actual);

    let actual = Error {
        error_name: ErrorType::IncorrectRegisterTypeFP,
        token_causing_error: "$f1".to_string(),
        start_end_columns: (3, 6),
        message: "Expected GP register but received FP register.\n".to_string(),
    };
    assert_eq!(lines[1].errors[0], actual);

    let actual = Error {
        error_name: ErrorType::ImproperlyFormattedLabel,
        token_causing_error: "word".to_string(),
        start_end_columns: (0, 4),
        message: "Label assignment recognized but does not end in a colon.\n".to_string(),
    };
    assert_eq!(lines[3].errors[0], actual);

    let actual = Error {
        error_name: ErrorType::UnrecognizedDataType,
        token_causing_error: ".wod".to_string(),
        start_end_columns: (5, 9),
        message: "A valid, similar data type is: .word.\n".to_string(),
    };
    assert_eq!(lines[3].errors[1], actual);
}
