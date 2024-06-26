use crate::parser::parser_assembler_main::append_binary;
use crate::parser::parser_structs_and_enums::ErrorType::{
    ImmediateOutOfBounds, ImproperlyFormattedASCII, ImproperlyFormattedChar,
    IncorrectNumberOfOperands, IncorrectRegisterTypeFP, IncorrectRegisterTypeGP,
    InvalidMemorySyntax, LabelNotFound, NonASCIIChar, NonASCIIString, NonFloatImmediate,
    NonIntImmediate, UnrecognizedDataType, UnrecognizedFPRegister, UnrecognizedGPRegister,
};
use crate::parser::parser_structs_and_enums::OperandType::{
    Immediate, LabelAbsolute, LabelRelative, MemoryAddress, RegisterFP, RegisterGP, ShiftAmount,
    UpperImmediate,
};
use crate::parser::parser_structs_and_enums::RegisterType::{FloatingPoint, GeneralPurpose};
use crate::parser::parser_structs_and_enums::TokenType::{
    Byte, Float, Half, Space, Word, ASCII, ASCIIZ,
};
use crate::parser::parser_structs_and_enums::{
    Data, Error, Instruction, OperandType, RegisterType, TokenType, FP_REGISTERS, GP_REGISTERS,
};
use std::collections::HashMap;

use super::parser_structs_and_enums::{RISCV_FP_REGISTERS, RISCV_GP_REGISTERS};

///This function takes an instruction whose operands it is supposed to read, the order of expected operand types and then
///the order these operands should be concatenated onto the binary representation of the string
///the function returns the instruction it was given with any errors and the binary of the operands added on.
pub fn read_operands(
    instruction: &mut Instruction,
    expected_operands: Vec<OperandType>,
    concat_order: Vec<usize>,
    labels_option: Option<HashMap<String, usize>>,
) -> &mut Instruction {
    //if the number of operands in the instruction does not match the expected number, there is an error
    if instruction.operands.len() != expected_operands.len() {
        instruction.errors.push(Error {
            error_name: IncorrectNumberOfOperands,
            token_causing_error: instruction.operator.token_name.clone(),
            start_end_columns: instruction.operator.start_end_columns,
            message: "".to_string(),
        });
        return instruction;
    }

    let labels = match labels_option {
        Some(labels_option) => labels_option,
        None => HashMap::new(),
    };

    //operands aren't represented in the binary in the order they're read so the vec<u32> allows us to concatenate them in the proper order after they're all read.
    let mut binary_representation: Vec<u32> = Vec::new();
    let mut bit_lengths: Vec<u8> = Vec::new();
    //goes through once for each expected operand
    for (i, operand_type) in expected_operands.iter().enumerate() {
        //break if there are no more operands to read. Should only occur if IncorrectNumberOfOperands occurs above
        if i >= instruction.operands.len() {
            break;
        };

        //match case calls the proper functions based on the expected operand type. The data returned from these functions is always
        //the binary of the read operand and the option for any errors encountered while reading the operand. If there were no errors,
        //the binary is pushed to the string representations vec. Otherwise, the errors are pushed to the instruction.errors vec.
        match operand_type {
            RegisterGP => {
                instruction.operands[i].token_type = TokenType::RegisterGP;
                bit_lengths.push(5);

                let register_results = read_register(
                    &instruction.operands[i].token_name,
                    instruction.operands[i].start_end_columns,
                    GeneralPurpose,
                );

                binary_representation.push(register_results.0 as u32);
                if register_results.1.is_some() {
                    instruction.errors.push(register_results.1.unwrap());
                }
            }
            Immediate => {
                instruction.operands[i].token_type = TokenType::Immediate;
                bit_lengths.push(16);

                let immediate_results = read_immediate(
                    &instruction.operands[i].token_name,
                    instruction.operands[i].start_end_columns,
                    16,
                );

                binary_representation.push(immediate_results.0);
                if immediate_results.1.is_some() {
                    instruction.errors.push(immediate_results.1.unwrap());
                }
            }
            UpperImmediate => {
                // Don't need to handle for MIPS
            }
            MemoryAddress => {
                instruction.operands[i].token_type = TokenType::MemoryAddress;

                bit_lengths.push(16);
                bit_lengths.push(5);
                //memory address works a bit differently because it really amounts to two operands: the offset and base
                //meaning there are two values to push and the possibility of errors on both operands
                let memory_results = read_memory_address(
                    &instruction.operands[i].token_name,
                    instruction.operands[i].start_end_columns,
                );

                binary_representation.push(memory_results.0);
                binary_representation.push(memory_results.1);
                if memory_results.2.is_some() {
                    for error in memory_results.2.unwrap() {
                        instruction.errors.push(error);
                    }
                }
            }
            RegisterFP => {
                instruction.operands[i].token_type = TokenType::RegisterFP;

                bit_lengths.push(5);
                let register_results = read_register(
                    &instruction.operands[i].token_name,
                    instruction.operands[i].start_end_columns,
                    FloatingPoint,
                );

                binary_representation.push(register_results.0 as u32);
                if register_results.1.is_some() {
                    instruction.errors.push(register_results.1.unwrap());
                }
            }
            LabelAbsolute => {
                instruction.operands[i].token_type = TokenType::LabelOperand;

                bit_lengths.push(26);
                let label_absolute_results = read_label_absolute(
                    &instruction.operands[i].token_name,
                    instruction.operands[i].start_end_columns,
                    labels.clone(),
                );

                binary_representation.push(label_absolute_results.0);
                if label_absolute_results.1.is_some() {
                    instruction.errors.push(label_absolute_results.1.unwrap());
                }
            }
            LabelRelative => {
                instruction.operands[i].token_type = TokenType::LabelOperand;

                bit_lengths.push(16);
                let label_relative_results = read_label_relative(
                    &instruction.operands[i].token_name,
                    instruction.operands[i].start_end_columns,
                    instruction.instruction_number,
                    labels.clone(),
                );
                binary_representation.push(label_relative_results.0);
                if label_relative_results.1.is_some() {
                    instruction.errors.push(label_relative_results.1.unwrap());
                }
            }
            ShiftAmount => {
                instruction.operands[i].token_type = TokenType::Immediate;
                bit_lengths.push(5);

                let immediate_results = read_immediate(
                    &instruction.operands[i].token_name,
                    instruction.operands[i].start_end_columns,
                    5,
                );

                binary_representation.push(immediate_results.0);
                if immediate_results.1.is_some() {
                    instruction.errors.push(immediate_results.1.unwrap());
                }
            }
        }
    }
    //once all operands are read, we can append them onto the instruction
    for element in concat_order {
        instruction.binary = append_binary(
            instruction.binary,
            binary_representation[element - 1],
            bit_lengths[element - 1],
        );
    }

    instruction
}

