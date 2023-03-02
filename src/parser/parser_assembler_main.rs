use crate::parser::assembling::{assemble_data_binary, read_operands};
use crate::parser::parser_structs_and_enums::instruction_tokenization::ErrorType::*;
use crate::parser::parser_structs_and_enums::instruction_tokenization::OperandType::*;
use crate::parser::parser_structs_and_enums::instruction_tokenization::ProgramInfo;
use crate::parser::parser_structs_and_enums::instruction_tokenization::*;
use crate::parser::parsing::*;
use std::collections::HashMap;

///Parser is the starting function of the parser / assembler process. It takes a string representation of a MIPS
/// program and builds the binary of the instructions while cataloging any errors that are found.
pub fn parser(file_string: String) -> (ProgramInfo, Vec<u32>) {
    let mut program_info = ProgramInfo::default();
    let (lines, mut updated_monaco_strings, mut monaco_line_info_vec) = tokenize_program(file_string);
    (program_info.instructions, program_info.data) = separate_data_and_text(lines);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut updated_monaco_strings,
        &mut monaco_line_info_vec
    );
    let vec_of_data = assemble_data_binary(&mut program_info.data);
    let labels: HashMap<String, u32> =
        create_label_map(&mut program_info.instructions, &mut program_info.data);
    complete_lw_sw_pseudo_instructions(
        &mut program_info.instructions,
        &labels,
        &mut updated_monaco_strings,
    );
    read_instructions(&mut program_info.instructions, &labels);

    suggest_error_corrections(
        &mut program_info.instructions,
        &mut program_info.data,
        &labels,
    );

    let binary = create_binary_vec(program_info.instructions.clone(), vec_of_data);

    for entry in updated_monaco_strings {
        program_info.updated_monaco_string.push_str(entry.as_str());
        program_info.updated_monaco_string.push('\n');
    }

    for instruction in program_info.instructions.clone() {
        program_info
            .address_to_line_number
            .push(instruction.line_number);
    }

    (program_info.clone(), binary)
}

