use crate::parser::parser_structs_and_enums::ErrorType::*;
use crate::parser::parser_structs_and_enums::TokenType::{Directive, Label, Operator, Unknown};
use crate::parser::parser_structs_and_enums::{
    Data, Error, Instruction, LabelInstance, MonacoLineInfo, Token, FP_REGISTERS, GP_REGISTERS,
    SUPPORTED_INSTRUCTIONS,
};
use levenshtein::levenshtein;
use std::collections::HashMap;

///Takes the initial string of the program given by the editor and turns it into a vector of Line,
/// a struct that holds tokens and the original line number.
pub fn tokenize_program(program: String) -> Vec<MonacoLineInfo> {
    let mut monaco_line_info_vec: Vec<MonacoLineInfo> = Vec::new();

    let mut token: Token = Token {
        token_name: "".to_string(),
        start_end_columns: (0, 0),
        token_type: Unknown,
    };

    for (i, line_of_program) in program.lines().enumerate() {
        let mut line = MonacoLineInfo {
            mouse_hover_string: "".to_string(),
            updated_monaco_string: line_of_program.to_string(),
            tokens: vec![],
            line_number: i,
            error_start_end_columns: vec![],
            errors: vec![],
        };

        let mut is_string = false;
        let mut check_escape = false;
        //iterates through every character on each line of the program
        for (j, char) in line_of_program.chars().enumerate() {
            token.start_end_columns.1 = j + 1;
            if char == '#' {
                if j > 0 {
                    token.start_end_columns.1 -= 1;
                }
                break;
            };
            //is string is a flag to handle strings and read them in as a single token
            if is_string {
                if char == '\\' {
                    check_escape = true;
                    continue;
                }
                if check_escape {
                    match char {
                        'n' => {
                            token.token_name.push('\n');
                        }
                        't' => {
                            token.token_name.push('\t');
                        }
                        '\\' => {
                            token.token_name.push('\\');
                        }
                        '\"' => {
                            token.token_name.push('\"');
                        }
                        '\'' => {
                            token.token_name.push('\'');
                        }
                        _ => {
                            token.token_name.push('\\');
                            token.token_name.push(char);
                        }
                    }
                    check_escape = false;
                } else if char == '\"' {
                    token.token_name.push('\"');
                    is_string = false;
                } else {
                    token.token_name.push(char);
                }
            } else if char == '\"' {
                if !token.token_name.is_empty() {
                    line.tokens.push(token.clone());
                }
                token.token_name = '\"'.to_string();
                token.start_end_columns.0 = j;
                is_string = true;
            } else if char != ' ' {
                if token.token_name.is_empty() {
                    token.start_end_columns.0 = j;
                }
                token.token_name.push(char);
                if char == ',' {
                    if token.token_name.len() == 1 {
                        let length = line.tokens.len();
                        line.tokens[length - 1].token_name.push(char);
                    } else {
                        token.start_end_columns.1 -= 1;
                        line.tokens.push(token.clone());
                    }
                    token.token_name = "".to_string();
                }
            } else if !token.token_name.is_empty() {
                token.start_end_columns.1 -= 1;
                line.tokens.push(token.clone());
                token.token_name = "".to_string();
            }
        }
        if !token.token_name.is_empty() {
            line.tokens.push(token.clone());
            token.token_name = "".to_string();
        }

        monaco_line_info_vec.push(line);
    }

    //creates an empty monaco line if there is nothing in Monaco.
    if monaco_line_info_vec.is_empty() {
        monaco_line_info_vec.push(MonacoLineInfo {
            mouse_hover_string: "".to_string(),
            updated_monaco_string: "".to_string(),
            tokens: vec![],
            line_number: 0,
            error_start_end_columns: vec![],
            errors: vec![],
        })
    }

    monaco_line_info_vec
}