///This function takes an instruction whose operands it is supposed to read, the order of expected operand types and then
///the order these operands should be concatenated onto the binary representation of the string
///the function returns the instruction it was given with any errors and the binary of the operands added on.
pub fn read_operands_riscv(
    instruction: &mut Instruction,
    expected_operands: Vec<OperandType>,
    concat_order: Vec<usize>,
    labels_option: Option<HashMap<String, usize>>,
    funct3: Option<u32>,
    fmt: Option<u32>,
) -> &mut Instruction {
    //if the number of operands in the instruction does not match the expected number, there is an error
    if instruction.operands.len() != expected_operands.len() {
        instruction.errors.push(Error {
            error_name: IncorrectNumberOfOperands,
            token_causing_error: instruction.operator.token_name.clone(),
            start_end_columns: instruction.operator.start_end_columns,
            message: "".to_string(),
        });
        return instruction;
    }

    let labels = match labels_option {
        Some(labels_option) => labels_option,
        None => HashMap::new(),
    };

    //operands aren't represented in the binary in the order they're read so the vec<u32> allows us to concatenate them in the proper order after they're all read.
    let mut binary_representation: Vec<u32> = Vec::new();
    let mut bit_lengths: Vec<u8> = Vec::new();
    //goes through once for each expected operand
    for (i, operand_type) in expected_operands.iter().enumerate() {
        //break if there are no more operands to read. Should only occur if IncorrectNumberOfOperands occurs above
        if i >= instruction.operands.len() {
            break;
        };

        //match case calls the proper functions based on the expected operand type. The data returned from these functions is always
        //the binary of the read operand and the option for any errors encountered while reading the operand. If there were no errors,
        //the binary is pushed to the string representations vec. Otherwise, the errors are pushed to the instruction.errors vec.
        match operand_type {
            RegisterGP => {
                instruction.operands[i].token_type = TokenType::RegisterGP;
                bit_lengths.push(5);

                let register_results = read_register_riscv(
                    &instruction.operands[i].token_name,
                    instruction.operands[i].start_end_columns,
                    GeneralPurpose,
                );

                // Vector holding all register arguments
                binary_representation.push(register_results.0 as u32);
                if register_results.1.is_some() {
                    instruction.errors.push(register_results.1.unwrap());
                }
            }
            Immediate => {
                instruction.operands[i].token_type = TokenType::Immediate;
                bit_lengths.push(12); // 12 bits to represent immediates

                let immediate_results = read_immediate(
                    &instruction.operands[i].token_name,
                    instruction.operands[i].start_end_columns,
                    12,
                );

                binary_representation.push(immediate_results.0);
                if immediate_results.1.is_some() {
                    instruction.errors.push(immediate_results.1.unwrap());
                }
            }
            UpperImmediate => {
                instruction.operands[i].token_type = TokenType::Immediate;
                bit_lengths.push(20); // 20 bits to represent upper immediates

                let immediate_results = read_immediate(
                    &instruction.operands[i].token_name,
                    instruction.operands[i].start_end_columns,
                    20,
                );

                binary_representation.push(immediate_results.0);
                if immediate_results.1.is_some() {
                    instruction.errors.push(immediate_results.1.unwrap());
                }
            }
            MemoryAddress => {
                instruction.operands[i].token_type = TokenType::MemoryAddress;

                bit_lengths.push(12);
                bit_lengths.push(5);
                //memory address works a bit differently because it really amounts to two operands: the offset and base
                //meaning there are two values to push and the possibility of errors on both operands
                let memory_results = read_memory_address_riscv(
                    &instruction.operands[i].token_name,
                    instruction.operands[i].start_end_columns,
                );

                binary_representation.push(memory_results.0);
                binary_representation.push(memory_results.1);
                if memory_results.2.is_some() {
                    for error in memory_results.2.unwrap() {
                        instruction.errors.push(error);
                    }
                }
            }
            RegisterFP => {
                instruction.operands[i].token_type = TokenType::RegisterFP;

                bit_lengths.push(5);
                let register_results = read_register_riscv(
                    &instruction.operands[i].token_name,
                    instruction.operands[i].start_end_columns,
                    FloatingPoint,
                );

                binary_representation.push(register_results.0 as u32);
                if register_results.1.is_some() {
                    instruction.errors.push(register_results.1.unwrap());
                }
            }
            LabelAbsolute => {
                instruction.operands[i].token_type = TokenType::LabelOperand;

                bit_lengths.push(20);
                let label_absolute_results = read_label_absolute(
                    &instruction.operands[i].token_name,
                    instruction.operands[i].start_end_columns,
                    labels.clone(),
                );

                binary_representation.push(label_absolute_results.0);
                if label_absolute_results.1.is_some() {
                    instruction.errors.push(label_absolute_results.1.unwrap());
                }
            }
            LabelRelative => {
                instruction.operands[i].token_type = TokenType::LabelOperand;

                bit_lengths.push(12);
                let label_relative_results = read_label_relative(
                    &instruction.operands[i].token_name,
                    instruction.operands[i].start_end_columns,
                    instruction.instruction_number,
                    labels.clone(),
                );
                binary_representation.push(label_relative_results.0);
                if label_relative_results.1.is_some() {
                    instruction.errors.push(label_relative_results.1.unwrap());
                }
            }
            ShiftAmount => {
                instruction.operands[i].token_type = TokenType::Immediate;
                bit_lengths.push(7);

                let immediate_results = read_immediate(
                    &instruction.operands[i].token_name,
                    instruction.operands[i].start_end_columns,
                    7,
                );

                binary_representation.push(immediate_results.0);
                if immediate_results.1.is_some() {
                    instruction.errors.push(immediate_results.1.unwrap());
                }
            }
        }
    }
    //once all operands are read, we can append them onto the instruction
    for (index, element) in concat_order.iter().rev().enumerate() {
        instruction.binary = append_binary(
            instruction.binary,
            binary_representation[element - 1],
            bit_lengths[element - 1],
        );
        if index == concat_order.len() - 2 && funct3.is_some()
        // Set funct3 value before the final argument
        {
            instruction.binary = append_binary(instruction.binary, funct3.unwrap_or(0), 3)
        } else if index == 0 && fmt.is_some() {
            instruction.binary = append_binary(instruction.binary, fmt.unwrap_or(0), 2)
        }
    }

    instruction
}

