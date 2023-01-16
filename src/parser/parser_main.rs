use crate::parser::parser_instruction_tokenization::instruction_tokenization::ErrorType::*;
use crate::parser::parser_instruction_tokenization::instruction_tokenization::OperandType::*;
use crate::parser::parser_instruction_tokenization::instruction_tokenization::RegisterType::{
    FloatingPoint, GeneralPurpose,
};
use crate::parser::parser_instruction_tokenization::instruction_tokenization::*;
use crate::parser::parser_preprocessing::*;

pub fn parser(mut file_string: String) -> Vec<Instruction> {
    file_string = file_string.to_ascii_lowercase();
    file_string = string_cleaning(file_string);

    let init_instruction_list = create_vector_of_instructions(file_string);
    let mut instruction_list: Vec<Instruction> = vec![];
    for mut instruction in init_instruction_list {
        instruction = confirm_commas_in_instruction(instruction);
        instruction = read_instruction(instruction);
        instruction_list.push(instruction);
    }

    instruction_list
}

//read_instruction takes an instruction and builds the binary and int representation of the instruction
pub fn read_instruction(mut instruction: Instruction) -> Instruction {
    //this match case is the heart of the parser and figures out which instruction type it is
    //then it can call the proper functions for that specific instruction
    match &*instruction.tokens[0] {
        "add" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b000000, 6);

            read_operands(
                &mut instruction,
                vec![RegisterGP, RegisterGP, RegisterGP],
                vec![3, 1, 2],
            );

            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b00000, 5);
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b100000, 6);
        }
        "sub" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b00000, 5);

            read_operands(
                &mut instruction,
                vec![RegisterGP, RegisterGP, RegisterGP],
                vec![3, 1, 2],
            );

            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b00000, 5);
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b100010, 5);
        }
        "mul" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b00000, 5);

            read_operands(
                &mut instruction,
                vec![RegisterGP, RegisterGP, RegisterGP],
                vec![3, 1, 2],
            );

            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b00000, 5);
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b000010, 6);

        }
        "div" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b00000, 5);


            read_operands(&mut instruction, vec![RegisterGP, RegisterGP], vec![1, 2]);

            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b0000000000, 10);
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b011010, 6);

        }
        "lw" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b100011, 6);

            read_operands(
                &mut instruction,
                vec![RegisterGP, MemoryAddress],
                vec![3, 1, 2],
            );
        }
        "sw" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b101011, 6);

            read_operands(
                &mut instruction,
                vec![RegisterGP, MemoryAddress],
                vec![3, 1, 2],
            );
        }
        "lui" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b001111, 6);
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b00000, 5);

            read_operands(&mut instruction, vec![RegisterGP, Immediate], vec![1, 2]);
        }
        "andi" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b001100, 6);

            read_operands(
                &mut instruction,
                vec![RegisterGP, RegisterGP, Immediate],
                vec![1, 2, 3],
            );
        }
        "ori" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b001101, 6);

            read_operands(
                &mut instruction,
                vec![RegisterGP, RegisterGP, Immediate],
                vec![2, 1, 3],
            );
        }
        "addi" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b001000, 6);

            read_operands(
                &mut instruction,
                vec![RegisterGP, RegisterGP, Immediate],
                vec![2, 1, 3],
            );
        }
        "dadd" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b000000, 6);

            read_operands(
                &mut instruction,
                vec![RegisterGP, RegisterGP, RegisterGP],
                vec![2, 3, 1],
            );

            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b00000, 5);
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b101100, 6);

        }
        "dsub" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b000000, 6);

            read_operands(
                &mut instruction,
                vec![RegisterGP, RegisterGP, RegisterGP],
                vec![2, 3, 1],
            );

            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b00000, 5);
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b101110, 6);

        }
        "dmul" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b000000, 6);

            read_operands(
                &mut instruction,
                vec![RegisterGP, RegisterGP, RegisterGP],
                vec![3, 1, 2],
            );

            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b00010, 5);
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b011100, 6);

        }
        "ddiv" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b000000, 6);

            read_operands(&mut instruction, vec![RegisterGP, RegisterGP], vec![1, 2]);

            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b0000000000, 10);
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b011110, 6);

        }
        "or" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b000000, 6);

            read_operands(
                &mut instruction,
                vec![RegisterGP, RegisterGP, RegisterGP],
                vec![3, 1, 2],
            );

            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b00000, 5);
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b100101, 6);

        }
        "and" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b000000, 6);


            read_operands(
                &mut instruction,
                vec![RegisterGP, RegisterGP, RegisterGP],
                vec![3, 1, 2],
            );

            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b00000, 5);
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 100100, 6);

        }
        "add.s" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b010001, 6); //cop1
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 10000, 5); //fmt: s (16)

            read_operands(
                &mut instruction,
                vec![RegisterFP, RegisterFP, RegisterFP],
                vec![3, 2, 1],
            );

            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b000000, 6); //add

        }
        "add.d" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b010001, 6); //cop1
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b10001, 5); //fmt: d (17)


            read_operands(
                &mut instruction,
                vec![RegisterFP, RegisterFP, RegisterFP],
                vec![3, 2, 1],
            );

            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b000000, 6); //add

        }
        "sub.s" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b010001, 6);//cop1
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b10000, 5);//fmt: s (16)

            read_operands(
                &mut instruction,
                vec![RegisterFP, RegisterFP, RegisterFP],
                vec![3, 2, 1],
            );

            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b000001, 6);//sub

        }
        "sub.d" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b010001, 6);//cop1
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b10001, 5);//fmt: d (17)

            read_operands(
                &mut instruction,
                vec![RegisterFP, RegisterFP, RegisterFP],
                vec![3, 2, 1],
            );

            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b000001, 6);

        }
        "mul.s" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b010001, 6);//cop1
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b10000, 5);//fmt: s (16)

            read_operands(
                &mut instruction,
                vec![RegisterFP, RegisterFP, RegisterFP],
                vec![3, 2, 1],
            );

            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b000010, 6);//mul

        }
        "mul.d" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b010001, 6);//cop1
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b10001, 5);//fmt: d (17)


            read_operands(
                &mut instruction,
                vec![RegisterFP, RegisterFP, RegisterFP],
                vec![3, 2, 1],
            );

            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b000010, 6);//mul

        }
        "div.s" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b010001, 6); //cop1
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b10000, 5); //fmt: s (16)

            read_operands(
                &mut instruction,
                vec![RegisterFP, RegisterFP, RegisterFP],
                vec![3, 2, 1],
            );

            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b000011, 6);//div

        }
        "div.d" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b010001, 6); //cop1
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b10001, 5);//fmt: d (17)

            read_operands(
                &mut instruction,
                vec![RegisterFP, RegisterFP, RegisterFP],
                vec![3, 2, 1],
            );

            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b000011, 6);//div

        }
        "dahi" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b000001, 6);//regimm

            read_operands(&mut instruction, vec![RegisterGP, Immediate], vec![1, 2]);

            instruction.int_representation = place_binary_in_middle_of_another(instruction.int_representation, 0b00110, 5, 11);
        }
        "dati" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b000001, 6);//regimm

            read_operands(&mut instruction, vec![RegisterGP, Immediate], vec![1, 2]);

            instruction.int_representation = place_binary_in_middle_of_another(instruction.int_representation, 0b11110, 5, 11);
        }
        "daddiu" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b011001, 6);//daddiu


            read_operands(
                &mut instruction,
                vec![RegisterGP, RegisterGP, Immediate],
                vec![2, 1, 3],
            );
        }
        "slt" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b000000, 6);//special

            read_operands(
                &mut instruction,
                vec![RegisterGP, RegisterGP, RegisterGP],
                vec![3, 1, 2],
            );

            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b00000, 5);//0
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b101010, 6);//slt

        }
        "sltu" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b000000, 6);//special

            read_operands(
                &mut instruction,
                vec![RegisterGP, RegisterGP, RegisterGP],
                vec![3, 1, 2],
            );

            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b00000, 5);//0
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b101011, 6);//sltu
        }
        "swc1" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b111001, 6);//swc1

            read_operands(
                &mut instruction,
                vec![RegisterFP, MemoryAddress],
                vec![3, 1, 2],
            );
        }
        "lwc1" => {
            instruction.int_representation =
                append_instruction_component(instruction.int_representation, 0b110001, 6);//lwc1

            read_operands(
                &mut instruction,
                vec![RegisterFP, MemoryAddress],
                vec![3, 1, 2],
            );
        }
        _ => instruction.errors.push(Error {
            error_name: UnrecognizedInstruction,
            token_number_giving_error: 0,
        }),
    }

    instruction
}
///This function takes two numbers and inserts the binary of the second at a given index in the binary of the first.
///All binary values at and past the insertion index of the original string will be moved to the end of the resultant string.
pub fn place_binary_in_middle_of_another(
    wrapper: u32,
    middle: u32,
    middle_length: u32,
    insertion_index: u8,
) -> u32{
    let mut new_binary = wrapper >> insertion_index;
    new_binary = new_binary << middle_length;
    new_binary = new_binary | middle;
    new_binary = new_binary << insertion_index;
    let mut end = wrapper << (32  - insertion_index);
    end = end >> (32 - insertion_index);
    new_binary = new_binary | end;
    new_binary
}


