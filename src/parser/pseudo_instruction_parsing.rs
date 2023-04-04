use crate::parser::parser_structs_and_enums::ErrorType::IncorrectNumberOfOperands;
use crate::parser::parser_structs_and_enums::TokenType::Operator;
use crate::parser::parser_structs_and_enums::{
    Data, Error, Instruction, MonacoLineInfo, PseudoDescription, Token,
};
use std::collections::HashMap;

///Iterates through the instruction list and translates pseudo-instructions into real instructions.
/// LW and SW with labelled memory are not completely translated in this step because they require
/// the address of the labelled memory to be known which is not found until after all other pseudo-instructions
/// have been translated. Updated pseudo-instructions are added to updated_monaco_string to appear in the editor after assembly.
/// Also ensures a syscall is at the end of the program
pub fn expand_pseudo_instructions_and_assign_instruction_numbers(
    instructions: &mut Vec<Instruction>,
    data: &Vec<Data>,
    monaco_line_info: &mut [MonacoLineInfo],
) {
    //figure out list of labels to be used for lw and sw labels
    let mut list_of_labels: Vec<String> = Vec::new();
    for instruction in instructions.clone() {
        for label in instruction.labels {
            list_of_labels.push(label.token.token_name);
        }
    }
    for data in data {
        list_of_labels.push(data.label.token_name.clone());
    }

    //vec_of_added_instructions is needed because of rust ownership rules. It will not let us
    //insert into instruction_list while instruction_list is being iterated over.
    let mut vec_of_added_instructions: Vec<Instruction> = Vec::new();

    //iterate through every instruction and check if the operator is a pseudo-instruction
    for (i, mut instruction) in &mut instructions.iter_mut().enumerate() {
        instruction.instruction_number = i + vec_of_added_instructions.len();
        match &*instruction.operator.token_name.to_lowercase() {
            "li" => {
                let info = PseudoDescription {
                    name: "li".to_string(),
                    syntax: "li rt, immediate".to_string(),
                    translation_lines: vec!["ori rt, $zero, immediate".to_string()],
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();

                if instruction.operands.len() != 2 {
                    instruction.errors.push(Error {
                        error_name: IncorrectNumberOfOperands,
                        token_causing_error: "".to_string(),
                        start_end_columns: instruction.operator.start_end_columns,
                        message: "".to_string(),
                    });
                    continue;
                }

                instruction.operator.token_name = "ori".to_string();

                instruction.operands.insert(
                    1,
                    Token {
                        token_name: "$zero".to_string(),
                        start_end_columns: (0, 0),
                        token_type: Default::default(),
                    },
                );
                instruction.operands[2].start_end_columns = (0, 0);

                monaco_line_info[instruction.line_number].update_pseudo_string(vec![instruction]);
            }
            "move" => {
                let info = PseudoDescription {
                    name: "move".to_string(),
                    syntax: "move rt, rs".to_string(),
                    translation_lines: vec!["or rt, $zero, rs".to_string()],
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();

                if instruction.operands.len() != 2 {
                    instruction.errors.push(Error {
                        error_name: IncorrectNumberOfOperands,
                        token_causing_error: "".to_string(),
                        start_end_columns: instruction.operator.start_end_columns,
                        message: "".to_string(),
                    });
                    continue;
                }

                instruction.operator.token_name = "or".to_string();

                instruction.operands.insert(
                    1,
                    Token {
                        token_name: "$zero".to_string(),
                        start_end_columns: (0, 0),
                        token_type: Default::default(),
                    },
                );
                //instruction.operands[2].start_end_columns = (0, 0);

                monaco_line_info[instruction.line_number].update_pseudo_string(vec![instruction]);
            }
            "seq" => {
                let info = PseudoDescription {
                    name: "seq".to_string(),
                    syntax: "seq rd, rs, rt".to_string(),
                    translation_lines: vec![
                        "sub rd, rs, rt".to_string(),
                        "ori $at, $zero, 1".to_string(),
                        "sltu rd, rd, $at".to_string(),
                    ],
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();

                //make sure there are the correct number operands
                if instruction.operands.len() != 3 {
                    instruction.errors.push(Error {
                        error_name: IncorrectNumberOfOperands,
                        token_causing_error: "".to_string(),
                        start_end_columns: instruction.operator.start_end_columns,
                        message: "".to_string(),
                    });
                    continue;
                }
                //sub the two registers to find the difference
                let mut extra_instruction = instruction.clone();
                extra_instruction.operator.token_name = "sub".to_string();
                extra_instruction.operator.start_end_columns = (0, 0);
                vec_of_added_instructions.push(extra_instruction.clone());

                instruction.labels = Vec::new(); //if the pseudo-instruction had a label, remove it so it's only on the first expanded instruction

                //put a 1 in $at
                let mut extra_instruction_2 = Instruction {
                    operator: Token {
                        token_name: "ori".to_string(),
                        start_end_columns: (0, 0),
                        token_type: Operator,
                    },
                    operands: vec![
                        Token {
                            token_name: "$at".to_string(),
                            start_end_columns: (0, 0),
                            token_type: Default::default(),
                        },
                        Token {
                            token_name: "$zero".to_string(),
                            start_end_columns: (0, 0),
                            token_type: Default::default(),
                        },
                        Token {
                            token_name: "1".to_string(),
                            start_end_columns: (0, 0),
                            token_type: Default::default(),
                        },
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number + 1,
                    line_number: instruction.line_number,
                    errors: vec![],
                    labels: Vec::new(),
                };
                vec_of_added_instructions.push(extra_instruction_2.clone());

                //set r0 to 1 if r1 - r2 == 0
                instruction.operator.token_name = "sltu".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[1] = instruction.operands[0].clone();
                instruction.operands[2].token_name = "$at".to_string();
                instruction.operands[2].start_end_columns = (0, 0);
                instruction.instruction_number += 2;

                monaco_line_info[instruction.line_number].update_pseudo_string(vec![
                    &mut extra_instruction,
                    &mut extra_instruction_2,
                    instruction,
                ]);
            }
            "sne" => {
                let info = PseudoDescription {
                    name: "sne".to_string(),
                    syntax: "sne rd, rs, rt".to_string(),
                    translation_lines: vec![
                        "sub rd, rs, rt".to_string(),
                        "sltu rd, $zero, rd".to_string(),
                    ],
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();

                //make sure there are enough operands
                if instruction.operands.len() != 3 {
                    instruction.errors.push(Error {
                        error_name: IncorrectNumberOfOperands,
                        token_causing_error: "".to_string(),
                        start_end_columns: instruction.operator.start_end_columns,
                        message: "".to_string(),
                    });
                    continue;
                }
                //sub the two registers to find the difference
                let mut extra_instruction = instruction.clone();
                extra_instruction.operator.token_name = "sub".to_string();
                extra_instruction.operator.start_end_columns = (0, 0);
                vec_of_added_instructions.push(extra_instruction.clone());

                instruction.labels = Vec::new(); //if the pseudo-instruction had a label, remove it so it's only on the first expanded instruction

                //set r0 to 1 if r1 - r2 != 0
                instruction.operator.token_name = "sltu".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[1].token_name = "$zero".to_string();
                instruction.operands[1].start_end_columns = (0, 0);
                instruction.operands[2] = instruction.operands[0].clone();
                instruction.instruction_number += 1;

                monaco_line_info[instruction.line_number]
                    .update_pseudo_string(vec![&mut extra_instruction, instruction]);
            }
            "sle" => {
                let info = PseudoDescription {
                    name: "sle".to_string(),
                    syntax: "sle rd, rs, rt".to_string(),
                    translation_lines: vec![
                        "slt rd, rt, rs".to_string(),
                        "addi rd, rd, 1".to_string(),
                        "andi rd, rd, 1".to_string(),
                    ],
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();

                //make sure there are enough operands
                if instruction.operands.len() != 3 {
                    instruction.errors.push(Error {
                        error_name: IncorrectNumberOfOperands,
                        token_causing_error: "".to_string(),
                        start_end_columns: instruction.operator.start_end_columns,
                        message: "".to_string(),
                    });
                    continue;
                }

                //slt
                let mut extra_instruction = instruction.clone();
                let temp = extra_instruction.operands[1].clone();
                extra_instruction.operands[1] = extra_instruction.operands[2].clone();
                extra_instruction.operands[2] = temp.clone();
                extra_instruction.operator.token_name = "slt".to_string();
                extra_instruction.operator.start_end_columns = (0, 0);
                vec_of_added_instructions.push(extra_instruction.clone());

                instruction.labels = Vec::new(); //if the pseudo-instruction had a label, remove it so it's only on the first expanded instruction

                //addi
                let mut extra_instruction_2 = Instruction {
                    operator: Token {
                        token_name: "addi".to_string(),
                        start_end_columns: (0, 0),
                        token_type: Operator,
                    },
                    operands: vec![
                        instruction.operands[0].clone(),
                        instruction.operands[0].clone(),
                        Token {
                            token_name: "1".to_string(),
                            start_end_columns: (0, 0),
                            token_type: Default::default(),
                        },
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number + 1,
                    line_number: instruction.line_number,
                    errors: vec![],
                    labels: Vec::new(),
                };
                vec_of_added_instructions.push(extra_instruction_2.clone());

                //andi
                instruction.operator.token_name = "andi".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[1] = instruction.operands[0].clone();
                instruction.operands[2].token_name = "1".to_string();
                instruction.operands[2].start_end_columns = (0, 0);
                instruction.instruction_number += 2;

                monaco_line_info[instruction.line_number].update_pseudo_string(vec![
                    &mut extra_instruction,
                    &mut extra_instruction_2,
                    instruction,
                ]);
            }
            "sleu" => {
                let info = PseudoDescription {
                    name: "sle".to_string(),
                    syntax: "sle rd, rs, rt".to_string(),
                    translation_lines: vec![
                        "sltu rd, rt, rs".to_string(),
                        "addi rd, rd, 1".to_string(),
                        "andi rd, rd, 1".to_string(),
                    ],
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();

                //make sure there are enough operands
                if instruction.operands.len() != 3 {
                    instruction.errors.push(Error {
                        error_name: IncorrectNumberOfOperands,
                        token_causing_error: "".to_string(),
                        start_end_columns: instruction.operator.start_end_columns,
                        message: "".to_string(),
                    });
                    continue;
                }

                //sltu
                let mut extra_instruction = instruction.clone();
                let temp = extra_instruction.operands[1].clone();
                extra_instruction.operands[1] = extra_instruction.operands[2].clone();
                extra_instruction.operands[2] = temp.clone();
                extra_instruction.operator.token_name = "sltu".to_string();
                extra_instruction.operator.start_end_columns = (0, 0);
                vec_of_added_instructions.push(extra_instruction.clone());

                instruction.labels = Vec::new(); //if the pseudo-instruction had a label, remove it so it's only on the first expanded instruction

                //addi
                let mut extra_instruction_2 = Instruction {
                    operator: Token {
                        token_name: "addi".to_string(),
                        start_end_columns: (0, 0),
                        token_type: Operator,
                    },
                    operands: vec![
                        instruction.operands[0].clone(),
                        instruction.operands[0].clone(),
                        Token {
                            token_name: "1".to_string(),
                            start_end_columns: (0, 0),
                            token_type: Default::default(),
                        },
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number + 1,
                    line_number: instruction.line_number,
                    errors: vec![],
                    labels: Vec::new(),
                };
                vec_of_added_instructions.push(extra_instruction_2.clone());

                //andi
                instruction.operator.token_name = "andi".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[1] = instruction.operands[0].clone();
                instruction.operands[2].token_name = "1".to_string();
                instruction.operands[2].start_end_columns = (0, 0);
                instruction.instruction_number += 2;

                monaco_line_info[instruction.line_number].update_pseudo_string(vec![
                    &mut extra_instruction,
                    &mut extra_instruction_2,
                    instruction,
                ]);
            }
            "sgt" => {
                let info = PseudoDescription {
                    name: "sgt".to_string(),
                    syntax: "sgt rd, rs, rt".to_string(),
                    translation_lines: vec!["slt rd, rt, rs".to_string()],
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();

                //make sure that there actually is a third operand
                if instruction.operands.len() != 3 {
                    instruction.errors.push(Error {
                        error_name: IncorrectNumberOfOperands,
                        token_causing_error: "".to_string(),
                        start_end_columns: instruction.operator.start_end_columns,
                        message: "".to_string(),
                    });
                    continue;
                }
                let temp = instruction.operands[1].clone();
                instruction.operands[1] = instruction.operands[2].clone();
                instruction.operands[2] = temp.clone();
                instruction.operator.token_name = "slt".to_string();
                instruction.operator.start_end_columns = (0, 0);

                monaco_line_info[instruction.line_number].update_pseudo_string(vec![instruction]);
            }
            "sgtu" => {
                let info = PseudoDescription {
                    name: "sgtu".to_string(),
                    syntax: "sgtu rd, rs, rt".to_string(),
                    translation_lines: vec!["sltu rd, rt, rs".to_string()],
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();

                //make sure that there actually is a third operand
                if instruction.operands.len() != 3 {
                    instruction.errors.push(Error {
                        error_name: IncorrectNumberOfOperands,
                        token_causing_error: "".to_string(),
                        start_end_columns: instruction.operator.start_end_columns,
                        message: "".to_string(),
                    });
                    continue;
                }
                let temp = instruction.operands[1].clone();
                instruction.operands[1] = instruction.operands[2].clone();
                instruction.operands[2] = temp.clone();
                instruction.operator.token_name = "sltu".to_string();
                instruction.operator.start_end_columns = (0, 0);

                monaco_line_info[instruction.line_number].update_pseudo_string(vec![instruction]);
            }
            "sge" => {
                let info = PseudoDescription {
                    name: "sge".to_string(),
                    syntax: "sge rd, rs, rt".to_string(),
                    translation_lines: vec![
                        "slt rd, rs, rt".to_string(),
                        "addi rd, rd, 1".to_string(),
                        "andi rd, rd, 1".to_string(),
                    ],
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();

                //make sure there are enough operands
                if instruction.operands.len() != 3 {
                    instruction.errors.push(Error {
                        error_name: IncorrectNumberOfOperands,
                        token_causing_error: "".to_string(),
                        start_end_columns: instruction.operator.start_end_columns,
                        message: "".to_string(),
                    });
                    continue;
                }

                //slt
                let mut extra_instruction = instruction.clone();
                extra_instruction.operator.token_name = "slt".to_string();
                extra_instruction.operator.start_end_columns = (0, 0);
                vec_of_added_instructions.push(extra_instruction.clone());

                instruction.labels = Vec::new(); //if the pseudo-instruction had a label, remove it so it's only on the first expanded instruction

                //addi
                let mut extra_instruction_2 = Instruction {
                    operator: Token {
                        token_name: "addi".to_string(),
                        start_end_columns: (0, 0),
                        token_type: Operator,
                    },
                    operands: vec![
                        instruction.operands[0].clone(),
                        instruction.operands[0].clone(),
                        Token {
                            token_name: "1".to_string(),
                            start_end_columns: (0, 0),
                            token_type: Default::default(),
                        },
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number + 1,
                    line_number: instruction.line_number,
                    errors: vec![],
                    labels: Vec::new(),
                };
                vec_of_added_instructions.push(extra_instruction_2.clone());

                //andi
                instruction.operator.token_name = "andi".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[1] = instruction.operands[0].clone();
                instruction.operands[2].token_name = "1".to_string();
                instruction.operands[2].start_end_columns = (0, 0);
                instruction.instruction_number += 2;

                monaco_line_info[instruction.line_number].update_pseudo_string(vec![
                    &mut extra_instruction,
                    &mut extra_instruction_2,
                    instruction,
                ]);
            }
            "sgeu" => {
                let info = PseudoDescription {
                    name: "sgeu".to_string(),
                    syntax: "sgeu rd, rs, rt".to_string(),
                    translation_lines: vec![
                        "sltu rd, rs, rt".to_string(),
                        "addi rd, rd, 1".to_string(),
                        "andi rd, rd, 1".to_string(),
                    ],
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();

                //make sure there are enough operands
                if instruction.operands.len() != 3 {
                    instruction.errors.push(Error {
                        error_name: IncorrectNumberOfOperands,
                        token_causing_error: "".to_string(),
                        start_end_columns: instruction.operator.start_end_columns,
                        message: "".to_string(),
                    });
                    continue;
                }

                //sltu
                let mut extra_instruction = instruction.clone();
                extra_instruction.operator.token_name = "sltu".to_string();
                extra_instruction.operator.start_end_columns = (0, 0);
                vec_of_added_instructions.push(extra_instruction.clone());

                instruction.labels = Vec::new(); //if the pseudo-instruction had a label, remove it so it's only on the first expanded instruction

                //addi
                let mut extra_instruction_2 = Instruction {
                    operator: Token {
                        token_name: "addi".to_string(),
                        start_end_columns: (0, 0),
                        token_type: Operator,
                    },
                    operands: vec![
                        instruction.operands[0].clone(),
                        instruction.operands[0].clone(),
                        Token {
                            token_name: "1".to_string(),
                            start_end_columns: (0, 0),
                            token_type: Default::default(),
                        },
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number + 1,
                    line_number: instruction.line_number,
                    errors: vec![],
                    labels: Vec::new(),
                };
                vec_of_added_instructions.push(extra_instruction_2.clone());

                //andi
                instruction.operator.token_name = "andi".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[1] = instruction.operands[0].clone();
                instruction.operands[2].token_name = "1".to_string();
                instruction.operands[2].start_end_columns = (0, 0);
                instruction.instruction_number += 2;

                monaco_line_info[instruction.line_number].update_pseudo_string(vec![
                    &mut extra_instruction,
                    &mut extra_instruction_2,
                    instruction,
                ]);
            }
            "lw" | "sw" => {
                //lw $regA, label is translated to:
                //lui $at, label
                //lw $regA, lower16($at)

                if instruction.operands.len() > 1
                    && list_of_labels.contains(&instruction.operands[1].token_name)
                {
                    //make sure there are enough operands
                    if instruction.operands.len() != 2 {
                        instruction.errors.push(Error {
                            error_name: IncorrectNumberOfOperands,
                            token_causing_error: "".to_string(),
                            start_end_columns: instruction.operator.start_end_columns,
                            message: "".to_string(),
                        });
                        continue;
                    }

                    //create mouse hover message dependent on lw / sw
                    if instruction.operator.token_name == "lw" {
                        let info = PseudoDescription {
                            name: "lw rt target".to_string(),
                            syntax: "lw rt target".to_string(),
                            translation_lines: vec![
                                "lui $at, upper48".to_string(),
                                "lw rt, lower16($at)".to_string(),
                            ],
                        };
                        monaco_line_info[instruction.line_number].mouse_hover_string =
                            info.to_string();
                    } else {
                        let info = PseudoDescription {
                            name: "sw rt target".to_string(),
                            syntax: "sw rt target".to_string(),
                            translation_lines: vec![
                                "lui $at, upper48".to_string(),
                                "sw rt, lower16($at)".to_string(),
                            ],
                        };
                        monaco_line_info[instruction.line_number].mouse_hover_string =
                            info.to_string();
                    }
                    monaco_line_info[instruction.line_number]
                        .mouse_hover_string
                        .push_str(
                            "where lower16 and upper48 refer to bits of the labelled address.\n",
                        );

                    let extra_instruction = Instruction {
                        operator: Token {
                            token_name: "lui".to_string(),
                            start_end_columns: (0, 0),
                            token_type: Operator,
                        },
                        operands: vec![
                            Token {
                                token_name: "$at".to_string(),
                                start_end_columns: (0, 0),
                                token_type: Default::default(),
                            },
                            instruction.operands[1].clone(),
                        ],
                        binary: 0,
                        instruction_number: instruction.instruction_number,
                        line_number: instruction.line_number,
                        errors: vec![],
                        labels: instruction.labels.clone(),
                    };

                    instruction.labels = Vec::new(); //if the pseudo-instruction had a label, remove it so it's only on the first expanded instruction

                    vec_of_added_instructions.push(extra_instruction);
                    instruction.operands[1].token_name = "$at".to_string();
                    instruction.operands[1].start_end_columns = (0, 0);
                    instruction.instruction_number += 1;
                }
            }
            "subi" => {
                let info = PseudoDescription {
                    name: "subi".to_string(),
                    syntax: "subi rt, rs, immediate".to_string(),
                    translation_lines: vec![
                        "ori $at, $zero, immediate".to_string(),
                        "sub rt, rs, $at".to_string(),
                    ],
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();

                //make sure there are enough operands
                if instruction.operands.len() != 3 {
                    instruction.errors.push(Error {
                        error_name: IncorrectNumberOfOperands,
                        token_causing_error: "".to_string(),
                        start_end_columns: instruction.operator.start_end_columns,
                        message: "".to_string(),
                    });
                    continue;
                }
                let mut extra_instruction = Instruction {
                    operator: Token {
                        token_name: "ori".to_string(),
                        start_end_columns: (0, 0),
                        token_type: Operator,
                    },
                    operands: vec![
                        Token {
                            token_name: "$at".to_string(),
                            start_end_columns: (0, 0),
                            token_type: Default::default(),
                        },
                        Token {
                            token_name: "$zero".to_string(),
                            start_end_columns: (0, 0),
                            token_type: Default::default(),
                        },
                        instruction.operands[2].clone(),
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number,
                    line_number: instruction.line_number,
                    errors: vec![],
                    labels: instruction.labels.clone(),
                };

                vec_of_added_instructions.push(extra_instruction.clone());

                //adjust subi for the added instruction
                instruction.operator.token_name = "sub".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[2].token_name = "$at".to_string();
                instruction.operands[2].start_end_columns = (0, 0);
                instruction.instruction_number += 1;
                instruction.labels = Vec::new(); //if the pseudo-instruction had a label, remove it so it's only on the first expanded instruction

                monaco_line_info[instruction.line_number]
                    .update_pseudo_string(vec![&mut extra_instruction, instruction]);
            }
            "dsubi" => {
                let info = PseudoDescription {
                    name: "dsubi".to_string(),
                    syntax: "dsubi rt, rs, immediate".to_string(),
                    translation_lines: vec![
                        "ori $at, $zero, immediate".to_string(),
                        "dsub rt, rs, $at".to_string(),
                    ],
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();

                //make sure there are the right number of operands
                if instruction.operands.len() != 3 {
                    instruction.errors.push(Error {
                        error_name: IncorrectNumberOfOperands,
                        token_causing_error: "".to_string(),
                        start_end_columns: instruction.operator.start_end_columns,
                        message: "".to_string(),
                    });
                    continue;
                }
                let mut extra_instruction = Instruction {
                    operator: Token {
                        token_name: "ori".to_string(),
                        start_end_columns: (0, 0),
                        token_type: Operator,
                    },
                    operands: vec![
                        Token {
                            token_name: "$at".to_string(),
                            start_end_columns: (0, 0),
                            token_type: Default::default(),
                        },
                        Token {
                            token_name: "$zero".to_string(),
                            start_end_columns: (0, 0),
                            token_type: Default::default(),
                        },
                        instruction.operands[2].clone(),
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number,
                    line_number: instruction.line_number,
                    errors: vec![],
                    labels: instruction.labels.clone(),
                };

                vec_of_added_instructions.push(extra_instruction.clone());
                //adjust dsubi for the added instruction
                instruction.operator.token_name = "dsub".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[2].token_name = "$at".to_string();
                instruction.operands[2].start_end_columns = (0, 0);
                instruction.instruction_number += 1;
                instruction.labels = Vec::new(); //if the pseudo-instruction had a label, remove it so it's only on the first expanded instruction

                monaco_line_info[instruction.line_number]
                    .update_pseudo_string(vec![&mut extra_instruction, instruction]);
            }
            "dsubiu" => {
                let info = PseudoDescription {
                    name: "dsubiu".to_string(),
                    syntax: "dsubiu rt, rs, immediate".to_string(),
                    translation_lines: vec![
                        "ori $at, $zero, immediate".to_string(),
                        "dsubu rt, rs, $at".to_string(),
                    ],
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();

                //make sure there are the right number of operands
                if instruction.operands.len() != 3 {
                    instruction.errors.push(Error {
                        error_name: IncorrectNumberOfOperands,
                        token_causing_error: "".to_string(),
                        start_end_columns: instruction.operator.start_end_columns,
                        message: "".to_string(),
                    });
                    continue;
                }

                let mut extra_instruction = Instruction {
                    operator: Token {
                        token_name: "ori".to_string(),
                        start_end_columns: (0, 0),
                        token_type: Operator,
                    },
                    operands: vec![
                        Token {
                            token_name: "$at".to_string(),
                            start_end_columns: (0, 0),
                            token_type: Default::default(),
                        },
                        Token {
                            token_name: "$zero".to_string(),
                            start_end_columns: (0, 0),
                            token_type: Default::default(),
                        },
                        instruction.operands[2].clone(),
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number,
                    line_number: instruction.line_number,
                    errors: vec![],
                    labels: instruction.labels.clone(),
                };
                vec_of_added_instructions.push(extra_instruction.clone());

                //adjust subiu for the added instruction
                instruction.operator.token_name = "dsubu".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[2].token_name = "$at".to_string();
                instruction.operands[2].start_end_columns = (0, 0);
                instruction.instruction_number += 1;
                instruction.labels = Vec::new(); //if the pseudo-instruction had a label, remove it so it's only on the first expanded instruction

                monaco_line_info[instruction.line_number]
                    .update_pseudo_string(vec![&mut extra_instruction, instruction]);
            }
            "muli" => {
                let info = PseudoDescription {
                    name: "muli".to_string(),
                    syntax: "muli rt, rs, immediate".to_string(),
                    translation_lines: vec![
                        "ori $at, $zero, immediate".to_string(),
                        "mul rt, rs, $at".to_string(),
                    ],
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();

                //make sure the are the right number of operands
                if instruction.operands.len() != 3 {
                    instruction.errors.push(Error {
                        error_name: IncorrectNumberOfOperands,
                        token_causing_error: "".to_string(),
                        start_end_columns: instruction.operator.start_end_columns,
                        message: "".to_string(),
                    });
                    continue;
                }
                let mut extra_instruction = Instruction {
                    operator: Token {
                        token_name: "ori".to_string(),
                        start_end_columns: (0, 0),
                        token_type: Operator,
                    },
                    operands: vec![
                        Token {
                            token_name: "$at".to_string(),
                            start_end_columns: (0, 0),
                            token_type: Default::default(),
                        },
                        Token {
                            token_name: "$zero".to_string(),
                            start_end_columns: (0, 0),
                            token_type: Default::default(),
                        },
                        instruction.operands[2].clone(),
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number,
                    line_number: instruction.line_number,
                    errors: vec![],
                    labels: instruction.labels.clone(),
                };
                vec_of_added_instructions.push(extra_instruction.clone());

                //adjust muli for the added instruction
                instruction.operator.token_name = "mul".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[2].token_name = "$at".to_string();
                instruction.operands[2].start_end_columns = (0, 0);
                instruction.instruction_number += 1;
                instruction.labels = Vec::new(); //if the pseudo-instruction had a label, remove it so it's only on the first expanded instruction

                monaco_line_info[instruction.line_number]
                    .update_pseudo_string(vec![&mut extra_instruction, instruction]);
            }
            "dmuli" => {
                let info = PseudoDescription {
                    name: "dmuli".to_string(),
                    syntax: "dmuli rt, rs, immediate".to_string(),
                    translation_lines: vec![
                        "ori $at, $zero, immediate".to_string(),
                        "dmul rt, rs, $at".to_string(),
                    ],
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();

                //make sure the are the right number of operands
                if instruction.operands.len() != 3 {
                    instruction.errors.push(Error {
                        error_name: IncorrectNumberOfOperands,
                        token_causing_error: "".to_string(),
                        start_end_columns: instruction.operator.start_end_columns,
                        message: "".to_string(),
                    });
                    continue;
                }
                let mut extra_instruction = Instruction {
                    operator: Token {
                        token_name: "ori".to_string(),
                        start_end_columns: (0, 0),
                        token_type: Operator,
                    },
                    operands: vec![
                        Token {
                            token_name: "$at".to_string(),
                            start_end_columns: (0, 0),
                            token_type: Default::default(),
                        },
                        Token {
                            token_name: "$zero".to_string(),
                            start_end_columns: (0, 0),
                            token_type: Default::default(),
                        },
                        instruction.operands[2].clone(),
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number,
                    line_number: instruction.line_number,
                    errors: vec![],
                    labels: instruction.labels.clone(),
                };
                vec_of_added_instructions.push(extra_instruction.clone());
                //adjust dmuli for the added instruction
                instruction.operator.token_name = "dmul".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[2].token_name = "$at".to_string();
                instruction.operands[2].start_end_columns = (0, 0);
                instruction.instruction_number += 1;
                instruction.labels = Vec::new(); //if the pseudo-instruction had a label, remove it so it's only on the first expanded instruction

                monaco_line_info[instruction.line_number]
                    .update_pseudo_string(vec![&mut extra_instruction, instruction]);
            }
            "dmuliu" => {
                let info = PseudoDescription {
                    name: "dmuliu".to_string(),
                    syntax: "dmuliu rt, rs, immediate".to_string(),
                    translation_lines: vec![
                        "ori $at, $zero, immediate".to_string(),
                        "dmulu rt, rs, $at".to_string(),
                    ],
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();

                //make sure the are the right number of operands
                if instruction.operands.len() != 3 {
                    instruction.errors.push(Error {
                        error_name: IncorrectNumberOfOperands,
                        token_causing_error: "".to_string(),
                        start_end_columns: instruction.operator.start_end_columns,
                        message: "".to_string(),
                    });
                    continue;
                }
                let mut extra_instruction = Instruction {
                    operator: Token {
                        token_name: "ori".to_string(),
                        start_end_columns: (0, 0),
                        token_type: Operator,
                    },
                    operands: vec![
                        Token {
                            token_name: "$at".to_string(),
                            start_end_columns: (0, 0),
                            token_type: Default::default(),
                        },
                        Token {
                            token_name: "$zero".to_string(),
                            start_end_columns: (0, 0),
                            token_type: Default::default(),
                        },
                        instruction.operands[2].clone(),
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number,
                    line_number: instruction.line_number,
                    errors: vec![],
                    labels: instruction.labels.clone(),
                };
                vec_of_added_instructions.push(extra_instruction.clone());
                //adjust dmuliu for the added instruction
                instruction.operator.token_name = "dmulu".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[2].token_name = "$at".to_string();
                instruction.operands[2].start_end_columns = (0, 0);
                instruction.instruction_number += 1;
                instruction.labels = Vec::new(); //if the pseudo-instruction had a label, remove it so it's only on the first expanded instruction

                monaco_line_info[instruction.line_number]
                    .update_pseudo_string(vec![&mut extra_instruction, instruction]);
            }
            "divi" => {
                let info = PseudoDescription {
                    name: "divi".to_string(),
                    syntax: "divi rt, rs, immediate".to_string(),
                    translation_lines: vec![
                        "ori $at, $zero, immediate".to_string(),
                        "div rt, rs, $at".to_string(),
                    ],
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();

                //make sure the are the right number of operands
                if instruction.operands.len() != 3 {
                    instruction.errors.push(Error {
                        error_name: IncorrectNumberOfOperands,
                        token_causing_error: "".to_string(),
                        start_end_columns: instruction.operator.start_end_columns,
                        message: "".to_string(),
                    });
                    continue;
                }
                let mut extra_instruction = Instruction {
                    operator: Token {
                        token_name: "ori".to_string(),
                        start_end_columns: (0, 0),
                        token_type: Operator,
                    },
                    operands: vec![
                        Token {
                            token_name: "$at".to_string(),
                            start_end_columns: (0, 0),
                            token_type: Default::default(),
                        },
                        Token {
                            token_name: "$zero".to_string(),
                            start_end_columns: (0, 0),
                            token_type: Default::default(),
                        },
                        instruction.operands[2].clone(),
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number,
                    line_number: instruction.line_number,
                    errors: vec![],
                    labels: instruction.labels.clone(),
                };
                vec_of_added_instructions.push(extra_instruction.clone());

                //adjust divi for the added instruction
                instruction.operator.token_name = "div".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[2].token_name = "$at".to_string();
                instruction.operands[2].start_end_columns = (0, 0);
                instruction.instruction_number += 1;
                instruction.labels = Vec::new(); //if the pseudo-instruction had a label, remove it so it's only on the first expanded instruction

                monaco_line_info[instruction.line_number]
                    .update_pseudo_string(vec![&mut extra_instruction, instruction]);
            }
            "ddivi" => {
                let info = PseudoDescription {
                    name: "ddivi".to_string(),
                    syntax: "ddivi rt, rs, immediate".to_string(),
                    translation_lines: vec![
                        "ori $at, $zero, immediate".to_string(),
                        "ddiv rt, rs, $at".to_string(),
                    ],
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();

                //make sure the are the right number of operands
                if instruction.operands.len() != 3 {
                    continue;
                }
                let mut extra_instruction = Instruction {
                    operator: Token {
                        token_name: "ori".to_string(),
                        start_end_columns: (0, 0),
                        token_type: Operator,
                    },
                    operands: vec![
                        Token {
                            token_name: "$at".to_string(),
                            start_end_columns: (0, 0),
                            token_type: Default::default(),
                        },
                        Token {
                            token_name: "$zero".to_string(),
                            start_end_columns: (0, 0),
                            token_type: Default::default(),
                        },
                        instruction.operands[2].clone(),
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number,
                    line_number: instruction.line_number,
                    errors: vec![],
                    labels: instruction.labels.clone(),
                };
                vec_of_added_instructions.push(extra_instruction.clone());

                //adjust ddivi for the added instruction
                instruction.operator.token_name = "ddiv".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[2].token_name = "$at".to_string();
                instruction.operands[2].start_end_columns = (0, 0);
                instruction.instruction_number += 1;
                instruction.labels = Vec::new(); //if the pseudo-instruction had a label, remove it so it's only on the first expanded instruction

                monaco_line_info[instruction.line_number]
                    .update_pseudo_string(vec![&mut extra_instruction, instruction]);
            }
            "ddiviu" => {
                let info = PseudoDescription {
                    name: "ddiviu".to_string(),
                    syntax: "ddiviu rt, rs, immediate".to_string(),
                    translation_lines: vec![
                        "ori $at, $zero, immediate".to_string(),
                        "ddivu rt, rs, $at".to_string(),
                    ],
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();

                monaco_line_info[instruction.line_number].mouse_hover_string =
                    "ddiviu $regA, $regB, immediate is a pseudo-instruction.\nddiviu $regA, $regB, immediate =>\n\tori $at, $zero, immediate\n\tddivu $regA, $regB, $at\n"
                        .to_string();

                //make sure the are the right number of operands
                if instruction.operands.len() != 3 {
                    continue;
                }
                let mut extra_instruction = Instruction {
                    operator: Token {
                        token_name: "ori".to_string(),
                        start_end_columns: (0, 0),
                        token_type: Operator,
                    },
                    operands: vec![
                        Token {
                            token_name: "$at".to_string(),
                            start_end_columns: (0, 0),
                            token_type: Default::default(),
                        },
                        Token {
                            token_name: "$zero".to_string(),
                            start_end_columns: (0, 0),
                            token_type: Default::default(),
                        },
                        instruction.operands[2].clone(),
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number,
                    line_number: instruction.line_number,
                    errors: vec![],
                    labels: instruction.labels.clone(),
                };
                vec_of_added_instructions.push(extra_instruction.clone());

                //adjust ddiviu for the added instruction
                instruction.operator.token_name = "ddivu".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[2].token_name = "$at".to_string();
                instruction.operands[2].start_end_columns = (0, 0);
                instruction.instruction_number += 1;
                instruction.labels = Vec::new(); //if the pseudo-instruction had a label, remove it so it's only on the first expanded instruction

                monaco_line_info[instruction.line_number]
                    .update_pseudo_string(vec![&mut extra_instruction, instruction]);
            }
            _ => {}
        }
    }

    //insert all new new instructions
    for instruction in vec_of_added_instructions {
        instructions.insert(instruction.instruction_number, instruction);
    }

    //if there aren't any instructions, add a syscall to monaco's updated string so the emulation core does not try to run data as an instruction
    if instructions.is_empty() {
        //try to find an instance of .text
        let mut dot_text_index: Option<usize> = None;
        for (i, monaco_line) in monaco_line_info.iter_mut().enumerate() {
            if !monaco_line.tokens.is_empty() && monaco_line.tokens[0].token_name == ".text" {
                dot_text_index = Some(i);
                break;
            }
        }
        if let Some(dot_text_index) = dot_text_index {
            let offset = monaco_line_info[dot_text_index].get_tab_space_offset().0;
            //add syscall after first index of .text if it exists
            monaco_line_info[dot_text_index]
                .updated_monaco_string
                .push_str(format!("\n{offset}syscall").as_str());

            instructions.push(Instruction {
                operator: Token {
                    token_name: "syscall".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Operator,
                },
                operands: vec![],
                binary: 0,
                instruction_number: 0,
                line_number: dot_text_index,
                errors: vec![],
                labels: Vec::new(),
            });
        } else {
            let offset = monaco_line_info[0].get_tab_space_offset().0;
            //otherwise, add it at the beginning of monaco
            monaco_line_info[0]
                .updated_monaco_string
                .insert_str(0, format!("{offset}.text\n{offset}syscall\n").as_str());

            instructions.push(Instruction {
                operator: Token {
                    token_name: "syscall".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Operator,
                },
                operands: vec![],
                binary: 0,
                instruction_number: 0,
                line_number: 0,
                errors: vec![],
                labels: Vec::new(),
            });
        }
    } else {
        let last_instruction = instructions.last().unwrap();
        //if the last instruction in monaco is not a syscall, add it in to updated_monaco_strings and to instructions
        if last_instruction.operator.token_name != "syscall" {
            let offset = monaco_line_info[last_instruction.line_number]
                .get_tab_space_offset()
                .0;
            monaco_line_info[last_instruction.line_number]
                .updated_monaco_string
                .push_str(format!("\n{offset}syscall").as_str());

            instructions.push(Instruction {
                operator: Token {
                    token_name: "syscall".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Operator,
                },
                operands: vec![],
                binary: 0,
                instruction_number: last_instruction.instruction_number + 1,
                line_number: last_instruction.line_number,
                errors: vec![],
                labels: Vec::new(),
            })
        }
    }
}

///the second part of completing pseudo-instructions. LW and SW with labels requires the address of the label to be known,
/// the second part of this must occur after the label hashmap is completed.
pub fn complete_lw_sw_pseudo_instructions(
    instructions: &mut Vec<Instruction>,
    labels: &HashMap<String, usize>,
    monaco_line_info: &mut [MonacoLineInfo],
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
            instructions[index].operands[1].start_end_columns = (0, 0);

            index += 1;

            //lower 16 bits are stored as the offset for the load/store operation
            let lower_16_bits = address as u16;
            let mut memory_operand = lower_16_bits.to_string();
            memory_operand.push_str("($at)");
            instructions[index].operands[1].token_name = memory_operand;
            instructions[index].operands[1].start_end_columns = (0, 0);

            monaco_line_info[instructions[index].line_number].update_pseudo_string(vec![
                &mut instructions.clone()[index - 1],
                &mut instructions.clone()[index],
            ]);
        }
    }
}
