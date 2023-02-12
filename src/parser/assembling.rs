use crate::parser::parser_assembler_main::append_binary;
use crate::parser::parser_structs_and_enums::instruction_tokenization::ErrorType::{
    ImmediateOutOfBounds, IncorrectNumberOfOperands, IncorrectRegisterType, InvalidMemorySyntax,
    LabelNotFound, NonIntImmediate, UnrecognizedFPRegister, UnrecognizedGPRegister,
};
use crate::parser::parser_structs_and_enums::instruction_tokenization::OperandType::{
    Immediate, LabelAbsolute, LabelRelative, MemoryAddress, RegisterFP, RegisterGP,
};
use crate::parser::parser_structs_and_enums::instruction_tokenization::RegisterType::{
    FloatingPoint, GeneralPurpose,
};
use crate::parser::parser_structs_and_enums::instruction_tokenization::TokenType::{Byte, Half, Space, Word};
use crate::parser::parser_structs_and_enums::instruction_tokenization::{
    Data, Error, Instruction, OperandType, RegisterType, TokenType,
};
use std::collections::HashMap;

///This function takes an instruction whose operands it is supposed to read, the order of expected operand types and then
///the order these operands should be concatenated onto the binary representation of the string
///the function returns the instruction it was given with any errors and the binary of the operands added on.
pub fn read_operands(
    instruction: &mut Instruction,
    expected_operands: Vec<OperandType>,
    concat_order: Vec<usize>,
    labels: Option<HashMap<String, u32>>,
) -> &mut Instruction {
    //if the number of operands in the instruction does not match the expected number, there is an error
    if instruction.operands.len() != expected_operands.len() {
        instruction.errors.push(Error {
            error_name: IncorrectNumberOfOperands,
            operand_number: None,
        });
        return instruction;
    }
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
                    i as i32,
                    GeneralPurpose,
                );

                binary_representation.push(register_results.0);
                if register_results.1.is_some() {
                    instruction.errors.push(register_results.1.unwrap());
                }
            }
            Immediate => {
                instruction.operands[i].token_type = TokenType::Immediate;
                bit_lengths.push(16);

                let immediate_results =
                    read_immediate(&instruction.operands[i].token_name, i as i32, 16);

                binary_representation.push(immediate_results.0);
                if immediate_results.1.is_some() {
                    instruction.errors.push(immediate_results.1.unwrap());
                }
            }
            MemoryAddress => {
                instruction.operands[i].token_type = TokenType::MemoryAddress;

                bit_lengths.push(16);
                bit_lengths.push(5);
                //memory address works a bit differently because it really amounts to two operands: the offset and base
                //meaning there are two values to push and the possibility of errors on both operands
                let memory_results =
                    read_memory_address(&instruction.operands[i].token_name, i as i32);

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
                let register_results =
                    read_register(&instruction.operands[i].token_name, i as i32, FloatingPoint);

                binary_representation.push(register_results.0);
                if register_results.1.is_some() {
                    instruction.errors.push(register_results.1.unwrap());
                }
            }
            LabelAbsolute => {
                instruction.operands[i].token_type = TokenType::LabelOperand;

                bit_lengths.push(26);
                let label_absolute_results = read_label_absolute(
                    &instruction.operands[i].token_name,
                    i as i32,
                    labels.clone().unwrap(),
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
                    i as i32,
                    instruction.instruction_number,
                    labels.clone().unwrap(),
                );
                binary_representation.push(label_relative_results.0);
                if label_relative_results.1.is_some() {
                    instruction.errors.push(label_relative_results.1.unwrap());
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

///Returns distance to an address of a labeled instruction relative to the instruction after the current instruction.
pub fn read_label_relative(
    given_label: &str,
    operand_number: i32,
    current_instruction_number: u32,
    labels: HashMap<String, u32>,
) -> (u32, Option<Error>) {
    let result = labels.get(given_label);

    if result.is_none() {
        return (
            0,
            Some(Error {
                error_name: LabelNotFound,
                operand_number: Some(operand_number as u8),
            }),
        );
    }

    let offset = *result.unwrap() as i32 - (current_instruction_number as i32 + 1);
    (offset as u32, None)
}

///Takes a string and returns the address of the matching label in memory. If there is no match, an error is returned
pub fn read_label_absolute(
    given_label: &str,
    operand_number: i32,
    labels: HashMap<String, u32>,
) -> (u32, Option<Error>) {
    let result = labels.get(given_label);
    if result.is_none() {
        return (
            0,
            Some(Error {
                error_name: LabelNotFound,
                operand_number: Some(operand_number as u8),
            }),
        );
    }
    (*result.unwrap(), None)
}

///This function takes in a memory address and token number and returns the binary for the offset value, base register value, and any errors
pub fn read_memory_address(
    orig_string: &str,
    operand_number: i32,
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
                operand_number: Some(operand_number as u8),
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
                operand_number: Some(operand_number as u8),
            }]),
        );
    }

    //removes the open and close parentheses characters and then turns it into a string
    base = base[1..base.len() - 1].to_owned();
    let mut cleaned_base: String = base.into_iter().collect();
    cleaned_base = cleaned_base.to_string();

    //offset is an immediate while base is a register so the read functions for those operands
    //will confirm they are properly formatted
    let immediate_results = read_immediate(offset_str, operand_number, 16);
    let register_results = read_register(&cleaned_base, operand_number, GeneralPurpose);

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
    (immediate_results.0, register_results.0, None)
}