///Returns distance to a labeled instruction relative to the instruction after the current instruction.
/// The value represents instruction numbers NOT bytes.
pub fn read_label_relative(
    given_label: &str,
    start_end_columns: (usize, usize),
    current_instruction_number: usize,
    labels: HashMap<String, usize>,
) -> (u32, Option<Error>) {
    let result = labels.get(given_label);

    if result.is_none() {
        return (
            0,
            Some(Error {
                error_name: LabelNotFound,
                token_causing_error: given_label.to_string(),
                start_end_columns,
                message: "".to_string(),
            }),
        );
    }
    let mut offset = *result.unwrap() as i32;
    offset -= (current_instruction_number as i32 + 1) << 2;
    offset >>= 2;

    (offset as u32, None)
}

///Takes a string and returns the instruction number of the matching label in memory. If there is no match, an error is returned
/// This value corresponds to instruction number, NOT byte address.
pub fn read_label_absolute(
    given_label: &str,
    start_end_columns: (usize, usize),
    labels: HashMap<String, usize>,
) -> (u32, Option<Error>) {
    let result = labels.get(given_label);
    if result.is_none() {
        return (
            0,
            Some(Error {
                error_name: LabelNotFound,
                token_causing_error: given_label.to_string(),
                start_end_columns,
                message: "".to_string(),
            }),
        );
    }
    ((*result.unwrap() >> 2) as u32, None)
}