///Checks the name of every token on a line and makes sure that labels, directives, and operators do not end in commas while
/// all but the last operand or datum on a line does. Also, pops commas off of the end of all tokens on the line.
pub fn remove_commas(line: &mut MonacoLineInfo) {
    //first check every token to see if they end in commas
    let mut has_commas: Vec<bool> = Vec::new();
    for token in &mut line.tokens {
        if token.token_name.ends_with(',') {
            token.token_name.pop();
            has_commas.push(true);
        } else {
            has_commas.push(false);
        }
    }
    //check all labels. No label should end in a comma
    let mut i = 0;
    while line.tokens.len() > i && line.tokens[i].token_name.ends_with(':') {
        if has_commas[i] {
            line.errors.push(Error {
                error_name: UnnecessaryComma,
                token_causing_error: line.tokens[i].clone().token_name,
                start_end_columns: line.tokens[i].start_end_columns,
                message: "".to_string(),
            });
        }
        i += 1;
    }
    //the following token should be the operator or the directive. Also should not end in a comma
    if line.tokens.len() > i && has_commas[i] {
        line.errors.push(Error {
            error_name: UnnecessaryComma,
            token_causing_error: line.tokens[i].clone().token_name,
            start_end_columns: line.tokens[i].start_end_columns,
            message: "".to_string(),
        });
    }
    i += 1;

    //all remaining tokens except the last should end in a comma
    while i < line.tokens.len() - 1 {
        if !has_commas[i] {
            line.errors.push(Error {
                error_name: MissingComma,
                token_causing_error: line.tokens[i].clone().token_name,
                start_end_columns: line.tokens[i].start_end_columns,
                message: "".to_string(),
            });
        }
        i += 1;
    }

    //finally, make sure the last token on the line does not end in a comma
    if line.tokens.len() == i + 1 && has_commas[i] {
        line.errors.push(Error {
            error_name: UnnecessaryComma,
            token_causing_error: line.tokens[i].clone().token_name,
            start_end_columns: line.tokens[i].start_end_columns,
            message: "".to_string(),
        });
    }
}

///This function takes the vector of lines created by tokenize program and turns them into instructions
///assigning labels, operators, operands, and line numbers and data assigning labels, data types, and values
pub fn separate_data_and_text(lines: &mut Vec<MonacoLineInfo>) -> (Vec<Instruction>, Vec<Data>) {
    let mut instruction_list: Vec<Instruction> = Vec::new();
    let mut data_list: Vec<Data> = Vec::new();
    let mut labels: Vec<LabelInstance> = Vec::new();

    let mut is_text = true;
    let mut i = 0;
    while i < lines.len() {
        if lines[i].tokens.is_empty() {
            i += 1;
            continue;
        }
        //check commas and remove them
        remove_commas(&mut lines[i]);

        //handle transitions between .data and .text
        if lines[i].tokens[0].token_name.to_lowercase() == ".text"
            || lines[i].tokens[0].token_name.to_lowercase() == ".data"
        {
            lines[i].tokens[0].token_type = Directive;
            while !labels.is_empty() {
                let last = labels.pop().unwrap();
                lines[last.token_line].errors.push(Error {
                    error_name: LabelAssignmentError,
                    token_causing_error: last.token.token_name,
                    start_end_columns: last.token.start_end_columns,
                    message: "".to_string(),
                });
            }
            if lines[i].tokens[0].token_name.to_lowercase() == ".text" {
                is_text = true;
            } else {
                is_text = false;
            }
            i += 1;
            continue;
        }
        let mut j = 0;
        //add all labels to the label stack
        while lines[i].tokens.len() > j && lines[i].tokens[j].token_name.ends_with(':') {
            lines[i].tokens[j].token_name.pop();
            lines[i].tokens[j].start_end_columns.1 -= 1;
            lines[i].tokens[j].token_type = Label;
            labels.push(LabelInstance {
                token: lines[i].tokens[j].clone(),
                token_line: i,
            });
            j += 1;
        }
        //make sure there are still tokens remaining on the line
        if lines[i].tokens.len() == j {
            i += 1;
            continue;
        }
        //this chunk handles how we read .text
        if is_text {
            let mut instruction = Instruction {
                line_number: i,
                ..Default::default()
            };
            //push all incomplete labels to reference this instruction
            while !labels.is_empty() {
                instruction.labels.push(labels.pop().unwrap());
            }
            //the next token is the operator
            lines[i].tokens[j].token_type = Operator;
            instruction.operator = lines[i].tokens[j].clone();
            j += 1;
            //any remaining tokens are the operands
            while lines[i].tokens.len() > j {
                instruction.operands.push(lines[i].tokens[j].clone());
                j += 1;
            }
            instruction_list.push(instruction);

            //this chunk handles how we read .data
        } else {
            let mut data = Data {
                line_number: i,
                ..Default::default()
            };
            if labels.is_empty() {
                //if labels is empty, generate an error but assume the first token on the line was supposed to be the label
                //Remove commas should have already generated a MissingComma error so we replace that with a more accurate error.
                lines[i].errors.pop();
                let start_end = lines[i].tokens[j].start_end_columns;
                let token_name = lines[i].tokens[j].token_name.clone();
                lines[i].errors.push(Error {
                    error_name: ImproperlyFormattedLabel,
                    token_causing_error: token_name,
                    start_end_columns: start_end,
                    message: "".to_string(),
                });
                data.label = lines[i].tokens[j].clone();
                j += 1;
            } else {
                data.label = labels.pop().unwrap().token;
            }
            //any other labels in the stack are pushed to the data_list to be initialized as empty words in the assemble data function
            for label in &labels {
                data_list.push(Data {
                    line_number: label.token_line,
                    label: label.token.clone(),
                    ..Default::default()
                });
            }
            labels = Vec::new();
            //the next token should be the data type directive
            data.data_type = lines[i].tokens[j].clone();
            j += 1;
            //any remaining tokens should be data entries
            while lines[i].tokens.len() > j {
                data.data_entries_and_values
                    .push((lines[i].tokens[j].clone(), 0));
                j += 1;
            }
            data_list.push(data);
        }
        i += 1;
    }

    //handle any unfinished labels
    if is_text {
        //unfinished labels in text cause an error
        for label in labels {
            lines[label.token_line].errors.push(Error {
                error_name: LabelAssignmentError,
                token_causing_error: label.token.token_name,
                start_end_columns: label.token.start_end_columns,
                message: "".to_string(),
            });
        }
    } else {
        //unfinished labels in data are pushed to the list to be initialized as empty words in the assembler
        for label in labels {
            data_list.push(Data {
                line_number: label.token_line,
                label: label.token,
                ..Default::default()
            });
        }
    }

    (instruction_list, data_list)
}

