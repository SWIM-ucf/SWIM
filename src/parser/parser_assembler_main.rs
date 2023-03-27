use crate::parser::assembling::{assemble_data_binary, read_operands};
use crate::parser::parser_structs_and_enums::ErrorType::*;
use crate::parser::parser_structs_and_enums::OperandType::*;
use crate::parser::parser_structs_and_enums::ProgramInfo;
use crate::parser::parser_structs_and_enums::*;
use crate::parser::parsing::*;
use crate::parser::pseudo_instruction_parsing::{
    complete_lw_sw_pseudo_instructions, expand_pseudo_instructions_and_assign_instruction_numbers,
};
use std::collections::HashMap;

///Parser is the starting function of the parser / assembler process. It takes a string representation of a MIPS
/// program and builds the binary of the instructions while cataloging any errors that are found.
pub fn parser(file_string: String) -> (ProgramInfo, Vec<u32>) {
    let mut program_info = ProgramInfo {
        monaco_line_info: tokenize_program(file_string),
        ..Default::default()
    };

    (program_info.instructions, program_info.data) =
        separate_data_and_text(&mut program_info.monaco_line_info);

    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut program_info.monaco_line_info,
    );

    let vec_of_data = assemble_data_binary(&mut program_info.data);

    let labels: HashMap<String, usize> =
        create_label_map(&mut program_info.instructions, &mut program_info.data);

    complete_lw_sw_pseudo_instructions(
        &mut program_info.instructions,
        &labels,
        &mut program_info.monaco_line_info,
    );

    read_instructions(
        &mut program_info.instructions,
        &labels,
        &mut program_info.monaco_line_info,
    );

    program_info.console_out_post_assembly = suggest_error_corrections(
        &mut program_info.instructions,
        &mut program_info.data,
        &labels,
        &mut program_info.monaco_line_info,
    );

    let binary = create_binary_vec(program_info.instructions.clone(), vec_of_data);

    for entry in &program_info.monaco_line_info {
        program_info
            .updated_monaco_string
            .push_str(&format!("{}\n", entry.updated_monaco_string));
    }

    for instruction in program_info.instructions.clone() {
        program_info
            .address_to_line_number
            .push(instruction.line_number);
    }

    (program_info.clone(), binary)
}