///Takes in a memory address and token number and returns the binary for the offset value, base register value, and any errors.
/// If the string given matches a label, that address is returned instead
pub fn read_memory_address(
    orig_string: &str,
    start_end_columns: (usize, usize),
) -> (u32, u32, Option<Vec<Error>>) {
    //the indices of the open and close parentheses are checked.
    //If either are missing or they are in the wrong order, an error is returned
    let open_index = orig_string.find('(');
    let close_index = orig_string.find(')');
    if close_index.is_none() || open_index.is_none() || close_index < open_index {
        return (
            0,
            0,
            Some(vec![Error {
                error_name: InvalidMemorySyntax,
                token_causing_error: orig_string.to_string(),
                start_end_columns,
                message: "".to_string(),
            }]),
        );
    }

    //splits the string at the index of the open parenthesis to isolate the base and offset
    let (offset_str, base_str) = orig_string.split_at(open_index.unwrap());

    let mut base: Vec<char> = base_str.chars().collect();

    //returns an error if there are any characters after the close parenthesis
    if base[base.len() - 1] != ')' {
        return (
            0,
            0,
            Some(vec![Error {
                error_name: InvalidMemorySyntax,
                token_causing_error: orig_string.to_string(),
                start_end_columns,
                message: "".to_string(),
            }]),
        );
    }

    //removes the open and close parentheses characters and then turns it into a string
    base = base[1..base.len() - 1].to_owned();
    let mut cleaned_base: String = base.into_iter().collect();
    cleaned_base = cleaned_base.to_string();

    //offset is an immediate while base is a register so the read functions for those operands
    //will confirm they are properly formatted
    let immediate_results = read_immediate(offset_str, start_end_columns, 16);
    let register_results = read_register(&cleaned_base, start_end_columns, GeneralPurpose);

    //any errors found in the read_immediate or read_register functions are collected into a vec
    //if there were any errors, those are returned
    let mut return_errors: Vec<Error> = Vec::new();
    if immediate_results.1.is_some() {
        return_errors.push(immediate_results.1.unwrap())
    }
    if register_results.1.is_some() {
        return_errors.push(register_results.1.unwrap());
    }
    if !return_errors.is_empty() {
        return (0, 0, Some(return_errors));
    }

    //if the function reaches here and hasn't already returned, there aren't any errors
    (immediate_results.0, register_results.0 as u32, None)
}

