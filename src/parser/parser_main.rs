use crate::parser::parser_instruction_tokenization::instruction_tokenization::ErrorType::*;
use crate::parser::parser_instruction_tokenization::instruction_tokenization::OperandType::*;
use crate::parser::parser_instruction_tokenization::instruction_tokenization::*;
use crate::parser::parser_instruction_tokenization::instruction_tokenization::RegisterType::GeneralPurpose;
use crate::parser::parser_preprocessing::*;

pub fn parser(mut file_string: String) -> Vec<Instruction> {
    file_string = file_string.to_ascii_lowercase();
    file_string = remove_extra_spaces(file_string);

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
            instruction.binary_representation.push_str("000000");

            read_operands(
                &mut instruction,
                vec![RegisterGp, RegisterGp, RegisterGp],
                vec![3, 1, 2],
            );

            instruction.binary_representation.push_str("00000");
            instruction.binary_representation.push_str("100000");
        }
        "sub" => {
            instruction.binary_representation.push_str("00000");

            read_operands(
                &mut instruction,
                vec![RegisterGp, RegisterGp, RegisterGp],
                vec![3, 1, 2],
            );

            instruction.binary_representation.push_str("00000");
            instruction.binary_representation.push_str("100010");
        }
        "mul" => {
            instruction.binary_representation.push_str("00000");

            read_operands(
                &mut instruction,
                vec![RegisterGp, RegisterGp, RegisterGp],
                vec![3, 1, 2],
            );

            instruction.binary_representation.push_str("00000");
            instruction.binary_representation.push_str("000010");
        }
        "div" => {
            instruction.binary_representation.push_str("00000");

            read_operands(&mut instruction, vec![RegisterGp, RegisterGp], vec![1, 2]);

            instruction.binary_representation.push_str("0000000000");
            instruction.binary_representation.push_str("011010");
        }
        "addi" => {
            instruction.binary_representation.push_str("001000");

            read_operands(
                &mut instruction,
                vec![RegisterGp, RegisterGp, Immediate],
                vec![2, 1, 3],
            );
        }
        "ori" => {
            instruction.binary_representation.push_str("001101");

            read_operands(
                &mut instruction,
                vec![RegisterGp, RegisterGp, Immediate],
                vec![2, 1, 3],
            );
        }
        "lw" => {
            instruction.binary_representation.push_str("100011");

            read_operands(
                &mut instruction,
                vec![RegisterGp, MemoryAddress],
                vec![3, 1, 2],
            );
        }
        "sw" => {
            instruction.binary_representation.push_str("101011");

            read_operands(
                &mut instruction,
                vec![RegisterGp, MemoryAddress],
                vec![3, 1, 2],
            );
        }
        "lui" => {
            instruction.binary_representation.push_str("001111");

            instruction.binary_representation.push_str("00000");

            read_operands(&mut instruction, vec![RegisterGp, Immediate], vec![1, 2]);
        }

        _ => {}
    }
    //the binary of the instruction is converted to u32 here as well
    instruction.int_representation = convert_to_u32(instruction.binary_representation.clone());

    instruction
}

//this function takes an instruction whose operands it is supposed to read, the order of expected operand types and then
//the order these operands should be concatenated onto the binary representation of the string
//the function returns the instruction it was given with any errors and the binary of the operands added on
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
    //operands aren't represented in the binary in the order they're read so the vec<string> allows us to concatenate them in the proper order after they're all read.
    let mut string_representations: Vec<String> = Vec::new();
    //goes through once for each expected operand
    for (i, operand_type) in expected_operands.iter().enumerate() {
        //match case calls the proper functions based on the expected operand type. The data returned from these functions is always
        //the binary of the read operand and the option for any errors encountered while reading the operand. If there were no errors,
        //the binary is pushed to the string representations vec. Otherwise, the errors are pushed to the instruction.errors vec.
        match operand_type {
            RegisterGp => {
                let register_results = read_register(&instruction.tokens[i + 1], i as i32, GeneralPurpose);

                match register_results.1 {
                    None => string_representations.push(register_results.0.to_string()),
                    Some(error) => instruction.errors.push(error),
                }
            }
            Immediate => {
                let immediate_results = read_immediate(&instruction.tokens[i + 1], i as i32, 16);

                match immediate_results.1 {
                    None => {
                        string_representations.push(immediate_results.0);
                    }
                    Some(error) => instruction.errors.push(error),
                }
            }
            MemoryAddress => {
                //memory address works a bit differently because it really amounts to two operands: the offset and base
                //meaning there are two values to push and the possibility of errors on both operands
                let memory_results = read_memory_address(&instruction.tokens[i + 1], i as i32);

                match memory_results.2 {
                    None => {
                        string_representations.push(memory_results.0);
                        string_representations.push(memory_results.1);
                    }
                    Some(..) => {
                        for error in memory_results.2.unwrap() {
                            instruction.errors.push(error);
                        }
                    }
                }
            } //RegisterFP => {}
              //Label => {}
        }
    }
    //if no errors are on the list by this point, we can safely push the operands' binaries onto the instruction representation
    if instruction.errors.is_empty() {
        for element in concat_order {
            instruction
                .binary_representation
                .push_str(string_representations.get(element as usize - 1).unwrap());
        }
    }

    instruction
}