///Create_label_map builds a hashmap of addresses for labels in memory
pub fn create_label_map(
    instruction_list: &mut Vec<Instruction>,
    data_list: &mut [Data],
) -> HashMap<String, usize> {
    let mut labels: HashMap<String, usize> = HashMap::new();
    //iterate through every instance of instruction and try to add the label to the map
    for instruction in &mut *instruction_list {
        for label in instruction.labels.clone() {
            //if the given label name is already used, an error is generated
            if labels.contains_key(&*label.token.token_name) {
                instruction.errors.push(Error {
                    error_name: LabelMultipleDefinition,
                    token_causing_error: label.token.token_name,
                    start_end_columns: label.token.start_end_columns,
                    message: "".to_string(),
                });
                //otherwise, it is inserted
            } else {
                labels.insert(
                    label.token.token_name,
                    instruction.clone().instruction_number << 2,
                );
            }
        }
    }

    let last_instruction = instruction_list.last();

    let offset_for_instructions: u32 = if let Some(..) = last_instruction {
        (last_instruction.unwrap().instruction_number + 1) << 2
    } else {
        0
    } as u32;

    for (_i, data) in data_list.iter_mut().enumerate() {
        //if the given label name is already used, an error is generated
        if labels.contains_key(&*data.label.clone().token_name) {
            data.errors.push(Error {
                error_name: LabelMultipleDefinition,
                token_causing_error: data.label.token_name.to_string(),
                start_end_columns: data.label.start_end_columns,
                message: "".to_string(),
            });
            //otherwise, it is inserted
        } else {
            labels.insert(
                data.label.token_name.clone(),
                data.data_number + offset_for_instructions as usize,
            );
        }
    }

    labels
}