///Takes in a memory address and token number and returns the binary for the offset value, base register value, and any errors.
/// If the string given matches a label, that address is returned instead
pub fn read_memory_address_riscv(
    orig_string: &str,
    start_end_columns: (usize, usize),
) -> (u32, u32, Option<Vec<Error>>) {
    //the indices of the open and close parentheses are checked.
    //If either are missing or they are in the wrong order, an error is returned
    let open_index = orig_string.find('(');
    let close_index = orig_string.find(')');
    if close_index.is_none() || open_index.is_none() || close_index < open_index {
        return (
            0,
            0,
            Some(vec![Error {
                error_name: InvalidMemorySyntax,
                token_causing_error: orig_string.to_string(),
                start_end_columns,
                message: "".to_string(),
            }]),
        );
    }

    //splits the string at the index of the open parenthesis to isolate the base and offset
    let (offset_str, base_str) = orig_string.split_at(open_index.unwrap());

    let mut base: Vec<char> = base_str.chars().collect();

    //returns an error if there are any characters after the close parenthesis
    if base[base.len() - 1] != ')' {
        return (
            0,
            0,
            Some(vec![Error {
                error_name: InvalidMemorySyntax,
                token_causing_error: orig_string.to_string(),
                start_end_columns,
                message: "".to_string(),
            }]),
        );
    }

    //removes the open and close parentheses characters and then turns it into a string
    base = base[1..base.len() - 1].to_owned();
    let mut cleaned_base: String = base.into_iter().collect();
    cleaned_base = cleaned_base.to_string();

    //offset is an immediate while base is a register so the read functions for those operands
    //will confirm they are properly formatted
    let immediate_results = read_immediate(offset_str, start_end_columns, 16);
    let register_results = read_register_riscv(&cleaned_base, start_end_columns, GeneralPurpose);

    //any errors found in the read_immediate or read_register functions are collected into a vec
    //if there were any errors, those are returned
    let mut return_errors: Vec<Error> = Vec::new();
    if immediate_results.1.is_some() {
        return_errors.push(immediate_results.1.unwrap())
    }
    if register_results.1.is_some() {
        return_errors.push(register_results.1.unwrap());
    }
    if !return_errors.is_empty() {
        return (0, 0, Some(return_errors));
    }

    //if the function reaches here and hasn't already returned, there aren't any errors
    (immediate_results.0, register_results.0 as u32, None)
}

///read_register takes the string of the register name, the token number the register is from the corresponding instruction
///and the expected register type. It calls the corresponding functions holding the match cases for the different register types.
pub fn read_register(
    register: &str,
    start_end_columns: (usize, usize),
    register_type: RegisterType,
) -> (u8, Option<Error>) {
    if register_type == GeneralPurpose {
        //this section is for matching general purpose registers
        let general_result = match_gp_register(register);
        match general_result {
            Some(general_result) => (general_result, None),
            None => {
                if match_fp_register(register).is_some() {
                    // Creates Error if supplied a fp register for gp
                    (
                        0,
                        Some(Error {
                            error_name: IncorrectRegisterTypeFP,
                            token_causing_error: register.to_string(),
                            start_end_columns,
                            message: "".to_string(),
                        }),
                    )
                } else {
                    (
                        0,
                        Some(Error {
                            error_name: UnrecognizedGPRegister,
                            token_causing_error: register.to_string(),
                            start_end_columns,
                            message: "".to_string(),
                        }),
                    )
                }
            }
        }
    } else {
        //this section is for matching floating point registers
        let floating_result = match_fp_register(register);
        match floating_result {
            Some(floating_result) => (floating_result, None),
            None => {
                if match_gp_register(register).is_some() {
                    (
                        0,
                        Some(Error {
                            error_name: IncorrectRegisterTypeGP,
                            token_causing_error: register.to_string(),
                            start_end_columns,
                            message: "".to_string(),
                        }),
                    )
                } else {
                    (
                        0,
                        Some(Error {
                            error_name: UnrecognizedFPRegister,
                            token_causing_error: register.to_string(),
                            start_end_columns,
                            message: "".to_string(),
                        }),
                    )
                }
            }
        }
    }
}

