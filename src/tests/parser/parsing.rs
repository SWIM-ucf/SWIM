use crate::parser::assembling::assemble_data_binary;
use crate::parser::parser_assembler_main::parser;
use crate::parser::parser_structs_and_enums::instruction_tokenization::ErrorType::{
    LabelAssignmentError, LabelMultipleDefinition, MissingComma,
};
use crate::parser::parser_structs_and_enums::instruction_tokenization::TokenType::{
    Label, Operator, Unknown,
};
use crate::parser::parser_structs_and_enums::instruction_tokenization::{
    Data, Error, Instruction, Line, ProgramInfo, Token,
};
use crate::parser::parsing::{
    complete_lw_sw_pseudo_instructions, create_label_map,
    expand_pseudo_instructions_and_assign_instruction_numbers,
};
#[cfg(test)]
use crate::parser::parsing::{separate_data_and_text, tokenize_program};
use std::collections::HashMap;

#[test]
fn tokenize_program_works_basic_version() {
    let result = tokenize_program("This line\nThis second line\nHere's a third!".to_string()).0;

    let i_0_t_0 = Token {
        token_name: "This".to_string(),
        starting_column: 0,
        token_type: Unknown,
    };
    let i_0_t_1 = Token {
        token_name: "line".to_string(),
        starting_column: 5,
        token_type: Unknown,
    };

    let i_1_t_0 = Token {
        token_name: "This".to_string(),
        starting_column: 0,
        token_type: Unknown,
    };
    let i_1_t_1 = Token {
        token_name: "second".to_string(),
        starting_column: 5,
        token_type: Unknown,
    };
    let i_1_t_2 = Token {
        token_name: "line".to_string(),
        starting_column: 12,
        token_type: Unknown,
    };
    let i_2_t_0 = Token {
        token_name: "Here's".to_string(),
        starting_column: 0,
        token_type: Unknown,
    };
    let i_2_t_1 = Token {
        token_name: "a".to_string(),
        starting_column: 7,
        token_type: Unknown,
    };
    let i_2_t_2 = Token {
        token_name: "third!".to_string(),
        starting_column: 9,
        token_type: Unknown,
    };
    let line_0 = Line {
        line_number: 0,
        tokens: vec![i_0_t_0, i_0_t_1],
    };

    let line_1 = Line {
        line_number: 1,
        tokens: vec![i_1_t_0, i_1_t_1, i_1_t_2],
    };

    let line_2 = Line {
        line_number: 2,
        tokens: vec![i_2_t_0, i_2_t_1, i_2_t_2],
    };

    let correct_result = vec![line_0, line_1, line_2];
    assert_eq!(result, correct_result);
}

#[test]
fn tokenize_program_handles_no_spaces_between_commas() {
    let result = tokenize_program("add $t1, $t2, $t3\nsub $s1,$s2,$s3\n".to_string()).0;

    let i_0_t_0 = Token {
        token_name: "add".to_string(),
        starting_column: 0,
        token_type: Unknown,
    };
    let i_0_t_1 = Token {
        token_name: "$t1,".to_string(),
        starting_column: 4,
        token_type: Unknown,
    };

    let i_0_t_2 = Token {
        token_name: "$t2,".to_string(),
        starting_column: 9,
        token_type: Unknown,
    };
    let i_0_t_3 = Token {
        token_name: "$t3".to_string(),
        starting_column: 14,
        token_type: Unknown,
    };
    let i_1_t_0 = Token {
        token_name: "sub".to_string(),
        starting_column: 0,
        token_type: Unknown,
    };
    let i_1_t_1 = Token {
        token_name: "$s1,".to_string(),
        starting_column: 4,
        token_type: Unknown,
    };

    let i_1_t_2 = Token {
        token_name: "$s2,".to_string(),
        starting_column: 8,
        token_type: Unknown,
    };
    let i_1_t_3 = Token {
        token_name: "$s3".to_string(),
        starting_column: 12,
        token_type: Unknown,
    };

    let line_0 = Line {
        line_number: 0,
        tokens: vec![i_0_t_0, i_0_t_1, i_0_t_2, i_0_t_3],
    };

    let line_1 = Line {
        line_number: 1,
        tokens: vec![i_1_t_0, i_1_t_1, i_1_t_2, i_1_t_3],
    };

    let correct_result = vec![line_0, line_1];
    assert_eq!(result, correct_result);
}

