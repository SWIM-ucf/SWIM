use std::collections::HashMap;
use crate::parser::parser_structs_and_enums::instruction_tokenization::ErrorType::{LabelAssignmentError, LabelMultipleDefinition, MissingComma};
use crate::parser::parser_structs_and_enums::instruction_tokenization::TokenType::Unknown;
use crate::parser::parser_structs_and_enums::instruction_tokenization::{
    Error, Instruction, Line, Token,
};
#[cfg(test)]
use crate::parser::preprocessing::{
    build_instruction_list_from_lines, confirm_operand_commas, tokenize_instructions,
};
use crate::parser::preprocessing::{assign_instruction_numbers, create_label_map, expand_pseudo_instruction};

#[test]
fn tokenize_instructions_works_basic_version() {
    let result = tokenize_instructions("This line\nThis second line\nHere's a third!".to_string());

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
fn tokenize_instructions_ignores_comments() {
    let results = tokenize_instructions(
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
    assert_eq!(results, correct_result);
}

#[test]
fn build_instruction_list_from_lines_works_basic_version() {
    let lines =
        tokenize_instructions("add $t1, $t2, $t3\nlw $t1, 400($t1)\naddi $t1, 100".to_string());
    let result = build_instruction_list_from_lines(lines.clone());

    let instruction_0 = Instruction {
        operator: lines[0].tokens[0].clone(),
        operands: vec![
            lines[0].tokens[1].clone(),
            lines[0].tokens[2].clone(),
            lines[0].tokens[3].clone(),
        ],
        line_number: 0,
        ..Default::default()
    };

    let instruction_1 = Instruction {
        operator: lines[1].tokens[0].clone(),
        operands: vec![lines[1].tokens[1].clone(), lines[1].tokens[2].clone()],
        line_number: 1,
        ..Default::default()
    };

    let instruction_2 = Instruction {
        operator: lines[2].tokens[0].clone(),
        operands: vec![lines[2].tokens[1].clone(), lines[2].tokens[2].clone()],
        line_number: 2,
        ..Default::default()
    };

    let correct_result = vec![instruction_0, instruction_1, instruction_2];

    assert_eq!(result, correct_result);
}

#[test]
fn build_instruction_list_from_lines_works_on_line_label() {
    let lines = tokenize_instructions(
        "add $t1, $t2, $t3\nLoad_from_memory: lw $t1, 400($t1)\naddi $t1, 100".to_string(),
    );
    let result = build_instruction_list_from_lines(lines.clone());

    let instruction_0 = Instruction {
        operator: lines[0].tokens[0].clone(),
        operands: vec![
            lines[0].tokens[1].clone(),
            lines[0].tokens[2].clone(),
            lines[0].tokens[3].clone(),
        ],
        instruction_number: 0,
        ..Default::default()
    };

    let token = Token {
        token_name: "Load_from_memory".to_string(),
        starting_column: 0,
        token_type: Default::default(),
    };
    let instruction_1 = Instruction {
        operator: lines[1].tokens[1].clone(),
        operands: vec![lines[1].tokens[2].clone(), lines[1].tokens[3].clone()],
        line_number: 1,
        label: Some((token, 1)),
        ..Default::default()
    };

    let instruction_2 = Instruction {
        operator: lines[2].tokens[0].clone(),
        operands: vec![lines[2].tokens[1].clone(), lines[2].tokens[2].clone()],
        line_number: 2,
        ..Default::default()
    };

    let correct_result = vec![instruction_0, instruction_1, instruction_2];

    assert_eq!(correct_result, result);
}

#[test]
fn build_instruction_list_from_lines_works_off_line_label() {
    let lines = tokenize_instructions(
        "add $t1, $t2, $t3\nLoad_from_memory:\nlw $t1, 400($t1)\naddi $t1, 100".to_string(),
    );
    let result = build_instruction_list_from_lines(lines.clone());

    let instruction_0 = Instruction {
        operator: lines[0].tokens[0].clone(),
        operands: vec![
            lines[0].tokens[1].clone(),
            lines[0].tokens[2].clone(),
            lines[0].tokens[3].clone(),
        ],
        instruction_number: 0,
        ..Default::default()
    };

    let token = Token {
        token_name: "Load_from_memory".to_string(),
        starting_column: 0,
        token_type: Default::default(),
    };
    let instruction_1 = Instruction {
        operator: lines[2].tokens[0].clone(),
        operands: vec![lines[2].tokens[1].clone(), lines[2].tokens[2].clone()],
        line_number: 2,
        label: Some((token, 1)),
        ..Default::default()
    };

    let instruction_2 = Instruction {
        operator: lines[3].tokens[0].clone(),
        operands: vec![lines[3].tokens[1].clone(), lines[3].tokens[2].clone()],
        line_number: 3,
        ..Default::default()
    };

    let correct_result = vec![instruction_0, instruction_1, instruction_2];

    assert_eq!(correct_result, result);
}

#[test]
fn build_instruction_list_generates_error_on_double_label() {
    let lines = tokenize_instructions(
        "lw $t1, 400($zero)\nLabel1:\nLabel2: add $t1, $t2, $t3\n".to_string(),
    );
    let result = build_instruction_list_from_lines(lines);
    assert_eq!(result[1].errors[0].error_name, LabelAssignmentError);
}

#[test]
fn build_instruction_list_generates_error_on_label_on_last_line() {
    let lines =
        tokenize_instructions("lw $t1, 400($zero)\nadd $t1, $t2, $t3\nlabel:\n".to_string());
    let result = build_instruction_list_from_lines(lines);
    assert_eq!(result[2].errors[0].error_name, LabelAssignmentError);
}

#[test]
fn confirm_operand_commas_removes_properly_placed_commas() {
    let lines = tokenize_instructions("Add $t1, $t2, $t3\nlw $t1, 400($t2)".to_string());
    let mut result = build_instruction_list_from_lines(lines);
    confirm_operand_commas(&mut result);

    let correct_lines = tokenize_instructions("Add $t1  $t2  $t3\nlw $t1  400($t2)".to_string());
    let correct_result = build_instruction_list_from_lines(correct_lines);

    assert_eq!(correct_result, result);
}

#[test]
fn confirm_operand_commas_generates_error_on_missing_commas() {
    let lines = tokenize_instructions("Add $t1, $t2, $t3\nlw $t1 400($t2)".to_string());
    let mut result = build_instruction_list_from_lines(lines);
    confirm_operand_commas(&mut result);
    let correct_error = Error {
        error_name: MissingComma,
        operand_number: Some(0),
    };
    assert_eq!(correct_error, result[1].errors[0]);
}

#[test]
fn create_label_map_generates_map_on_no_errors(){
    let lines = tokenize_instructions("add $t1, $t2, $t3\nload_from_memory: lw $t1 400($t2)\nadd $t1, #t2, $t3\nstore_in_memory: sw $t1, 400($t2)".to_string());
    let mut instruction_list: Vec<Instruction> = build_instruction_list_from_lines(lines);
    confirm_operand_commas(&mut instruction_list);
    expand_pseudo_instruction(&mut instruction_list);
    assign_instruction_numbers(&mut instruction_list);

    let results: HashMap<String, u32> = create_label_map(&mut instruction_list);

    let mut correct_map: HashMap<String, u32> = HashMap::new();
    correct_map.insert("load_from_memory".to_string(), 1);
    correct_map.insert("store_in_memory".to_string(), 3);

    assert_eq!(results, correct_map);
}

#[test]
fn create_label_map_pushes_errors_instead_of_inserting_duplicate_label_name(){
    let lines = tokenize_instructions("add $t1, $t2, $t3\nload_from_memory: lw $t1 400($t2)\nadd $t1, #t2, $t3\nload_from_memory: lw $t2, 400($t2)".to_string());
    let mut instruction_list: Vec<Instruction> = build_instruction_list_from_lines(lines);
    confirm_operand_commas(&mut instruction_list);
    expand_pseudo_instruction(&mut instruction_list);
    assign_instruction_numbers(&mut instruction_list);

    let results: HashMap<String, u32> = create_label_map(&mut instruction_list);

    let mut correct_map: HashMap<String, u32> = HashMap::new();
    correct_map.insert("load_from_memory".to_string(), 1);

    assert_eq!(results, correct_map);
    assert_eq!(instruction_list[3].errors[0].error_name, LabelMultipleDefinition);
}