///Takes the vector of instructions and assembles the binary for them.
pub fn read_instructions(instruction_list: &mut [Instruction], labels: &HashMap<String, u32>) {
    for mut instruction in &mut instruction_list.iter_mut() {
        //this match case is the heart of the parser and figures out which instruction type it is
        //then it can call the proper functions for that specific instruction
        match &*instruction.operator.token_name {
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
            }
            "mul" => {
                instruction.binary = append_binary(instruction.binary, 0b011100, 6);

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, RegisterGP],
                    vec![2, 3, 1],
                    None,
                );

                instruction.binary = append_binary(instruction.binary, 0b00000, 5);
                instruction.binary = append_binary(instruction.binary, 0b000010, 6);
            }
            "div" => {
                instruction.binary = append_binary(instruction.binary, 0b000000, 6);

                read_operands(instruction, vec![RegisterGP, RegisterGP], vec![1, 2], None);

                instruction.binary = append_binary(instruction.binary, 0b0000000000, 10);
                instruction.binary = append_binary(instruction.binary, 0b011010, 6);
            }
            "lw" => {
                instruction.binary = append_binary(instruction.binary, 0b100011, 6);

                read_operands(
                    instruction,
                    vec![RegisterGP, MemoryAddress],
                    vec![3, 1, 2],
                    None,
                );
            }
            "sw" => {
                instruction.binary = append_binary(instruction.binary, 0b101011, 6);

                read_operands(
                    instruction,
                    vec![RegisterGP, MemoryAddress],
                    vec![3, 1, 2],
                    None,
                );
            }
            "lui" => {
                instruction.binary = append_binary(instruction.binary, 0b001111, 6);
                instruction.binary = append_binary(instruction.binary, 0b00000, 5);

                read_operands(instruction, vec![RegisterGP, Immediate], vec![1, 2], None);
            }
            "aui" => {
                instruction.binary = append_binary(instruction.binary, 0b001111, 6);

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![2, 1, 3],
                    None,
                );
            }
            "andi" => {
                instruction.binary = append_binary(instruction.binary, 0b001100, 6);

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![2, 1, 3],
                    None,
                );
            }
            "ori" => {
                instruction.binary = append_binary(instruction.binary, 0b001101, 6);

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![2, 1, 3],
                    None,
                );
            }
            "addi" => {
                instruction.binary = append_binary(instruction.binary, 0b001000, 6);

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![2, 1, 3],
                    None,
                );
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
            }
            "ddiv" => {
                instruction.binary = append_binary(instruction.binary, 0b000000, 6);

                read_operands(instruction, vec![RegisterGP, RegisterGP], vec![1, 2], None);

                instruction.binary = append_binary(instruction.binary, 0b0000000000, 10);
                instruction.binary = append_binary(instruction.binary, 0b011110, 6);
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
            }
            "dahi" => {
                instruction.binary = append_binary(instruction.binary, 0b000001, 6); //regimm

                read_operands(instruction, vec![RegisterGP, Immediate], vec![1, 2], None);

                instruction.binary =
                    place_binary_in_middle_of_another(instruction.binary, 0b00110, 5, 15);
            }
            "dati" => {
                instruction.binary = append_binary(instruction.binary, 0b000001, 6); //regimm

                read_operands(instruction, vec![RegisterGP, Immediate], vec![1, 2], None);

                instruction.binary =
                    place_binary_in_middle_of_another(instruction.binary, 0b11110, 5, 15);
            }
            "daddiu" => {
                instruction.binary = append_binary(instruction.binary, 0b011001, 6); //daddiu

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, Immediate],
                    vec![2, 1, 3],
                    None,
                );
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
            }
            "swc1" => {
                instruction.binary = append_binary(instruction.binary, 0b111001, 6); //swc1

                read_operands(
                    instruction,
                    vec![RegisterFP, MemoryAddress],
                    vec![3, 1, 2],
                    None,
                );
            }
            "lwc1" => {
                instruction.binary = append_binary(instruction.binary, 0b110001, 6); //lwc1

                read_operands(
                    instruction,
                    vec![RegisterFP, MemoryAddress],
                    vec![3, 1, 2],
                    None,
                );
            }
            "mtc1" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b00100, 5); //mt

                read_operands(instruction, vec![RegisterGP, RegisterFP], vec![1, 2], None);

                instruction.binary = append_binary(instruction.binary, 0b00000000000, 11);
                //0
            }
            "dmtc1" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b00101, 5); //dmt

                read_operands(instruction, vec![RegisterGP, RegisterFP], vec![1, 2], None);

                instruction.binary = append_binary(instruction.binary, 0b00000000000, 11);
                //0
            }
            "mfc1" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b00000, 5); //mf

                read_operands(instruction, vec![RegisterGP, RegisterFP], vec![1, 2], None);

                instruction.binary = append_binary(instruction.binary, 0b00000000000, 11);
                //0
            }
            "dmfc1" => {
                instruction.binary = append_binary(instruction.binary, 0b010001, 6); //cop1
                instruction.binary = append_binary(instruction.binary, 0b00001, 5); //dmf

                read_operands(instruction, vec![RegisterGP, RegisterFP], vec![1, 2], None);

                instruction.binary = append_binary(instruction.binary, 0b00000000000, 11);
                //0
            }
            "j" => {
                instruction.binary = append_binary(instruction.binary, 0b000010, 6); //j

                read_operands(
                    instruction,
                    vec![LabelAbsolute],
                    vec![1],
                    Some(labels.clone()),
                );
            }
            "beq" => {
                instruction.binary = append_binary(instruction.binary, 0b000100, 6); //beq

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, LabelRelative],
                    vec![1, 2, 3],
                    Some(labels.clone()),
                );
            }
            "bne" => {
                instruction.binary = append_binary(instruction.binary, 0b000101, 6); //bne

                read_operands(
                    instruction,
                    vec![RegisterGP, RegisterGP, LabelRelative],
                    vec![1, 2, 3],
                    Some(labels.clone()),
                );
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
            }
            "syscall" => {
                //our support for syscall is limited. It is simply there to end emulation
                instruction.binary = append_binary(instruction.binary, 0b000000, 6); //special
                instruction.binary = append_binary(instruction.binary, 0b00000000000000000000, 20); //stub of code
                instruction.binary = append_binary(instruction.binary, 0b001100, 6);
                //syscall
            }

            _ => {
                let unsupported_instructions = [
                    "abs.s",
                    "abs.d",
                    "abs.ps",
                    "addiu",
                    "addiupc",
                    "addu",
                    "align",
                    "dalign",
                    "alnv.ps",
                    "aluipc",
                    "daui",
                    "auipc",
                    "b",
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
                    "ddivu",
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
                    "ddivu",
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
                    "dsubu",
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
                    "jalr",
                    "jalr.hb",
                    "jalx",
                    "jialc",
                    "jic",
                    "jr",
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
                    "dmulu",
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
                    "nop",
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
                    "sll",
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
                    "syscall",
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
                        operand_number: None,
                        message: "".to_string(),
                    })
                } else {
                    instruction.errors.push(Error {
                        error_name: UnrecognizedInstruction,
                        operand_number: None,
                        message: "".to_string(),
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
    let mod4 = vec_of_data.len() % 4;
    vec_of_data.resize(vec_of_data.len() + mod4, 255);

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