///Goes through each error found in the parsing & assembling process and suggests to the user a way of
/// correcting the error. This error message is attached to the corresponding instruction or data and monaco line info, and
/// compiled into a string to be output to the console and returns that
pub fn suggest_error_corrections(
    instructions: &mut [Instruction],
    data: &mut [Data],
    labels: &HashMap<String, usize>,
    monaco_line_info: &mut [MonacoLineInfo],
) -> String {
    let mut console_out_string: String = "".to_string();
    //go through each error in the instructions and suggest a correction
    for instruction in instructions {
        //if there are no errors, instead push the binary of the instruction to mouse hover
        if instruction.errors.is_empty() {
            monaco_line_info[instruction.line_number]
                .mouse_hover_string
                .push_str(&format!("\n\n**Binary:** `0b{:032b}`", instruction.binary));
        } else {
            for error in &mut instruction.errors {
                match error.error_name {
                    UnsupportedInstruction => {
                        error.message =
                             "While this is a valid instruction, it is not currently supported by SWIM\n"
                                 .to_string();
                    }
                    UnrecognizedGPRegister => {
                        let given_string = &error.token_causing_error;
                        let mut closest: (usize, String) = (usize::MAX, "".to_string());

                        for register in GP_REGISTERS {
                            if levenshtein(given_string, register.names[0]) < closest.0 {
                                closest.0 = levenshtein(given_string, register.names[0]);
                                closest.1 = register.names[0].to_string();
                            }
                        }

                        let mut suggestion =
                            "GP register is not recognized. A valid, similar register is: "
                                .to_string();
                        suggestion.push_str(&format!("{}.\n", &closest.1));
                        error.message = suggestion;
                    }
                    UnrecognizedFPRegister => {
                        let given_string = &error.token_causing_error;
                        let mut closest: (usize, String) = (usize::MAX, "".to_string());

                        for register in FP_REGISTERS {
                            if levenshtein(given_string, register.name) < closest.0 {
                                closest.0 = levenshtein(given_string, register.name);
                                closest.1 = register.name.to_string();
                            }
                        }

                        let mut suggestion =
                            "FP register is not recognized. A valid, similar register is: "
                                .to_string();
                        suggestion.push_str(&format!("{}.\n", &closest.1));
                        error.message = suggestion;
                    }
                    UnrecognizedInstruction => {
                        let given_string = &instruction.operator.token_name;
                        let mut closest: (usize, String) = (usize::MAX, "".to_string());

                        for instruction in SUPPORTED_INSTRUCTIONS {
                            if levenshtein(given_string, instruction) < closest.0 {
                                closest.0 = levenshtein(given_string, instruction);
                                closest.1 = instruction.to_string();
                            }
                        }

                        let mut suggestion = "A valid, similar instruction is: ".to_string();
                        suggestion.push_str(&format!("{}.\n", &closest.1));
                        error.message = suggestion;
                    }
                    IncorrectRegisterTypeGP => {
                        error.message =
                            "Expected FP register but received GP register.\n".to_string();
                    }
                    IncorrectRegisterTypeFP => {
                        error.message =
                            "Expected GP register but received FP register.\n".to_string();
                    }
                    MissingComma => {
                        error.message =
                            "Operand expected to end with a comma but it does not.\n".to_string()
                    }
                    ImmediateOutOfBounds => {
                        error.message = "Immediate value given cannot be expressed in the available number of bits.\n".to_string();
                    }
                    NonIntImmediate => {
                        error.message =
                            "The given string cannot be recognized as an integer.\n".to_string();
                    }
                    NonFloatImmediate => {
                        error.message =
                            "The given string cannot be recognized as a float.\n".to_string();
                    }
                    InvalidMemorySyntax => {
                        error.message = "The given string for memory does not match syntax of \"offset(base)\" or \"label\".\n".to_string();
                    }
                    IncorrectNumberOfOperands => {
                        error.message = "The given number of operands does not match the number expected for the given instruction.\n".to_string();
                    }
                    LabelMultipleDefinition => {
                        error.message =
                            "The given label name is already used elsewhere in the project.\n"
                                .to_string();
                    }
                    LabelAssignmentError => {
                        error.message = "A label is specified but it is not followed by data or an instruction committed to memory.\n".to_string();
                    }
                    LabelNotFound => {
                        if labels.is_empty() {
                            error.message = "There is no recognized labelled memory.\n".to_string();
                            continue;
                        }

                        let given_string = &error.token_causing_error;
                        let mut closest: (usize, String) = (usize::MAX, "".to_string());

                        for label in labels {
                            if levenshtein(given_string, label.0) < closest.0 {
                                closest.0 = levenshtein(given_string, label.0);
                                closest.1 = label.0.to_string();
                            }
                        }

                        let mut suggestion = "A valid, similar label is: ".to_string();
                        suggestion.push_str(&format!("{}.\n", &closest.1));
                        error.message = suggestion;
                    }
                    JALRRDRegisterZero => {
                        error.message =
                            "The destination address for JALR cannot be the zero register\n"
                                .to_string();
                    }
                    _ => {
                        error.message = format!("{:?} PARSER/ASSEMBLER ERROR. THIS ERROR TYPE SHOULD NOT BE ABLE TO BE ASSOCIATED WITH TEXT.\n", error.error_name);
                    }
                }

                //add the error to monaco_line_info
                if error.error_name == LabelAssignmentError
                    || error.error_name == LabelMultipleDefinition
                {
                    //todo remove following line once Jerrett has started referencing error and not just start_end_columns
                    monaco_line_info[instruction.labels.clone().last().unwrap().token_line]
                        .error_start_end_columns
                        .push(error.start_end_columns);

                    //add error to monaco_line_info
                    monaco_line_info[instruction.line_number]
                        .errors
                        .push(error.clone());
                } else {
                    //todo remove following line once Jerrett has started referencing error and not just start_end_columns
                    monaco_line_info[instruction.line_number]
                        .error_start_end_columns
                        .push(error.start_end_columns);

                    //add error to monaco_line_info
                    monaco_line_info[instruction.line_number]
                        .errors
                        .push(error.clone());
                }

                //push a message about the error to the string for console
                console_out_string.push_str(&format!(
                    "{} on line {} with token \"{}\"\n{}\n",
                    &error.error_name.to_string(),
                    &(instruction.line_number + 1).to_string(),
                    &error.token_causing_error,
                    &error.message
                ));
            }
        }
    }

    //special section to remove the syscall binary from mouse hover if syscall was added in by the parser since this would be added on to the mouse hover of a completely unrelated string
    let mut contains = false;
    for token in monaco_line_info.last().unwrap().tokens.clone() {
        if token.token_name == "syscall" {
            contains = true;
        }
    }
    let index = monaco_line_info.last().unwrap().line_number;
    if !contains {
        monaco_line_info[index].mouse_hover_string = monaco_line_info[index]
            .mouse_hover_string
            .replace("**Binary:** `0b00000000000000000000000000001100`", "");
    }

    //go through each error in the data and suggest a correction
    for datum in data {
        for error in &mut datum.errors {
            match &error.error_name {
                UnrecognizedDataType => {
                    let recognized_data_types = [
                        ".ascii", ".asciiz", ".byte", ".double", ".float", ".half", ".space",
                        ".word",
                    ];

                    let given_string = &datum.data_type.token_name.to_string();
                    let mut closest: (usize, String) = (usize::MAX, "".to_string());

                    for data_type in recognized_data_types {
                        if levenshtein(given_string, data_type) < closest.0 {
                            closest.0 = levenshtein(given_string, data_type);
                            closest.1 = data_type.to_string();
                        }
                    }

                    let mut suggestion = "A valid, similar data type is: ".to_string();
                    suggestion.push_str(&format!("{}.\n", &closest.1));
                    error.message = suggestion;
                }
                LabelAssignmentError => {
                    error.message = "A label is specified but it is not followed by data or an instruction committed to memory.\n".to_string();
                }
                LabelMultipleDefinition => {
                    error.message =
                        "The given label name is already used elsewhere in the project.\n"
                            .to_string();
                }
                ImmediateOutOfBounds => {
                    error.message = "Immediate value given cannot be expressed in the available number of bits.\n".to_string();
                }
                ImproperlyFormattedASCII => {
                    error.message =
                        "Token recognized as ASCII does not start and or end with double quotes (\").\n"
                            .to_string();
                }
                ImproperlyFormattedChar => {
                    error.message = "Token recognized as a char does not end with ' or is larger than a single char.\n".to_string();
                }
                MissingComma => {
                    error.message =
                        "Operand expected to end with a comma but it does not.\n".to_string()
                }
                NonIntImmediate => {
                    error.message =
                        "The given string cannot be recognized as an integer.\n".to_string();
                }
                NonFloatImmediate => {
                    error.message =
                        "The given string cannot be recognized as a float.\n".to_string();
                }
                ImproperlyFormattedLabel => {
                    error.message =
                        "Label assignment recognized but does not end in a colon.\n".to_string();
                }
                NonASCIIChar => {
                    error.message = "The given char cannot be represented in ASCII\n".to_string();
                }
                NonASCIIString => {
                    error.message =
                        "One or multiple characters within the given string cannot be represented in ASCII.\n".to_string();
                }
                _ => {
                    error.message = format!("{:?} PARSER/ASSEMBLER ERROR. THIS ERROR TYPE SHOULD NOT BE ABLE TO BE ASSOCIATED WITH DATA.\n", error.error_name);
                }
            }

            //todo remove following line once Jerrett has started referencing error and not just start_end_columns
            monaco_line_info[datum.line_number]
                .error_start_end_columns
                .push(error.start_end_columns);

            //add error to monaco_line_info
            monaco_line_info[datum.line_number]
                .errors
                .push(error.clone());

            console_out_string.push_str(&format!(
                "{} on line {} with token \"{}\"\n{}\n",
                &error.error_name.to_string(),
                &(datum.line_number + 1).to_string(),
                error.token_causing_error,
                error.message.clone()
            ));
        }
    }

    if console_out_string.is_empty() {
        return "Program assembled successfully!".to_string();
    }

    console_out_string
}