#[test]
fn tokenize_program_handles_comma_after_space() {
    let result = tokenize_program("add $t1 , $t2, $t3\n".to_string()).0;

    let i_0_t_0 = Token {
        token_name: "add".to_string(),
        starting_column: 0,
        token_type: Unknown,
    };
    let i_0_t_1 = Token {
        token_name: "$t1,".to_string(),
        starting_column: 4,
        token_type: Unknown,
    };
    let i_0_t_2 = Token {
        token_name: "$t2,".to_string(),
        starting_column: 10,
        token_type: Unknown,
    };
    let i_0_t_3 = Token {
        token_name: "$t3".to_string(),
        starting_column: 15,
        token_type: Unknown,
    };
    let line_0 = Line {
        line_number: 0,
        tokens: vec![i_0_t_0, i_0_t_1, i_0_t_2, i_0_t_3],
    };

    let correct_result = vec![line_0];
    assert_eq!(result, correct_result);
}

#[test]
fn tokenize_program_ignores_comments() {
    let results = tokenize_program(
        "This Line\n#this line is a comment\nbut_this_isn't\nthis#has a comment in the middle\n"
            .to_string(),
    )
    .0;

    let i_0_t_0 = Token {
        token_name: "This".to_string(),
        starting_column: 0,
        token_type: Unknown,
    };
    let i_0_t_1 = Token {
        token_name: "Line".to_string(),
        starting_column: 5,
        token_type: Unknown,
    };
    let line_0 = Line {
        line_number: 0,
        tokens: vec![i_0_t_0, i_0_t_1],
    };
    let line_2 = Line {
        line_number: 2,
        tokens: vec![Token {
            token_name: "but_this_isn't".to_string(),
            starting_column: 0,
            token_type: Unknown,
        }],
    };
    let line_3 = Line {
        line_number: 3,
        tokens: vec![Token {
            token_name: "this".to_string(),
            starting_column: 0,
            token_type: Unknown,
        }],
    };

    let correct_result = vec![line_0, line_2, line_3];
    assert_eq!(results, correct_result);
}

#[test]
fn tokenize_program_recognizes_comments() {
    let results = tokenize_program("Addi $t1, $t2, 300\n# this is a comment\nadd $t1, $t2, $t3\n#I'm making a note here. Huge comment".to_string()).0;
    assert_eq!(results[0].line_number, 0);
    assert_eq!(results[1].line_number, 2)
}

#[test]
fn tokenize_program_recognizes_comments_middle_of_line() {
    let results = tokenize_program("Addi $t1, $t2, 300 # this is a comment\nadd $t1, $t2, $t3#I'm making a note here. Huge comment".to_string()).0;

    assert_eq!(results[0].line_number, 0);
    assert_eq!(results[1].line_number, 1);
}

#[test]
fn tokenize_program_reads_ascii_properly() {
    let result = tokenize_program(".data\nlabel: .ascii \"this is a string\"".to_string()).0;

    assert_eq!(result[1].tokens[2].token_name, "\"this is a string\"");
}

#[test]
fn separate_data_and_text_works_basic_version() {
    let lines =
        tokenize_program("add $t1, $t2, $t3\nlw $t1, 400($t1)\naddi $t1, 100".to_string()).0;
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
fn separate_data_and_text_generates_error_on_missing_commas_text() {
    let lines = tokenize_program("add $t1, $t2, $t3\nlw $t1 400($t2)".to_string()).0;
    let result = separate_data_and_text(lines);
    let correct_error = Error {
        error_name: MissingComma,
        operand_number: Some(0),
        message: "".to_string(),
    };
    assert_eq!(correct_error, result.0[1].errors[0]);
}

#[test]
fn separate_data_and_text_works_on_line_label() {
    let lines = tokenize_program(
        "add $t1, $t2, $t3\nLoad_from_memory: lw $t1, 400($t1)\naddi $t1, 100".to_string(),
    )
    .0;
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
        starting_column: 0,
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
    )
    .0;
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
        starting_column: 0,
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
    let lines = tokenize_program(".text\nadd $t1, $t2, $t3\nlw $t1, 400($t1)\n".to_string()).0;
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
        ".data\nword1: .word 32\n.text\nadd $t1, $t2, $t3\n.data\nword2: .word 1,2,3\n.text\nlw $t1, 400($t1)\n"
            .to_string(),
    ).0;
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
    ).0;
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
    )
    .0;
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
        tokenize_program("lw $t1, 400($zero)\nLabel1:\nLabel2: add $t1, $t2, $t3\n".to_string()).0;
    let result = separate_data_and_text(lines);
    assert_eq!(result.0[1].errors[0].error_name, LabelAssignmentError);
}

