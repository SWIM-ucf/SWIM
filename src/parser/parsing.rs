use crate::parser::parser_structs_and_enums::instruction_tokenization::ErrorType::*;
use crate::parser::parser_structs_and_enums::instruction_tokenization::TokenType::{
    Label, Operator, Unknown,
};
use crate::parser::parser_structs_and_enums::instruction_tokenization::{Data, Error, Instruction, Line, MonacoLineInfo, Token};
use levenshtein::levenshtein;
use std::collections::HashMap;
///Takes the initial string of the program given by the editor and turns it into a vector of Line,
/// a struct that holds tokens and the original line number.
pub fn tokenize_program(program: String) -> (Vec<Line>, Vec<String>, Vec<MonacoLineInfo>) {
    let mut monaco_line_info_vec: Vec<MonacoLineInfo> = Vec::new();

    let mut line_vec: Vec<Line> = Vec::new();
    let mut token: Token = Token {
        token_name: "".to_string(),
        starting_column: 0,
        token_type: Unknown,
    };
    let mut lines_in_monaco: Vec<String> = Vec::new();

    for (i, line_of_program) in program.lines().enumerate() {
        monaco_line_info_vec.push(MonacoLineInfo{
            mouse_hover_string: "".to_string(),
            error_start_end_columns: vec![],
        });

        lines_in_monaco.push(line_of_program.parse().unwrap());

        let mut line_of_tokens = Line {
            line_number: i as u32,

            tokens: vec![],
        };
        let mut is_string = false;
        let mut check_escape = false;
        for (j, char) in line_of_program.chars().enumerate() {
            if char == '#' {
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
                    line_of_tokens.tokens.push(token.clone());
                }
                token.token_name = '\"'.to_string();
                token.starting_column = j as u32;
                is_string = true;
            } else if char != ' ' {
                if token.token_name.is_empty() {
                    token.starting_column = j as u32;
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

    (line_vec, lines_in_monaco, monaco_line_info_vec)
}

///This function takes the vector of lines created by tokenize program and turns them into instructions
///assigning labels, operators, operands, and line numbers and data assigning labels, data types, and values
pub fn separate_data_and_text(mut lines: Vec<Line>) -> (Vec<Instruction>, Vec<Data>) {
    let mut instruction_list: Vec<Instruction> = Vec::new();
    let mut instruction = Instruction::default();
    let mut data_list: Vec<Data> = Vec::new();
    let mut data = Data::default();
    let mut is_text = true;

    let mut i = 0;
    //goes through each line of the line vector and builds instructions as it goes
    while i < lines.len() {
        if lines[i].tokens[0].token_name == ".text" {
            is_text = true;
            i += 1;
            continue;
        } else if lines[i].tokens[0].token_name == ".data" {
            is_text = false;
            i += 1;
            continue;
        }

        if is_text {
            let mut operand_iterator = 1;

            if lines[i].tokens[0].token_name.ends_with(':') {
                //if the instruction already has a label at this point, that means that the user wrote a label on a line on its
                //own and then wrote another label on the next line without ever finishing the first
                if instruction.label.is_some() {
                    instruction.errors.push(Error {
                        error_name: LabelAssignmentError,
                        operand_number: None,
                        message: "".to_string(),
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
                            message: "".to_string(),
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

            let first_operand_index = operand_iterator;

            //push all operands to the instruction operand vec that will have commas
            while operand_iterator < (lines[i].tokens.len() - 1) {
                if lines[i].tokens[operand_iterator].token_name.ends_with(',') {
                    lines[i].tokens[operand_iterator].token_name.pop();
                } else {
                    instruction.errors.push(Error {
                        error_name: MissingComma,
                        operand_number: Some((operand_iterator - first_operand_index) as u8),
                        message: "".to_string(),
                    })
                }
                instruction
                    .operands
                    .push(lines[i].tokens[operand_iterator].clone());
                operand_iterator += 1;
            }

            //simple statement to handle cases where the user doesn't finish instructions
            if operand_iterator >= lines[i].tokens.len() {
                instruction_list.push(instruction.clone());
                i += 1;
                continue;
            }

            //push last operand that will not have a comma
            instruction
                .operands
                .push(lines[i].tokens[operand_iterator].clone());

            instruction.line_number = lines[i].line_number;

            //push completed instruction to the instruction vec
            instruction_list.push(instruction.clone());
            instruction = Instruction::default();
        }
        //if not text, it must be data
        else {
            data.line_number = lines[i].line_number;

            //the first token should be the label name
            if lines[i].tokens[0].token_name.ends_with(':') {
                lines[i].tokens[0].token_name.pop();
                lines[i].tokens[0].token_type = Label;
                data.label = lines[i].tokens[0].clone();
            } else {
                data.errors.push(Error {
                    error_name: ImproperlyFormattedLabel,
                    operand_number: Some(0),
                    message: "".to_string(),
                });
                lines[i].tokens[0].token_type = Label;
                data.label = lines[i].tokens[0].clone();
            }

            //just a simple check in case the user didn't complete a line
            if lines[i].tokens.len() < 2 {
                data.errors.push(Error {
                    error_name: ImproperlyFormattedData,
                    operand_number: None,
                    message: "".to_string(),
                });
                i += 1;
                continue;
            }

            //the second token on the line is the data type
            data.data_type = lines[i].tokens[1].clone();

            let mut value_iterator = 2;
            let first_value_index = value_iterator;

            //push all values to the data vec that will have commas
            while value_iterator < (lines[i].tokens.len() - 1) {
                if lines[i].tokens[value_iterator].token_name.ends_with(',') {
                    lines[i].tokens[value_iterator].token_name.pop();
                } else {
                    instruction.errors.push(Error {
                        error_name: MissingComma,
                        operand_number: Some((value_iterator - first_value_index) as u8),
                        message: "".to_string(),
                    })
                }
                data.data_entries_and_values
                    .push((lines[i].tokens[value_iterator].clone(), 0));
                value_iterator += 1;
            }

            //push last operand that will not have a comma
            data.data_entries_and_values
                .push((lines[i].tokens[value_iterator].clone(), 0));

            data_list.push(data.clone());
            data = Data::default();
        }
        i += 1;
    }

    (instruction_list, data_list)
}

///Iterates through the instruction list and translates pseudo-instructions into real instructions.
/// LW and SW with labelled memory are not completely translated in this step because they require
/// the address of the labelled memory to be known which is not found until after all other pseudo-instructions
/// have been translated. Updated pseudo-instructions are added to updated_monaco_string to appear in the editor after assembly.
/// Also ensures a syscall is at the end of the program
pub fn expand_pseudo_instructions_and_assign_instruction_numbers(
    instructions: &mut Vec<Instruction>,
    data: &Vec<Data>,
    updated_monaco_strings: &mut Vec<String>,
    monaco_line_info_vec: &mut Vec<MonacoLineInfo>,
) {
    //figure out list of labels to be used for lw and sw labels
    let mut list_of_labels: Vec<String> = Vec::new();
    for instruction in instructions.clone() {
        if instruction.label.is_some() {
            list_of_labels.push(instruction.clone().label.unwrap().0.token_name);
        }
    }
    for data in data {
        list_of_labels.push(data.label.token_name.clone());
    }

    //vec_of_added_instructions is needed because of rust ownership rules. It will not let us
    //insert into instruction_list while instruction_list is being iterated over.
    let mut vec_of_added_instructions: Vec<Instruction> = Vec::new();

    for (i, mut instruction) in &mut instructions.iter_mut().enumerate() {
        instruction.instruction_number = (i + vec_of_added_instructions.len()) as u32;
        match &*instruction.operator.token_name {
            "li" => {
                instruction.operator.token_name = "ori".to_string();

                instruction.operands.push(Token {
                    token_name: "$zero".to_string(),
                    starting_column: 0,
                    token_type: Default::default(),
                });

                monaco_line_info_vec[instruction.line_number as usize].mouse_hover_string = "li is a pseudo-instruction.\nli reg1, immediate => ori reg1, $zero, immediate".to_string();

            }
            "seq" => {
                //seq $regA, $regB, $regC turns into:
                //sub $regA, $regB, $regC
                //ori $at, $zero, 1
                //sltu $regA, $regA, $at

                //make sure there are enough operands
                if instruction.operands.len() < 3 {
                    continue;
                }
                //sub the two registers to find the difference
                let mut extra_instruction = instruction.clone();
                extra_instruction.operator.token_name = "sub".to_string();
                extra_instruction.line_number = 0;
                vec_of_added_instructions.push(extra_instruction);

                //put a 1 in $at
                let extra_instruction_2 = Instruction {
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
                        },
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number + 1,
                    line_number: 0,
                    errors: vec![],
                    label: None,
                };
                vec_of_added_instructions.push(extra_instruction_2);

                //set r0 to 1 if r1 - r2 == 0
                instruction.operator.token_name = "sltu".to_string();
                instruction.operands[1].token_name = instruction.operands[0].token_name.clone();
                instruction.operands[1].starting_column = instruction.operands[0].starting_column
                    + instruction.operands[0].token_name.len() as u32
                    + 2;
                instruction.operands[2].token_name = "$at".to_string();
                instruction.operands[2].starting_column = instruction.operands[1].starting_column
                    + instruction.operands[1].token_name.len() as u32
                    + 2;
                instruction.instruction_number += 2;
            }
            "sne" => {
                //sne $regA, $regB, $regC turns into:
                //sub $regA, $regB, $regC
                //sltu $regA, $zero, $regA

                //make sure there are enough operands
                if instruction.operands.len() < 3 {
                    continue;
                }
                //sub the two registers to find the difference
                let mut extra_instruction = instruction.clone();
                extra_instruction.operator.token_name = "sub".to_string();
                extra_instruction.line_number = 0;
                vec_of_added_instructions.push(extra_instruction);

                //set r0 to 1 if r1 - r2 != 0
                instruction.operator.token_name = "sltu".to_string();
                instruction.operands[1].token_name = "$zero".to_string();
                instruction.operands[1].starting_column = instruction.operands[0].starting_column
                    + instruction.operands[0].token_name.len() as u32
                    + 2;
                instruction.operands[2].token_name = instruction.operands[0].token_name.clone();
                instruction.operands[2].starting_column = instruction.operands[1].starting_column
                    + instruction.operands[1].token_name.len() as u32
                    + 2;
                instruction.instruction_number += 1;
            }
            "sle" => {
                //sle $regA, $regB, $regC is translated to:
                // slt $regA, $regC, $regB
                // addi $regA, $regA, 1
                // andi $regA, $regA, 1

                //make sure there are enough operands
                if instruction.operands.len() < 3 {
                    continue;
                }

                //slt
                let mut extra_instruction = instruction.clone();
                let temp = extra_instruction.operands[1].clone();
                extra_instruction.operands[1] = extra_instruction.operands[2].clone();
                extra_instruction.operands[1].starting_column = temp.starting_column;
                extra_instruction.operands[2] = temp.clone();
                extra_instruction.operands[2].starting_column =
                    temp.starting_column + temp.token_name.len() as u32 + 2;
                extra_instruction.operator.token_name = "slt".to_string();
                extra_instruction.line_number = 0;
                vec_of_added_instructions.push(extra_instruction);

                //addi
                let extra_instruction_2 = Instruction {
                    operator: Token {
                        token_name: "addi".to_string(),
                        starting_column: 0,
                        token_type: Operator,
                    },
                    operands: vec![
                        Token {
                            token_name: instruction.operands[0].token_name.clone(),
                            starting_column: 5,
                            token_type: Default::default(),
                        },
                        Token {
                            token_name: instruction.operands[0].token_name.clone(),
                            starting_column: (instruction.operands[0].token_name.len() + 7) as u32,
                            token_type: Default::default(),
                        },
                        Token {
                            token_name: "1".to_string(),
                            starting_column: (instruction.operands[0].token_name.len() * 2 + 9)
                                as u32,
                            token_type: Default::default(),
                        },
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number + 1,
                    line_number: 0,
                    errors: vec![],
                    label: None,
                };
                vec_of_added_instructions.push(extra_instruction_2);

                //andi
                instruction.operator.token_name = "andi".to_string();
                instruction.operands[0].starting_column += 1;
                instruction.operands[1].token_name = instruction.operands[0].token_name.clone();
                instruction.operands[1].starting_column += 1;
                instruction.operands[2].token_name = "1".to_string();
                instruction.operands[2].starting_column = instruction.operands[1].starting_column
                    + instruction.operands[1].token_name.len() as u32
                    + 2;
                instruction.instruction_number += 2;
            }
            "sleu" => {
                //sleu $regA, $regB, $regC is translated to:
                // sltu $regA, $regC, $regB
                // addi $regA, $regA, 1
                // andi $regA, $regA, 1

                //make sure there are enough operands
                if instruction.operands.len() < 3 {
                    continue;
                }

                //sltu
                let mut extra_instruction = instruction.clone();
                let temp = extra_instruction.operands[1].clone();
                extra_instruction.operands[1] = extra_instruction.operands[2].clone();
                extra_instruction.operands[1].starting_column = temp.starting_column;
                extra_instruction.operands[2] = temp.clone();
                extra_instruction.operands[2].starting_column =
                    temp.starting_column + temp.token_name.len() as u32 + 2;
                extra_instruction.operator.token_name = "sltu".to_string();
                extra_instruction.line_number = 0;
                vec_of_added_instructions.push(extra_instruction);

                //addi
                let extra_instruction_2 = Instruction {
                    operator: Token {
                        token_name: "addi".to_string(),
                        starting_column: 0,
                        token_type: Operator,
                    },
                    operands: vec![
                        Token {
                            token_name: instruction.operands[0].token_name.clone(),
                            starting_column: 5,
                            token_type: Default::default(),
                        },
                        Token {
                            token_name: instruction.operands[0].token_name.clone(),
                            starting_column: (instruction.operands[0].token_name.len() + 7) as u32,
                            token_type: Default::default(),
                        },
                        Token {
                            token_name: "1".to_string(),
                            starting_column: (instruction.operands[0].token_name.len() * 2 + 9)
                                as u32,
                            token_type: Default::default(),
                        },
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number + 1,
                    line_number: 0,
                    errors: vec![],
                    label: None,
                };
                vec_of_added_instructions.push(extra_instruction_2);

                //andi
                instruction.operator.token_name = "andi".to_string();
                instruction.operands[1].token_name = instruction.operands[0].token_name.clone();
                instruction.operands[2].token_name = "1".to_string();
                instruction.operands[2].starting_column = instruction.operands[1].starting_column
                    + instruction.operands[1].token_name.len() as u32
                    + 2;
                instruction.instruction_number += 2;
            }
            "sgt" => {
                //sgt $regA, $regB, $regC is translated to:
                // slt $regA, $regC, $regB

                //make sure that there actually is a third operand
                if instruction.operands.len() < 3 {
                    continue;
                }
                let temp = instruction.operands[1].clone();
                instruction.operands[1] = instruction.operands[2].clone();
                instruction.operands[1].starting_column = temp.starting_column;
                instruction.operands[2] = temp.clone();
                instruction.operands[2].starting_column =
                    temp.starting_column + temp.token_name.len() as u32 + 1;
                instruction.operator.token_name = "slt".to_string();
            }
            "sgtu" => {
                //sgtu $regA, $regB, $regC is translated to:
                // sltu $regA, $regC, $regB

                //make sure that there actually is a third operand
                if instruction.operands.len() < 3 {
                    continue;
                }
                let temp = instruction.operands[1].clone();
                instruction.operands[1] = instruction.operands[2].clone();
                instruction.operands[1].starting_column = temp.starting_column;
                instruction.operands[2] = temp.clone();
                instruction.operands[2].starting_column =
                    temp.starting_column + temp.token_name.len() as u32 + 1;
                instruction.operator.token_name = "sltu".to_string();
            }
            "sge" => {
                //sge $regA, $regB, $regC is translated to:
                // slt $regA, $regB, $regC
                // addi $regA, $regA, 1
                // andi $regA, $regA, 1

                //make sure there are enough operands
                if instruction.operands.len() < 3 {
                    continue;
                }

                //slt
                let mut extra_instruction = instruction.clone();
                extra_instruction.operator.token_name = "slt".to_string();
                extra_instruction.line_number = 0;
                vec_of_added_instructions.push(extra_instruction);

                //addi
                let extra_instruction_2 = Instruction {
                    operator: Token {
                        token_name: "addi".to_string(),
                        starting_column: 0,
                        token_type: Operator,
                    },
                    operands: vec![
                        Token {
                            token_name: instruction.operands[0].token_name.clone(),
                            starting_column: 5,
                            token_type: Default::default(),
                        },
                        Token {
                            token_name: instruction.operands[0].token_name.clone(),
                            starting_column: (instruction.operands[0].token_name.len() + 7) as u32,
                            token_type: Default::default(),
                        },
                        Token {
                            token_name: "1".to_string(),
                            starting_column: (instruction.operands[0].token_name.len() * 2 + 9)
                                as u32,
                            token_type: Default::default(),
                        },
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number + 1,
                    line_number: 0,
                    errors: vec![],
                    label: None,
                };
                vec_of_added_instructions.push(extra_instruction_2);

                //andi
                instruction.operator.token_name = "andi".to_string();
                instruction.operands[0].starting_column += 1;
                instruction.operands[1].token_name = instruction.operands[0].token_name.clone();
                instruction.operands[1].starting_column += 1;
                instruction.operands[2].token_name = "1".to_string();
                instruction.operands[2].starting_column = instruction.operands[1].starting_column
                    + instruction.operands[1].token_name.len() as u32
                    + 2;
                instruction.instruction_number += 2;
            }
            "sgeu" => {
                //sgeu $regA, $regB, $regC is translated to:
                // sltu $regA, $regC, $regB
                // addi $regA, $regA, 1
                // andi $regA, $regA, 1

                //make sure there are enough operands
                if instruction.operands.len() < 3 {
                    continue;
                }

                //sltu
                let mut extra_instruction = instruction.clone();
                extra_instruction.operator.token_name = "sltu".to_string();
                extra_instruction.line_number = 0;
                vec_of_added_instructions.push(extra_instruction);

                //addi
                let extra_instruction_2 = Instruction {
                    operator: Token {
                        token_name: "addi".to_string(),
                        starting_column: 0,
                        token_type: Operator,
                    },
                    operands: vec![
                        Token {
                            token_name: instruction.operands[0].token_name.clone(),
                            starting_column: 5,
                            token_type: Default::default(),
                        },
                        Token {
                            token_name: instruction.operands[0].token_name.clone(),
                            starting_column: (instruction.operands[0].token_name.len() + 7) as u32,
                            token_type: Default::default(),
                        },
                        Token {
                            token_name: "1".to_string(),
                            starting_column: (instruction.operands[0].token_name.len() * 2 + 9)
                                as u32,
                            token_type: Default::default(),
                        },
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number + 1,
                    line_number: 0,
                    errors: vec![],
                    label: None,
                };
                vec_of_added_instructions.push(extra_instruction_2);

                //andi
                instruction.operator.token_name = "andi".to_string();
                instruction.operands[1].token_name = instruction.operands[0].token_name.clone();
                instruction.operands[2].token_name = "1".to_string();
                instruction.operands[2].starting_column = instruction.operands[1].starting_column
                    + instruction.operands[1].token_name.len() as u32
                    + 2;
                instruction.instruction_number += 2;
            }
            "lw" | "sw" => {
                if instruction.operands.len() > 1
                    && list_of_labels.contains(&instruction.operands[1].token_name)
                {
                    let extra_instruction = Instruction {
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
                                token_name: instruction.operands[1].token_name.clone(),
                                starting_column: 9,
                                token_type: Default::default(),
                            },
                        ],
                        binary: 0,
                        instruction_number: instruction.instruction_number,
                        line_number: 0,
                        errors: vec![],
                        label: None,
                    };
                    vec_of_added_instructions.push(extra_instruction);
                    instruction.operands[1].token_name = "$at".to_string();
                    instruction.instruction_number += 1;
                }
            }
            "subi" => {
                //make sure that there actually is a third operand
                if instruction.operands.len() < 3 {
                    continue;
                }
                let extra_instruction = Instruction {
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
                            token_name: instruction.operands[2].token_name.clone(),
                            starting_column: 16,
                            token_type: Default::default(),
                        },
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number,
                    line_number: 0,
                    errors: vec![],
                    label: None,
                };
                vec_of_added_instructions.push(extra_instruction);
                //adjust subi for the added instruction
                instruction.operator.token_name = "sub".to_string();
                instruction.operands[0].starting_column -= 1;
                instruction.operands[1].starting_column -= 1;
                instruction.operands[2].starting_column -= 1;
                instruction.operands[2].token_name = "$at".to_string();
                instruction.instruction_number += 1;
            }
            "dsubi" => {
                //make sure that there actually is a third operand
                if instruction.operands.len() < 3 {
                    continue;
                }
                let extra_instruction = Instruction {
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
                            token_name: instruction.operands[2].token_name.clone(),
                            starting_column: 16,
                            token_type: Default::default(),
                        },
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number,
                    line_number: 0,
                    errors: vec![],
                    label: None,
                };
                vec_of_added_instructions.push(extra_instruction);
                //adjust subi for the added instruction
                instruction.operator.token_name = "dsub".to_string();
                instruction.operands[0].starting_column -= 1;
                instruction.operands[1].starting_column -= 1;
                instruction.operands[2].starting_column -= 1;
                instruction.operands[2].token_name = "$at".to_string();
                instruction.instruction_number += 1;
            }
            "dsubiu" => {}
            "muli" => {
                //make sure that there actually is a third operand
                if instruction.operands.len() < 3 {
                    continue;
                }
                let extra_instruction = Instruction {
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
                            token_name: instruction.operands[2].token_name.clone(),
                            starting_column: 16,
                            token_type: Default::default(),
                        },
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number,
                    line_number: 0,
                    errors: vec![],
                    label: None,
                };
                vec_of_added_instructions.push(extra_instruction);
                //adjust subi for the added instruction
                instruction.operator.token_name = "mul".to_string();
                instruction.operands[0].starting_column -= 1;
                instruction.operands[1].starting_column -= 1;
                instruction.operands[2].starting_column -= 1;
                instruction.operands[2].token_name = "$at".to_string();
                instruction.instruction_number += 1;
            }
            "dmuli" => {
                //make sure that there actually is a third operand
                if instruction.operands.len() < 3 {
                    continue;
                }
                let extra_instruction = Instruction {
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
                            token_name: instruction.operands[2].token_name.clone(),
                            starting_column: 16,
                            token_type: Default::default(),
                        },
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number,
                    line_number: 0,
                    errors: vec![],
                    label: None,
                };
                vec_of_added_instructions.push(extra_instruction);
                //adjust subi for the added instruction
                instruction.operator.token_name = "dmul".to_string();
                instruction.operands[0].starting_column -= 1;
                instruction.operands[1].starting_column -= 1;
                instruction.operands[2].starting_column -= 1;
                instruction.operands[2].token_name = "$at".to_string();
                instruction.instruction_number += 1;
            }
            "dmuliu" => {}
            "divi" => {
                //make sure that there actually is a second operand
                if instruction.operands.len() < 2 {
                    continue;
                }
                let extra_instruction = Instruction {
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
                            token_name: instruction.operands[1].token_name.clone(),
                            starting_column: 16,
                            token_type: Default::default(),
                        },
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number,
                    line_number: 0,
                    errors: vec![],
                    label: None,
                };
                vec_of_added_instructions.push(extra_instruction);
                //adjust subi for the added instruction
                instruction.operator.token_name = "div".to_string();
                instruction.operands[0].starting_column -= 1;
                instruction.operands[1].starting_column -= 1;
                instruction.operands[1].token_name = "$at".to_string();
                instruction.instruction_number += 1;
            }
            "ddivi" => {
                //make sure that there actually is a second operand
                if instruction.operands.len() < 2 {
                    continue;
                }
                let extra_instruction = Instruction {
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
                            token_name: instruction.operands[1].token_name.clone(),
                            starting_column: 16,
                            token_type: Default::default(),
                        },
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number,
                    line_number: 0,
                    errors: vec![],
                    label: None,
                };
                vec_of_added_instructions.push(extra_instruction);
                //adjust subi for the added instruction
                instruction.operator.token_name = "ddiv".to_string();
                instruction.operands[0].starting_column -= 1;
                instruction.operands[1].starting_column -= 1;
                instruction.operands[1].token_name = "$at".to_string();
                instruction.instruction_number += 1;
            }
            "ddiviu" => {}
            _ => {}
        }
    }

    //insert all new new instructions
    for instruction in vec_of_added_instructions {
        instructions.insert(instruction.instruction_number as usize, instruction);
    }

    //if there aren't any instructions, add a syscall to monaco's updated string so the emulation core does not try to run data as an instruction
    if instructions.is_empty() {
        //try to find an instance of .text
        let mut text_index: Option<u32> = None;
        for (i, mut line) in updated_monaco_strings.clone().into_iter().enumerate() {
            line = line.replace(' ', "");
            line = line.replace('#', " ");
            if line.starts_with(".text") {
                text_index = Some(i as u32);
                break;
            }
        }
        if let Some(..) = text_index {
            //add syscall after first index of .text if it exists
            updated_monaco_strings.insert(text_index.unwrap() as usize + 1, "syscall".to_string());
        } else {
            //otherwise, add it at the beginning of monaco
            updated_monaco_strings.insert(0, ".text".to_string());
            updated_monaco_strings.insert(1, "syscall".to_string());
        }
    } else {
        let last_instruction = instructions.last().unwrap();
        //if the last instruction in monaco is not a syscall, add it in
        if last_instruction.operator.token_name != "syscall" {
            updated_monaco_strings.insert(
                last_instruction.line_number as usize + 1,
                "syscall".to_string(),
            );
        }
    }
}

///Create_label_map builds a hashmap of addresses for labels in memory
pub fn create_label_map(
    instruction_list: &mut Vec<Instruction>,
    data_list: &mut [Data],
) -> HashMap<String, u32> {
    let mut labels: HashMap<String, u32> = HashMap::new();
    for instruction in &mut *instruction_list {
        if instruction.label.is_some() {
            //if the given label name is already used, an error is generated
            if labels.contains_key(&*instruction.label.clone().unwrap().0.token_name) {
                instruction.errors.push(Error {
                    error_name: LabelMultipleDefinition,
                    operand_number: None,
                    message: "".to_string(),
                });
                //otherwise, it is inserted
            } else {
                labels.insert(
                    instruction.clone().label.unwrap().0.token_name,
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
    };

    for (i, data) in data_list.iter_mut().enumerate() {
        //if the given label name is already used, an error is generated
        if labels.contains_key(&*data.label.clone().token_name) {
            data.errors.push(Error {
                error_name: LabelMultipleDefinition,
                operand_number: Some(i as u8),
                message: "".to_string(),
            });
            //otherwise, it is inserted
        } else {
            labels.insert(
                data.label.token_name.clone(),
                data.data_number + offset_for_instructions,
            );
        }
    }

    labels
}

///the second part of completing pseudo-instructions. LW and SW with labels requires the address of the label to be known,
/// the second part of this must occur after the label hashmap is completed.
pub fn complete_lw_sw_pseudo_instructions(
    instructions: &mut Vec<Instruction>,
    labels: &HashMap<String, u32>,
    _updated_monaco_strings: &mut [String],
) {
    if instructions.len() < 2 {
        return;
    }
    for mut index in 0..(instructions.len() - 1) {
        if instructions[index].operator.token_name == "lui"
            && instructions[index].operands.len() > 1
            && labels.contains_key(&*instructions[index].operands[1].token_name)
            && (instructions[index + 1].operator.token_name == "sw"
                || instructions[index + 1].operator.token_name == "lw")
        {
            //upper 16 bits are stored in $at using lui
            let address = *labels
                .get(&*instructions[index].operands[1].token_name)
                .unwrap();
            instructions[index].operands[1].token_name = (address >> 16).to_string();
            index += 1;

            //lower 16 bits are stored as the offset for the load/store operation
            let lower_16_bits = address as u16;
            let mut memory_operand = lower_16_bits.to_string();
            memory_operand.push_str("($at)");
            instructions[index].operands[1].token_name = memory_operand;
        }
    }
}

///Goes through each error found in the parsing & assembling process and suggests to the user a way of
/// correcting the error.
pub fn suggest_error_corrections(
    instructions: &mut [Instruction],
    data: &mut [Data],
    labels: &HashMap<String, u32>,
) {
    //go through each error in the instructions and suggest a correction
    for instruction in instructions {
        for error in &mut instruction.errors {
            match error.error_name {
                UnsupportedInstruction => {
                    error.message =
                        "While this is a valid instruction, it is not currently supported by SWIM"
                            .to_string();
                }
                UnrecognizedGPRegister => {
                    let gp_registers = [
                        "$zero", "$at", "$v0", "$v1", "$a0", "$a1", "$a2", "$a3", "$t0", "$t1",
                        "$t2", "$t3", "$t4", "$t5", "$t6", "$t7", "$s0", "$s1", "$s2", "$s3",
                        "$s4", "$s5", "$s6", "$s7", "$t8", "$t9", "$k0", "$k1", "$gp", "$sp",
                        "$fp", "$ra", "r0", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8", "r9",
                        "r10", "r11", "r12", "r13", "r14", "r15", "r16", "r17", "r18", "r19",
                        "r20", "r21", "r22", "r23", "r24", "r25", "r26", "r27", "r28", "r29",
                        "r30", "r31",
                    ];

                    let given_string =
                        &instruction.operands[error.operand_number.unwrap() as usize].token_name;
                    let mut closest: (usize, String) = (usize::MAX, "".to_string());

                    for register in gp_registers {
                        if levenshtein(given_string, register) < closest.0 {
                            closest.0 = levenshtein(given_string, register);
                            closest.1 = register.to_string();
                        }
                    }

                    let mut suggestion = "A valid, similar register is: ".to_string();
                    suggestion.push_str(&closest.1);
                    suggestion.push('.');
                    error.message = suggestion;
                }
                UnrecognizedFPRegister => {
                    let fp_registers = [
                        "$f0", "$f1", "$f2", "$f3", "$f4", "$f5", "$f6", "$f7", "$f8", "$f9",
                        "$f10", "$f11", "$f12", "$f13", "$f14", "$f15", "$f16", "$f17", "$f18",
                        "$f19", "$f20", "$f21", "$f22", "$f23", "$f24", "$f25", "$f26", "$f27",
                        "$f28", "$f29", "$f30", "$f31",
                    ];

                    let given_string =
                        &instruction.operands[error.operand_number.unwrap() as usize].token_name;
                    let mut closest: (usize, String) = (usize::MAX, "".to_string());

                    for register in fp_registers {
                        if levenshtein(given_string, register) < closest.0 {
                            closest.0 = levenshtein(given_string, register);
                            closest.1 = register.to_string();
                        }
                    }

                    let mut suggestion = "A valid, similar register is: ".to_string();
                    suggestion.push_str(&closest.1);
                    suggestion.push('.');
                    error.message = suggestion;
                }
                UnrecognizedInstruction => {
                    let recognized_instructions = [
                        "add", "sub", "mul", "div", "lw", "sw", "lui", "aui", "andi", "ori",
                        "addi", "dadd", "dsub", "dmul", "ddiv", "or", "and", "add.s", "add.d",
                        "sub.s", "sub.d", "mul.s", "mul.d", "div.s", "div.d", "dahi", "dati",
                        "daddiu", "slt", "sltu", "swc1", "lwc1", "mtc1", "dmtc1", "mfc1", "dmfc1",
                        "j", "beq", "bne", "c.eq.s", "c.eq.d", "c.lt.s", "c.le.s", "c.le.d",
                        "c.ngt.s", "c.ngt.d", "c.nge.s", "c.nge.d", "bc1t", "bc1f",
                    ];

                    let given_string = &instruction.operator.token_name;
                    let mut closest: (usize, String) = (usize::MAX, "".to_string());

                    for instruction in recognized_instructions {
                        if levenshtein(given_string, instruction) < closest.0 {
                            closest.0 = levenshtein(given_string, instruction);
                            closest.1 = instruction.to_string();
                        }
                    }

                    let mut suggestion = "A valid, similar instruction is: ".to_string();
                    suggestion.push_str(&closest.1);
                    suggestion.push('.');
                    error.message = suggestion;
                }
                IncorrectRegisterTypeGP => {
                    error.message = "Expected FP register but received GP register.".to_string();
                }
                IncorrectRegisterTypeFP => {
                    error.message = "Expected GP register but received FP register.".to_string();
                }
                MissingComma => {
                    error.message =
                        "Operand expected to end with a comma but it does not.".to_string()
                }
                ImmediateOutOfBounds => {
                    error.message = "Immediate value given cannot be expressed in the available number of bits.".to_string();
                }
                NonIntImmediate => {
                    error.message =
                        "The given string cannot be recognized as an integer.".to_string();
                }
                NonFloatImmediate => {
                    error.message = "The given string cannot be recognized as a float.".to_string();
                }
                InvalidMemorySyntax => {
                    error.message = "The given string for memory does not match syntax of \"offset(base)\" or \"label\"".to_string();
                }
                IncorrectNumberOfOperands => {
                    error.message = "The given number of operands does not match the number expected for the given instruction.".to_string();
                }
                LabelMultipleDefinition => {
                    error.message =
                        "The given label name is already used elsewhere in the project."
                            .to_string();
                }
                LabelNotFound => {
                    if labels.is_empty() {
                        error.message = "There is no recognized labelled memory.".to_string();
                        continue;
                    }

                    let given_string =
                        &instruction.operands[error.operand_number.unwrap() as usize].token_name;
                    let mut closest: (usize, String) = (usize::MAX, "".to_string());

                    for label in labels {
                        if levenshtein(given_string, label.0) < closest.0 {
                            closest.0 = levenshtein(given_string, label.0);
                            closest.1 = label.0.to_string();
                        }
                    }

                    let mut suggestion = "A valid, similar label is: ".to_string();
                    suggestion.push_str(&closest.1);
                    suggestion.push('.');
                    error.message = suggestion;
                }
                ImproperlyFormattedASCII => {
                    error.message =
                        "Token recognized as ASCII does not start and or end with \".".to_string();
                }
                ImproperlyFormattedChar => {
                    error.message = "Token recognized as a char does not end with ' or is larger than a single char.".to_string();
                }
                _ => {
                    error.message = "PARSER/ASSEMBLER ERROR. THIS ERROR TYPE SHOULD NOT BE ABLE TO BE ASSOCIATED WITH AN INSTRUCTION.".to_string();
                }
            }
        }
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
                    suggestion.push_str(&closest.1);
                    suggestion.push('.');
                    error.message = suggestion;
                }
                LabelAssignmentError => {
                    error.message = "A label is specified but it is not followed by data or an instruction committed to memory.".to_string();
                }
                LabelMultipleDefinition => {
                    error.message =
                        "The given label name is already used elsewhere in the project."
                            .to_string();
                }
                _ => {
                    error.message = "PARSER/ASSEMBLER ERROR. THIS ERROR TYPE SHOULD NOT BE ABLE TO BE ASSOCIATED WITH DATA.".to_string();
                }
            }
        }
    }
}