///This function takes an instruction whose operands it is supposed to read, the order of expected operand types and then
///the order these operands should be concatenated onto the binary representation of the string
///the function returns the instruction it was given with any errors and the binary of the operands added on
fn read_operands(
    instruction: &mut Instruction,
    expected_operands: Vec<OperandType>,
    concat_order: Vec<i32>,
) -> &mut Instruction {
    //the number of tokens associated with the instruction should be the number of operands plus 1 for the instruction name. If not, there's an error.
    if instruction.tokens.len() != expected_operands.len() + 1 {
        instruction.errors.push(Error {
            error_name: IncorrectNumberOfOperands,
            token_number_giving_error: 0,
        });
        return instruction;
    }
    //operands aren't represented in the binary in the order they're read so the vec<u32> allows us to concatenate them in the proper order after they're all read.
    let mut binary_representation: Vec<u32> = Vec::new();
    let mut bit_lengths: Vec<u8> = Vec::new();
    //goes through once for each expected operand
    for (i, operand_type) in expected_operands.iter().enumerate() {
        //match case calls the proper functions based on the expected operand type. The data returned from these functions is always
        //the binary of the read operand and the option for any errors encountered while reading the operand. If there were no errors,
        //the binary is pushed to the string representations vec. Otherwise, the errors are pushed to the instruction.errors vec.
        match operand_type {
            RegisterGP => {
                bit_lengths.push(5);
                let register_results =
                    read_register(&instruction.tokens[i + 1], i as i32, GeneralPurpose);

                match register_results.1 {
                    None => binary_representation.push(register_results.0),
                    Some(error) => instruction.errors.push(error),
                }
            }
            Immediate => {
                bit_lengths.push(16);
                let immediate_results = read_immediate(&instruction.tokens[i + 1], i as i32, 16);

                match immediate_results.1 {
                    None => {
                        binary_representation.push(immediate_results.0);
                    }
                    Some(error) => instruction.errors.push(error),
                }
            }
            MemoryAddress => {
                bit_lengths.push(16);
                bit_lengths.push(5);
                //memory address works a bit differently because it really amounts to two operands: the offset and base
                //meaning there are two values to push and the possibility of errors on both operands
                let memory_results = read_memory_address(&instruction.tokens[i + 1], i as i32);

                match memory_results.2 {
                    None => {
                        binary_representation.push(memory_results.0);
                        binary_representation.push(memory_results.1);
                    }
                    Some(..) => {
                        for error in memory_results.2.unwrap() {
                            instruction.errors.push(error);
                        }
                    }
                }
            }
            RegisterFP => {
                bit_lengths.push(5);
                let register_results =
                    read_register(&instruction.tokens[i + 1], i as i32, FloatingPoint);

                match register_results.1 {
                    None => binary_representation.push(register_results.0),
                    Some(error) => instruction.errors.push(error),
                }
            } //Label => {}
        }
    }
    //if no errors are on the list by this point, we can safely push the operands' binaries onto the instruction representation
    if instruction.errors.is_empty() {
        for (element, i) in concat_order.iter().enumerate() {
            instruction.int_representation = append_instruction_component(
                instruction.int_representation,
                binary_representation[element as usize - 1],
                bit_lengths[*i as usize] as u8,
            );
        }
    }

    instruction
}