#[test]
fn build_instruction_list_generates_error_on_label_on_last_line() {
    let lines = tokenize_program("lw $t1, 400($zero)\nadd $t1, $t2, $t3\nlabel:\n".to_string()).0;
    let result = separate_data_and_text(lines);
    assert_eq!(result.0[2].errors[0].error_name, LabelAssignmentError);
}

#[test]
fn create_label_map_generates_map_on_no_errors() {
    let (lines, mut updated_monaco_string, mut monaco_line_info_vec) = tokenize_program("add $t1, $t2, $t3\nload_from_memory: lw $t1, 400($t2)\nadd $t1, $t2, $t3\nstore_in_memory: sw $t1, 400($t2)".to_string());
    let (mut instruction_list, mut data) = separate_data_and_text(lines);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut instruction_list,
        &data,
        &mut updated_monaco_string,
        &mut monaco_line_info_vec,
    );

    let results: HashMap<String, u32> = create_label_map(&mut instruction_list, &mut data);

    let mut correct_map: HashMap<String, u32> = HashMap::new();
    correct_map.insert("load_from_memory".to_string(), 4);
    correct_map.insert("store_in_memory".to_string(), 12);

    assert_eq!(results, correct_map);
}

#[test]
fn create_label_map_recognizes_data_labels() {
    let (lines, mut updated_monaco_string, mut monaco_line_info_vec) = tokenize_program(".data\nlabel: .byte 'a'\nlabel2: .float 200\nlabel3: .word 200\n.text\nadd $t1, $t2, $t3\n".to_string());
    let (mut instruction_list, mut data) = separate_data_and_text(lines);
    assemble_data_binary(&mut data);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut instruction_list,
        &data,
        &mut updated_monaco_string,
        &mut monaco_line_info_vec,
    );
    let results: HashMap<String, u32> = create_label_map(&mut instruction_list, &mut data);

    let mut correct_map: HashMap<String, u32> = create_label_map(&mut instruction_list, &mut data);
    correct_map.insert("label".to_string(), 4);
    correct_map.insert("label2".to_string(), 5);
    correct_map.insert("label3".to_string(), 9);

    assert_eq!(results, correct_map);
}

#[test]
fn create_label_map_recognizes_data_labels_and_text_together() {
    let (lines, mut updated_monaco_string, mut monaco_line_info_vec) = tokenize_program(".data\nlabel: .byte 'a'\nlabel2: .float 200\nlabel3: .word 200\n.text\nadd $t1, $t2, $t3\ninstruction: sub $t1, $t2, $t3\n".to_string());
    let (mut instruction_list, mut data) = separate_data_and_text(lines);
    assemble_data_binary(&mut data);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut instruction_list,
        &data,
        &mut updated_monaco_string,
        &mut monaco_line_info_vec,
    );
    let results: HashMap<String, u32> = create_label_map(&mut instruction_list, &mut data);

    let mut correct_map: HashMap<String, u32> = create_label_map(&mut instruction_list, &mut data);
    correct_map.insert("instruction".to_string(), 4);
    correct_map.insert("label".to_string(), 8);
    correct_map.insert("label2".to_string(), 9);
    correct_map.insert("label3".to_string(), 13);

    assert_eq!(results, correct_map);
}

#[test]
fn create_label_map_pushes_errors_instead_of_inserting_duplicate_label_name() {
    let (lines, mut updated_monaco_string, mut monaco_line_info_vec) = tokenize_program("add $t1, $t2, $t3\nload_from_memory: lw $t1, 400($t2)\nadd $t1, $t2, $t3\nload_from_memory: lw $t2, 400($t2)".to_string());
    let (mut instruction_list, mut data) = separate_data_and_text(lines);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut instruction_list,
        &data,
        &mut updated_monaco_string,
        &mut monaco_line_info_vec,
    );

    let results: HashMap<String, u32> = create_label_map(&mut instruction_list, &mut data);

    let mut correct_map: HashMap<String, u32> = HashMap::new();
    correct_map.insert("load_from_memory".to_string(), 4);

    assert_eq!(results, correct_map);
    assert_eq!(
        instruction_list[3].errors[0].error_name,
        LabelMultipleDefinition
    );
}

