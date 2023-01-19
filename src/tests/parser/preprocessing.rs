use crate::parser::parser_instruction_tokenization::instruction_tokenization::TokenType::{Unknown};
use crate::parser::parser_instruction_tokenization::instruction_tokenization::{Error, Instruction, Line, Token};
use crate::parser::parser_instruction_tokenization::instruction_tokenization::ErrorType::{LabelAssignmentError, MissingComma};
#[cfg(test)]
use crate::parser::parser_preprocessing::string_cleaning;
use crate::parser::parser_preprocessing::{build_instruction_list_from_lines, confirm_operand_commas, tokenize_instructions};

#[test]
fn string_cleaning_removes_instances_of_double_spaces() {
    let result = string_cleaning("Double  space".to_string());
    assert_eq!(result, "Double space");
}

#[test]
fn string_cleaning_removes_multiple_instances_of_double_spaces() {
    let result = string_cleaning("THIS  HAS  MULTIPLE  DOUBLE SPACES".to_string());
    assert_eq!(result, "THIS HAS MULTIPLE DOUBLE SPACES");
}

#[test]
fn string_cleaning_culls_all_unnecessary_spaces_down_to_one() {
    let result = string_cleaning("Double  Triple   Triple   Quadruple   .".to_string());
    assert_eq!(result, "Double Triple Triple Quadruple .");
}

#[test]
fn string_cleaning_does_not_delete_single_spaces() {
    let result = string_cleaning("Single Single Double  .".to_string());
    assert_eq!(result, "Single Single Double .");
}

#[test]
fn string_cleaning_removes_extra_lines() {
    let result = string_cleaning("Space\nTwoSpaces\n\nSpace\nSpace".to_string());
    assert_eq!(result, "Space\nTwoSpaces\nSpace\nSpace");
}

#[test]
fn string_cleaning_removes_spaces_at_start_of_lines() {
    let result = string_cleaning("LINE\n SPACE-LINE\n SPACE-LINE\nLINE".to_string());
    assert_eq!(result, "LINE\nSPACE-LINE\nSPACE-LINE\nLINE");
}

#[test]
fn string_cleaning_removes_spaces_at_the_end_of_lines() {
    let result = string_cleaning("LINE\nSPACE-LINE \nSPACE-LINE \nLINE".to_string());
    assert_eq!(result, "LINE\nSPACE-LINE\nSPACE-LINE\nLINE");
}
#[test]
fn string_cleaning_removes_comments_at_the_end_of_a_line() {
    let result = string_cleaning("line\nline#comment\nline".to_string());
    assert_eq!(result, "line\nline\nline");
}
#[test]
fn string_cleaning_removes_comments_on_their_own_line() {
    let result = string_cleaning("line\n #this is a comment \nbut this isn't".to_string());
    assert_eq!(result, "line\nbut this isn't");
}
#[test]
fn string_cleaning_removes_spaces_and_new_lines_at_start_of_string() {
    let result_space = string_cleaning(" space at start\nsecond line".to_string());
    let result_new_line = string_cleaning("\nnew line at start\nsecond line".to_string());
    assert_eq!(result_space, "space at start\nsecond line");
    assert_eq!(result_new_line, "new line at start\nsecond line");
}
#[test]
fn string_cleaning_removes_spaces_and_new_lines_at_end_of_string() {
    let result_space = string_cleaning("line\nspace at end ".to_string());
    let result_new_line = string_cleaning("line\nnew line at end ".to_string());
    assert_eq!(result_space, "line\nspace at end");
    assert_eq!(result_new_line, "line\nnew line at end");
}

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
fn build_instruction_list_generates_error_on_double_label(){
    let lines = tokenize_instructions("lw $t1, 400($zero)\nLabel1:\nLabel2: add $t1, $t2, $t3\n".to_string());
    let result = build_instruction_list_from_lines(lines);
    assert_eq!(result[1].errors[0].error_name, LabelAssignmentError);
}

#[test]
fn build_instruction_list_generates_error_on_label_on_last_line(){
    let lines = tokenize_instructions("lw $t1, 400($zero)\nadd $t1, $t2, $t3\nlabel:\n".to_string());
    let result = build_instruction_list_from_lines(lines);
    assert_eq!(result[2].errors[0].error_name, LabelAssignmentError);
}

#[test]
fn confirm_operand_commas_removes_properly_placed_commas(){
    let lines = tokenize_instructions("Add $t1, $t2, $t3\nlw $t1, 400($t2)".to_string());
    let mut result = build_instruction_list_from_lines(lines);
    confirm_operand_commas(&mut result);

    let correct_lines = tokenize_instructions("Add $t1  $t2  $t3\nlw $t1  400($t2)".to_string());
    let correct_result = build_instruction_list_from_lines(correct_lines);

    assert_eq!(correct_result, result);
}

#[test]
fn confirm_operand_commas_generates_error_on_missing_commas(){
    let lines = tokenize_instructions("Add $t1, $t2, $t3\nlw $t1 400($t2)".to_string());
    let mut result = build_instruction_list_from_lines(lines);
    confirm_operand_commas(&mut result);
    let correct_error = Error{ error_name: MissingComma, token_number_giving_error: 0 };
    assert_eq!(correct_error, result[1].errors[0]);
}