//this function takes the string representation of the binary of the instruction and converts it into an int
pub fn _convert_to_u32(binary_as_string: String) -> u32 {
    let mut instruction_integer: u32 = 0;

    //converts instruction from string representation of binary to the unsigned 32 bit integer representation of it.
    for (i, char) in binary_as_string.chars().rev().enumerate() {
        let bit = char as u32 - '0' as u32;
        let exponential_multiplier = 2_u32.pow(i as u32);
        instruction_integer += bit * exponential_multiplier;
    }

    instruction_integer
}

///read_register takes the string of the register name, the token number the register is from the corresponding instruction
///and the expected register type. It calls the corresponding functions holding the match cases for the different register types.
pub fn read_register(
    register: &str,
    operand_number: i32,
    register_type: RegisterType,
) -> (u32, Option<Error>) {
    if register_type == GeneralPurpose {
        //this section is for matching general purpose registers
        let general_result = match_gp_register(register);
        if let Some(..) = general_result {
            (general_result.unwrap(), None)
        } else if match_fp_register(register).is_some() {
            (
                0,
                Some(Error {
                    error_name: IncorrectRegisterType,
                    operand_number: Some(operand_number as u8),
                }),
            )
        } else {
            (
                0,
                Some(Error {
                    error_name: UnrecognizedGPRegister,
                    operand_number: Some(operand_number as u8),
                }),
            )
        }
    } else {
        //this section is for matching floating point registers
        let floating_result = match_fp_register(register);
        if let Some(..) = floating_result {
            (floating_result.unwrap(), None)
        } else if match_gp_register(register).is_some() {
            (
                0,
                Some(Error {
                    error_name: IncorrectRegisterType,
                    operand_number: Some(operand_number as u8),
                }),
            )
        } else {
            (
                0,
                Some(Error {
                    error_name: UnrecognizedFPRegister,
                    operand_number: Some(operand_number as u8),
                }),
            )
        }
    }
}

///This function takes a register string as an argument and returns the string of the binary of the matching
///general register or none if there is not one that matches.
pub fn match_gp_register(register: &str) -> Option<u32> {
    match register {
        "$zero" | "r0" => Some(0b00000), //0
        "$at" | "r1" => Some(0b00001),   //1

        "$v0" | "r2" => Some(0b00010), //2
        "$v1" | "r3" => Some(0b00011), //3

        "$a0" | "r4" => Some(0b00100), //4
        "$a1" | "r5" => Some(0b00101), //5
        "$a2" | "r6" => Some(0b00110), //6
        "$a3" | "r7" => Some(0b00111), //7

        "$t0" | "r8" => Some(0b01000),  //8
        "$t1" | "r9" => Some(0b01001),  //9
        "$t2" | "r10" => Some(0b01010), //10
        "$t3" | "r11" => Some(0b01011), //11
        "$t4" | "r12" => Some(0b01100), //12
        "$t5" | "r13" => Some(0b01101), //13
        "$t6" | "r14" => Some(0b01110), //14
        "$t7" | "r15" => Some(0b01111), //15

        "$s0" | "r16" => Some(0b10000), //16
        "$s1" | "r17" => Some(0b10001), //17
        "$s2" | "r18" => Some(0b10010), //18
        "$s3" | "r19" => Some(0b10011), //19
        "$s4" | "r20" => Some(0b10100), //20
        "$s5" | "r21" => Some(0b10101), //21
        "$s6" | "r22" => Some(0b10110), //22
        "$s7" | "r23" => Some(0b10111), //23

        "$t8" | "r24" => Some(0b11000), //24
        "$t9" | "r25" => Some(0b11001), //25

        "$k0" | "r26" => Some(0b11010), //26
        "$k1" | "r27" => Some(0b11011), //27

        "$gp" | "r28" => Some(0b11100), //28
        "$sp" | "r29" => Some(0b11101), //29
        "$fp" | "r30" => Some(0b11110), //30
        "$ra" | "r31" => Some(0b11111), //31
        _ => None,
    }
}