#[test]
fn complete_lw_sw_pseudo_instructions_works() {
    let mut program_info = ProgramInfo::default();

    let file_string = ".data\nlabel: .word 100\n.text\nlw $t1, label\nsw $t1, label".to_string();

    let (lines, mut updated_monaco_string, mut monaco_line_info_vec) = tokenize_program(file_string);
    (program_info.instructions, program_info.data) = separate_data_and_text(lines);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut updated_monaco_string,
        &mut monaco_line_info_vec,
    );
    let _vec_of_data = assemble_data_binary(&mut program_info.data);
    let labels: HashMap<String, u32> =
        create_label_map(&mut program_info.instructions, &mut program_info.data);

    complete_lw_sw_pseudo_instructions(
        &mut program_info.instructions,
        &labels,
        &mut updated_monaco_string,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "lui".to_string(),
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$at".to_string(),
                    starting_column: 4,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "0".to_string(),
                    starting_column: 9,
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
                token_name: "lw".to_string(),
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 3,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "16($at)".to_string(),
                    starting_column: 8,
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
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$at".to_string(),
                    starting_column: 4,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "0".to_string(),
                    starting_column: 9,
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
    assert_eq!(
        program_info.instructions[3],
        Instruction {
            operator: Token {
                token_name: "sw".to_string(),
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 3,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "16($at)".to_string(),
                    starting_column: 8,
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

    let (lines, mut updated_monaco_string, mut monaco_line_info_vec) = tokenize_program(file_string);
    (program_info.instructions, program_info.data) = separate_data_and_text(lines);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut updated_monaco_string,
        &mut monaco_line_info_vec,
    );
    let _vec_of_data = assemble_data_binary(&mut program_info.data);
    let labels: HashMap<String, u32> =
        create_label_map(&mut program_info.instructions, &mut program_info.data);

    complete_lw_sw_pseudo_instructions(
        &mut program_info.instructions,
        &labels,
        &mut updated_monaco_string,
    );
}

#[test]
fn expand_pseudo_instructions_and_assign_instruction_number_adds_syscall_if_it_is_missing() {
    let mut program_info = ProgramInfo::default();
    let file_string = "addi $t1, $t2, 100\nsw $t1, label".to_string();
    let (lines, mut result, mut monaco_line_info_vec) = tokenize_program(file_string);
    (program_info.instructions, program_info.data) = separate_data_and_text(lines);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut result,
        &mut monaco_line_info_vec,
    );

    let correct_result: Vec<String> = vec![
        "addi $t1, $t2, 100".to_string(),
        "sw $t1, label".to_string(),
        "syscall".to_string(),
    ];
    assert_eq!(result, correct_result);
}

#[test]
fn expand_pseudo_instructions_and_assign_instruction_number_adds_syscall_at_beginning_if_no_instruction(
) {
    let mut program_info = ProgramInfo::default();
    let file_string = ".data\nword .word 100\nother .byte 'a','a'\n".to_string();
    let (lines, mut result, mut monaco_line_info_vec) = tokenize_program(file_string);
    (program_info.instructions, program_info.data) = separate_data_and_text(lines);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut result,
        &mut monaco_line_info_vec,
    );

    let correct_result: Vec<String> = vec![
        ".text".to_string(),
        "syscall".to_string(),
        ".data".to_string(),
        "word .word 100".to_string(),
        "other .byte 'a','a'".to_string(),
    ];

    assert_eq!(result, correct_result);
}

