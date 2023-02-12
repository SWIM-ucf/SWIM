use crate::parser::parser_structs_and_enums::instruction_tokenization::ErrorType::{
    LabelAssignmentError, LabelMultipleDefinition, MissingComma,
};
use crate::parser::parser_structs_and_enums::instruction_tokenization::TokenType::{
    Label, Operator, Unknown,
};
use crate::parser::parser_structs_and_enums::instruction_tokenization::{
    Data, Error, Instruction, Line, Token,
};
use crate::parser::parsing::{
    assign_instruction_numbers, create_label_map, expand_pseudo_instruction,
};
#[cfg(test)]
use crate::parser::parsing::{separate_data_and_text, tokenize_program};
use std::collections::HashMap;

#[test]
fn tokenize_program_works_basic_version() {
    let result = tokenize_program("This line\nThis second line\nHere's a third!".to_string());

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
    assert_eq!(result.0, correct_result);
}

#[test]
fn tokenize_program_handles_no_spaces_between_commas() {
    let result = tokenize_program("add $t1, $t2, $t3\nsub $s1,$s2,$s3\n".to_string());

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
    assert_eq!(result.0, correct_result);
}

#[test]
fn tokenize_program_handles_comma_after_space() {
    let result = tokenize_program("add $t1 , $t2, $t3\n".to_string());

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
    assert_eq!(result.0, correct_result);
}

#[test]
fn tokenize_program_ignores_comments() {
    let results = tokenize_program(
        "This Line\n#this line is a comment\nbut_this_isn't\nthis#has a comment in the middle\n"
            .to_string(),
    );

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
    assert_eq!(results.0, correct_result);
}

#[test]
fn tokenize_program_recognizes_comments() {
    let results = tokenize_program("Addi $t1, $t2, 300\n# this is a comment\nadd $t1, $t2, $t3\n#I'm making a note here. Huge comment".to_string());
    assert_eq!(results.1[0][0], 1);
    assert_eq!(results.1[0][1], 0);
    assert_eq!(results.1[1][0], 3);
    assert_eq!(results.1[1][1], 0);
}

#[test]
fn tokenize_program_recognizes_comments_middle_of_line() {
    let results = tokenize_program("Addi $t1, $t2, 300 # this is a comment\nadd $t1, $t2, $t3#I'm making a note here. Huge comment".to_string());
    assert_eq!(results.1[0][0], 0);
    assert_eq!(results.1[0][1], 19);
    assert_eq!(results.1[1][0], 1);
    assert_eq!(results.1[1][1], 17);
}

#[test]
fn separate_data_and_text_works_basic_version() {
    let (lines, _comments) =
        tokenize_program("add $t1, $t2, $t3\nlw $t1, 400($t1)\naddi $t1, 100".to_string());
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
    };
    assert_eq!(correct_error, result.0[1].errors[0]);
}

#[test]
fn separate_data_and_text_works_on_line_label() {
    let (lines, _comments) = tokenize_program(
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
    let (lines, _comments) = tokenize_program(
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
    let (lines, _comments) =
        tokenize_program(".text\nadd $t1, $t2, $t3\nlw $t1, 400($t1)\n".to_string());
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
    let (lines, _comments) = tokenize_program(
        ".data\nword1: .word 32\n.text\nadd $t1, $t2, $t3\n.data\nword2: .word 1,2,3\n.text\nlw $t1, 400($t1)\n"
            .to_string(),
    );
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
fn separate_data_and_text_recognizes_data_and_text() {
    let (lines, _comments) = tokenize_program(
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
    let (lines, _comments) =
        tokenize_program("lw $t1, 400($zero)\nLabel1:\nLabel2: add $t1, $t2, $t3\n".to_string());
    let result = separate_data_and_text(lines);
    assert_eq!(result.0[1].errors[0].error_name, LabelAssignmentError);
}

#[test]
fn build_instruction_list_generates_error_on_label_on_last_line() {
    let (lines, _comments) =
        tokenize_program("lw $t1, 400($zero)\nadd $t1, $t2, $t3\nlabel:\n".to_string());
    let result = separate_data_and_text(lines);
    assert_eq!(result.0[2].errors[0].error_name, LabelAssignmentError);
}

#[test]
fn create_label_map_generates_map_on_no_errors() {
    let (lines, _comments) = tokenize_program("add $t1, $t2, $t3\nload_from_memory: lw $t1, 400($t2)\nadd $t1, #t2, $t3\nstore_in_memory: sw $t1, 400($t2)".to_string());
    let (mut instruction_list, _data) = separate_data_and_text(lines);
    expand_pseudo_instruction(&mut instruction_list);
    assign_instruction_numbers(&mut instruction_list);

    let results: HashMap<String, u32> = create_label_map(&mut instruction_list);

    let mut correct_map: HashMap<String, u32> = HashMap::new();
    correct_map.insert("load_from_memory".to_string(), 1);
    correct_map.insert("store_in_memory".to_string(), 3);

    assert_eq!(results, correct_map);
}

#[test]
fn create_label_map_pushes_errors_instead_of_inserting_duplicate_label_name() {
    let (lines, _comments) = tokenize_program("add $t1, $t2, $t3\nload_from_memory: lw $t1, 400($t2)\nadd $t1, $t2, $t3\nload_from_memory: lw $t2, 400($t2)".to_string());
    let (mut instruction_list, _data) = separate_data_and_text(lines);
    expand_pseudo_instruction(&mut instruction_list);
    assign_instruction_numbers(&mut instruction_list);

    let results: HashMap<String, u32> = create_label_map(&mut instruction_list);

    let mut correct_map: HashMap<String, u32> = HashMap::new();
    correct_map.insert("load_from_memory".to_string(), 1);

    assert_eq!(results, correct_map);
    assert_eq!(
        instruction_list[3].errors[0].error_name,
        LabelMultipleDefinition
    );
}