//function takes in a memory address and token number and returns the binary for the offset value, base register value, and any errors
pub(crate) fn read_memory_address(
    orig_string: &str,
    token_number: i32,
) -> (String, String, Option<Vec<Error>>) {
    //the indices of the open and close parentheses are checked.
    //If either are missing or they are in the wrong order, an error is returned
    let open_index = orig_string.find('(');
    let close_index = orig_string.find(')');
    if close_index.is_none() || open_index.is_none() || close_index < open_index {
        return (
            "".to_string(),
            "".to_string(),
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
            "".to_string(),
            "".to_string(),
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
        return ("".to_string(), "".to_string(), Some(return_errors));
    }

    //if the function reaches here and hasn't already returned, there aren't any errors
    (
        immediate_results.0,
        register_results.0.parse().unwrap(),
        None,
    )
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
pub(crate) fn read_register(register: &str, token_number: i32, register_type: RegisterType) -> (&str, Option<Error>) {

    if register_type == GeneralPurpose {
        let general_result = match_gp_register(register);
        if general_result.is_some {
            (general_result.unwrap(), None)
        } else if match_fp_register(register).is_some(){
            ("", Some(Error{
                error_name: IncorrectRegisterType,
                token_number_giving_error: token_number as u8
            }))
        } else {
            ("", Some(Error{
                error_name: UnrecognizedGPRegister,
                token_number_giving_error: token_number as u8
            }))
        }
    } else{
        let floating_result = match_fp_register(register);
        if floating_result.is_some() {
            (floating_result.unwrap(), None)
        } else if match_gp_register(register).is_some(){
            ("", Some(Error{
                error_name: IncorrectRegisterType,
                token_number_giving_error: token_number as u8
            }))
        } else {
            ("", Some(Error{
                error_name: UnrecognizedFPRegister,
                token_number_giving_error: token_number as u8
            }))
        }
    }
}

//This function takes a register string as an argument and returns the string of the binary of the matching
//general register or none if there is not one that matches.
pub fn match_gp_register(register: &str) -> Option<&str>{
    match register{
        "$zero" | "r0" => Some("00000"), //0
        "$at" | "r1" => Some("00001"),   //1

        "$v0" | "r2" => Some("00010"), //2
        "$v1" | "r3" => Some("00011"), //3

        "$a0" | "r4" => Some("00100"), //4
        "$a1" | "r5" => Some("00101"), //5
        "$a2" | "r6" => Some("00110"), //6
        "$a3" | "r7" => Some("00111"), //7

        "$t0" | "r8" => Some("01000"),  //8
        "$t1" | "r9" => Some("01001"),  //9
        "$t2" | "r10" => Some("01010"), //10
        "$t3" | "r11" => Some("01011"), //11
        "$t4" | "r12" => Some("01100"), //12
        "$t5" | "r13" => Some("01101"), //13
        "$t6" | "r14" => Some("01110"), //14
        "$t7" | "r15" => Some("01111"), //15

        "$s0" | "r16" => Some("10000"), //16
        "$s1" | "r17" => Some("10001"), //17
        "$s2" | "r18" => Some("10010"), //18
        "$s3" | "r19" => Some("10011"), //19
        "$s4" | "r20" => Some("10100"), //20
        "$s5" | "r21" => Some("10101"), //21
        "$s6" | "r22" => Some("10110"), //22
        "$s7" | "r23" => Some("10111"), //23

        "$t8" | "r24" => Some("11000"), //24
        "$t9" | "r25" => Some("11001"), //25

        "$k0" | "r26" => Some("11010"), //26
        "$k1" | "r27" => Some("11011"), //27

        "$gp" | "r28" => Some("11100"), //28
        "$sp" | "r29" => Some("11101"), //29
        "$fp" | "r30" => Some("11110"), //30
        "$ra" | "r31" => Some("11111"), //31
        _ => None
    }
}

//This function takes a register string as an argument and returns the string of the binary of the matching
//floating point register or none if there is not one that matches.
pub fn match_fp_register(register: &str) -> Option<&str>{
    match register{
        "$f0" => Some("00000"),
        "$f1" => Some("00001"),
        "$f2" => Some("00010"),
        "$f3" => Some("00011"),
        "$f4" => Some("00100"),
        "$f5" => Some("00101"),
        "$f6" => Some("00110"),
        "$f7" => Some("00111"),
        "$f8" => Some("01000"),
        "$f9" => Some("01001"),
        "$f10" => Some("01010"),
        "$f11" => Some("01011"),
        "$f12" => Some("01100"),
        "$f13" => Some("01101"),
        "$f14" => Some("01110"),
        "$f15" => Some("01111"),
        "$f16" => Some("10000"),
        "$f17" => Some("10001"),
        "$f18" => Some("10010"),
        "$f19" => Some("10011"),
        "$f20" => Some("10100"),
        "$f21" => Some("10101"),
        "$f22" => Some("10110"),
        "$f23" => Some("10111"),
        "$f24" => Some("11000"),
        "$f25" => Some("11001"),
        "$f26" => Some("11010"),
        "$f27" => Some("11011"),
        "$f28" => Some("11100"),
        "$f29" => Some("11101"),
        "$f30" => Some("11110"),
        "$f31" => Some("11111"),
        _ => None
    }
}

//takes the string representation of a integer, the token number, and the number of bits in the instruction to represent the result
//and translates that integer into a string representation of the binary value represented using num_bits
//it also recognizes errors when a given value cannot be cast to int or if the int is too big to be represented with num_bits
pub fn read_immediate(
    given_text: &str,
    token_number: i32,
    num_bits: u32,
) -> (String, Option<Error>) {
    //attempts to cast the text into a large int
    let parse_results = given_text.parse::<i128>();

    //if there was an error typecasting, the function returns with an error to add to the instruction
    if parse_results.is_err() {
        return (
            "".to_string(),
            Some(Error {
                error_name: NonIntImmediate,
                token_number_giving_error: token_number as u8,
            }),
        );
    }

    let int_representation: i32 = parse_results.unwrap() as i32;
    //finds the max and min values of a signed integer with specified number of bits
    let max_value = i32::pow(2, num_bits);
    let min_value = (-max_value) - 1;
    //if the parsed value is out of possible bounds, an error is returned to add to the instruction
    if int_representation > max_value || int_representation < min_value {
        return (
            "".to_string(),
            Some(Error {
                error_name: ImmediateOutOfBounds,
                token_number_giving_error: token_number as u8,
            }),
        );
    }

    //formats the integer representation as a 32-bit binary
    let mut binary_representation = format!("{:b}", int_representation);

    //removes any unnecessary leading 1s on a negative representation
    if int_representation < 0 && binary_representation.len() > num_bits as usize {
        let mut char_vec: Vec<char> = binary_representation.chars().collect();
        char_vec = char_vec[32 - num_bits as usize..32].to_owned();
        binary_representation = char_vec.into_iter().collect();
    }
    //adds extra leading 0s on a positive representation
    else if int_representation >= 0 && binary_representation.len() < num_bits as usize {
        let mut extra_zeroes = String::new();
        while binary_representation.len() + extra_zeroes.len() < num_bits as usize {
            extra_zeroes.push('0');
        }
        extra_zeroes.push_str(&binary_representation);
        binary_representation = extra_zeroes;
    }

    (binary_representation, None)
}