//function takes in a memory address and token number and returns the binary for the offset value, base register value, and any errors
pub(crate) fn read_memory_address(
    orig_string: &str,
    token_number: i32,
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
                token_number_giving_error: token_number as u8,
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
                token_number_giving_error: token_number as u8,
            }]),
        );
    }

    //removes the open and close parentheses characters and then turns it into a string
    base = base[1..base.len() - 1].to_owned();
    let mut cleaned_base: String = base.into_iter().collect();
    cleaned_base = cleaned_base.to_string();

    //offset is an immediate while base is a register so the read functions for those operands
    //will confirm they are properly formatted
    let immediate_results = read_immediate(offset_str, token_number, 16);
    let register_results = read_register(&cleaned_base, token_number, GeneralPurpose);

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
pub(crate) fn convert_to_u32(binary_as_string: String) -> u32 {
    let mut instruction_integer: u32 = 0;

    //converts instruction from string representation of binary to the unsigned 32 bit integer representation of it.
    for (i, char) in binary_as_string.chars().rev().enumerate() {
        let bit = char as u32 - '0' as u32;
        let exponential_multiplier = 2_u32.pow(i as u32);
        instruction_integer += bit * exponential_multiplier;
    }

    instruction_integer
}

//read_register takes the string of the register name, the token number the register is from the corresponding instruction
//and the expected register type. It calls the corresponding functions holding the match cases for the different register types.
pub(crate) fn read_register(
    register: &str,
    token_number: i32,
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
                    token_number_giving_error: token_number as u8,
                }),
            )
        } else {
            (
                0,
                Some(Error {
                    error_name: UnrecognizedGPRegister,
                    token_number_giving_error: token_number as u8,
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
                    token_number_giving_error: token_number as u8,
                }),
            )
        } else {
            (
                0,
                Some(Error {
                    error_name: UnrecognizedFPRegister,
                    token_number_giving_error: token_number as u8,
                }),
            )
        }
    }
}

