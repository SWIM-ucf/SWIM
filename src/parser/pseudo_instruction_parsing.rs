use crate::parser::parser_structs_and_enums::instruction_tokenization::ErrorType::IncorrectNumberOfOperands;
use crate::parser::parser_structs_and_enums::instruction_tokenization::TokenType::Operator;
use crate::parser::parser_structs_and_enums::instruction_tokenization::{
    Data, Error, Instruction, MonacoLineInfo, Token,
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

    //iterate through every instruction and check if the operator is a pseudo-instruction
    for (i, mut instruction) in &mut instructions.iter_mut().enumerate() {
        instruction.instruction_number = (i + vec_of_added_instructions.len()) as u32;
        match &*instruction.operator.token_name {
            "li" => {
                monaco_line_info[instruction.line_number as usize].mouse_hover_string =
                    "li is a pseudo-instruction.\nli regA, immediate =>\n\tori $regA, $zero, immediate\n"
                        .to_string();

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

                instruction.operands.push(Token {
                    token_name: "$zero".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Default::default(),
                });

                monaco_line_info[instruction.line_number as usize]
                    .update_pseudo_string(vec![instruction]);
            }
            "seq" => {
                //seq $regA, $regB, $regC turns into:
                //sub $regA, $regB, $regC
                //ori $at, $zero, 1
                //sltu $regA, $regA, $at
                monaco_line_info[instruction.line_number as usize].mouse_hover_string =
                    "seq is a pseudo-instruction.\nseq $regA, $regB, $regC =>\n\tsub $regA, $regB, $regC\n\tori $at, $zero, 1\n\tsltu $regA, $regA, $at\n"
                        .to_string();

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
                    label: None,
                };
                vec_of_added_instructions.push(extra_instruction_2.clone());

                //set r0 to 1 if r1 - r2 == 0
                instruction.operator.token_name = "sltu".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[1] = instruction.operands[0].clone();
                instruction.operands[2].token_name = "$at".to_string();
                instruction.operands[2].start_end_columns = (0, 0);
                instruction.instruction_number += 2;

                monaco_line_info[instruction.line_number as usize].update_pseudo_string(vec![
                    &mut extra_instruction,
                    &mut extra_instruction_2,
                    instruction,
                ]);
            }
            "sne" => {
                //sne $regA, $regB, $regC turns into:
                //sub $regA, $regB, $regC
                //sltu $regA, $zero, $regA

                monaco_line_info[instruction.line_number as usize].mouse_hover_string =
                    "sne is a pseudo-instruction.\nsne $regA, $regB, $regC =>\n\tsub $regA, $regB, $regC\n\tsltu $regA, $zero, $regA\n"
                        .to_string();

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

                //set r0 to 1 if r1 - r2 != 0
                instruction.operator.token_name = "sltu".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[1].token_name = "$zero".to_string();
                instruction.operands[1].start_end_columns = (0, 0);
                instruction.operands[2] = instruction.operands[0].clone();
                instruction.instruction_number += 1;

                monaco_line_info[instruction.line_number as usize]
                    .update_pseudo_string(vec![&mut extra_instruction, instruction]);
            }
            "sle" => {
                //sle $regA, $regB, $regC is translated to:
                // slt $regA, $regC, $regB
                // addi $regA, $regA, 1
                // andi $regA, $regA, 1

                monaco_line_info[instruction.line_number as usize].mouse_hover_string =
                    "sle is a pseudo-instruction.\nsle $regA, $regB, $regC =>\n\tslt $regA, $regC, $regB\n\taddi $regA, $regA, 1\n\tandi $regA, $regA, 1\n"
                        .to_string();

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
                    label: None,
                };
                vec_of_added_instructions.push(extra_instruction_2.clone());

                //andi
                instruction.operator.token_name = "andi".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[1] = instruction.operands[0].clone();
                instruction.operands[2].token_name = "1".to_string();
                instruction.operands[2].start_end_columns = (0, 0);
                instruction.instruction_number += 2;

                monaco_line_info[instruction.line_number as usize].update_pseudo_string(vec![
                    &mut extra_instruction,
                    &mut extra_instruction_2,
                    instruction,
                ]);
            }
            "sleu" => {
                //sleu $regA, $regB, $regC is translated to:
                //sltu $regA, $regC, $regB
                //addi $regA, $regA, 1
                //andi $regA, $regA, 1

                monaco_line_info[instruction.line_number as usize].mouse_hover_string =
                    "sleu is a pseudo-instruction.\nsleu $regA, $regB, $regC =>\n\tsltu $regA, $regC, $regB\n\taddi $regA, $regA, 1\n\tandi $regA, $regA, 1\n"
                        .to_string();

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
                    label: None,
                };
                vec_of_added_instructions.push(extra_instruction_2.clone());

                //andi
                instruction.operator.token_name = "andi".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[1] = instruction.operands[0].clone();
                instruction.operands[2].token_name = "1".to_string();
                instruction.operands[2].start_end_columns = (0, 0);
                instruction.instruction_number += 2;

                monaco_line_info[instruction.line_number as usize].update_pseudo_string(vec![
                    &mut extra_instruction,
                    &mut extra_instruction_2,
                    instruction,
                ]);
            }
            "sgt" => {
                //sgt $regA, $regB, $regC is translated to:
                // slt $regA, $regC, $regB

                monaco_line_info[instruction.line_number as usize].mouse_hover_string =
                    "sgt is a pseudo-instruction.\nsgt $regA, $regB, $regC =>\n\tslt $regA, $regC, $regB\n"
                        .to_string();

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

                monaco_line_info[instruction.line_number as usize]
                    .update_pseudo_string(vec![instruction]);
            }
            "sgtu" => {
                //sgtu $regA, $regB, $regC is translated to:
                // sltu $regA, $regC, $regB

                monaco_line_info[instruction.line_number as usize].mouse_hover_string =
                    "sgtu is a pseudo-instruction.\nsgtu $regA, $regB, $regC =>\n\tsltu $regA, $regC, $regB\n"
                        .to_string();

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

                monaco_line_info[instruction.line_number as usize]
                    .update_pseudo_string(vec![instruction]);
            }
            "sge" => {
                //sge $regA, $regB, $regC is translated to:
                // slt $regA, $regB, $regC
                // addi $regA, $regA, 1
                // andi $regA, $regA, 1

                monaco_line_info[instruction.line_number as usize].mouse_hover_string =
                    "sge is a pseudo-instruction.\nsge $regA, $regB, $regC =>\n\tslt $regA, $regB, $regC\n\taddi $regA, $regA, 1\n\tandi $regA, $regA, 1\n"
                        .to_string();

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
                    label: None,
                };
                vec_of_added_instructions.push(extra_instruction_2.clone());

                //andi
                instruction.operator.token_name = "andi".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[1] = instruction.operands[0].clone();
                instruction.operands[2].token_name = "1".to_string();
                instruction.operands[2].start_end_columns = (0, 0);
                instruction.instruction_number += 2;

                monaco_line_info[instruction.line_number as usize].update_pseudo_string(vec![
                    &mut extra_instruction,
                    &mut extra_instruction_2,
                    instruction,
                ]);
            }
            "sgeu" => {
                //sgeu $regA, $regB, $regC is translated to:
                // sltu $regA, $regC, $regB
                // addi $regA, $regA, 1
                // andi $regA, $regA, 1

                monaco_line_info[instruction.line_number as usize].mouse_hover_string =
                    "sgeu is a pseudo-instruction.\nsgeu $regA, $regB, $regC =>\n\tsltu $regA, $regB, $regC\n\taddi $regA, $regA, 1\n\tandi $regA, $regA, 1\n"
                        .to_string();

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
                    label: None,
                };
                vec_of_added_instructions.push(extra_instruction_2.clone());

                //andi
                instruction.operator.token_name = "andi".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[1] = instruction.operands[0].clone();
                instruction.operands[2].token_name = "1".to_string();
                instruction.operands[2].start_end_columns = (0, 0);
                instruction.instruction_number += 2;

                monaco_line_info[instruction.line_number as usize].update_pseudo_string(vec![
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
                        monaco_line_info[instruction.line_number as usize].mouse_hover_string =
                            "lw $regA, label is a pseudo-instruction.\nlw $regA, label =>\n\tlui $at, label\n\tlw $regA, lower16($at)\n\twhere lower16 is the lower 16 bits of the labelled address.\n"
                                .to_string();
                    } else {
                        monaco_line_info[instruction.line_number as usize].mouse_hover_string =
                            "sw $regA, label is a pseudo-instruction.\nsw $regA, label =>\n\tlui $at, label\n\tsw $regA, lower16($at)\n\twhere lower16 is the lower 16 bits of the labelled address.\n"
                                .to_string();
                    }

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
                        label: None,
                    };
                    vec_of_added_instructions.push(extra_instruction);
                    instruction.operands[1].token_name = "$at".to_string();
                    instruction.operands[1].start_end_columns = (0, 0);
                    instruction.instruction_number += 1;
                }
            }
            "subi" => {
                //subi $regA, $regB, immediate is translated to:
                //ori $at, $zero, immediate
                //sub $regA, $regB, $at

                monaco_line_info[instruction.line_number as usize].mouse_hover_string =
                    "subi $regA, $regB, immediate is a pseudo-instruction.\nsubi $regA, $regB, immediate =>\n\tori $at, $zero, immediate\n\tsub $regA, $regB, $at\n"
                        .to_string();

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
                    label: None,
                };

                vec_of_added_instructions.push(extra_instruction.clone());

                //adjust subi for the added instruction
                instruction.operator.token_name = "sub".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[2].token_name = "$at".to_string();
                instruction.operands[2].start_end_columns = (0, 0);
                instruction.instruction_number += 1;

                monaco_line_info[instruction.line_number as usize]
                    .update_pseudo_string(vec![&mut extra_instruction, instruction]);
            }
            "dsubi" => {
                //dsubi $regA, $regB, immediate is translated to:
                //ori $at, $zero, immediate
                //dsub $regA, $regB, $at

                monaco_line_info[instruction.line_number as usize].mouse_hover_string =
                    "dsubi $regA, $regB, immediate is a pseudo-instruction.\ndsubi $regA, $regB, immediate =>\n\tori $at, $zero, immediate\n\tdsub $regA, $regB, $at\n"
                        .to_string();

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
                    label: None,
                };
                vec_of_added_instructions.push(extra_instruction.clone());
                //adjust dsubi for the added instruction
                instruction.operator.token_name = "dsub".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[2].token_name = "$at".to_string();
                instruction.operands[2].start_end_columns = (0, 0);
                instruction.instruction_number += 1;

                monaco_line_info[instruction.line_number as usize]
                    .update_pseudo_string(vec![&mut extra_instruction, instruction]);
            }
            "dsubiu" => {
                //dsubiu $regA, $regB, immediate is translated to:
                //ori $at, $zero, immediate
                //dsubu $regA, $regB, $at

                monaco_line_info[instruction.line_number as usize].mouse_hover_string =
                    "dsubiu $regA, $regB, immediate is a pseudo-instruction.\ndsubiu $regA, $regB, immediate =>\n\tori $at, $zero, immediate\n\tdsubu $regA, $regB, $at\n"
                        .to_string();

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
                    label: None,
                };
                vec_of_added_instructions.push(extra_instruction.clone());

                //adjust subiu for the added instruction
                instruction.operator.token_name = "dsubu".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[2].token_name = "$at".to_string();
                instruction.operands[2].start_end_columns = (0, 0);
                instruction.instruction_number += 1;

                monaco_line_info[instruction.line_number as usize]
                    .update_pseudo_string(vec![&mut extra_instruction, instruction]);
            }
            "muli" => {
                //muli $regA, $regB, immediate is translated to:
                //ori $at, $zero, immediate
                //mul $regA, $regB, $at

                monaco_line_info[instruction.line_number as usize].mouse_hover_string =
                    "muli $regA, $regB, immediate is a pseudo-instruction.\nmuli $regA, $regB, immediate =>\n\tori $at, $zero, immediate\n\tmul $regA, $regB, $at\n"
                        .to_string();

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
                    label: None,
                };
                vec_of_added_instructions.push(extra_instruction.clone());

                //adjust muli for the added instruction
                instruction.operator.token_name = "mul".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[2].token_name = "$at".to_string();
                instruction.operands[2].start_end_columns = (0, 0);
                instruction.instruction_number += 1;

                monaco_line_info[instruction.line_number as usize]
                    .update_pseudo_string(vec![&mut extra_instruction, instruction]);
            }
            "dmuli" => {
                //dmuli $regA, $regB, immediate is translated to:
                //ori $at, $zero, immediate
                //dmul $regA, $regB, $at

                monaco_line_info[instruction.line_number as usize].mouse_hover_string =
                    "dmuli $regA, $regB, immediate is a pseudo-instruction.\ndmuli $regA, $regB, immediate =>\n\tori $at, $zero, immediate\n\tdmul $regA, $regB, $at\n"
                        .to_string();

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
                    label: None,
                };
                vec_of_added_instructions.push(extra_instruction.clone());
                //adjust dmuli for the added instruction
                instruction.operator.token_name = "dmul".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[2].token_name = "$at".to_string();
                instruction.operands[2].start_end_columns = (0, 0);
                instruction.instruction_number += 1;

                monaco_line_info[instruction.line_number as usize]
                    .update_pseudo_string(vec![&mut extra_instruction, instruction]);
            }
            "dmuliu" => {
                //dmuliu $regA, $regB, immediate is translated to:
                //ori $at, $zero, immediate
                //dmulu $regA, $regB, $at

                monaco_line_info[instruction.line_number as usize].mouse_hover_string =
                    "dmuliu $regA, $regB, immediate is a pseudo-instruction.\ndmuliu $regA, $regB, immediate =>\n\tori $at, $zero, immediate\n\tdmulu $regA, $regB, $at\n"
                        .to_string();

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
                    label: None,
                };
                vec_of_added_instructions.push(extra_instruction.clone());
                //adjust dmuliu for the added instruction
                instruction.operator.token_name = "dmulu".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[2].token_name = "$at".to_string();
                instruction.operands[2].start_end_columns = (0, 0);
                instruction.instruction_number += 1;

                monaco_line_info[instruction.line_number as usize]
                    .update_pseudo_string(vec![&mut extra_instruction, instruction]);
            }
            "divi" => {
                //divi $regA, immediate is translated to:
                //ori $at, $zero, immediate
                //div $regA, $at

                monaco_line_info[instruction.line_number as usize].mouse_hover_string =
                    "divi $regA, immediate is a pseudo-instruction.\ndivi $regA, immediate =>\n\tori $at, $zero, immediate\n\tdiv $regA, $at\n"
                        .to_string();

                //make sure the are the right number of operands a second operand
                if instruction.operands.len() != 2 {
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
                        instruction.operands[1].clone(),
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number,
                    line_number: instruction.line_number,
                    errors: vec![],
                    label: None,
                };
                vec_of_added_instructions.push(extra_instruction.clone());
                //adjust divi for the added instruction
                instruction.operator.token_name = "div".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[1].token_name = "$at".to_string();
                instruction.operands[1].start_end_columns = (0, 0);
                instruction.instruction_number += 1;

                monaco_line_info[instruction.line_number as usize]
                    .update_pseudo_string(vec![&mut extra_instruction, instruction]);
            }
            "ddivi" => {
                //ddivi $regA, immediate is translated to:
                //ori $at, $zero, immediate
                //ddiv $regA, $at

                monaco_line_info[instruction.line_number as usize].mouse_hover_string =
                    "ddivi $regA, immediate is a pseudo-instruction.\nddivi $regA, immediate =>\n\tori $at, $zero, immediate\n\tddiv $regA, $at\n"
                        .to_string();

                //make sure the are the right number of operands
                if instruction.operands.len() != 2 {
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
                        instruction.operands[1].clone(),
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number,
                    line_number: instruction.line_number,
                    errors: vec![],
                    label: None,
                };
                vec_of_added_instructions.push(extra_instruction.clone());
                //adjust ddivi for the added instruction
                instruction.operator.token_name = "ddiv".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[1].token_name = "$at".to_string();
                instruction.operands[1].start_end_columns = (0, 0);
                instruction.instruction_number += 1;

                monaco_line_info[instruction.line_number as usize]
                    .update_pseudo_string(vec![&mut extra_instruction, instruction]);
            }
            "ddiviu" => {
                //ddiviu $regA, immediate is translated to:
                //ori $at, $zero, immediate
                //ddivu $regA, $at

                monaco_line_info[instruction.line_number as usize].mouse_hover_string =
                    "ddiviu $regA, immediate is a pseudo-instruction.\nddiviu $regA, immediate =>\n\tori $at, $zero, immediate\n\tddivu $regA, $at\n"
                        .to_string();

                //make sure the are the right number of operands
                if instruction.operands.len() != 2 {
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
                        instruction.operands[1].clone(),
                    ],
                    binary: 0,
                    instruction_number: instruction.instruction_number,
                    line_number: instruction.line_number,
                    errors: vec![],
                    label: None,
                };
                vec_of_added_instructions.push(extra_instruction.clone());
                //adjust ddiviu for the added instruction
                instruction.operator.token_name = "ddivu".to_string();
                instruction.operator.start_end_columns = (0, 0);
                instruction.operands[1].token_name = "$at".to_string();
                instruction.operands[1].start_end_columns = (0, 0);
                instruction.instruction_number += 1;

                monaco_line_info[instruction.line_number as usize]
                    .update_pseudo_string(vec![&mut extra_instruction, instruction]);
            }
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
        for (i, monaco_line) in monaco_line_info.iter_mut().enumerate() {
            if !monaco_line.tokens.is_empty() && monaco_line.tokens[0].token_name == ".text" {
                text_index = Some(i as u32);
                break;
            }
        }
        if let Some(..) = text_index {
            //add syscall after first index of .text if it exists
            monaco_line_info[text_index.unwrap() as usize + 1]
                .updated_monaco_string
                .insert_str(0, "syscall\n");

            instructions.push(Instruction {
                operator: Token {
                    token_name: "syscall".to_string(),
                    start_end_columns: (0, 0),
                    token_type: Operator,
                },
                operands: vec![],
                binary: 0,
                instruction_number: 0,
                line_number: text_index.unwrap(),
                errors: vec![],
                label: None,
            });
        } else {
            //otherwise, add it at the beginning of monaco
            monaco_line_info[0]
                .updated_monaco_string
                .insert_str(0, ".text\nsyscall\n");

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
                label: None,
            });
        }
    } else {
        let last_instruction = instructions.last().unwrap();
        //if the last instruction in monaco is not a syscall, add it in to updated_monaco_strings and to instructions
        if last_instruction.operator.token_name != "syscall" {
            monaco_line_info[last_instruction.line_number as usize]
                .updated_monaco_string
                .push_str("\nsyscall");

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
                label: None,
            })
        }
    }
}

///the second part of completing pseudo-instructions. LW and SW with labels requires the address of the label to be known,
/// the second part of this must occur after the label hashmap is completed.
pub fn complete_lw_sw_pseudo_instructions(
    instructions: &mut Vec<Instruction>,
    labels: &HashMap<String, u32>,
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

            monaco_line_info[instructions[index].line_number as usize].update_pseudo_string(vec![
                &mut instructions.clone()[index - 1],
                &mut instructions.clone()[index],
            ]);
        }
    }
}
