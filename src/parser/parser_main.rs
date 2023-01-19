use crate::parser::parser_preprocessing::*;
use crate::parser::parser_structs_and_enums::instruction_tokenization::ErrorType::*;
use crate::parser::parser_structs_and_enums::instruction_tokenization::OperandType::*;
use crate::parser::parser_structs_and_enums::instruction_tokenization::RegisterType::{
    FloatingPoint, GeneralPurpose,
};
use crate::parser::parser_structs_and_enums::instruction_tokenization::*;
use std::collections::HashMap;

///Parser is the starting function of the parser / assembler process. It takes a string representation of a MIPS
/// program and builds the binary of the instructions while cataloging any errors that are found.
pub fn parser(mut file_string: String) -> Vec<Instruction> {
    file_string = file_string.to_lowercase();

    let lines = tokenize_instructions(file_string);
    let mut instruction_list: Vec<Instruction> = build_instruction_list_from_lines(lines);
    confirm_operand_commas(&mut instruction_list);
    expand_pseudo_instruction(&mut instruction_list);
    assign_instruction_numbers(&mut instruction_list);

    let labels: HashMap<String, u32> = create_label_map(instruction_list.clone());

    read_instructions(&mut instruction_list, labels);

    instruction_list
}

///This function takes an instruction with nothing filled in about it besides the tokens and the instruction number
/// and builds the binary by calling the proper functions based on a match case for the first token (the instruction name)
pub fn read_instructions(instruction_list: &mut Vec<Instruction>, labels: HashMap<String, u32>) {
    for i in 0..instruction_list.len() {
        //this match case is the heart of the parser and figures out which instruction type it is
        //then it can call the proper functions for that specific instruction
        match &*instruction_list[i].operator.token_name {
            "add" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b000000, 6);

                read_operands(
                    &mut &mut instruction_list[i],
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b00000, 5);
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b100000, 6);
            }
            "sub" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b000000, 6);

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b00000, 5);
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b100010, 6);
            }
            "mul" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b011100, 6);

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b00000, 5);
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b000010, 6);
            }
            "div" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b000000, 6);

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterGP, RegisterGP],
                    vec![1, 2],
                    None,
                );

                instruction_list[i].binary =
                    append_binary(instruction_list[i].binary, 0b0000000000, 10);
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b011010, 6);
            }
            "lw" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b100011, 6);

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterGP, MemoryAddress],
                    vec![3, 1, 2],
                    None,
                );
            }
            "sw" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b101011, 6);

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterGP, MemoryAddress],
                    vec![3, 1, 2],
                    None,
                );
            }
            "lui" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b001111, 6);
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b00000, 5);

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterGP, Immediate],
                    vec![1, 2],
                    None,
                );
            }
            "andi" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b001100, 6);

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![2, 1, 3],
                    None,
                );
            }
            "ori" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b001101, 6);

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![2, 1, 3],
                    None,
                );
            }
            "addi" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b001000, 6);

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![2, 1, 3],
                    None,
                );
            }
            "dadd" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b000000, 6);

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b00000, 5);
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b101100, 6);
            }
            "dsub" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b000000, 6);

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b00000, 5);
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b101110, 6);
            }
            "dmul" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b000000, 6);

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b00010, 5);
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b011100, 6);
            }
            "ddiv" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b000000, 6);

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterGP, RegisterGP],
                    vec![1, 2],
                    None,
                );

                instruction_list[i].binary =
                    append_binary(instruction_list[i].binary, 0b0000000000, 10);
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b011110, 6);
            }
            "or" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b000000, 6);

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b00000, 5);
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b100101, 6);
            }
            "and" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b000000, 6);

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b00000, 5);
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b100100, 6);
            }
            "add.s" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b010001, 6); //cop1
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b10000, 5); //fmt: s (16)

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![3, 2, 1],
                    None,
                );

                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b000000, 6);
                //add
            }
            "add.d" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b010001, 6); //cop1
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b10001, 5); //fmt: d (17)

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![3, 2, 1],
                    None,
                );

                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b000000, 6);
                //add
            }
            "sub.s" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b010001, 6); //cop1
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b10000, 5); //fmt: s (16)

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![3, 2, 1],
                    None,
                );

                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b000001, 6);
                //sub
            }
            "sub.d" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b010001, 6); //cop1
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b10001, 5); //fmt: d (17)

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![3, 2, 1],
                    None,
                );

                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b000001, 6);
            }
            "mul.s" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b010001, 6); //cop1
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b10000, 5); //fmt: s (16)

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![3, 2, 1],
                    None,
                );

                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b000010, 6);
                //mul
            }
            "mul.d" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b010001, 6); //cop1
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b10001, 5); //fmt: d (17)

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![3, 2, 1],
                    None,
                );

                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b000010, 6);
                //mul
            }
            "div.s" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b010001, 6); //cop1
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b10000, 5); //fmt: s (16)

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![3, 2, 1],
                    None,
                );

                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b000011, 6);
                //div
            }
            "div.d" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b010001, 6); //cop1
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b10001, 5); //fmt: d (17)

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![3, 2, 1],
                    None,
                );

                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b000011, 6);
                //div
            }
            "dahi" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b000001, 6); //regimm

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterGP, Immediate],
                    vec![1, 2],
                    None,
                );

                instruction_list[i].binary =
                    place_binary_in_middle_of_another(instruction_list[i].binary, 0b00110, 5, 15);
            }
            "dati" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b000001, 6); //regimm

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterGP, Immediate],
                    vec![1, 2],
                    None,
                );

                instruction_list[i].binary =
                    place_binary_in_middle_of_another(instruction_list[i].binary, 0b11110, 5, 15);
            }
            "daddiu" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b011001, 6); //daddiu

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![2, 1, 3],
                    None,
                );
            }
            "slt" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b000000, 6); //special

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b00000, 5); //0
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b101010, 6);
                //slt
            }
            "sltu" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b000000, 6); //special

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b00000, 5); //0
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b101011, 6);
                //sltu
            }
            "swc1" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b111001, 6); //swc1

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterFP, MemoryAddress],
                    vec![3, 1, 2],
                    None,
                );
            }
            "lwc1" => {
                instruction_list[i].binary = append_binary(instruction_list[i].binary, 0b110001, 6); //lwc1

                read_operands(
                    &mut instruction_list[i],
                    vec![RegisterFP, MemoryAddress],
                    vec![3, 1, 2],
                    None,
                );
            }
            _ => instruction_list[i].errors.push(Error {
                error_name: UnrecognizedInstruction,
                operand_number: None,
            }),
        }
    }
}
///This function takes two numbers and inserts the binary of the second at a given index in the binary of the first.
///All binary values at and past the insertion index of the original string will be moved to the end of the resultant string.
///Since binary is sign extended on the left to 32 bits, insertion index must be the index from the end of the string.
pub fn place_binary_in_middle_of_another(
    wrapper: u32,
    middle: u32,
    middle_length: usize,
    index_from_right: usize,
) -> u32 {
    //Step 1: Remove End Bits from Wrapper to make New Binary
    //Step 2: Move New Binary Left By length of Middle
    //Step 3: Or with Middle
    //Step 4: Move New Binary Left by length of End
    //Step 5: Shift Wrapper Left 32 - Length of End to get End
    //Step 6: Shift End Right by 32 - Length of End
    //Step 7: Or with New Binary
    //Step 8: Return New Binary
    let end_length = index_from_right + 1;
    let mut new_binary = wrapper >> end_length;
    new_binary <<= middle_length;
    new_binary |= middle;
    new_binary <<= end_length;
    let mut end = wrapper << (32 - end_length);
    end >>= 32 - end_length;
    new_binary |= end;
    new_binary
}

///Append binary takes two numbers, shifts the first by a specified amount and then bitwise ors the
/// two numbers together effectively appending the second onto the first.
pub fn append_binary(mut first: u32, second: u32, shift_amount: u8) -> u32 {
    first <<= shift_amount;
    first |= second;
    first
}
