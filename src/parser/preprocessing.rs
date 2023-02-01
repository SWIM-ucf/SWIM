use crate::parser::parser_structs_and_enums::instruction_tokenization::ErrorType::{
    LabelAssignmentError, LabelMultipleDefinition, MissingComma,
};
use crate::parser::parser_structs_and_enums::instruction_tokenization::TokenType::{
    Label, Operator, Unknown,
};
use crate::parser::parser_structs_and_enums::instruction_tokenization::{
    Error, Instruction, Line, Token,
};
use std::collections::HashMap;

///Takes the initial string of the program given by the editor and turns it into a vector of Line,
/// a struct that holds tokens and the original line number, and finds the starting point for all comments.
pub fn tokenize_instructions(program: String) -> (Vec<Line>, Vec<[u32; 2]>) {
    let mut line_vec: Vec<Line> = Vec::new();
    let mut token: Token = Token {
        token_name: "".to_string(),
        starting_column: 0,
        token_type: Unknown,
    };
    let mut comments: Vec<[u32; 2]> = Vec::new();

    for (i, line_of_program) in program.lines().enumerate() {
        let mut line_of_tokens = Line {
            line_number: i as i32,

            tokens: vec![],
        };

        for (j, char) in line_of_program.chars().enumerate() {
            if char == '#' {
                comments.push([i as u32, j as u32]);
                break;
            };
            if char != ' ' {
                if token.token_name.is_empty() {
                    token.starting_column = j as i32;
                }
                token.token_name.push(char);
                if char == ',' {
                    if token.token_name.len() == 1 {
                        let length = line_of_tokens.tokens.len();
                        line_of_tokens.tokens[length - 1].token_name.push(char);
                    } else {
                        line_of_tokens.tokens.push(token.clone());
                    }
                    token.token_name = "".to_string();
                }
            } else if !token.token_name.is_empty() {
                line_of_tokens.tokens.push(token.clone());
                token.token_name = "".to_string();
            }
        }
        if !token.token_name.is_empty() {
            line_of_tokens.tokens.push(token.clone());
            token.token_name = "".to_string();
        }
        if !line_of_tokens.tokens.is_empty() {
            line_vec.push(line_of_tokens.clone());
        }
    }

    (line_vec, comments)
}

///This function takes the vector of lines created by tokenize instructions and turns them into instructions
///assigning labels, operators, operands, and line numbers
pub fn build_instruction_list_from_lines(mut lines: Vec<Line>) -> Vec<Instruction> {
    let mut instruction_list: Vec<Instruction> = Vec::new();
    let mut instruction = Instruction::default();

    let mut i = 0;
    //goes through each line of the line vector and builds instructions as it goes
    while i < lines.len() {
        let mut operand_iterator = 1;

        if lines[i].tokens[0].token_name.ends_with(':') {
            //if the instruction already has a label at this point, that means that the user wrote a label on a line on its
            //own and then wrote another label on the next line without ever finishing the first
            if instruction.label.is_some() {
                instruction.errors.push(Error {
                    error_name: LabelAssignmentError,
                    operand_number: None,
                })
                //if the above error doesn't occur, we can push the label to the instruction struct.
            } else {
                lines[i].tokens[0].token_name.pop();
                lines[i].tokens[0].token_type = Label;
                instruction.label = Some((lines[i].tokens[0].clone(), lines[i].line_number));
            }

            if lines[i].tokens.len() == 1 {
                //if the only token on the last line of the program is a label, the user never finished assigning a value to the label
                if i == (lines.len() - 1) {
                    instruction.errors.push(Error {
                        error_name: LabelAssignmentError,
                        operand_number: None,
                    });
                    instruction_list.push(instruction.clone());
                }

                i += 1;
                continue;
            }
            //since token[0] was a label, the operator will be token[1] and operands start at token[2]
            lines[i].tokens[1].token_type = Operator;
            instruction.operator = lines[i].tokens[1].clone();
            operand_iterator = 2;
        } else {
            lines[i].tokens[0].token_type = Operator;
            instruction.operator = lines[i].tokens[0].clone();
        }
        //push all operands to the instruction operand vec
        while operand_iterator < lines[i].tokens.len() {
            instruction
                .operands
                .push(lines[i].tokens[operand_iterator].clone());
            operand_iterator += 1;
        }
        instruction.line_number = lines[i].line_number as u32;

        //push completed instruction to the instruction vec
        instruction_list.push(instruction.clone());
        instruction = Instruction::default();

        i += 1;
    }

    instruction_list
}

///This function goes through all but the last operands of each instruction checking that they end in a comma.
/// If they do, the comma is removed. If they don't a missing comma error is generated.
pub fn confirm_operand_commas(instructions: &mut Vec<Instruction>) {
    for instruction in instructions {
        for i in 0..(instruction.operands.len() - 1) {
            if instruction.operands[i].token_name.ends_with(',') {
                instruction.operands[i].token_name.pop();
            } else {
                instruction.errors.push(Error {
                    error_name: MissingComma,
                    operand_number: Some(i as u8),
                })
            }
        }
    }
}

//TODO Add more pseudo instructions. Especially ones that are converted into more than a single instruction to make sure this method works
pub fn expand_pseudo_instruction(instruction_list: &mut [Instruction]) {
    for (_i, mut instruction) in instruction_list.iter_mut().enumerate() {
        match &*instruction.operator.token_name {
            "li" => {
                instruction.operator.token_name = "ori".to_string();

                instruction.operands.push(Token {
                    token_name: "$zero".to_string(),
                    starting_column: 0,
                    token_type: Default::default(),
                });
            }
            "temp_text_to_appease_Clippy_until_more_are_added" => {}
            _ => {}
        }
    }
}

///This function assigns the instruction number to each instruction
pub fn assign_instruction_numbers(instruction_list: &mut [Instruction]) {
    for (i, instruction) in instruction_list.iter_mut().enumerate() {
        instruction.instruction_number = i as u32;
    }
}

///Create_label_map builds a hashmap of addresses for labels in memory
pub fn create_label_map(instruction_list: &mut Vec<Instruction>) -> HashMap<String, u32> {
    let mut labels: HashMap<String, u32> = HashMap::new();

    for instruction in instruction_list {
        if instruction.label.is_some() {
            //if the given label name is already used, an error is generated
            if labels.contains_key(&*instruction.label.clone().unwrap().0.token_name) {
                instruction.errors.push(Error {
                    error_name: LabelMultipleDefinition,
                    operand_number: None,
                });
                //otherwise, it is inserted
            } else {
                labels.insert(
                    instruction.clone().label.unwrap().0.token_name,
                    instruction.clone().label.unwrap().1 as u32,
                );
            }
        }
    }

    //When support for labelled data is added, a for loop going through that vec right here will put it all after instructions in memory

    labels
}
