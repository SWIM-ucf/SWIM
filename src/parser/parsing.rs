use crate::parser::parser_structs_and_enums::instruction_tokenization::ErrorType::{
    ImproperlyFormattedData, ImproperlyFormattedLabel, LabelAssignmentError,
    LabelMultipleDefinition, MissingComma,
};
use crate::parser::parser_structs_and_enums::instruction_tokenization::TokenType::{
    Label, Operator, Unknown,
};
use crate::parser::parser_structs_and_enums::instruction_tokenization::{
    Data, Error, Instruction, Line, Token,
};
use std::collections::HashMap;

///Takes the initial string of the program given by the editor and turns it into a vector of Line,
/// a struct that holds tokens and the original line number, and finds the starting point for all comments.
pub fn tokenize_program(program: String) -> (Vec<Line>, Vec<[u32; 2]>) {
    let mut line_vec: Vec<Line> = Vec::new();
    let mut token: Token = Token {
        token_name: "".to_string(),
        starting_column: 0,
        token_type: Unknown,
    };
    let mut comments: Vec<[u32; 2]> = Vec::new();

    for (i, line_of_program) in program.lines().enumerate() {
        let mut line_of_tokens = Line {
            line_number: i as u32,

            tokens: vec![],
        };

        for (j, char) in line_of_program.chars().enumerate() {
            if char == '#' {
                comments.push([i as u32, j as u32]);
                break;
            };
            if char != ' ' {
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

    (line_vec, comments)
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

            let first_operand_index = operand_iterator;

            //push all operands to the instruction operand vec that will have commas
            while operand_iterator < (lines[i].tokens.len() - 1) {
                if lines[i].tokens[operand_iterator].token_name.ends_with(',') {
                    lines[i].tokens[operand_iterator].token_name.pop();
                } else {
                    instruction.errors.push(Error {
                        error_name: MissingComma,
                        operand_number: Some((operand_iterator - first_operand_index) as u8),
                    })
                }
                instruction
                    .operands
                    .push(lines[i].tokens[operand_iterator].clone());
                operand_iterator += 1;
            }

            //simple statement to handle cases where the user doesn't finish instructions
            if operand_iterator >= lines[i].tokens.len() {
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
                });
                lines[i].tokens[0].token_type = Label;
                data.label = lines[i].tokens[0].clone();
            }

            //just a simple check in case the user didn't complete a line
            if lines[i].tokens.len() < 2 {
                data.errors.push(Error {
                    error_name: ImproperlyFormattedData,
                    operand_number: None,
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
/// have been translated.
pub fn expand_pseudo_instructions_and_assign_instruction_numbers(
    instructions: &mut Vec<Instruction>,
    data: &Vec<Data>,
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
            "seq" => {}
            "sne" => {}
            "sle" => {}
            "sgt" => {
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
            "sge" => {}
            "sleu" => {}
            "sgtu" => {}
            "sgeu" => {}
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
            "dsubiu" => {}
            "dmuliu" => {}
            "ddiviu" => {}

            _ => {}
        }
    }

    //insert all new new instructions
    for instruction in vec_of_added_instructions {
        instructions.insert(instruction.instruction_number as usize, instruction);
    }
}

///This function assigns the instruction number to each instruction
pub fn assign_instruction_numbers(instruction_list: &mut [Instruction]) {
    for (i, instruction) in instruction_list.iter_mut().enumerate() {
        instruction.instruction_number = i as u32;
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
) {
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