//This function takes a register string as an argument and returns the string of the binary of the matching
//general register or none if there is not one that matches.
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

//This function takes a register string as an argument and returns the string of the binary of the matching
//floating point register or none if there is not one that matches.
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
pub fn read_immediate(given_text: &str, token_number: i32, num_bits: u32) -> (u32, Option<Error>) {
        //attempts to cast the text into a large int
     let parse_results = given_text.parse::<i32>();

    //if there was an error typecasting, the function returns with an error to add to the instruction
    if parse_results.is_err() {
        return (
            0,
            Some(Error {
                error_name: NonIntImmediate,
                token_number_giving_error: token_number as u8,
            }),
        );
    }

    let int_representation: i32 = parse_results.unwrap();

    //finds the max and min values of a signed integer with specified number of bits
    let max_value = i32::pow(2, num_bits);
    let min_value = (-max_value) - 1;
    //if the parsed value is out of possible bounds, an error is returned to add to the instruction
    if int_representation > max_value || int_representation < min_value {
        return (
            0,
            Some(Error {
                error_name: ImmediateOutOfBounds,
                token_number_giving_error: token_number as u8,
            }),
        );
    }

    (int_representation as u32, None)
}

pub fn append_instruction_component(mut first: u32, second: u32, shift_amount: u8) -> u32 {
    first = first << shift_amount;
    first = first | second;
    first
}