///This function takes a register string as an argument and returns the string of the binary of the matching
///floating point register or none if there is not one that matches.
pub fn match_fp_register(register: &str) -> Option<u32> {
    match register {
        "$f0" => Some(0b00000),
        "$f1" => Some(0b00001),
        "$f2" => Some(0b00010),
        "$f3" => Some(0b00011),
        "$f4" => Some(0b00100),
        "$f5" => Some(0b00101),
        "$f6" => Some(0b00110),
        "$f7" => Some(0b00111),
        "$f8" => Some(0b01000),
        "$f9" => Some(0b01001),
        "$f10" => Some(0b01010),
        "$f11" => Some(0b01011),
        "$f12" => Some(0b01100),
        "$f13" => Some(0b01101),
        "$f14" => Some(0b01110),
        "$f15" => Some(0b01111),
        "$f16" => Some(0b10000),
        "$f17" => Some(0b10001),
        "$f18" => Some(0b10010),
        "$f19" => Some(0b10011),
        "$f20" => Some(0b10100),
        "$f21" => Some(0b10101),
        "$f22" => Some(0b10110),
        "$f23" => Some(0b10111),
        "$f24" => Some(0b11000),
        "$f25" => Some(0b11001),
        "$f26" => Some(0b11010),
        "$f27" => Some(0b11011),
        "$f28" => Some(0b11100),
        "$f29" => Some(0b11101),
        "$f30" => Some(0b11110),
        "$f31" => Some(0b11111),
        _ => None,
    }
}

///This function takes a string representation of an immediate value and the number of bits available to represent it
/// and attempts to translate it to an actual integer. If the value cannot be cast to int or is too big to be represented
/// by the available bits, an error is returned.
pub fn read_immediate(
    given_text: &str,
    operand_number: i32,
    num_bits: u32,
) -> (u32, Option<Error>) {
    //attempts to cast the text into a large int
    let parse_results = given_text.parse::<i32>();

    //if there was an error typecasting, the function returns with an error to add to the instruction or data
    if parse_results.is_err() {
        return (
            0,
            Some(Error {
                error_name: NonIntImmediate,
                operand_number: Some(operand_number as u8),
            }),
        );
    }

    let int_representation: i64 = parse_results.unwrap() as i64;

    //finds the max and min values of a signed integer with specified number of bits
    let max_value = i64::pow(2, num_bits);
    let min_value = (-max_value) - 1;
    //if the parsed value is out of possible bounds, an error is returned to add to the instruction
    if int_representation > max_value || int_representation < min_value {
        return (
            0,
            Some(Error {
                error_name: ImmediateOutOfBounds,
                operand_number: Some(operand_number as u8),
            }),
        );
    }

    (int_representation as u32, None)
}

///Takes the data list and finds the actual values for each data entry that will be put into memory
pub fn assemble_data_binary(data_list: &mut [Data]) -> Vec<u8> {
    let mut vec_of_data: Vec<u8> = Vec::new();
    for data_entry in data_list.iter_mut() {
        match &*data_entry.data_type.token_name {
            ".ascii" => {}
            ".asciiz" => {}
            ".byte" => {
                for (i, value) in data_entry.data_entries_and_values.iter_mut().enumerate() {
                    value.0.token_type = Byte;

                    if value.0.token_name.starts_with('\''){
                        //todo implement reading an ascii char
                    }else{
                        let immediate_results = read_immediate(&value.0.token_name, i as i32, 8);
                        value.1 = immediate_results.0;
                        vec_of_data.push(immediate_results.0 as u8);
                        if immediate_results.1.is_some(){
                            data_entry.errors.push(immediate_results.1.unwrap());
                        }
                    }
                }
            }
            ".double" => {}
            ".float" => {}
            ".half" => {//half words are 16 bits each. The vec is of u32s so 2 half words are in each u32
                for (i, value) in data_entry.data_entries_and_values.iter_mut().enumerate() {
                    value.0.token_type = Half;
                    let immediate_results = read_immediate(&value.0.token_name, i as i32, 16);
                    value.1 = immediate_results.0;

                    vec_of_data.push((value.1 >> 8) as u8);
                    vec_of_data.push(value.1 as u8);

                    if immediate_results.1.is_some() {
                        data_entry.errors.push(immediate_results.1.unwrap());
                    }
                }
            }
            ".space" => {
                for (i, value) in data_entry.data_entries_and_values.iter_mut().enumerate() {
                    value.0.token_type = Space;
                    let immediate_results = read_immediate(&value.0.token_name, i as i32, 32);
                    value.1 = immediate_results.0;

                    for _i in 0..immediate_results.0{
                        vec_of_data.push(0);
                    }

                    if immediate_results.1.is_some(){
                        data_entry.errors.push(immediate_results.1.unwrap());
                    }
                }
            }
            ".word" => {
                for (i, value) in data_entry.data_entries_and_values.iter_mut().enumerate() {
                    value.0.token_type = Word;
                    let immediate_results = read_immediate(&value.0.token_name, i as i32, 32);
                    value.1 = immediate_results.0;
                    if immediate_results.1.is_some() {
                        data_entry.errors.push(immediate_results.1.unwrap());
                    }

                    //push all four bytes of the word to the vector
                    vec_of_data.push((value.1 >> 24) as u8);
                    vec_of_data.push((value.1 >> 16) as u8);
                    vec_of_data.push((value.1 >> 8) as u8);
                    vec_of_data.push(value.1 as u8);
                }
            }

            _ => {}
        }
    }
    vec_of_data
}
