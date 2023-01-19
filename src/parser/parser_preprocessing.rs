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

pub fn convert_pseudo_instruction_into_real_instruction() {}

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
                error_name: ErrorType::MissingComma,
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