///Takes the vector of instructions and assembles the binary for them.
pub fn read_instructions(
    instruction_list: &mut [Instruction],
    labels: &HashMap<String, usize>,
    monaco_line_info: &mut [MonacoLineInfo],
) {
    for mut instruction in &mut instruction_list.iter_mut() {
        //this match case is the heart of the parser and figures out which instruction type it is
        //then it can call the proper functions for that specific instruction
        match &*instruction.operator.token_name.to_lowercase() {
            "add" => {
                instruction.binary = append_binary(instruction.binary, 0b000000, 6);

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction.binary = append_binary(instruction.binary, 0b00000, 5);
                instruction.binary = append_binary(instruction.binary, 0b100000, 6);

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "add rd, rs, rt".to_string(),
                        description: "Adds the 32-bit values in `rs` and `rt`, and places the result in `rd`.\n\nIn hardware implementations, the result is not placed in `rd` if adding `rs` and `rt` causes a 32-bit overflow. However, SWIM places the result in `rd` regardless since there is no exception handling.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "sub" => {
                instruction.binary = append_binary(instruction.binary, 0b000000, 6);

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction.binary = append_binary(instruction.binary, 0b00000, 5);
                instruction.binary = append_binary(instruction.binary, 0b100010, 6);

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "sub rd, rs, rt".to_string(),
                        description: "Subtracts the 32-bit value in `rt` from the 32-bit value in `rd`, and places the result in `rd`.\n\nIn hardware implementations, the result is not placed in `rd` if subtracting `rs` and `rt` causes a 32-bit overflow. However, SWIM places the result in `rd` regardless since there is no exception handling.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "mul" => {
                instruction.binary = append_binary(instruction.binary, 0b000000, 6);

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction.binary = append_binary(instruction.binary, 0b00010, 5);
                instruction.binary = append_binary(instruction.binary, 0b011000, 6);

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "mul rd, rs, rt".to_string(),
                        description: "Multiplies the signed 32-bit values in `rs` and `rt`, and places the lower 32 bits of the result in `rd`.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "div" => {
                instruction.binary = append_binary(instruction.binary, 0b000000, 6);

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction.binary = append_binary(instruction.binary, 0b00010, 5);
                instruction.binary = append_binary(instruction.binary, 0b011010, 6);

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "div rd, rs, rt".to_string(),
                        description: "Divides the 32-bit value in `rs` by the 32-bit value in `rt` and places the 32-bit quotient into `rd`.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "lw" => {
                instruction.binary = append_binary(instruction.binary, 0b100011, 6);

                read_operands(
                    instruction,
                    vec![RegisterGP, MemoryAddress],
                    vec![3, 1, 2],
                    None,
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "lw rt, offset(base)".to_string(),
                        description: "Loads the contents of the 32-bit at the specified memory address into `rt`.\n\nMemory address is calculated as the sum of `offset` and the contents of the `base` register.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "sw" => {
                instruction.binary = append_binary(instruction.binary, 0b101011, 6);

                read_operands(
                    instruction,
                    vec![RegisterGP, MemoryAddress],
                    vec![3, 1, 2],
                    None,
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "sw rt, offset(base)".to_string(),
                        description: "Stores the value of the lower 32-bits in `rt` at the specified memory address.\n\nMemory address is calculated as the sum of `offset` and the contents of the `base` register.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "lui" => {
                instruction.binary = append_binary(instruction.binary, 0b001111, 6);
                instruction.binary = append_binary(instruction.binary, 0b00000, 5);

                read_operands(instruction, vec![RegisterGP, Immediate], vec![1, 2], None);

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription {
                        syntax: "lui rt, immediate".to_string(),
                        description:
                            "Loads the 16-bit `immediate` value shifted left by 16 into `rt`."
                                .to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "aui" => {
                instruction.binary = append_binary(instruction.binary, 0b001111, 6);

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![2, 1, 3],
                    None,
                );

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "aui rt, rs, immediate".to_string(),
                    description: "Adds the sign-extended 16-bit `immediate` value shifted left by 16 to the contents of `rs`, and stores the result in `rt`.\n\nResult is sign-extended as if it is a 32-bit signed address.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "andi" => {
                instruction.binary = append_binary(instruction.binary, 0b001100, 6);

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![2, 1, 3],
                    None,
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "andi rt, rs, immediate".to_string(),
                        description: "Bitwise ands the contents of `rs` with the left zero-extended `immediate` value, and stores the result in `rt`.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "ori" => {
                instruction.binary = append_binary(instruction.binary, 0b001101, 6);

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![2, 1, 3],
                    None,
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "ori rt, rs, immediate".to_string(),
                        description: "Bitwise ors the contents of `rs` with the left zero-extended `immediate` value, and stores the result in `rt`.\n\n".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "addi" => {
                instruction.binary = append_binary(instruction.binary, 0b001000, 6); //addi

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![2, 1, 3],
                    None,
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "addi rt, rs, immediate".to_string(),
                        description: "Adds the 32-bit value in `rs` and the 16-bit `immediate`, and places the result in `rt`.\n\nIn hardware implementations, the result is not placed in `rt` if adding `rs` and the `immediate` causes a 32-bit overflow. However, SWIM places the result in `rd`, regardless since there is no exception handling.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "addiu" => {
                instruction.binary = append_binary(instruction.binary, 0b001001, 6); //addiu

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![2, 1, 3],
                    None,
                );

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "addiu rt, rs, immediate".to_string(),
                    description: "Adds the 32-bit value in `rs` and the 16-bit `immediate`, and places the result in `rt`.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "dadd" => {
                instruction.binary = append_binary(instruction.binary, 0b000000, 6);

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction.binary = append_binary(instruction.binary, 0b00000, 5);
                instruction.binary = append_binary(instruction.binary, 0b101100, 6);

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "dadd rd, rs, rt".to_string(),
                        description: "Adds the 64-bit values in `rs` and `rt`, and places the result in `rd`.\n\nIn hardware implementations, the result is not placed in `rd` if adding `rs` and `rt` causes a 64-bit overflow. However, SWIM places the result in `rd`, regardless since there is no exception handling.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "dsub" => {
                instruction.binary = append_binary(instruction.binary, 0b000000, 6);

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction.binary = append_binary(instruction.binary, 0b00000, 5);
                instruction.binary = append_binary(instruction.binary, 0b101110, 6);

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "dsub rd, rs, rt".to_string(),
                        description: "Subtracts the 64-bit values in `rt` from the 64-bit value in `rs`, and places the result in `rd`.\n\nIn hardware implementations, the result is not placed in `rd` if subtracting `rs` and `rt` causes a 64-bit overflow. However, SWIM places the result in `rd`, regardless since there is no exception handling.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "dmul" => {
                instruction.binary = append_binary(instruction.binary, 0b000000, 6);

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction.binary = append_binary(instruction.binary, 0b00010, 5);
                instruction.binary = append_binary(instruction.binary, 0b011100, 6);

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "dmul rd, rs, rt".to_string(),
                        description: "Multiplies the signed 64-bit values in `rs` and `rt`, and places the lower 64 bits of the result in `rd`.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "ddiv" => {
                instruction.binary = append_binary(instruction.binary, 0b000000, 6);

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction.binary = append_binary(instruction.binary, 0b00010, 5);
                instruction.binary = append_binary(instruction.binary, 0b011110, 6);

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "ddiv rd, rs, rt".to_string(),
                        description: "Divides the 64-bit value in `rs` by the 64-bit value in `rt` and places the quotient into `rd`.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "or" => {
                instruction.binary = append_binary(instruction.binary, 0b000000, 6);

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction.binary = append_binary(instruction.binary, 0b00000, 5);
                instruction.binary = append_binary(instruction.binary, 0b100101, 6);

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "or rd, rs, rt".to_string(),
                    description: "Bitwise ors the contents of `rs` with the contents of `rt`, and stores the result in `rd`.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "and" => {
                instruction.binary = append_binary(instruction.binary, 0b000000, 6);

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction.binary = append_binary(instruction.binary, 0b00000, 5);
                instruction.binary = append_binary(instruction.binary, 0b100100, 6);

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "and rd, rs, rt".to_string(),
                    description: "Bitwise ands the contents of `rs` with the contents of `rt`, and stores the result in `rd`.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "add.s" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b10000, 5); //fmt: s (16)

                read_operands(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![3, 2, 1],
                    None,
                );

                instruction.binary = append_binary(instruction.binary, 0b000000, 6);
                //add

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "add.s fd, fs, ft".to_string(),
                    description: "Adds the single-precision values in `ft` and `fs` and stores the result in `fd`.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "add.d" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b10001, 5); //fmt: d (17)

                read_operands(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![3, 2, 1],
                    None,
                );

                instruction.binary = append_binary(instruction.binary, 0b000000, 6);
                //add

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "add.d fd, fs, ft".to_string(),
                    description: "Adds the double-precision values in `ft` and `fs` and stores the result in `fd`.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "sub.s" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b10000, 5); //fmt: s (16)

                read_operands(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![3, 2, 1],
                    None,
                );

                instruction.binary = append_binary(instruction.binary, 0b000001, 6);
                //sub

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "sub.s fd, fs, ft".to_string(),
                    description: "Subtracts the single-precision value in `ft` from the single-precision value in `fs`, and places the result in `fd`.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "sub.d" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b10001, 5); //fmt: d (17)

                read_operands(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![3, 2, 1],
                    None,
                );

                instruction.binary = append_binary(instruction.binary, 0b000001, 6);

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "sub.d fd, fs, ft".to_string(),
                    description: "Subtracts the double-precision value in `ft` from the single-precision value in `fs`, and places the result in `fd`.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "mul.s" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b10000, 5); //fmt: s (16)

                read_operands(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![3, 2, 1],
                    None,
                );

                instruction.binary = append_binary(instruction.binary, 0b000010, 6);
                //mul

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "mul.s fd, fs, ft".to_string(),
                    description: "Multiplies the single-precision values in `ft` and `fs` and stores the result in `fd`.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "mul.d" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b10001, 5); //fmt: d (17)

                read_operands(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![3, 2, 1],
                    None,
                );

                instruction.binary = append_binary(instruction.binary, 0b000010, 6);
                //mul

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "mul.d fd, fs, ft".to_string(),
                    description: "Multiplies the double-precision values in `ft` and `fs` and stores the result in `fd`.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "div.s" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b10000, 5); //fmt: s (16)

                read_operands(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![3, 2, 1],
                    None,
                );

                instruction.binary = append_binary(instruction.binary, 0b000011, 6);
                //div

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "div.s fd, fs, ft".to_string(),
                    description: "Divides the single-precision value in `fs` by the single-precision value in `ft` and stores the result in `fd`.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "div.d" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b10001, 5); //fmt: d (17)

                read_operands(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![3, 2, 1],
                    None,
                );

                instruction.binary = append_binary(instruction.binary, 0b000011, 6);
                //div

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "div.d fd, fs, ft".to_string(),
                    description: "Divides the double-precision value in `fs` by the double-precision value in `ft` and stores the result in `fd`.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "dahi" => {
                instruction.binary = append_binary(instruction.binary, 0b000001, 6); //regimm

                read_operands(instruction, vec![RegisterGP, Immediate], vec![1, 2], None);

                instruction.binary =
                    place_binary_in_middle_of_another(instruction.binary, 0b00110, 5, 15);

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "dahi rs, immediate".to_string(),
                    description: "Adds the sign-extended 16-bit `immediate` value shifted left by 32 to the contents of `rs`, and stores the result in `rs`.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "dati" => {
                instruction.binary = append_binary(instruction.binary, 0b000001, 6); //regimm

                read_operands(instruction, vec![RegisterGP, Immediate], vec![1, 2], None);

                instruction.binary =
                    place_binary_in_middle_of_another(instruction.binary, 0b11110, 5, 15);

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "dati rs, immediate".to_string(),
                    description: "Adds the sign-extended 16-bit `immediate` value shifted left by 48 to the contents of `rs`, and stores the result in `rs`.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "daddi" => {
                instruction.binary = append_binary(instruction.binary, 0b011000, 6); //daddi

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![2, 1, 3],
                    None,
                );
                let info = InstructionDescription{
                    syntax: "daddi rt, rs, immediate".to_string(),
                    description: "Adds the 64-bit value in `rs` and the 16-bit `immediate`, and places the result in `rt`.\n\nIn hardware implementations, the result is not placed in `rt` if adding `rs` and `immediate` causes a 64-bit overflow. However, SWIM places the result in `rt`, regardless since there is no exception handling.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "daddiu" => {
                instruction.binary = append_binary(instruction.binary, 0b011001, 6); //daddiu

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![2, 1, 3],
                    None,
                );

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "daddiu rt, rs, immediate".to_string(),
                    description: "Adds the 64-bit value in `rs` and the 16-bit `immediate`, and places the result in `rt`.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "daddu" => {
                instruction.binary = append_binary(instruction.binary, 0b000000, 6); //special

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction.binary = append_binary(instruction.binary, 0b00000, 5); //0
                instruction.binary = append_binary(instruction.binary, 0b101101, 6);
                //daddu

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "daddu rd, rs, rt".to_string(),
                    description: "Adds the 64-bit values in `rs` and `rt`, and places the result in `rd`.\n\nIgnores overflow.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "dsubu" => {
                instruction.binary = append_binary(instruction.binary, 0b000000, 6); //special

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction.binary = append_binary(instruction.binary, 0b00000, 5); //0
                instruction.binary = append_binary(instruction.binary, 0b101111, 6);
                //dsubu

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "dsubu rd, rs, rt".to_string(),
                        description: "Subtracts the 64-bit values in `rt` from the 64-bit value in `rs`, and places the result in `rd`.\n\nIgnores overflow.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "dmulu" => {
                instruction.binary = append_binary(instruction.binary, 0b000000, 6); //special

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction.binary = append_binary(instruction.binary, 0b00010, 5); //dmulu
                instruction.binary = append_binary(instruction.binary, 0b011101, 6);
                //sop35

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "dmulu rd, rs, rt".to_string(),
                        description: "Multiplies the signed 64-bit values in `rs` and `rt`, and places the lower 64 bits of the result in `rd`.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "ddivu" => {
                instruction.binary = append_binary(instruction.binary, 0b000000, 6); //special

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction.binary = append_binary(instruction.binary, 0b00010, 5); //ddivu
                instruction.binary = append_binary(instruction.binary, 0b011111, 6);
                //DDIVU

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "ddivu rd, rs, rt".to_string(),
                        description: "Divides the unsigned 64-bit value in `rs` by the unsigned 64-bit value in `rt` and places the quotient into `rd`.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "slt" => {
                instruction.binary = append_binary(instruction.binary, 0b000000, 6); //special

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction.binary = append_binary(instruction.binary, 0b00000, 5); //0
                instruction.binary = append_binary(instruction.binary, 0b101010, 6);
                //slt

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "slt rd, rs, rt".to_string(),
                        description: "Compares the contents of `rs` and `rt` as signed integers and stores the value 1 in `rd` if `rs` is less than rt. Otherwise, stores the value 0 in `rd`.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "sltu" => {
                instruction.binary = append_binary(instruction.binary, 0b000000, 6); //special

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction.binary = append_binary(instruction.binary, 0b00000, 5); //0
                instruction.binary = append_binary(instruction.binary, 0b101011, 6);
                //sltu

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "sltu rd, rs, rt".to_string(),
                        description: "Compares the contents of `rs` and `rt` as unsigned integers and stores the value 1 in `rd` if `rs` is less than `rt`. Otherwise, stores the value 0 in `rd`.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "swc1" => {
                instruction.binary = append_binary(instruction.binary, 0b111001, 6); //swc1

                read_operands(
                    instruction,
                    vec![RegisterFP, MemoryAddress],
                    vec![3, 1, 2],
                    None,
                );

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "swc1 ft, offset(base)".to_string(),
                    description: "Stores the value of the lower 32 bits in `ft` at the specified memory address.\n\nMemory address is calculated as the sum of `offset` and the contents of the `base` register.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "lwc1" => {
                instruction.binary = append_binary(instruction.binary, 0b110001, 6); //lwc1

                read_operands(
                    instruction,
                    vec![RegisterFP, MemoryAddress],
                    vec![3, 1, 2],
                    None,
                );

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "lwc1 ft offset(base)".to_string(),
                    description: "Loads the contents of the 32-bit word at the specified memory address into `ft`.\n\nMemory address is calculated as the sum of `offset` and the contents of the `base` register.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "mtc1" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b00100, 5); //mt

                read_operands(instruction, vec![RegisterGP, RegisterFP], vec![1, 2], None);

                instruction.binary = append_binary(instruction.binary, 0b00000000000, 11);
                //0

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription {
                    syntax: "mtc1 rt, fs".to_string(),
                    description: "Moves the lower 32 bits in `rt` into the lower 32 bits in `fs`."
                        .to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "dmtc1" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b00101, 5); //dmt

                read_operands(instruction, vec![RegisterGP, RegisterFP], vec![1, 2], None);

                instruction.binary = append_binary(instruction.binary, 0b00000000000, 11);
                //0

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription {
                    syntax: "dmtc1 rt, fs".to_string(),
                    description: "Moves the double-word contents in `rt` into `fs`.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "mfc1" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b00000, 5); //mf

                read_operands(instruction, vec![RegisterGP, RegisterFP], vec![1, 2], None);

                instruction.binary = append_binary(instruction.binary, 0b00000000000, 11);
                //0

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription {
                    syntax: "mfc1 rt, fs".to_string(),
                    description: "Sign-extends contents in `fs` and moves it into `rt`."
                        .to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "dmfc1" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b00001, 5); //dmf

                read_operands(instruction, vec![RegisterGP, RegisterFP], vec![1, 2], None);

                instruction.binary = append_binary(instruction.binary, 0b00000000000, 11);
                //0

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription {
                    syntax: "dmfc1 rt, fs".to_string(),
                    description: "Moves the double-word contents in `fs` into `rt`.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "j" => {
                instruction.binary = append_binary(instruction.binary, 0b000010, 6); //j

                read_operands(
                    instruction,
                    vec![LabelAbsolute],
                    vec![1],
                    Some(labels.clone()),
                );

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription {
                    syntax: "j target".to_string(),
                    description:
                        "Moves the program counter to point to the targeted instruction’s address."
                            .to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "jr" => {
                instruction.binary = append_binary(instruction.binary, 0b000000, 6); //special

                read_operands(instruction, vec![RegisterGP], vec![1], None);

                instruction.binary = append_binary(instruction.binary, 0b001001, 6); //jalr

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "jr rs".to_string(),
                    description: "Reads the contents of the register as an address and moves the program counter to point to that instruction.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "jal" => {
                instruction.binary = append_binary(instruction.binary, 0b000011, 6); //jal

                read_operands(
                    instruction,
                    vec![LabelAbsolute],
                    vec![1],
                    Some(labels.clone()),
                );

                //this instruction is not used  in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "jal target".to_string(),
                    description: "Execute a procedure call. Sets the $ra (\"return address\") register to the next instruction, then moves the program counter to point to the targeted instruction’s address.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "jalr" => {
                //JALR is unique in a number of ways.
                //if rd is left blank, rd = 31 is implied.
                //in release 6, rd cannot be 00000
                //it makes use of hint bits in some releases. In our implementation, we are not implementing hint bits.

                instruction.binary = append_binary(instruction.binary, 0b000000, 6); //special

                if instruction.operands.len() == 1 {
                    //if len() == 1, rd is omitted and assumed to equal 31
                    read_operands(instruction, vec![RegisterGP], vec![1], None);
                    instruction.binary = append_binary(instruction.binary, 0b11111, 5);
                    //rd = 31
                } else {
                    if instruction.operands[0].token_name == "$zero" {
                        instruction.errors.push(Error {
                            error_name: JALRRDRegisterZero,
                            token_causing_error: "$zero".to_string(),
                            start_end_columns: instruction.operands[0].start_end_columns,
                            message: "".to_string(),
                        })
                    }
                    read_operands(instruction, vec![RegisterGP, RegisterGP], vec![2, 1], None);
                }

                instruction.binary =
                    place_binary_in_middle_of_another(instruction.binary, 0b00000, 5, 5); //0

                instruction.binary = append_binary(instruction.binary, 0b00000, 5); //hint
                instruction.binary = append_binary(instruction.binary, 0b001001, 6);
                //JALR

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "jalr rs` (`rd` = `$ra` implied) || `jalr rd, rs".to_string(),
                    description: "Execute a procedure call. Sets the $ra (\"return address\") register to the next instruction, then moves the program counter to point to the address read from `rs`.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "beq" => {
                instruction.binary = append_binary(instruction.binary, 0b000100, 6); //beq

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, LabelRelative],
                    vec![1, 2, 3],
                    Some(labels.clone()),
                );

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "beq rs, rt, target".to_string(),
                    description: "Compares the contents of `rs` and `rt` and, if they are equal, moves the program counter to point to the targeted instruction’s address.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "b" => {
                instruction.binary = append_binary(instruction.binary, 0b000100, 6); //beq

                instruction.binary = append_binary(instruction.binary, 0b00000, 5); //0
                instruction.binary = append_binary(instruction.binary, 0b00000, 5); //0

                read_operands(
                    instruction,
                    vec![LabelRelative],
                    vec![1],
                    Some(labels.clone()),
                );

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription {
                    syntax: "b target".to_string(),
                    description:
                        "Moves the program counter to point to the targeted instruction’s address."
                            .to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "bne" => {
                instruction.binary = append_binary(instruction.binary, 0b000101, 6); //bne

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, LabelRelative],
                    vec![1, 2, 3],
                    Some(labels.clone()),
                );

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "bne rs, rt, target".to_string(),
                    description: "Compares the contents of `rs` and `rt` and, if they are not equal, moves the program counter to point to the targeted instruction’s address.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "c.eq.s" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b10000, 5); //fmt: s

                read_operands(
                    instruction,
                    vec![RegisterFP, RegisterFP],
                    vec![2, 1],
                    Some(labels.clone()),
                );

                instruction.binary = append_binary(instruction.binary, 0b0000011, 7); //cc, 0, A, FC
                instruction.binary = append_binary(instruction.binary, 0b1010, 4);
                //EQ

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "c.eq.s fs, ft".to_string(),
                    description: "Compares the contents of `fs` and `ft` as single-precision floats and, if they are equal, stores the value 1 into FPConditionCode. Otherwise, stores the value 0 into FPConditionCode.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "c.eq.d" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b10001, 5); //fmt: d

                read_operands(
                    instruction,
                    vec![RegisterFP, RegisterFP],
                    vec![2, 1],
                    Some(labels.clone()),
                );

                instruction.binary = append_binary(instruction.binary, 0b0000011, 7); //cc, 0, A, FC
                instruction.binary = append_binary(instruction.binary, 0b1010, 4);
                //EQ

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "c.eq.d fs, ft".to_string(),
                    description: "Compares the contents of `fs` and `ft` as double-precision floats and, if they are equal, stores the value 1 into FPConditionCode. Otherwise, stores the value 0 into FPConditionCode.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "c.lt.s" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b10000, 5); //fmt: s

                read_operands(
                    instruction,
                    vec![RegisterFP, RegisterFP],
                    vec![2, 1],
                    Some(labels.clone()),
                );

                instruction.binary = append_binary(instruction.binary, 0b0000011, 7); //cc, 0, A, FC
                instruction.binary = append_binary(instruction.binary, 0b1100, 4);
                //lt

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "c.lt.s fs, ft".to_string(),
                    description: "Compares the contents of `fs` and `ft` as single-precision floats and, if the contents of `fs` is less than the contents of `ft`, stores the value 1 into FPConditionCode. Otherwise, stores the value 0 into FPConditionCode.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "c.lt.d" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b10001, 5); //fmt: d

                read_operands(
                    instruction,
                    vec![RegisterFP, RegisterFP],
                    vec![2, 1],
                    Some(labels.clone()),
                );

                instruction.binary = append_binary(instruction.binary, 0b0000011, 7); //cc, 0, A, FC
                instruction.binary = append_binary(instruction.binary, 0b1100, 4);
                //lt

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "c.lt.d fs, ft".to_string(),
                    description: "Compares the contents of `fs` and `ft` as double-precision floats and, if the contents of `fs` is less than the contents of `ft`, stores the value 1 into FPConditionCode. Otherwise, stores the value 0 into FPConditionCode.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "c.le.s" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b10000, 5); //fmt: s

                read_operands(
                    instruction,
                    vec![RegisterFP, RegisterFP],
                    vec![2, 1],
                    Some(labels.clone()),
                );

                instruction.binary = append_binary(instruction.binary, 0b0000011, 7); //cc, 0, A, FC
                instruction.binary = append_binary(instruction.binary, 0b1110, 4);
                //le

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "c.le.s fs, ft".to_string(),
                    description: "Compares the contents of `fs` and `ft` as single-precision floats and, if the contents of `fs` is less than or equal to the contents of `ft`, stores the value 1 into FPConditionCode. Otherwise, stores the value 0 into FPConditionCode.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "c.le.d" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b10001, 5); //fmt: d

                read_operands(
                    instruction,
                    vec![RegisterFP, RegisterFP],
                    vec![2, 1],
                    Some(labels.clone()),
                );

                instruction.binary = append_binary(instruction.binary, 0b0000011, 7); //cc, 0, A, FC
                instruction.binary = append_binary(instruction.binary, 0b1110, 4);
                //le

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "c.le.d fs, ft".to_string(),
                    description: "Compares the contents of `fs` and `ft` as double-precision floats and, if the contents of `fs` is less than or equal to the contents of `ft`, stores the value 1 into FPConditionCode. Otherwise, stores the value 0 into FPConditionCode.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "c.ngt.s" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b10000, 5); //fmt: s

                read_operands(
                    instruction,
                    vec![RegisterFP, RegisterFP],
                    vec![2, 1],
                    Some(labels.clone()),
                );

                instruction.binary = append_binary(instruction.binary, 0b0000011, 7); //cc, 0, A, FC
                instruction.binary = append_binary(instruction.binary, 0b1111, 4);
                //ngt

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "c.ngt.s fs, ft".to_string(),
                    description: "Compares the contents of `fs` and `ft` as single-precision floats and, if the contents of `fs` not greater than the contents of `ft`, stores the value 1 into FPConditionCode. Otherwise, stores the value 0 into FPConditionCode.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "c.ngt.d" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b10001, 5); //fmt: d

                read_operands(
                    instruction,
                    vec![RegisterFP, RegisterFP],
                    vec![2, 1],
                    Some(labels.clone()),
                );

                instruction.binary = append_binary(instruction.binary, 0b0000011, 7); //cc, 0, A, FC
                instruction.binary = append_binary(instruction.binary, 0b1111, 4);
                //ngt

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "c.ngt.d fs, ft".to_string(),
                    description: "Compares the contents of `fs` and `ft` as double-precision floats and, if the contents of `fs` not greater than the contents of `ft`, stores the value 1 into FPConditionCode. Otherwise, stores the value 0 into FPConditionCode.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "c.nge.s" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b10000, 5); //fmt: s

                read_operands(
                    instruction,
                    vec![RegisterFP, RegisterFP],
                    vec![2, 1],
                    Some(labels.clone()),
                );

                instruction.binary = append_binary(instruction.binary, 0b0000011, 7); //cc, 0, A, FC
                instruction.binary = append_binary(instruction.binary, 0b1101, 4);
                //nge

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "c.nge.d fs, ft".to_string(),
                    description: "Compares the contents of `fs` and `ft` as single-precision floats and, if the contents of `fs` not greater than or equal to the contents of `ft`, stores the value 1 into FPConditionCode. Otherwise, stores the value 0 into FPConditionCode.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "c.nge.d" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b10001, 5); //fmt: d

                read_operands(
                    instruction,
                    vec![RegisterFP, RegisterFP],
                    vec![2, 1],
                    Some(labels.clone()),
                );

                instruction.binary = append_binary(instruction.binary, 0b0000011, 7); //cc, 0, A, FC
                instruction.binary = append_binary(instruction.binary, 0b1101, 4);
                //nge

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "c.nge.d fs, ft".to_string(),
                    description: "Compares the contents of `fs` and `ft` as double-precision floats and, if the contents of `fs` not greater than or equal to the contents of `ft`, stores the value 1 into FPConditionCode. Otherwise, stores the value 0 into FPConditionCode.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "bc1t" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b01000, 5); //BC
                instruction.binary = append_binary(instruction.binary, 0b00001, 5); //CC ND TF

                read_operands(
                    instruction,
                    vec![LabelRelative],
                    vec![1],
                    Some(labels.clone()),
                );

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "bc1t target".to_string(),
                    description: "If FPConditionCode is 1, moves the program counter to point to the targeted instruction’s address.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "bc1f" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b01000, 5); //BC
                instruction.binary = append_binary(instruction.binary, 0b00000, 5); //CC ND TF

                read_operands(
                    instruction,
                    vec![LabelRelative],
                    vec![1],
                    Some(labels.clone()),
                );

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "bc1f target".to_string(),
                    description: "If FPConditionCode is 0, moves the program counter to point to the targeted instruction’s address.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "sll" => {
                instruction.binary = append_binary(instruction.binary, 0b000000, 6); //special
                instruction.binary = append_binary(instruction.binary, 0b00000, 5); //0

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, ShiftAmount],
                    vec![2, 1, 3],
                    Some(labels.clone()),
                );

                instruction.binary = append_binary(instruction.binary, 0b00000, 6); //sll

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "sll rt, rs, sa".to_string(),
                    description: "Shifts the lower 32-bit word in `rs` to the left by sa number of bits and placing the sign-extended result into `rt`.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "nop" => {
                instruction.binary = append_binary(instruction.binary, 0b000000, 6); //special
                instruction.binary = append_binary(instruction.binary, 0b00000, 5); //0

                instruction.binary = append_binary(instruction.binary, 0b00000, 5); //0
                instruction.binary = append_binary(instruction.binary, 0b00000, 5); //0
                instruction.binary = append_binary(instruction.binary, 0b00000, 5); //0

                instruction.binary = append_binary(instruction.binary, 0b00000, 6); //sll

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription {
                    syntax: "nop".to_string(),
                    description: "This instruction does not do anything when it is run".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
            }
            "syscall" => {
                //our support for syscall is limited. It is simply there to end emulation
                instruction.binary = append_binary(instruction.binary, 0b000000, 6); //special
                instruction.binary = append_binary(instruction.binary, 0b00000000000000000000, 20); //stub of code
                instruction.binary = append_binary(instruction.binary, 0b001100, 6);
                //syscall

                //only adds mouse hover for syscall if the syscall was actually there already and not just inserted by the parser
                if (!monaco_line_info[instruction.line_number].tokens.is_empty()
                    && monaco_line_info[instruction.line_number].tokens[0].token_name == "syscall")
                    || (monaco_line_info[instruction.line_number].tokens.len() > 1
                        && monaco_line_info[instruction.line_number].tokens[0]
                            .token_name
                            .ends_with(':')
                        && monaco_line_info[instruction.line_number].tokens[1].token_name
                            == "syscall")
                {
                    let info = InstructionDescription{
                        syntax: "syscall".to_string(),
                        description: "This function is currently stubbed in SWIM. Normally, it reverts control back to the OS. SWIM uses it to effectively end the program.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }

            _ => {
                let unsupported_instructions = [
                    "abs.s",
                    "abs.d",
                    "abs.ps",
                    "addiupc",
                    "addu",
                    "align",
                    "dalign",
                    "alnv.ps",
                    "aluipc",
                    "daui",
                    "auipc",
                    "bal",
                    "balc",
                    "bc",
                    "bc1eqz",
                    "bc1nez",
                    "bc1f",
                    "bc1fl",
                    "bc1t",
                    "bc1tl",
                    "bc2eqz",
                    "bc2nez",
                    "bc2f",
                    "bc2fl",
                    "bc2t",
                    "bc2tl",
                    "beql",
                    "bgez",
                    "bgezal",
                    "blezalc",
                    "bgezalc",
                    "bgtzalc",
                    "bltzalc",
                    "beqzalc",
                    "bnezalc",
                    "bgezall",
                    "beqc",
                    "bnec",
                    "bltc",
                    "bgec",
                    "bltuc",
                    "bgeuc",
                    "bgtc",
                    "blec",
                    "bgtuc",
                    "bleuc",
                    "bltzc",
                    "blezc",
                    "bgezc",
                    "bgtzc",
                    "beqzc",
                    "bnezc",
                    "bgezl",
                    "bgtz",
                    "bgtzl",
                    "bitswap",
                    "dbitswap",
                    "blez",
                    "blezl",
                    "bltz",
                    "bltzal",
                    "bltzall",
                    "bltzl",
                    "bnel",
                    "bovc",
                    "bnvc",
                    "break",
                    "c.f.s",
                    "c.un.s",
                    "c.ueq.s",
                    "c.olt.s",
                    "c.ult.s",
                    "c.ole.s",
                    "c.ule.s",
                    "c.sf.s",
                    "c.ngle.s",
                    "c.seq.s",
                    "c.ngl.s",
                    "c.f.d",
                    "c.un.d",
                    "c.ueq.d",
                    "c.olt.d",
                    "c.ult.d",
                    "c.ole.d",
                    "c.ule.d",
                    "c.sf.d",
                    "c.ngle.d",
                    "c.seq.d",
                    "c.ngl.d",
                    "cache",
                    "cachee",
                    "ceil.l.s",
                    "ceil.l.d",
                    "ceil.w.s",
                    "ceil.w.d",
                    "cfc1",
                    "cfc2",
                    "class.s",
                    "class.d",
                    "clo",
                    "clz",
                    "cmp.f.s",
                    "cmp.un.s",
                    "cmp.eq.s",
                    "cmp.ueq.s",
                    "cmp.olt.s",
                    "cmp.ult.s",
                    "cmp.ole.s",
                    "cmp.ule.s",
                    "cmp.sf.s",
                    "cmp.ngle.s",
                    "cmp.seq.s",
                    "cmp.ngl.s",
                    "cmp.lt.s",
                    "cmp.nge.s",
                    "cmp.le.s",
                    "cmp.ngt.s",
                    "cmp.f.d",
                    "cmp.un.d",
                    "cmp.eq.d",
                    "cmp.ueq.d",
                    "cmp.olt.d",
                    "cmp.ult.d",
                    "cmp.ole.d",
                    "cmp.ule.d",
                    "cmp.sf.d",
                    "cmp.ngle.d",
                    "cmp.seq.d",
                    "cmp.ngl.d",
                    "cmp.lt.d",
                    "cmp.nge.d",
                    "cmp.le.d",
                    "cmp.ngt.d",
                    "cop2",
                    "crc32b",
                    "crc32h",
                    "crc32w",
                    "crc32d",
                    "crc32cb",
                    "crc32ch",
                    "crc32cw",
                    "crc32cd",
                    "ctc1",
                    "ctc2",
                    "cvt.d.s",
                    "cvt.d.w",
                    "cvt.d.l",
                    "cvt.l.s",
                    "cvt.l.d",
                    "cvt.ps.s",
                    "cvt.s.pl",
                    "cvt.s.pu",
                    "cvt.s.d",
                    "cvt.s.w",
                    "cvt.s.l",
                    "cvt.w.s",
                    "cvt.w.d",
                    "dclo",
                    "dclz",
                    "deret",
                    "dext",
                    "dextm",
                    "dextu",
                    "di",
                    "dins",
                    "dinsm",
                    "dinsu",
                    "mod",
                    "divu",
                    "modu",
                    "dmod",
                    "dmodu",
                    "dmfc0",
                    "dmtc0",
                    "dmtc2",
                    "dmult",
                    "dmultu",
                    "drotr",
                    "drotr32",
                    "drotrv",
                    "dsbh",
                    "dshd",
                    "dsll",
                    "dsll32",
                    "dsllv",
                    "dsra",
                    "dsra32",
                    "dsrav",
                    "dsrl",
                    "dsrl32",
                    "dsrlv",
                    "dvp",
                    "ehb",
                    "ei",
                    "eret",
                    "eretnc",
                    "evp",
                    "ext",
                    "floor.l.s",
                    "floor.l.d",
                    "floor.w.s",
                    "floor.w.d",
                    "ginvi",
                    "ginvt",
                    "ins",
                    "jalr.hb",
                    "jalx",
                    "jialc",
                    "jic",
                    "jr.hb",
                    "lb",
                    "lbe",
                    "lbu",
                    "lbue",
                    "ldc1",
                    "ldc2",
                    "ldl",
                    "ldpc",
                    "ldr",
                    "ldxc1",
                    "lh",
                    "lhe",
                    "lhu",
                    "lhue",
                    "ll",
                    "lld",
                    "lle",
                    "lldp",
                    "llwp",
                    "llwpe",
                    "lsa",
                    "dlsa",
                    "luxc1",
                    "lwc2",
                    "lwe",
                    "lwl",
                    "lwle",
                    "lwpc",
                    "lwr",
                    "lwre",
                    "lwu",
                    "lwupc",
                    "lwxc1",
                    "madd",
                    "madd.s",
                    "madd.d",
                    "madd.ps",
                    "maddf.s",
                    "maddf.d",
                    "maddf.s",
                    "msubf.s",
                    "msubf.d",
                    "maddu",
                    "max.s",
                    "max.d",
                    "maxa.s",
                    "maxa.d",
                    "min.s",
                    "mina.d",
                    "mcf0",
                    "mcf1",
                    "mfc2",
                    "mfhi",
                    "mflo",
                    "mov.s",
                    "mov.d",
                    "mov.ps",
                    "movf",
                    "movf.s",
                    "movf.d",
                    "movf.ps",
                    "movn",
                    "movn.s",
                    "movn.d",
                    "movn.ps",
                    "movt",
                    "movt.s",
                    "movt.d",
                    "movt.ps",
                    "movz",
                    "movz.s",
                    "movz.d",
                    "movz.ps",
                    "msub",
                    "msub.s",
                    "msub.d",
                    "msub.ps",
                    "msubu",
                    "mtc0",
                    "mtc2",
                    "mthc0",
                    "mthc1",
                    "mthc2",
                    "mthi",
                    "mtlo",
                    "muh",
                    "mulu",
                    "muhu",
                    "dmuh",
                    "dmuhu",
                    "mul.ps",
                    "mult",
                    "multu",
                    "nal",
                    "neg.s",
                    "neg.d",
                    "neg.ps",
                    "nmadd.s",
                    "nmadd.d",
                    "nmadd.ps",
                    "nmsub.s",
                    "nmsub.d",
                    "nmsub.ps",
                    "nor",
                    "pause",
                    "pll.ps",
                    "plu.ps",
                    "pref",
                    "prefe",
                    "prefx",
                    "pul.ps",
                    "puu.ps",
                    "rdhwr",
                    "rdpgpr",
                    "recip.s",
                    "recip.d",
                    "rint.s",
                    "rint.d",
                    "rotzr",
                    "rotrv",
                    "round.l.s",
                    "round.l.d",
                    "round.w.s",
                    "round.w.d",
                    "rsqrt.s",
                    "rsqrt.d",
                    "sb",
                    "sbe",
                    "sc",
                    "scd",
                    "scdp",
                    "sce",
                    "scwp",
                    "scwpe",
                    "sdbbp",
                    "sdc1",
                    "sdc2",
                    "sdl",
                    "sdr",
                    "sdxc1",
                    "seb",
                    "seh",
                    "sel.s",
                    "sel.d",
                    "seleqz",
                    "selnez",
                    "seleqz.s",
                    "seleqz.d",
                    "selneqz.s",
                    "selneqz.d",
                    "sh",
                    "she",
                    "sigrie",
                    "sllv",
                    "slti",
                    "sltiu",
                    "sqrt.s",
                    "sqrt.d",
                    "sra",
                    "srav",
                    "srl",
                    "srlv",
                    "ssnop",
                    "sub.ps",
                    "subu",
                    "suxc1",
                    "swc2",
                    "swe",
                    "swl",
                    "swle",
                    "swr",
                    "swre",
                    "swxc1",
                    "sync",
                    "synci",
                    "teq",
                    "teqi",
                    "tge",
                    "tgei",
                    "tgeiu",
                    "tgeu",
                    "tlbinv",
                    "tlbinvf",
                    "tlbp",
                    "tlbr",
                    "tlbwi",
                    "tlbwr",
                    "tlt",
                    "tlti",
                    "tltiu",
                    "tltu",
                    "tne",
                    "tnei",
                    "trunc.l.s",
                    "trunc.l.d",
                    "trunc.w.s",
                    "trunc.w.d",
                    "wait",
                    "wrpgpr",
                    "xor",
                    "xori",
                ];

                if unsupported_instructions.contains(&&*instruction.operator.token_name) {
                    instruction.errors.push(Error {
                        error_name: UnsupportedInstruction,
                        token_causing_error: instruction.operator.token_name.to_string(),
                        start_end_columns: instruction.operator.start_end_columns,
                        message: "\n\n".to_string(),
                    })
                } else {
                    instruction.errors.push(Error {
                        error_name: UnrecognizedInstruction,
                        token_causing_error: instruction.operator.token_name.clone(),
                        start_end_columns: instruction.operator.start_end_columns,
                        message: "\n\n".to_string(),
                    });
                }
            }
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
pub fn append_binary(mut first: u32, mut second: u32, shift_amount: u8) -> u32 {
    second <<= 32 - shift_amount;
    second >>= 32 - shift_amount;
    first <<= shift_amount;
    first |= second;
    first
}

///Creates a vector of u32 from the data found in the parser / assembler to put into memory.
pub fn create_binary_vec(instructions: Vec<Instruction>, mut vec_of_data: Vec<u8>) -> Vec<u32> {
    //push all instructions
    let mut binary: Vec<u32> = Vec::new();
    for instruction in instructions {
        binary.push(instruction.binary);
    }

    //makes sure the byte array length is a multiple of 4
    let mut mod4 = 4 - (vec_of_data.len() % 4);
    if mod4 == 4 {
        mod4 = 0;
    }
    vec_of_data.resize(vec_of_data.len() + mod4, 0);

    //push the .data
    let mut i = 0;
    while i < vec_of_data.len() {
        //create a word from 4 bytes and then push it to the vec
        let mut word = vec_of_data[i] as u32;
        word <<= 8;
        i += 1;
        word |= vec_of_data[i] as u32;
        word <<= 8;
        i += 1;
        word |= vec_of_data[i] as u32;
        word <<= 8;
        i += 1;
        word |= vec_of_data[i] as u32;
        binary.push(word);
        i += 1;
    }

    binary
}