///read_register takes the string of the register name, the token number the register is from the corresponding instruction
///and the expected register type. It calls the corresponding functions holding the match cases for the different register types.
pub fn read_register_riscv(
    register: &str,
    start_end_columns: (usize, usize),
    register_type: RegisterType,
) -> (u8, Option<Error>) {
    if register_type == GeneralPurpose {
        //this section is for matching general purpose registers
        let general_result = match_gp_register_riscv(register);
        match general_result {
            Some(general_result) => (general_result, None),
            None => {
                if match_fp_register_riscv(register).is_some() {
                    // Creates Error if supplied a fp register for gp
                    (
                        0,
                        Some(Error {
                            error_name: IncorrectRegisterTypeFP,
                            token_causing_error: register.to_string(),
                            start_end_columns,
                            message: "".to_string(),
                        }),
                    )
                } else {
                    (
                        0,
                        Some(Error {
                            error_name: UnrecognizedGPRegister,
                            token_causing_error: register.to_string(),
                            start_end_columns,
                            message: "".to_string(),
                        }),
                    )
                }
            }
        }
    } else {
        //this section is for matching floating point registers
        let floating_result = match_fp_register_riscv(register);
        match floating_result {
            Some(floating_result) => (floating_result, None),
            None => {
                if match_gp_register_riscv(register).is_some() {
                    (
                        0,
                        Some(Error {
                            error_name: IncorrectRegisterTypeGP,
                            token_causing_error: register.to_string(),
                            start_end_columns,
                            message: "".to_string(),
                        }),
                    )
                } else {
                    (
                        0,
                        Some(Error {
                            error_name: UnrecognizedFPRegister,
                            token_causing_error: register.to_string(),
                            start_end_columns,
                            message: "".to_string(),
                        }),
                    )
                }
            }
        }
    }
}

///This function takes a register string as an argument and returns the string of the binary of the matching
///general register or none if there is not one that matches.
pub fn match_gp_register(given_string: &str) -> Option<u8> {
    for register in GP_REGISTERS {
        for name in register.names {
            if &given_string.to_lowercase().as_str() == name {
                return Some(register.binary);
            }
        }
    }
    None
}

///This function takes a register string as an argument and returns the string of the binary of the matching
///general register or none if there is not one that matches.
pub fn match_gp_register_riscv(given_string: &str) -> Option<u8> {
    for register in RISCV_GP_REGISTERS {
        for name in register.names {
            if &given_string.to_lowercase().as_str() == name {
                return Some(register.binary);
            }
        }
    }
    None
}

///This function takes a register string as an argument and returns the string of the binary of the matching
///floating point register or none if there is not one that matches.
pub fn match_fp_register(given_string: &str) -> Option<u8> {
    for register in FP_REGISTERS {
        if given_string.to_lowercase() == register.name {
            return Some(register.binary);
        }
    }
    None
}

///This function takes a register string as an argument and returns the string of the binary of the matching
///floating point register or none if there is not one that matches.
pub fn match_fp_register_riscv(given_string: &str) -> Option<u8> {
    for register in RISCV_FP_REGISTERS {
        for name in register.names {
            if &given_string.to_lowercase().as_str() == name {
                return Some(register.binary);
            }
        }
    }
    None
}

///This function takes a string representation of an immediate value and the number of bits available to represent it
/// and attempts to translate it to an actual integer. If the value cannot be cast to int or is too big to be represented
/// by the available bits, an error is returned.
pub fn read_immediate(
    given_text: &str,
    start_end_columns: (usize, usize),
    num_bits: u32,
) -> (u32, Option<Error>) {
    //attempts to cast the text into a large int
    let mut parse_results = given_text.parse::<i64>();

    //if that results in an error, try to read it as hex
    if parse_results.is_err() {
        let removed_prefix = given_text.strip_prefix("0x");
        if let Some(removed_prefix) = removed_prefix {
            parse_results = i64::from_str_radix(removed_prefix, 16);
        }
    }

    //if there was an error typecasting, the function returns with an error to add to the instruction or data
    if parse_results.is_err() {
        return (
            0,
            Some(Error {
                error_name: NonIntImmediate,
                token_causing_error: given_text.to_string(),
                start_end_columns,
                message: "".to_string(),
            }),
        );
    }
    let int_representation = parse_results.unwrap();

    //finds the max and min values of a signed integer with specified number of bits
    let max_value = i64::pow(2, num_bits);
    let min_value = (-max_value) - 1;
    //if the parsed value is out of possible bounds, an error is returned to add to the instruction
    if int_representation > max_value || int_representation < min_value {
        return (
            0,
            Some(Error {
                error_name: ImmediateOutOfBounds,
                token_causing_error: given_text.to_string(),
                start_end_columns,
                message: "".to_string(),
            }),
        );
    }

    (int_representation as u32, None)
}