#[test]
fn expand_pseudo_instructions_and_assign_instruction_number_adds_syscall_after_first_instance_of_text(
) {
    let mut program_info = ProgramInfo::default();
    let file_string = ".data\nword .word 100\n.text\n.data\nother .byte 'a','a'\n.text\n.data\nfinal: .space 10\n".to_string();
    let (lines, mut result, mut monaco_line_info_vec) = tokenize_program(file_string);
    (program_info.instructions, program_info.data) = separate_data_and_text(lines);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut result,
        &mut monaco_line_info_vec,
    );

    let correct_result: Vec<String> = vec![
        ".data".to_string(),
        "word .word 100".to_string(),
        ".text".to_string(),
        "syscall".to_string(),
        ".data".to_string(),
        "other .byte 'a','a'".to_string(),
        ".text".to_string(),
        ".data".to_string(),
        "final: .space 10".to_string(),
    ];

    assert_eq!(result, correct_result);
}

#[test]
fn expand_pseudo_instructions_and_assign_instruction_number_does_not_add_syscall_if_it_is_present()
{
    let mut program_info = ProgramInfo::default();
    let file_string = "addi $t1, $t2, 100\nsw $t1, label\nsyscall\n".to_string();
    let (lines, mut result, mut monaco_line_info_vec) = tokenize_program(file_string);
    (program_info.instructions, program_info.data) = separate_data_and_text(lines);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut result,
        &mut monaco_line_info_vec,
    );

    let correct_result: Vec<String> = vec![
        "addi $t1, $t2, 100".to_string(),
        "sw $t1, label".to_string(),
        "syscall".to_string(),
    ];

    assert_eq!(result, correct_result);
}

#[test]
fn expand_pseudo_instructions_and_assign_instruction_number_adds_syscall_at_proper_spot_with_data_after(
) {
    let mut program_info = ProgramInfo::default();
    let file_string = "addi $t1, $t2, 100\nsw $t1, label\n.data\n word: .word 100\n".to_string();
    let (lines, mut result, mut monaco_line_info_vec) = tokenize_program(file_string);
    (program_info.instructions, program_info.data) = separate_data_and_text(lines);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut result,
        &mut monaco_line_info_vec,
    );

    let correct_result: Vec<String> = vec![
        "addi $t1, $t2, 100".to_string(),
        "sw $t1, label".to_string(),
        "syscall".to_string(),
        ".data".to_string(),
        " word: .word 100".to_string(),
    ];

    assert_eq!(result, correct_result);
}

