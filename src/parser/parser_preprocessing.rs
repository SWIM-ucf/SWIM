use crate::parser::parser_instruction_tokenization::instruction_tokenization::ErrorType::{LabelAssignmentError, MissingComma};
use crate::parser::parser_instruction_tokenization::instruction_tokenization::TokenType::Unknown;
use crate::parser::parser_instruction_tokenization::instruction_tokenization::{
    Error, ErrorType, Instruction, Line, Token,
};

///This function takes the initial version of the string provided by the user and removes any unnecessary spaces, extra lines,
///and comments from the string and returns this new, cleaned version.
pub fn string_cleaning(string: String) -> String {
    let mut new_string = String::new();
    let mut is_comment = false;

    for c in string.chars() {
        //If the character is part of a comment, it is not appended to the new_string so the rest of this iteration is skipped.
        if is_comment {
            if c == '\n' {
                //If the character is a new line character, the comment is considered ended.
                is_comment = false;
            } else {
                continue;
            }
        }
        if c == '#' {
            //If the character is a #, the rest of that line is not appended to the new_string
            is_comment = true;
            continue;
        }

        //This removes spaces at the end of lines.
        if c == '\n' && new_string.ends_with(' ') {
            new_string.pop();
        }

        //The character is only pushed to the new string if it is not a space following another space or new line character
        //and it is not a new line character following a new line character
        if (c != ' ' || (!new_string.ends_with(c) && !new_string.ends_with('\n')))
            && (c != '\n' || !new_string.ends_with(c))
        {
            new_string.push(c);
        }
    }

    //Removes any starting or ending spaces or new line characters.
    if new_string.starts_with(' ') || new_string.starts_with('\n') {
        new_string.remove(0);
    }
    if new_string.ends_with(' ') || new_string.ends_with('\n') {
        new_string.pop();
    }

    new_string
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
                    token_number_giving_error: 0,
                })
                //if the above error doesn't occur, we can push the label to the instruction struct.
            } else {
                lines[i].tokens[0].token_name.pop();
                instruction.label = Some((lines[i].tokens[0].clone(), lines[i].line_number));
            }

            if lines[i].tokens.len() == 1 {
                //if the only token on the last line of the program is a label, the user never finished assigning a value to the label
                if i == (lines.len() - 1) {
                    instruction.errors.push(Error {
                        error_name: LabelAssignmentError,
                        token_number_giving_error: 0,
                    });
                    instruction_list.push(instruction.clone());
                }

                i = i + 1;
                continue;
            }
            //since token[0] was a label, the operator will be token[1] and operands start at token[2]
            instruction.operator = lines[i].tokens[1].clone();
            operand_iterator = 2;
        } else {
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

        i = i + 1;
    }

    return instruction_list;
}

///This function goes through all but the last operands of each instruction checking that they end in a comma.
/// If they do, the comma is removed. If they don't a missing comma error is generated.
pub fn confirm_operand_commas( instructions: &mut Vec<Instruction>) {
    for instruction in instructions{
        for i in 0..(instruction.operands.len() - 1) {
            if instruction.operands[i].token_name.ends_with(','){
                instruction.operands[i].token_name.pop();
            }else{
                instruction.errors.push(Error{ error_name: MissingComma, token_number_giving_error: i as u8 })
            }
        }
    }
}

//TODO Add more pseudo instructions. Especially ones that are converted into more than a single instruction to make sure this method works
pub fn convert_pseudo_instruction_into_real_instruction(
    instruction_list: Vec<Instruction>,
) -> Vec<Instruction> {
    for (_i, mut instruction) in instruction_list.into_iter().enumerate().clone() {
        match &*instruction.operator.token_name {
            "li" => {
                instruction.operator.token_name = "ori".to_string();

                instruction.operands.push(Token {
                    token_name: "$zero".to_string(),
                    starting_column: 0,
                    token_type: Default::default(),
                });
            }

            _ => {}
        }
    }
    return Vec::new();
}

///Create_vector_of_instructions takes the string of the MIPS program after comments, extra spaces, and label names have been removed
///and turns each line into an Instruction and returns the vec of these Instructions with the contents as tokens
pub fn create_vector_of_instructions(file_string: String) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    for (i, line) in file_string.lines().enumerate() {
        let instruction = create_instruction(line, i as u32);

        instructions.push(instruction);
    }
    instructions
}

///takes the string representation of a line of MIPS code and breaks it up into tokens delimited by space characters
pub fn create_instruction(line: &str, instruction_number: u32) -> Instruction {
    //breaks up line into a vector delimited by space characters
    let mut contents: Vec<String> = Vec::new();

    for token in line.split(' ') {
        contents.push(token.parse().unwrap());
    }

    //if the first token in the instruction ends with a :, it is a label.
    if contents[0].ends_with(':') {
        //remove the label from the contents and remove the : from the label
        let mut label = contents.get(0).unwrap().clone();
        contents.remove(0);
        label.pop();
    }

    //creates an instruction from the vector without a label
    Instruction {
        tokens: contents,
        instruction_number,
        ..Default::default()
    }
}

///This function takes an instruction as its argument and checks that every token within it except the first and the last (ie all but the last operand) ends with the ',' character
///for each of these that does end in a comma, the comma is removed. Any instance that this isn't the case generates a missingComma error that is added to the error list for that instruction.
///the updated version of the instruction is then returned
pub fn confirm_commas_in_instruction(mut instruction: Instruction) -> Instruction {
    //for loop goes through all but the first and last tokens
    for i in 1..(instruction.tokens.len() - 1) {
        let last_char = instruction.tokens.get(i).unwrap().chars().last().unwrap();

        if last_char == ',' {
            //this chunk of code removes the last char of the string if it is a ','
            //due to mutability issues, instruction.tokens.get(i).pop() does not work so instead we create a new string without the comma and replace the token instead
            let mut token_as_chars: Vec<char> =
                instruction.tokens.get(i).unwrap().chars().collect();
            token_as_chars.remove(token_as_chars.len() - 1);
            instruction
                .tokens
                .push(token_as_chars.into_iter().collect());
            let length = instruction.tokens.len() - 1;
            instruction.tokens.swap(i, length);
            instruction.tokens.pop();
        } else {
            //if the last char of the token is not ',', an error is pushed to the list
            instruction.errors.push(Error {
                error_name: MissingComma,
                token_number_giving_error: i as u8,
            })
        }
    }

    instruction
}

///This function takes the initial string of the program given by the editor and
pub fn tokenize_instructions(program: String) -> Vec<Line> {
    let mut line_vec: Vec<Line> = Vec::new();
    let mut token: Token = Token {
        token_name: "".to_string(),
        starting_column: 0,
        token_type: Unknown,
    };

    for (i, line_of_program) in program.lines().enumerate() {
        let mut line_of_tokens = Line {
            line_number: i as i32,

            tokens: vec![],
        };

        for (j, char) in line_of_program.chars().enumerate() {
            if char == '#' {
                break;
            };
            if char != ' ' {
                if token.token_name.is_empty() {
                    token.starting_column = j as i32;
                }
                token.token_name.push(char);
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

    line_vec
}