///Takes the data list and finds the actual values for each data entry that will be put into memory
pub fn assemble_data_binary(data_list: &mut [Data]) -> Vec<u8> {
    let mut vec_of_data: Vec<u8> = Vec::new();
    for datum in data_list.iter_mut() {
        datum.data_number = vec_of_data.len();
        match &*datum.data_type.token_name.to_lowercase() {
            ".ascii" => {
                //pushes a string of characters to memory
                for value in datum.data_entries.iter_mut() {
                    value.token_type = ASCII;
                    let chars = value.token_name.as_bytes();
                    if chars[0] != b'\"' || chars[chars.len() - 1] != b'\"' || chars.len() <= 2 {
                        datum.errors.push(Error {
                            error_name: ImproperlyFormattedASCII,
                            token_causing_error: value.token_name.to_string(),
                            start_end_columns: value.start_end_columns,
                            message: "".to_string(),
                        });
                    } else if !chars.is_ascii() {
                        datum.errors.push(Error {
                            error_name: NonASCIIString,
                            token_causing_error: value.token_name.to_string(),
                            start_end_columns: value.start_end_columns,
                            message: "".to_string(),
                        })
                    } else {
                        for char in chars.iter().take(chars.len() - 1).skip(1) {
                            vec_of_data.push(*char);
                        }
                    }
                }
            }
            ".asciiz" => {
                //same as ascii but pushes a \0 to memory as well
                for value in datum.data_entries.iter_mut() {
                    value.token_type = ASCIIZ;
                    let chars = value.token_name.as_bytes();
                    if chars[0] != b'\"' || chars[chars.len() - 1] != b'\"' || chars.len() <= 2 {
                        datum.errors.push(Error {
                            error_name: ImproperlyFormattedASCII,
                            token_causing_error: value.token_name.to_string(),
                            start_end_columns: value.start_end_columns,
                            message: "".to_string(),
                        });
                    } else if !chars.is_ascii() {
                        datum.errors.push(Error {
                            error_name: NonASCIIString,
                            token_causing_error: value.token_name.to_string(),
                            start_end_columns: value.start_end_columns,
                            message: "".to_string(),
                        })
                    } else {
                        for char in chars.iter().take(chars.len() - 1).skip(1) {
                            vec_of_data.push(*char);
                        }
                    }
                }
                vec_of_data.push(0);
            }
            ".byte" => {
                for value in datum.data_entries.iter_mut() {
                    value.token_type = Byte;
                    //this if block handles chars
                    if value.token_name.starts_with('\'') {
                        if !value.token_name.is_ascii() {
                            datum.errors.push(Error {
                                error_name: NonASCIIChar,
                                token_causing_error: value.token_name.to_string(),
                                start_end_columns: value.start_end_columns,
                                message: "".to_string(),
                            });
                        } else if value.token_name.len() != 3 || !value.token_name.ends_with('\'') {
                            datum.errors.push(Error {
                                error_name: ImproperlyFormattedChar,
                                token_causing_error: value.token_name.clone().to_string(),
                                start_end_columns: value.start_end_columns,
                                message: "".to_string(),
                            });
                        } else {
                            let mut chars = value.token_name.chars();
                            chars.next();
                            let char = chars.next().unwrap();
                            vec_of_data.push(char as u8);
                        }
                    } else {
                        //otherwise we can assume it is an int
                        let immediate_results =
                            read_immediate(&value.token_name, value.start_end_columns, 8);
                        vec_of_data.push(immediate_results.0 as u8);
                        if immediate_results.1.is_some() {
                            datum.errors.push(immediate_results.1.unwrap());
                        }
                    }
                }
            }
            ".double" => {
                //pushes the given 64 bit float values
                for value in datum.data_entries.iter_mut() {
                    value.token_type = Float;
                    let parse_results = value.token_name.parse::<f64>();
                    match parse_results {
                        Ok(..) => {
                            let float_bits = parse_results.unwrap().to_bits();
                            vec_of_data.push((float_bits >> 56) as u8);
                            vec_of_data.push((float_bits >> 48) as u8);
                            vec_of_data.push((float_bits >> 40) as u8);
                            vec_of_data.push((float_bits >> 32) as u8);
                            vec_of_data.push((float_bits >> 24) as u8);
                            vec_of_data.push((float_bits >> 16) as u8);
                            vec_of_data.push((float_bits >> 8) as u8);
                            vec_of_data.push(float_bits as u8);
                        }
                        Err(_) => datum.errors.push(Error {
                            error_name: NonFloatImmediate,
                            token_causing_error: value.token_name.to_string(),
                            start_end_columns: value.start_end_columns,
                            message: "".to_string(),
                        }),
                    }
                }
            }
            ".float" => {
                //pushes the given 32 bit float values
                for value in datum.data_entries.iter_mut() {
                    value.token_type = Float;
                    let parse_results = value.token_name.parse::<f32>();
                    match parse_results {
                        Ok(..) => {
                            let float_bits = parse_results.unwrap().to_bits();
                            vec_of_data.push((float_bits >> 24) as u8);
                            vec_of_data.push((float_bits >> 16) as u8);
                            vec_of_data.push((float_bits >> 8) as u8);
                            vec_of_data.push(float_bits as u8);
                        }
                        Err(_) => datum.errors.push(Error {
                            error_name: NonFloatImmediate,
                            token_causing_error: value.token_name.to_string(),
                            start_end_columns: value.start_end_columns,
                            message: "".to_string(),
                        }),
                    }
                }
            }
            ".half" => {
                //half words are 16 bits each. The vec is of u32s so 2 half words are in each u32
                for value in datum.data_entries.iter_mut() {
                    value.token_type = Half;
                    let immediate_results =
                        read_immediate(&value.token_name, value.start_end_columns, 16);

                    vec_of_data.push((immediate_results.0 >> 8) as u8);
                    vec_of_data.push(immediate_results.0 as u8);

                    if immediate_results.1.is_some() {
                        datum.errors.push(immediate_results.1.unwrap());
                    }
                }
            }
            ".space" => {
                //pushes specified number of empty bytes
                for value in datum.data_entries.iter_mut() {
                    value.token_type = Space;
                    let immediate_results =
                        read_immediate(&value.token_name, value.start_end_columns, 32);

                    for _i in 0..immediate_results.0 {
                        vec_of_data.push(0);
                    }

                    if immediate_results.1.is_some() {
                        datum.errors.push(immediate_results.1.unwrap());
                    }
                }
            }
            ".word" => {
                for value in datum.data_entries.iter_mut() {
                    value.token_type = Word;
                    let immediate_results =
                        read_immediate(&value.token_name, value.start_end_columns, 32);
                    if immediate_results.1.is_some() {
                        datum.errors.push(immediate_results.1.unwrap());
                    }

                    //push all four bytes of the word to the vector
                    vec_of_data.push((immediate_results.0 >> 24) as u8);
                    vec_of_data.push((immediate_results.0 >> 16) as u8);
                    vec_of_data.push((immediate_results.0 >> 8) as u8);
                    vec_of_data.push(immediate_results.0 as u8);
                }
            }
            "" => {
                //if the user only put a label on a line in .data and nothing else, it is defaulted to an empty word
                vec_of_data.push(0);
                vec_of_data.push(0);
                vec_of_data.push(0);
                vec_of_data.push(0);
            }
            _ => datum.errors.push(Error {
                error_name: UnrecognizedDataType,
                token_causing_error: datum.data_type.token_name.to_string(),
                start_end_columns: datum.data_type.start_end_columns,
                message: "".to_string(),
            }),
        }
    }
    vec_of_data
}