#[test]
fn expand_pseudo_instructions_and_assign_instruction_numbers_works_subi() {
    let mut program_info = ProgramInfo::default();

    let file_string = "subi $t1, $t2, 100\nsw $t1, label".to_string();

    let (lines, mut updated_monaco_string, mut monaco_line_info_vec) = tokenize_program(file_string);
    (program_info.instructions, program_info.data) = separate_data_and_text(lines);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut updated_monaco_string,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "ori".to_string(),
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$at".to_string(),
                    starting_column: 4,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$zero".to_string(),
                    starting_column: 9,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "100".to_string(),
                    starting_column: 16,
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
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 4,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t2".to_string(),
                    starting_column: 9,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$at".to_string(),
                    starting_column: 14,
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

    let (lines, mut updated_monaco_string, mut monaco_line_info_vec) = tokenize_program(file_string);
    (program_info.instructions, program_info.data) = separate_data_and_text(lines);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut updated_monaco_string,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "ori".to_string(),
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$at".to_string(),
                    starting_column: 4,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$zero".to_string(),
                    starting_column: 9,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "100".to_string(),
                    starting_column: 16,
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
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 4,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t2".to_string(),
                    starting_column: 9,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$at".to_string(),
                    starting_column: 14,
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

    let file_string = "divi $t1, 100\nsw $t1, label".to_string();

    let (lines, mut updated_monaco_string, mut monaco_line_info_vec) = tokenize_program(file_string);
    (program_info.instructions, program_info.data) = separate_data_and_text(lines);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut updated_monaco_string,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "ori".to_string(),
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$at".to_string(),
                    starting_column: 4,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$zero".to_string(),
                    starting_column: 9,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "100".to_string(),
                    starting_column: 16,
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
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 4,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$at".to_string(),
                    starting_column: 9,
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

    let (lines, mut updated_monaco_string, mut monaco_line_info_vec) = tokenize_program(file_string);
    (program_info.instructions, program_info.data) = separate_data_and_text(lines);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut updated_monaco_string,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "ori".to_string(),
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$at".to_string(),
                    starting_column: 4,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$zero".to_string(),
                    starting_column: 9,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "100".to_string(),
                    starting_column: 16,
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
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 5,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t2".to_string(),
                    starting_column: 10,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$at".to_string(),
                    starting_column: 15,
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

    let (lines, mut updated_monaco_string, mut monaco_line_info_vec) = tokenize_program(file_string);
    (program_info.instructions, program_info.data) = separate_data_and_text(lines);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut updated_monaco_string,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "ori".to_string(),
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$at".to_string(),
                    starting_column: 4,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$zero".to_string(),
                    starting_column: 9,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "100".to_string(),
                    starting_column: 16,
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
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 5,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t2".to_string(),
                    starting_column: 10,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$at".to_string(),
                    starting_column: 15,
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

    let file_string = "ddivi $t1, 100\nsw $t1, label".to_string();

    let (lines, mut updated_monaco_string, mut monaco_line_info_vec) = tokenize_program(file_string);
    (program_info.instructions, program_info.data) = separate_data_and_text(lines);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut updated_monaco_string,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "ori".to_string(),
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$at".to_string(),
                    starting_column: 4,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$zero".to_string(),
                    starting_column: 9,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "100".to_string(),
                    starting_column: 16,
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
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 5,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$at".to_string(),
                    starting_column: 10,
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

    let (lines, mut updated_monaco_string, mut monaco_line_info_vec) = tokenize_program(file_string);
    (program_info.instructions, program_info.data) = separate_data_and_text(lines);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut updated_monaco_string,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "slt".to_string(),
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 4,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t3".to_string(),
                    starting_column: 9,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t2".to_string(),
                    starting_column: 13,
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

    let (lines, mut updated_monaco_string, mut monaco_line_info_vec) = tokenize_program(file_string);
    (program_info.instructions, program_info.data) = separate_data_and_text(lines);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut updated_monaco_string,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "sltu".to_string(),
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 5,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t3".to_string(),
                    starting_column: 10,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t2".to_string(),
                    starting_column: 14,
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

    let (lines, mut updated_monaco_string, mut monaco_line_info_vec) = tokenize_program(file_string);
    (program_info.instructions, program_info.data) = separate_data_and_text(lines);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut updated_monaco_string,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "sub".to_string(),
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 4,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t2".to_string(),
                    starting_column: 9,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t3".to_string(),
                    starting_column: 14,
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
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$at".to_string(),
                    starting_column: 4,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$zero".to_string(),
                    starting_column: 9,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "1".to_string(),
                    starting_column: 16,
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
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 4,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 9,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$at".to_string(),
                    starting_column: 14,
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

    let (lines, mut updated_monaco_string, mut monaco_line_info_vec) = tokenize_program(file_string);
    (program_info.instructions, program_info.data) = separate_data_and_text(lines);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut updated_monaco_string,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "sub".to_string(),
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 4,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t2".to_string(),
                    starting_column: 9,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t3".to_string(),
                    starting_column: 14,
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
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 4,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$zero".to_string(),
                    starting_column: 9,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 16,
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

    let (lines, mut updated_monaco_string, mut monaco_line_info_vec) = tokenize_program(file_string);
    (program_info.instructions, program_info.data) = separate_data_and_text(lines);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut updated_monaco_string,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "slt".to_string(),
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 4,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t3".to_string(),
                    starting_column: 9,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t2".to_string(),
                    starting_column: 14,
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
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 5,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 10,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "1".to_string(),
                    starting_column: 15,
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
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 5,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 10,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "1".to_string(),
                    starting_column: 15,
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

    let (lines, mut updated_monaco_string, mut monaco_line_info_vec) = tokenize_program(file_string);
    (program_info.instructions, program_info.data) = separate_data_and_text(lines);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut updated_monaco_string,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "sltu".to_string(),
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 5,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t3".to_string(),
                    starting_column: 10,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t2".to_string(),
                    starting_column: 15,
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
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 5,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 10,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "1".to_string(),
                    starting_column: 15,
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
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 5,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 10,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "1".to_string(),
                    starting_column: 15,
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

    let (lines, mut updated_monaco_string, mut monaco_line_info_vec) = tokenize_program(file_string);
    (program_info.instructions, program_info.data) = separate_data_and_text(lines);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut updated_monaco_string,
        &mut monaco_line_info_vec,
    );

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "slt".to_string(),
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 4,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t2".to_string(),
                    starting_column: 9,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t3".to_string(),
                    starting_column: 14,
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
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 5,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 10,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "1".to_string(),
                    starting_column: 15,
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
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 5,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 10,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "1".to_string(),
                    starting_column: 15,
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
    let (lines, mut updated_monaco_string, mut monaco_line_info_vec) = tokenize_program(file_string);
    (program_info.instructions, program_info.data) = separate_data_and_text(lines);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut updated_monaco_string,
        &mut monaco_line_info_vec,
    );

    let mut correct_program_info = ProgramInfo::default();
    let correct_string =
        "sltu $t1, $t2, $t3\naddi $t1, $t1, 1\nandi $t1, $t1, 1\nsw $t1, label".to_string();
    let (correct_lines, mut updated_monaco_string, mut monaco_line_info_vec) = tokenize_program(correct_string);
    (correct_program_info.instructions, correct_program_info.data) =
        separate_data_and_text(correct_lines);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut correct_program_info.instructions,
        &program_info.data,
        &mut updated_monaco_string,
        &mut monaco_line_info_vec,
    );

    //    assert_eq!(correct_program_info.instructions, program_info.instructions);

    assert_eq!(
        program_info.instructions[0],
        Instruction {
            operator: Token {
                token_name: "sltu".to_string(),
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 5,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t2".to_string(),
                    starting_column: 10,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t3".to_string(),
                    starting_column: 15,
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
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 5,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 10,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "1".to_string(),
                    starting_column: 15,
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
                starting_column: 0,
                token_type: Operator,
            },
            operands: vec![
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 5,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "$t1".to_string(),
                    starting_column: 10,
                    token_type: Default::default(),
                },
                Token {
                    token_name: "1".to_string(),
                    starting_column: 15,
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
fn suggest_error_corrections_works_with_various_gp_registers() {
    let result = parser("add $t1, $t2, t3\nori not, ro, 100".to_string())
        .0
        .instructions;

    assert_eq!(
        result[0].errors[0].message,
        "A valid, similar register is: $t3."
    );
    assert_eq!(
        result[1].errors[0].message,
        "A valid, similar register is: $at."
    );
    assert_eq!(
        result[1].errors[1].message,
        "A valid, similar register is: r0."
    );
}

#[test]
fn suggest_error_corrections_works_with_various_fp_registers() {
    let result = parser("add.s $f1, $f2, f3\nadd.d fake, $052, 1qp".to_string())
        .0
        .instructions;

    assert_eq!(
        result[0].errors[0].message,
        "A valid, similar register is: $f3."
    );
    assert_eq!(
        result[1].errors[0].message,
        "A valid, similar register is: $f0."
    );
    assert_eq!(
        result[1].errors[1].message,
        "A valid, similar register is: $f2."
    );
    assert_eq!(
        result[1].errors[2].message,
        "A valid, similar register is: $f0."
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
        "A valid, similar label is: table."
    );
    assert_eq!(
        result[3].errors[0].message,
        "A valid, similar label is: label."
    );
}

#[test]
fn suggest_error_corrections_works_with_labels_when_no_labels_specified() {
    let result = parser("add $t1, $t2, $t3\nj stable\nlw $t1, 100($zero)\n".to_string())
        .0
        .instructions;
    assert_eq!(
        result[1].errors[0].message,
        "There is no recognized labelled memory."
    );
}

#[test]
fn suggest_error_corrections_works_with_instructions() {
    let result = parser("sun $t1, $t2, $t3\nlq $t1, 100($zero)\n.c.eqd $f1, $f1, $f3".to_string())
        .0
        .instructions;

    assert_eq!(
        result[0].errors[0].message,
        "A valid, similar instruction is: sub."
    );
    assert_eq!(
        result[1].errors[0].message,
        "A valid, similar instruction is: lw."
    );
    assert_eq!(
        result[2].errors[0].message,
        "A valid, similar instruction is: c.eq.d."
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
        "A valid, similar data type is: .word."
    );
    assert_eq!(
        result[1].errors[0].message,
        "A valid, similar data type is: .byte."
    );
    assert_eq!(
        result[2].errors[0].message,
        "A valid, similar data type is: .double."
    );
}
