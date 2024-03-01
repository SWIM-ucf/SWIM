use crate::parser::assembling::{assemble_data_binary, read_operands, read_operands_riscv};
use crate::parser::parser_structs_and_enums::ErrorType::*;
use crate::parser::parser_structs_and_enums::OperandType::*;
use crate::parser::parser_structs_and_enums::ProgramInfo;
use crate::parser::parser_structs_and_enums::*;
use crate::parser::parsing::*;
use crate::parser::pseudo_instruction_parsing::{
    complete_lw_sw_pseudo_instructions, expand_pseudo_instructions_and_assign_instruction_numbers,
};
use std::collections::HashMap;

use gloo_console::log;

///Parser is the starting function of the parser / assembler process. It takes a string representation of a MIPS
/// program and builds the binary of the instructions while cataloging any errors that are found.
pub fn parser(file_string: String) -> (ProgramInfo, Vec<u32>) {
    // Force MIPS to pass unit tests until I change the function arguments for SWIMv1 test cases``
    let arch = Architecture::MIPS;

    if arch == Architecture::MIPS {
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

        let (binary, data_starting_point) =
            create_binary_vec(program_info.instructions.clone(), vec_of_data);

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

        program_info.pc_starting_point = determine_pc_starting_point(labels);
        program_info.data_starting_point = data_starting_point;

        (program_info.clone(), binary)
    } else {
        let mut program_info = ProgramInfo {
            monaco_line_info: tokenize_program(file_string),
            ..Default::default()
        };

        (program_info.instructions, program_info.data) =
            separate_data_and_text(&mut program_info.monaco_line_info);

        // Implement a RISC-V version
        /*expand_pseudo_instructions_and_assign_instruction_numbers(
            &mut program_info.instructions,
            &program_info.data,
            &mut program_info.monaco_line_info,
        );*/

        let vec_of_data = assemble_data_binary(&mut program_info.data);

        let labels: HashMap<String, usize> =
            create_label_map(&mut program_info.instructions, &mut program_info.data);

        // Implement a RISC-V version
        /*complete_lw_sw_pseudo_instructions(
            &mut program_info.instructions,
            &labels,
            &mut program_info.monaco_line_info,
        );*/

        read_instructions_riscv(
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

        let (binary, data_starting_point) =
            create_binary_vec(program_info.instructions.clone(), vec_of_data);

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

        program_info.pc_starting_point = determine_pc_starting_point(labels);
        program_info.data_starting_point = data_starting_point;

        (program_info.clone(), binary)
    }
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
            "addu" => {
                instruction.binary = append_binary(instruction.binary, 0b000000, 6); //special

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction.binary = append_binary(instruction.binary, 0b00000, 5); //0, shamt
                instruction.binary = append_binary(instruction.binary, 0b100001, 6); // funct code
                                                                                     //addu

                //this instruction is not used in pseudo-instructions so we can push it to mouse_hover_string without checking if mouse_hover_string is empty
                let info = InstructionDescription{
                    syntax: "addu rd, rs, rt".to_string(),
                    description: "Adds the 32-bit values in `rs` and `rt`, and places the result in `rd`.\n\nIgnores overflow.".to_string(),
                };
                monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
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

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription {
                        syntax: "or rd, rs, rt".to_string(),
                        description: "Bitwise ors the contents of `rs` with the contents of `rt`, and stores the result in `rd`.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
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
                    syntax: "lwc1 ft, offset(base)".to_string(),
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
                instruction.binary = append_binary(instruction.binary, 0b00000, 5); // rt
                instruction.binary = append_binary(instruction.binary, 0b00000, 5); // rd
                instruction.binary = append_binary(instruction.binary, 0b00000, 5); // shift/hint

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
                    if !instruction.operands.is_empty()
                        && instruction.operands[0].token_name == "$zero"
                    {
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
                instruction.binary = append_binary(instruction.binary, 0b0010, 4);
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
                instruction.binary = append_binary(instruction.binary, 0b0010, 4);
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
                if UNSUPPORTED_INSTRUCTIONS.contains(&&*instruction.operator.token_name) {
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
        //print_instruction_contents(instruction.clone());
    }
}

pub fn read_instructions_riscv(
    instruction_list: &mut [Instruction],
    _labels: &HashMap<String, usize>,
    monaco_line_info: &mut [MonacoLineInfo],
) {
    for mut instruction in &mut instruction_list.iter_mut() {
        match &*instruction.operator.token_name.to_lowercase() {
            "add" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0000000, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b000),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0110011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "add rd, rs1, rs2".to_string(),
                        description: "Adds the registers rs1 and rs2 and stores the result in rd.\n\nArithmetic overflow is ignored and the result is simply the low XLEN bits of the result.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "sub" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0100000, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b000),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0110011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "sub rd, rs1, rs2".to_string(),
                        description: "Subs the register rs2 from rs1 and stores the result in rd.\n\nArithmetic overflow is ignored and the result is simply the low XLEN bits of the result.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "sll" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0000000, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b001),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0110011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "sll rd, rs1, rs2".to_string(),
                        description: "Performs logical left shift on the value in register rs1 by the shift amount held in the lower 5 bits of register rs2.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "slt" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0000000, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b010),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0110011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "slt rd, rs1, rs2".to_string(),
                        description: "Place the value 1 in register rd if register rs1 is less than register rs2 when both are treated as signed numbers, else 0 is written to rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "sltu" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0000000, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b011),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0110011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "sltu rd, rs1, rs2".to_string(),
                        description: "Place the value 1 in register rd if register rs1 is less than register rs2 when both are treated as unsigned numbers, else 0 is written to rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "xor" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0000000, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b100),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0110011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "xor rd, rs1, rs2".to_string(),
                        description: "Performs bitwise XOR on registers rs1 and rs2 and place the result in rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "srl" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0000000, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b101),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0110011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "srl rd, rs1, rs2".to_string(),
                        description: "Logical right shift on the value in register rs1 by the shift amount held in the lower 5 bits of register rs2.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "sra" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0100000, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b101),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0110011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "sra rd, rs1, rs2".to_string(),
                        description: "Performs arithmetic right shift on the value in register rs1 by the shift amount held in the lower 5 bits of register rs2.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "or" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0000000, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b110),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0110011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "or rd, rs1, rs2".to_string(),
                        description: "Performs bitwise OR on registers rs1 and rs2 and place the result in rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "and" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0000000, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b111),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0110011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "and rd, rs1, rs2".to_string(),
                        description: "Performs bitwise AND on registers rs1 and rs2 and place the result in rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "addi" =>
            // This instruction requires the 12-bit immediate to be sign extended before moving to the emulator's register
            {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![1, 2, 3],
                    None,
                    Some(0),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "addi rd, rs1, imm".to_string(),
                        description: "Adds the sign-extended 12-bit immediate to register rs1.\n\nArithmetic overflow is ignored and the result is simply the low XLEN bits of the result.\n\naddi rd, rs1, 0 is used to implement the MV rd, rs1 assembler pseudo-instruction.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "slti" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![1, 2, 3],
                    None,
                    Some(0b010),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "slti rd, rs1, imm".to_string(),
                        description: "Place the value 1 in register rd if register rs1 is less than the signextended immediate when both are treated as signed numbers, else 0 is written to rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "sltiu" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![1, 2, 3],
                    None,
                    Some(0b011),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "sltiu rd, rs1, imm".to_string(),
                        description: "Place the value 1 in register rd if register rs1 is less than the immediate when both are treated as unsigned numbers, else 0 is written to rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "xori" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![1, 2, 3],
                    None,
                    Some(0b100),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "xori rd, rs1, imm".to_string(),
                        description: "Performs bitwise XOR on register rs1 and the sign-extended 12-bit immediate and place the result in rd.\n\nNote, “xori rd, rs1, -1” performs a bitwise logical inversion of register rs1(assembler pseudo-instruction NOT rd, rs)".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "ori" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![1, 2, 3],
                    None,
                    Some(0b110),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "ori rd, rs1, imm".to_string(),
                        description: "Performs bitwise OR on register rs1 and the sign-extended 12-bit immediate and place the result in rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "andi" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![1, 2, 3],
                    None,
                    Some(0b111),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "andi rd, rs1, imm".to_string(),
                        description: "Performs bitwise AND on register rs1 and the sign-extended 12-bit immediate and place the result in rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "slli" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b00000, 5); // Check if the next 2 bits are needed

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, ShiftAmount],
                    vec![1, 2, 3],
                    None,
                    Some(0b001),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "slli rd, rs1, shamt".to_string(),
                        description: "Performs logical left shift on the value in register rs1 by the shift amount held in the lower 5 bits of the immediate.\n\nIn RV64, bit-25 is used to shamt[5].".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "srli" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b00000, 5); // Check if the next 2 bits are needed

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, ShiftAmount],
                    vec![1, 2, 3],
                    None,
                    Some(0b101),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "srli rd, rs1, shamt".to_string(),
                        description: "Performs logical right shift on the value in register rs1 by the shift amount held in the lower 5 bits of the immediate.\n\nIn RV64, bit-25 is used to shamt[5].".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "srai" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b01000, 5); // Check if the next 2 bits are needed

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, ShiftAmount],
                    vec![1, 2, 3],
                    None,
                    Some(0b101),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "srai rd, rs1, shamt".to_string(),
                        description: "Performs arithmetic right shift on the value in register rs1 by the shift amount held in the lower 5 bits of the immediate.\n\nIn RV64, bit-25 is used to shamt[5].".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "lb" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, MemoryAddress],
                    vec![1, 3, 2],
                    None,
                    Some(0b000),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0000011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "lb rd, offset(rs1)".to_string(),
                        description: "Loads a 8-bit value from memory and sign-extends this to XLEN bits before storing it in register rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "lh" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, MemoryAddress],
                    vec![1, 3, 2],
                    None,
                    Some(0b001),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0000011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "lh rd, offset(rs1)".to_string(),
                        description: "Loads a 16-bit value from memory and sign-extends this to XLEN bits before storing it in register rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "lw" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, MemoryAddress],
                    vec![1, 3, 2],
                    None,
                    Some(0b010),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0000011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "lw rd, offset(rs1)".to_string(),
                        description: "Loads a 32-bit value from memory and sign-extends this to XLEN bits before storing it in register rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "lbu" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, MemoryAddress],
                    vec![1, 3, 2],
                    None,
                    Some(0b100),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0000011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "lbu rd, offset(rs1)".to_string(),
                        description: "Loads a 8-bit value from memory and zero-extends this to XLEN bits before storing it in register rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "lhu" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, MemoryAddress],
                    vec![1, 3, 2],
                    None,
                    Some(0b101),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0000011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "lhu rd, offset(rs1)".to_string(),
                        description: "Loads a 16-bit value from memory and zero-extends this to XLEN bits before storing it in register rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "sb" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, MemoryAddress],
                    vec![1, 3, 2],
                    None,
                    Some(0b000),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0100011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Encoded as I-type, but needs reordering for S-type
                instruction.binary = immediate_to_stored(instruction.binary);
                log!("3. Reordered: ", format!("{:032b}", instruction.binary));

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription {
                        syntax: "sb rs2, offset(rs1)".to_string(),
                        description:
                            "Store 8-bit, values from the low bits of register rs2 to memory."
                                .to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "sh" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, MemoryAddress],
                    vec![1, 3, 2],
                    None,
                    Some(0b001),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0100011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Encoded as I-type, but needs reordering for S-type
                instruction.binary = immediate_to_stored(instruction.binary);
                log!("3. Reordered: ", format!("{:032b}", instruction.binary));

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription {
                        syntax: "sh rs2, offset(rs1)".to_string(),
                        description:
                            "Store 16-bit, values from the low bits of register rs2 to memory."
                                .to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "sw" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, MemoryAddress],
                    vec![1, 3, 2],
                    None,
                    Some(0b010),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0100011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Encoded as I-type, but needs reordering for S-type
                instruction.binary = immediate_to_stored(instruction.binary);
                log!("3. Reordered: ", format!("{:032b}", instruction.binary));

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription {
                        syntax: "sw rs2, offset(rs1)".to_string(),
                        description:
                            "Store 32-bit, values from the low bits of register rs2 to memory."
                                .to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "jal" =>
            // Finish J instructions
            {
                log!("jal instruction");
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Read as U-type instruction and reorder immediate value after
                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, UpperImmediate],
                    vec![1, 2],
                    None,
                    None,
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1101111, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Reorder immediate
                instruction.binary = upper_to_jump(instruction.binary);
                log!(
                    "3. Reordered Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription {
                        syntax: "jal rd, offset".to_string(),
                        description: "Jump to address and place return address in rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "jalr" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, MemoryAddress],
                    vec![1, 3, 2],
                    None,
                    Some(0b000),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1100111, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription {
                        syntax: "jalr rd, rs1, offset".to_string(),
                        description: "Jump to address and place return address in rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "beq" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![1, 2, 3],
                    None,
                    Some(0b000),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1100011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                instruction.binary = immediate_to_branch(instruction.binary);
                log!(
                    "3. Reordered Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription {
                        syntax: "beq rs1, rs2, offset".to_string(),
                        description: "Take the branch if registers rs1 and rs2 are equal."
                            .to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "bne" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![1, 2, 3],
                    None,
                    Some(0b001),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1100011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                instruction.binary = immediate_to_branch(instruction.binary);
                log!(
                    "3. Reordered Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription {
                        syntax: "bne rs1, rs2, offset".to_string(),
                        description: "Take the branch if registers rs1 and rs2 are not equal."
                            .to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "blt" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![1, 2, 3],
                    None,
                    Some(0b100),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1100011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                instruction.binary = immediate_to_branch(instruction.binary);
                log!(
                    "3. Reordered Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "blt rs1, rs2, offset".to_string(),
                        description: "Take the branch if registers rs1 is less than rs2, using signed comparison.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "bge" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![1, 2, 3],
                    None,
                    Some(0b101),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1100011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                instruction.binary = immediate_to_branch(instruction.binary);
                log!(
                    "3. Reordered Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "bge rs1, rs2, offset".to_string(),
                        description: "Take the branch if registers rs1 is greater than or equal to rs2, using signed comparison.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "bltu" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![1, 2, 3],
                    None,
                    Some(0b110),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1100011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                instruction.binary = immediate_to_branch(instruction.binary);
                log!(
                    "3. Reordered Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "bltu rs1, rs2, offset".to_string(),
                        description: "Take the branch if registers rs1 is less than rs2, using unsigned comparison.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "bgeu" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![1, 2, 3],
                    None,
                    Some(0b111),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1100011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                instruction.binary = immediate_to_branch(instruction.binary);
                log!(
                    "3. Reordered Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "bgeu rs1, rs2, offset".to_string(),
                        description: "Take the branch if registers rs1 is greater than or equal to rs2, using unsigned comparison.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "ecall" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // ecall instruction encoding does not change
                instruction.binary = 0b00000000000000000000000001110011;
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "ecall".to_string(),
                        description: "Make a request to the supporting execution environment.\n\nWhen executed in U-mode, S-mode, or M-mode, it generates an environment-call-from-U-mode exception, environment-call-from-S-mode exception, or environment-call-from-M-mode exception, respectively, and performs no other operation.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "ebreak" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // ebreak instruction encoding does not change
                instruction.binary = 0b00000000000100000000000001110011;
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "ebreak".to_string(),
                        description: "Used by debuggers to cause control to be transferred back to a debugging environment.\n\nIt generates a breakpoint exception and performs no other operation.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "uret" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // uret instruction encoding does not change
                instruction.binary = 0b00000000001000000000000001110011;
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "uret".to_string(),
                        description: "Return from traps in U-mode, and URET copies UPIE into UIE, then sets UPIE.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "sret" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // uret instruction encoding does not change
                instruction.binary = 0b00010000001000000000000001110011;
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "uret".to_string(),
                        description: "Return from traps in U-mode, and URET copies UPIE into UIE, then sets UPIE.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "mret" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // uret instruction encoding does not change
                instruction.binary = 0b00110000001000000000000001110011;
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "mret".to_string(),
                        description: "Return from traps in M-mode, and MRET copies MPIE into MIE, then sets MPIE.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "wfi" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // uret instruction encoding does not change
                instruction.binary = 0b00010000010100000000000001110011;
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "wfi".to_string(),
                        description: "Provides a hint to the implementation that the current hart can be stalled until an interrupt might need servicing.\n\nExecution of the WFI instruction can also be used to inform the hardware platform that suitable interrupts should preferentially be routed to this hart.\n\nWFI is available in all privileged modes, and optionally available to U-mode.\n\nThis instruction may raise an illegal instruction exception when TW=1 in mstatus.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fence.i" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // fence.i instruction encoding does not change
                instruction.binary = 0b00000000000000000001000000001111;
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fence.i".to_string(),
                        description: "Provides explicit synchronization between writes to instruction memory and instruction fetches on the same hart.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "lui" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, UpperImmediate],
                    vec![1, 2],
                    None,
                    None,
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0110111, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "lui rd, imm".to_string(),
                        description: "Build 32-bit constants and uses the U-type format. LUI places the U-immediate value in the top 20 bits of the destination register rd, filling in the lowest 12 bits with zeros.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "auipc" => {
                log!("auipc instruction");
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, UpperImmediate],
                    vec![1, 2],
                    None,
                    None,
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0010111, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "auipc rd, imm".to_string(),
                        description: "Build pc-relative addresses and uses the U-type format.\n\nAUIPC forms a 32-bit offset from the 20-bit U-immediate, filling in the lowest 12 bits with zeros, adds this offset to the pc, then places the result in register rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "addiw" =>
            // Start of RV64I Instructions
            {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![1, 2, 3],
                    None,
                    Some(0b000),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0011011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "addiw rd, rs1, imm".to_string(),
                        description: "Adds the sign-extended 12-bit immediate to register rs1 and produces the proper sign-extension of a 32-bit result in rd.\n\nOverflows are ignored and the result is the low 32 bits of the result sign-extended to 64 bits.\n\nNote, ADDIW rd, rs1, 0 writes the sign-extension of the lower 32 bits of register rs1 into register rd (assembler pseudoinstruction SEXT.W).".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "slliw" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0000000, 7);

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, ShiftAmount],
                    vec![1, 2, 3],
                    None,
                    Some(0b001),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0011011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "slliw rd, rs1, shamt".to_string(),
                        description: "Performs logical left shift on the 32-bit of value in register rs1 by the shift amount held in the lower 5 bits of the immediate.\n\nEncodings with $imm[5] neq 0$ are reserved.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "srliw" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0000000, 7);

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, ShiftAmount],
                    vec![1, 2, 3],
                    None,
                    Some(0b101),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0011011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "srliw rd, rs1, shamt".to_string(),
                        description: "Performs logical right shift on the 32-bit of value in register rs1 by the shift amount held in the lower 5 bits of the immediate.\n\nEncodings with $imm[5] neq 0$ are reserved.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "sraiw" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b01000, 7);

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, ShiftAmount],
                    vec![1, 2, 3],
                    None,
                    Some(0b101),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0011011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "sraiw rd, rs1, shamt".to_string(),
                        description: "Performs arithmetic right shift on the 32-bit of value in register rs1 by the shift amount held in the lower 5 bits of the immediate.\n\nEncodings with $imm[5] neq 0$ are reserved.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "addw" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0000000, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b000),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0111011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "addw rd, rs1, rs2".to_string(),
                        description: "Adds the 32-bit of registers rs1 and 32-bit of register rs2 and stores the result in rd.\n\nArithmetic overflow is ignored and the low 32-bits of the result is sign-extended to 64-bits and written to the destination register.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "subw" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0100000, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b000),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0111011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "subw rd, rs1, rs2".to_string(),
                        description: "Subtract the 32-bit of registers rs1 and 32-bit of register rs2 and stores the result in rd.\n\nArithmetic overflow is ignored and the low 32-bits of the result is sign-extended to 64-bits and written to the destination register.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "sllw" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0000000, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b001),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0111011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "sllw rd, rs1, rs2".to_string(),
                        description: "Performs logical left shift on the low 32-bits value in register rs1 by the shift amount held in the lower 5 bits of register rs2 and produce 32-bit results and written to the destination register rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "srlw" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0000000, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b101),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0111011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "srlw rd, rs1, rs2".to_string(),
                        description: "Performs logical right shift on the low 32-bits value in register rs1 by the shift amount held in the lower 5 bits of register rs2 and produce 32-bit results and written to the destination register rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "sraw" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0100000, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b101),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0111011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "sraw rd, rs1, rs2".to_string(),
                        description: "Performs arithmetic right shift on the low 32-bits value in register rs1 by the shift amount held in the lower 5 bits of register rs2 and produce 32-bit results and written to the destination register rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "lwu" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, MemoryAddress],
                    vec![1, 3, 2],
                    None,
                    Some(0b110),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0000011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "lwu rd, offset(rs1)".to_string(),
                        description: "Loads a 32-bit value from memory and zero-extends this to 64 bits before storing it in register rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "ld" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, MemoryAddress],
                    vec![1, 3, 2],
                    None,
                    Some(0b011),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0000011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription {
                        syntax: "ld rd, offset(rs1)".to_string(),
                        description: "Loads a 64-bit value from memory into register rd for RV64I."
                            .to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "sd" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, MemoryAddress],
                    vec![1, 3, 2],
                    None,
                    Some(0b011),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0100011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Encoded as I-type, but needs reordering for S-type
                instruction.binary = immediate_to_stored(instruction.binary);
                log!("3. Reordered: ", format!("{:032b}", instruction.binary));

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription {
                        syntax: "sd rs2, offset(rs1)".to_string(),
                        description: "Store 64-bit, values from register rs2 to memory."
                            .to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "mul" =>
            // Start of RV32M
            {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0000001, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b000),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0110011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "mul rd, rs1, rs2".to_string(),
                        description: "Performs an XLEN-bit * XLEN-bit multiplication of signed rs1 by signed rs2 and places the lower XLEN bits in the destination register.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "mulh" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0000001, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b001),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0110011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "mulh rd, rs1, rs2".to_string(),
                        description: "Performs an XLEN-bit * XLEN-bit multiplication of signed rs1 by signed rs2 and places the upper XLEN bits in the destination register.
                       ".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "mulhsu" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0000001, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b010),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0110011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "mulhsu rd, rs1, rs2".to_string(),
                        description: "Performs an XLEN-bit * XLEN-bit multiplication of signed rs1 by unsigned rs2 and places the upper XLEN bits in the destination register.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "mulhu" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0000001, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b011),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0110011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "mulhu rd, rs1, rs2".to_string(),
                        description: "Performs an XLEN-bit * XLEN-bit multiplication of unsigned rs1 by unsigned rs2 and places the upper XLEN bits in the destination register.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "div" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0000001, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b100),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0110011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "div rd, rs1, rs2".to_string(),
                        description: "Perform an XLEN bits by XLEN bits signed integer division of rs1 by rs2, rounding towards zero.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "divu" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0000001, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b101),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0110011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "divu rd, rs1, rs2".to_string(),
                        description: "Perform an XLEN bits by XLEN bits unsigned integer division of rs1 by rs2, rounding towards zero.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "rem" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0000001, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b110),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0110011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "rem rd, rs1, rs2".to_string(),
                        description: "Perform an XLEN bits by XLEN bits signed integer remainder of rs1 by rs2.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "remu" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0000001, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b111),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0110011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "remu rd, rs1, rs2".to_string(),
                        description: "Perform an XLEN bits by XLEN bits unsigned integer remainder of rs1 by rs2.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "mulw" =>
            // Start of RV64M
            {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0000001, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b000),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0111011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "mulw rd, rs1, rs2".to_string(),
                        description: "Multiplies the lower 32 bits of the source registers, placing the sign-extension of the lower 32 bits of the result into the destination register.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "divw" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0000001, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b100),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0111011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "divw rd, rs1, rs2".to_string(),
                        description: "Perform an XLEN bits by XLEN bits signed integer division of rs1 by rs2.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "divuw" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0000001, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b101),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0111011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "divuw rd, rs1, rs2".to_string(),
                        description: "Perform an XLEN bits by XLEN bits unsigned integer division of rs1 by rs2.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "remw" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0000001, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b110),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0111011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "remw rd, rs1, rs2".to_string(),
                        description: "Perform an XLEN bits by XLEN bits signed integer remainder of rs1 by rs2.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "remuw" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct7
                instruction.binary = append_binary(instruction.binary, 0b0000001, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![1, 2, 3],
                    None,
                    Some(0b111),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0111011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "remuw rd, rs1, rs2".to_string(),
                        description: "Perform an XLEN bits by XLEN bits unsigned integer reminder of rs1 by rs2.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fmadd.s" =>
            // Start of RV32F
            {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP, RegisterFP],
                    vec![1, 2, 3, 4],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    Some(0b00),
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1000011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fmadd.s rd, rs1, rs2, rs3".to_string(),
                        description: "Multiplies the values in rs1 and rs2, adds the value in rs3, and writes the final result to rd.\n\nFMADD.S computes (rs1 * rs2) + rs3".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fmsub.s" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP, RegisterFP],
                    vec![1, 2, 3, 4],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    Some(0b00),
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1000111, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fmsub.s rd, rs1, rs2, rs3".to_string(),
                        description: "Multiplies the values in rs1 and rs2, subtracts the value in rs3, and writes the final result to rd.\n\nFMSUB.S computes (rs1×rs2) - rs3.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fnmsub.s" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP, RegisterFP],
                    vec![1, 2, 3, 4],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    Some(0b00),
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1001011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fnmsub.s rd, rs1, rs2, rs3".to_string(),
                        description: "Multiplies the values in rs1 and rs2, negates the product, adds the value in rs3, and writes the final result to rd.\n\nFNMSUB.S computes -(rs1 * rs2) + rs3.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fnmadd.s" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP, RegisterFP],
                    vec![1, 2, 3, 4],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    Some(0b00),
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1001111, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fnmadd.s rd, rs1, rs2, rs3".to_string(),
                        description: "multiplies the values in rs1 and rs2, negates the product, subtracts the value in rs3, and writes the final result to rd.\n\nFNMADD.S computes -(rs1 * rs2) - rs3.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fadd.s" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt
                instruction.binary = append_binary(instruction.binary, 0b0000000, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![1, 2, 3],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription {
                        syntax: "fadd.s rd, rs1, rs2".to_string(),
                        description: "Perform single-precision floating-point addition."
                            .to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fsub.s" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt
                instruction.binary = append_binary(instruction.binary, 0b0000100, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![1, 2, 3],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription {
                        syntax: "fsub.s rd, rs1, rs2".to_string(),
                        description: "Perform single-precision floating-point substraction."
                            .to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fmul.s" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt
                instruction.binary = append_binary(instruction.binary, 0b0001000, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![1, 2, 3],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription {
                        syntax: "fmul.s rd, rs1, rs2".to_string(),
                        description: "Perform single-precision floating-point multiplication."
                            .to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fdiv.s" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt
                instruction.binary = append_binary(instruction.binary, 0b0001100, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![1, 2, 3],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription {
                        syntax: "fdiv.s rd, rs1, rs2".to_string(),
                        description: "Perform single-precision floating-point division."
                            .to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fsqrt.s" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt + padding for absent register
                instruction.binary = append_binary(instruction.binary, 0b010110000000, 12);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP],
                    vec![1, 2],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription {
                        syntax: "fsqrt.s rd, rs1".to_string(),
                        description: "Perform single-precision square root.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fsgnj.s" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt
                instruction.binary = append_binary(instruction.binary, 0b0010000, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![1, 2, 3],
                    None,
                    Some(0b000), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fsgnj.s rd, rs1, rs2".to_string(),
                        description: "Produce a result that takes all bits except the sign bit from rs1.\n\nThe result's sign bit is rs2's sign bit.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fsgnjn.s" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt
                instruction.binary = append_binary(instruction.binary, 0b0010000, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![1, 2, 3],
                    None,
                    Some(0b001), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fsgnjn.s rd, rs1, rs2".to_string(),
                        description: "Produce a result that takes all bits except the sign bit from rs1.\n\nThe result's sign bit is opposite of rs2's sign bit.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fsgnjx.s" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt
                instruction.binary = append_binary(instruction.binary, 0b0010000, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![1, 2, 3],
                    None,
                    Some(0b010), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fsgnjx.s rd, rs1, rs2".to_string(),
                        description: "Produce a result that takes all bits except the sign bit from rs1.\n\nThe result's sign bit is XOR of sign bit of rs1 and rs2.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fmin.s" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt
                instruction.binary = append_binary(instruction.binary, 0b0010100, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![1, 2, 3],
                    None,
                    Some(0b000), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription {
                        syntax: "fmin.s rd, rs1, rs2".to_string(),
                        description:
                            "Write the smaller of single precision data in rs1 and rs2 to rd."
                                .to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fmax.s" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt
                instruction.binary = append_binary(instruction.binary, 0b0010100, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![1, 2, 3],
                    None,
                    Some(0b001), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription {
                        syntax: "fmax.s rd, rs1, rs2".to_string(),
                        description:
                            "Write the larger of single precision data in rs1 and rs2 to rd."
                                .to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fcvt.w.s" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt + padding for absent register
                instruction.binary = append_binary(instruction.binary, 0b110000000000, 12);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterFP],
                    vec![1, 2],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fcvt.w.s rd, rs1".to_string(),
                        description: "Convert a floating-point number in floating-point register rs1 to a signed 32-bit in integer register rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fcvt.wu.s" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt + rs2
                instruction.binary = append_binary(instruction.binary, 0b110000000001, 12);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterFP],
                    vec![1, 2],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fcvt.wu.s rd, rs1".to_string(),
                        description: "Convert a floating-point number in floating-point register rs1 to a signed 32-bit in unsigned integer register rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fmv.x.w" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt + rs2
                instruction.binary = append_binary(instruction.binary, 0b111000000000, 12);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterFP],
                    vec![1, 2],
                    None,
                    Some(0b000), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fmv.x.w rd, rs1".to_string(),
                        description: "Move the single-precision value in floating-point register rs1 represented in IEEE 754-2008 encoding to the lower 32 bits of integer register rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "feq.s" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt
                instruction.binary = append_binary(instruction.binary, 0b1010000, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterFP, RegisterFP],
                    vec![1, 2, 3],
                    None,
                    Some(0b010), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "feq.s rd, rs1, rs2".to_string(),
                        description: "Performs a quiet equal comparison between floating-point registers rs1 and rs2 and record the Boolean result in integer register rd.\n\nOnly signaling NaN inputs cause an Invalid Operation exception.\n\nThe result is 0 if either operand is NaN.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "flt.s" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt
                instruction.binary = append_binary(instruction.binary, 0b1010000, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterFP, RegisterFP],
                    vec![1, 2, 3],
                    None,
                    Some(0b001), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "flt.s rd, rs1, rs2".to_string(),
                        description: "Performs a quiet less comparison between floating-point registers rs1 and rs2 and record the Boolean result in integer register rd.\n\nOnly signaling NaN inputs cause an Invalid Operation exception.\n\nThe result is 0 if either operand is NaN.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fle.s" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt
                instruction.binary = append_binary(instruction.binary, 0b1010000, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterFP, RegisterFP],
                    vec![1, 2, 3],
                    None,
                    Some(0b000), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fle.s rd, rs1, rs2".to_string(),
                        description: "Performs a quiet less or equal comparison between floating-point registers rs1 and rs2 and record the Boolean result in integer register rd.\n\nOnly signaling NaN inputs cause an Invalid Operation exception.\n\nThe result is 0 if either operand is NaN.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fclass.s" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt + rs2
                instruction.binary = append_binary(instruction.binary, 0b111000000000, 12);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterFP],
                    vec![1, 2],
                    None,
                    Some(0b001), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fclass.s rd, rs1".to_string(),
                        description: "Examines the value in floating-point register rs1 and writes to integer register rd a 10-bit mask that indicates the class of the floating-point number.\n\nThe corresponding bit in rd will be set if the property is true and clear otherwise. All other bits in rd are cleared.\n\nNote that exactly one bit in rd will be set.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fcvt.s.w" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt + rs2
                instruction.binary = append_binary(instruction.binary, 0b110100000000, 12);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterGP],
                    vec![1, 2],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fcvt.s.w rd, rs1".to_string(),
                        description: "Converts a 32-bit signed integer, in integer register rs1 into a floating-point number in floating-point register rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fcvt.s.wu" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt + rs2
                instruction.binary = append_binary(instruction.binary, 0b110100000001, 12);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterGP],
                    vec![1, 2],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fcvt.s.wu rd, rs1".to_string(),
                        description: "Converts a 32-bit unsigned integer, in integer register rs1 into a floating-point number in floating-point register rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fmv.w.x" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt + rs2
                instruction.binary = append_binary(instruction.binary, 0b111100000000, 12);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterGP],
                    vec![1, 2],
                    None,
                    Some(0b000), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fmv.w.x rd, rs1".to_string(),
                        description: "Move the single-precision value encoded in IEEE 754-2008 standard encoding from the lower 32 bits of integer register rs1 to the floating-point register rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fmadd.d" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP, RegisterFP],
                    vec![1, 2, 3, 4],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    Some(0b01),
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1000011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fmadd.d rd, rs1, rs2, rs3".to_string(),
                        description: "Multiplies the values in rs1 and rs2, adds the value in rs3, and writes the final result tord.\n\nFMADD.D computes (rs1 * rs2) + rs3.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fmsub.d" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP, RegisterFP],
                    vec![1, 2, 3, 4],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    Some(0b01),
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1000111, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fmsub.d rd, rs1, rs2, rs3".to_string(),
                        description: "Multiplies the values in rs1 and rs2, subtracts the value in rs3, and writes the final result to rd. FMSUB.D computes (rs1 * rs2) - rs3".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fnmsub.d" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP, RegisterFP],
                    vec![1, 2, 3, 4],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    Some(0b01),
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1001011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fnmsub.d rd, rs1, rs2, rs3".to_string(),
                        description: "Multiplies the values in rs1 and rs2, negates the product, adds the value in rs3, and writes the final result to rd.\n\nFNMSUB.D computes -(rs1 * rs2) + rs3.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fnmadd.d" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP, RegisterFP],
                    vec![1, 2, 3, 4],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    Some(0b01),
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1001111, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fnmadd.d rd, rs1, rs2, rs3".to_string(),
                        description: "Multiplies the values in rs1 and rs2, negates the product, subtracts the value in rs3, and writes the final result to rd.\n\nFNMADD.D computes -(rs1 * rs2) - rs3".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fadd.d" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt
                instruction.binary = append_binary(instruction.binary, 0b0000001, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![1, 2, 3],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription {
                        syntax: "fadd.d rd, rs1, rs2".to_string(),
                        description: "Perform single-precision floating-point addition."
                            .to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fsub.d" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt
                instruction.binary = append_binary(instruction.binary, 0b0000101, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![1, 2, 3],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription {
                        syntax: "fsub.d rd, rs1, rs2".to_string(),
                        description: "Perform single-precision floating-point subtraction."
                            .to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fmul.d" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt
                instruction.binary = append_binary(instruction.binary, 0b0001001, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![1, 2, 3],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription {
                        syntax: "fmul.d rd, rs1, rs2".to_string(),
                        description: "Perform single-precision floating-point multiplication."
                            .to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fdiv.d" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt
                instruction.binary = append_binary(instruction.binary, 0b0001101, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![1, 2, 3],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription {
                        syntax: "fdiv.d rd, rs1, rs2".to_string(),
                        description: "Perform single-precision floating-point division."
                            .to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fsqrt.d" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt + rs2
                instruction.binary = append_binary(instruction.binary, 0b010110100000, 12);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP],
                    vec![1, 2],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription {
                        syntax: "fsqrt.d rd, rs1".to_string(),
                        description: "Perform single-precision square root.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fsgnj.d" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt
                instruction.binary = append_binary(instruction.binary, 0b0010001, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![1, 2, 3],
                    None,
                    Some(0b000), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fsgnj.d rd, rs1, rs2".to_string(),
                        description: "Produce a result that takes all bits except the sign bit from rs1.\n\nThe result's sign bit is rs2's sign bit.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fsgnjn.d" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt
                instruction.binary = append_binary(instruction.binary, 0b0010001, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![1, 2, 3],
                    None,
                    Some(0b001), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fsgnjn.d rd, rs1, rs2".to_string(),
                        description: "Produce a result that takes all bits except the sign bit from rs1.\n\nThe result's sign bit is opposite of rs2's sign bit.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fsgnjx.d" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt
                instruction.binary = append_binary(instruction.binary, 0b0010001, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![1, 2, 3],
                    None,
                    Some(0b010), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fsgnjx.d rd, rs1, rs2".to_string(),
                        description: "Produce a result that takes all bits except the sign bit from rs1.\n\nThe result's sign bit is XOR of sign bit of rs1 and rs2.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fmin.d" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt
                instruction.binary = append_binary(instruction.binary, 0b0010101, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![1, 2, 3],
                    None,
                    Some(0b000), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription {
                        syntax: "fmin.d rd, rs1, rs2".to_string(),
                        description:
                            "Write the smaller of single precision data in rs1 and rs2 to rd."
                                .to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fmax.d" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt
                instruction.binary = append_binary(instruction.binary, 0b0010101, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP, RegisterFP],
                    vec![1, 2, 3],
                    None,
                    Some(0b001), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription {
                        syntax: "fmax.d rd, rs1, rs2".to_string(),
                        description:
                            "Write the larger of single precision data in rs1 and rs2 to rd."
                                .to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fcvt.s.d" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt + rs2
                instruction.binary = append_binary(instruction.binary, 0b010000000001, 12);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP],
                    vec![1, 2],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fcvt.s.d rd, rs1".to_string(),
                        description: "Converts double floating-point register in rs1 into a floating-point number in floating-point register rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fcvt.d.s" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt + rs2
                instruction.binary = append_binary(instruction.binary, 0b010000100000, 12);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterFP],
                    vec![1, 2],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fcvt.d.s rd, rs1".to_string(),
                        description: "Converts single floating-point register in rs1 into a double floating-point number in floating-point register rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "feq.d" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt
                instruction.binary = append_binary(instruction.binary, 0b1010001, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterFP, RegisterFP],
                    vec![1, 2, 3],
                    None,
                    Some(0b010), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "feq.d rd, rs1, rs2".to_string(),
                        description: "Performs a quiet equal comparison between floating-point registers rs1 and rs2 and record the Boolean result in integer register rd.\n\nOnly signaling NaN inputs cause an Invalid Operation exception.\n\nThe result is 0 if either operand is NaN.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "flt.d" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt
                instruction.binary = append_binary(instruction.binary, 0b1010001, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterFP, RegisterFP],
                    vec![1, 2, 3],
                    None,
                    Some(0b001), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "flt.d rd, rs1, rs2".to_string(),
                        description: "Performs a quiet less comparison between floating-point registers rs1 and rs2 and record the Boolean result in integer register rd.\n\nOnly signaling NaN inputs cause an Invalid Operation exception.\n\nThe result is 0 if either operand is NaN.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fle.d" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt
                instruction.binary = append_binary(instruction.binary, 0b1010001, 7);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterFP, RegisterFP],
                    vec![1, 2, 3],
                    None,
                    Some(0b000), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fle.d rd, rs1, rs2".to_string(),
                        description: "Performs a quiet less or equal comparison between floating-point registers rs1 and rs2 and record the Boolean result in integer register rd.\n\nOnly signaling NaN inputs cause an Invalid Operation exception.\n\nThe result is 0 if either operand is NaN.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fclass.d" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt + rs2
                instruction.binary = append_binary(instruction.binary, 0b111000100000, 12);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterFP],
                    vec![1, 2],
                    None,
                    Some(0b001), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fclass.d rd, rs1".to_string(),
                        description: "Examines the value in floating-point register rs1 and writes to integer register rd a 10-bit mask that indicates the class of the floating-point number.\n\nThe corresponding bit in rd will be set if the property is true and clear otherwise.\nAll other bits in rd are cleared.\n\nNote that exactly one bit in rd will be set.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fcvt.w.d" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt + rs2
                instruction.binary = append_binary(instruction.binary, 0b110000100000, 12);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterFP],
                    vec![1, 2],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fcvt.w.d rd, rs1".to_string(),
                        description: "Converts a double-precision floating-point number in floating-point register rs1 to a signed 32-bit integer, in integer register rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fcvt.wu.d" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt + rs2
                instruction.binary = append_binary(instruction.binary, 0b110000100001, 12);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterFP],
                    vec![1, 2],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fcvt.wu.d rd, rs1".to_string(),
                        description: "Converts a double-precision floating-point number in floating-point register rs1 to a unsigned 32-bit integer, in integer register rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fcvt.d.w" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt + rs2
                instruction.binary = append_binary(instruction.binary, 0b110100100000, 12);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterGP],
                    vec![1, 2],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fcvt.d.w rd, rs1".to_string(),
                        description: "Converts a 32-bit signed integer, in integer register rs1 into a double-precision floating-point number in floating-point register rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fcvt.d.wu" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt + rs2
                instruction.binary = append_binary(instruction.binary, 0b110100100001, 12);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterGP],
                    vec![1, 2],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fcvt.d.wu rd, rs1".to_string(),
                        description: "Converts a 32-bit unsigned integer, in integer register rs1 into a double-precision floating-point number in floating-point register rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "flw" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, MemoryAddress],
                    vec![1, 3, 2],
                    None,
                    Some(0b010),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0000111, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "flw rd, offset(rs1)".to_string(),
                        description: "Load a single-precision floating-point value from memory into floating-point register rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fsw" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, MemoryAddress],
                    vec![1, 3, 2],
                    None,
                    Some(0b010),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0100111, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Encoded as I-type, but needs reordering for S-type
                instruction.binary = immediate_to_stored(instruction.binary);
                log!("3. Reordered: ", format!("{:032b}", instruction.binary));

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fsw rs2, offset(rs1)".to_string(),
                        description: "Store a single-precision value from floating-point register rs2 to memory.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fld" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, MemoryAddress],
                    vec![1, 3, 2],
                    None,
                    Some(0b011),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0000111, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fld rd, rs1, offset".to_string(),
                        description: "Load a double-precision floating-point value from memory into floating-point register rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fsd" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, MemoryAddress],
                    vec![1, 3, 2],
                    None,
                    Some(0b011),
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b0100111, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Encoded as I-type, but needs reordering for S-type
                instruction.binary = immediate_to_stored(instruction.binary);
                log!("3. Reordered: ", format!("{:032b}", instruction.binary));

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fsd rs2, offset(rs1)".to_string(),
                        description: "Store a double-precision value from the floating-point registers to memory.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fcvt.l.s" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt + rs2
                instruction.binary = append_binary(instruction.binary, 0b110000000010, 12);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterFP],
                    vec![1, 2],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fcvt.l.s rd, rs1".to_string(),
                        description: "Converts the floating-point value in register rs1 (interpreted as a single-precision floating-point number) to a signed 32-bit integer.\n\nThe result is then stored in integer register rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fcvt.lu.s" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt + rs2
                instruction.binary = append_binary(instruction.binary, 0b110000000011, 12);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterGP, RegisterFP],
                    vec![1, 2],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fcvt.lu.s rd, rs1".to_string(),
                        description: "This instruction converts the floating-point value in register rs1 (interpreted as a single-precision floating-point number) to an unsigned 32-bit integer.\n\nThe result is then stored in integer register rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fcvt.s.l" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt + rs2
                instruction.binary = append_binary(instruction.binary, 0b110100000010, 12);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterGP],
                    vec![1, 2],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fcvt.s.l rd, rs1".to_string(),
                        description: "Converts the signed 32-bit integer value in register rs1 to a single-precision floating-point number.\n\nThe result is then stored in floating-point register rd.".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            "fcvt.s.lu" => {
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                // Funct5 + fmt + rs2
                instruction.binary = append_binary(instruction.binary, 0b110100000011, 12);
                log!(
                    "Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                read_operands_riscv(
                    instruction,
                    vec![RegisterFP, RegisterGP],
                    vec![1, 2],
                    None,
                    Some(0b111), // This funct3 is used for the rm value
                    None,
                );

                // Opcode
                instruction.binary = append_binary(instruction.binary, 0b1010011, 7);
                log!(
                    "2. Instruction Binary: ",
                    format!("{:032b}", instruction.binary)
                );

                //Pseudo-instructions already have text in mouse_hover_string so we check if there's text there already before adding in the blurb
                if monaco_line_info[instruction.line_number]
                    .mouse_hover_string
                    .is_empty()
                {
                    let info = InstructionDescription{
                        syntax: "fcvt.s.lu rd, rs1".to_string(),
                        description: "Converts a single-precision floating-point value to an unsigned 32-bit integer.\n\nTakes the value in a floating-point register (f-register), performs the conversion, and stores the result in an integer register (x-register).".to_string(),
                    };
                    monaco_line_info[instruction.line_number].mouse_hover_string = info.to_string();
                }
            }
            _ => {
                if UNSUPPORTED_INSTRUCTIONS.contains(&&*instruction.operator.token_name) {
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

// Reorder store instruction to the correct format
fn immediate_to_stored(mut bin: u32) -> u32 {
    // Extract bits 24-20 from the first segment
    let lower_imm = (bin >> 20) & 0b11111;

    // Extract bits 11-7 from the second segment
    let rs2 = (bin >> 7) & 0b11111;

    log!("Bits to move (24-20): ", format!("{:05b}", lower_imm));
    log!("Bits to move (11-7): ", format!("{:05b}", rs2));

    // Clear bits 24-20 and 11-7
    bin &= !((0b11111 << 20) | (0b11111 << 6));

    log!("Cleared bits: ", format!("{:032b}", bin));

    // Move bits 24-20 to positions 11-7
    let moved_imm = lower_imm << 7;

    // Move bits 11-7 to positions 24-20
    let moved_rs2 = rs2 << 20;

    // Combine the manipulated bits
    bin |= moved_imm | moved_rs2;

    bin
}

// Converts an I-type instruction to B-type instruction
// Easier to encode in this manner
fn immediate_to_branch(mut bin: u32) -> u32 {
    log!("Binary: ", format!("{:032b}", bin));
    log!("Immediate: ", format!("{:012b}", (bin >> 20)));

    // Extract bits imm[4:1] from the immediate, last bit is ignored
    let lower_imm = (bin >> 21) & 0b1111;

    // Extract imm[10:5]
    let upper_imm = (bin >> 24) & 0b111111;

    // Extract bit 11 and bit 12
    let bit_11 = (bin >> 30) & 0b1;
    let bit_12 = (bin >> 31) & 0b1;

    // Extract rs1 and rs2
    let rs1 = (bin >> 7) & 0b11111;
    let rs2 = (bin >> 15) & 0b11111;

    // Clear bits 24-20, rs1, and rs2
    bin &= !((0b11111 << 20) | (0b11111 << 15) | (0b11111 << 7));

    // Move bits imm[4:1] to positions 10-8
    let moved_lower = lower_imm << 8;

    let moved_upper = upper_imm << 25;

    let moved_bit_11 = bit_11 << 7;

    let moved_bit_12 = bit_12 << 31;

    // Move rs2 to positions 24-20
    let moved_rs2 = rs2 << 20;

    // Move rs1 to positions 15-19
    let moved_rs1 = rs1 << 15;

    // Combine the manipulated bits
    bin |= moved_bit_12 | moved_upper | moved_rs2 | moved_rs1 | moved_lower | moved_bit_11;

    bin
}

// Reorder the immediate value to comply with J-type format
fn upper_to_jump(mut bin: u32) -> u32 {
    // Extract bits 24-20 from the first segment
    let lower_imm = (bin >> 20) & 0b11111;

    // Extract bits 11-7 from the second segment
    let rs2 = (bin >> 7) & 0b11111;

    log!("Bits to move (24-20): ", format!("{:05b}", lower_imm));
    log!("Bits to move (11-7): ", format!("{:05b}", rs2));

    // Clear bits 24-20 and 11-7
    bin &= !((0b11111 << 20) | (0b11111 << 6));

    log!("Cleared bits: ", format!("{:032b}", bin));

    // Move bits 24-20 to positions 11-7
    let moved_imm = lower_imm << 7;

    // Move bits 11-7 to positions 24-20
    let moved_rs2 = rs2 << 20;

    // Combine the manipulated bits
    bin |= moved_imm | moved_rs2;

    bin
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

///returns the address of the labelled main instruction. If none exists, returns address of labelled start instruction.
///Otherwise returns 0.
pub fn determine_pc_starting_point(labels: HashMap<String, usize>) -> usize {
    return match labels.get("main") {
        Some(main_address) => *main_address,
        None => match labels.get("start") {
            Some(start_address) => *start_address,
            None => 0,
        },
    };
}

///Creates a vector of u32 from the data found in the parser / assembler to put into memory.
pub fn create_binary_vec(
    instructions: Vec<Instruction>,
    mut vec_of_data: Vec<u8>,
) -> (Vec<u32>, usize) {
    //push all instructions
    let mut binary: Vec<u32> = Vec::new();
    for instruction in instructions {
        binary.push(instruction.binary);
    }

    let data_starting_point = binary.len();

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

    (binary, data_starting_point)
}
