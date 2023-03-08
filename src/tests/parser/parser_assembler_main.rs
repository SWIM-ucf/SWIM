#[cfg(test)]
mod parser_main_function_tests {
    use crate::parser::parser_assembler_main::*;

    #[test]
    fn parser_takes_string_and_returns_vec_of_instructions() {
        let results =
            parser("lw $t1, 512($t1)\nadd $t1, $s6, $t2\naddi $t1, $t2, 43690".to_string());

        assert_eq!(
            results.0.instructions[0].binary,
            0b10001101001010010000001000000000
        );
        assert_eq!(
            results.0.instructions[1].binary,
            0b00000010110010100100100000100000
        );
        assert_eq!(
            results.0.instructions[2].binary,
            0b00100001010010011010101010101010
        );
    }
}

mod read_instructions_tests {
    use crate::tests::parser::parser_assembler_main::helper_functions::instruction_parser;

    #[test]
    fn read_instructions_add() {
        let file_string = "add $t1, $s6, $t2".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000010110010100100100000100000
        );
    }

    #[test]
    fn read_instructions_sub() {
        let file_string = "sub $t1, $s6, $t2".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000010110010100100100000100010
        );
    }

    #[test]
    fn read_instructions_mul() {
        let file_string = "mul $t1, $s6, $t2".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01110010110010100100100000000010
        );
    }

    #[test]
    fn read_instructions_div() {
        let file_string = "div $t1, $s6".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001001101100000000000011010
        );
    }

    #[test]
    fn read_instructions_lw() {
        let file_string = "lw $t1, 512($t1)".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b10001101001010010000001000000000
        );
    }

    #[test]
    fn read_instructions_sw() {
        let file_string = "sw $t1, 512($t1)".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b10101101001010010000001000000000
        );
    }

    #[test]
    fn read_instructions_lui() {
        let file_string = "lui $t1, 43690".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00111100000010011010101010101010
        );
    }

    #[test]
    fn read_instructions_aui() {
        let file_string = "aui $t1, $t1, 43690".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00111101001010011010101010101010
        );
    }

    #[test]
    fn read_instructions_addi() {
        let file_string = "addi $t1, $t2, 43690".to_string();

        let instruction_list = instruction_parser(file_string);
        assert_eq!(
            instruction_list[0].binary,
            0b00100001010010011010101010101010
        );
    }

    #[test]
    fn read_instructions_and() {
        let file_string = "and $t1, $s6, $t2".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000010110010100100100000100100
        );
    }

    #[test]
    fn read_instructions_or() {
        let file_string = "or $t1, $s6, $t2".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000010110010100100100000100101
        );
    }

    #[test]
    fn read_instructions_ori() {
        let file_string = "ori $t1, $t2, 43690".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00110101010010011010101010101010
        );
    }

    #[test]
    fn read_instructions_andi() {
        let file_string = "andi $t1, $t2, 43690".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00110001010010011010101010101010
        );
    }

    #[test]
    fn read_instructions_dadd() {
        let file_string = "dadd $t1, $t2, $s6".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001010101100100100000101100
        );
    }

    #[test]
    fn read_instructions_dsub() {
        let file_string = "dsub $t1, $t2, $s6".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001010101100100100000101110
        );
    }

    #[test]
    fn read_instructions_dmul() {
        let file_string = "dmul $t1, $t2, $s6".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001010101100100100010011100
        );
    }

    #[test]
    fn read_instructions_ddiv() {
        let file_string = "ddiv $t1, $t2".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001001010100000000000011110
        );
    }

    #[test]
    fn read_instructions_add_s() {
        let file_string = "add.s $f9, $f10, $f22".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000110000101100101001001000000
        );
    }

    #[test]
    fn read_instructions_add_d() {
        let file_string = "add.d $f9, $f10, $f22".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000110001101100101001001000000
        );
    }

    #[test]
    fn read_instructions_sub_s() {
        let file_string = "sub.s $f9, $f10, $f22".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000110000101100101001001000001
        );
    }

    #[test]
    fn read_instructions_sub_d() {
        let file_string = "sub.d $f9, $f10, $f22".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000110001101100101001001000001
        );
    }

    #[test]
    fn read_instructions_mul_s() {
        let file_string = "mul.s $f9, $f10, $f22".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000110000101100101001001000010
        );
    }

    #[test]
    fn read_instructions_mul_d() {
        let file_string = "mul.d $f9, $f10, $f22".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000110001101100101001001000010
        );
    }

    #[test]
    fn read_instructions_div_s() {
        let file_string = "div.s $f9, $f10, $f22".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000110000101100101001001000011
        );
    }

    #[test]
    fn read_instructions_div_d() {
        let file_string = "div.d $f9, $f10, $f22".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000110001101100101001001000011
        );
    }

    #[test]
    fn read_instructions_dahi() {
        let file_string = "dahi $t1, 43690".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000101001001101010101010101010
        );
    }

    #[test]
    fn read_instructions_dati() {
        let file_string = "dati $t1, 43690".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000101001111101010101010101010
        );
    }

    #[test]
    fn read_instructions_daddiu() {
        let file_string = "daddiu $t1, $t2, 43690".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01100101010010011010101010101010
        );
    }

    #[test]
    fn read_instructions_daddu() {
        let file_string = "daddu $t1, $t2, $t3".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001010010110100100000101101
        );
    }

    #[test]
    fn read_instructions_dsubu() {
        let file_string = "dsubu $t1, $t2, $t3".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001010010110100100000101111
        );
    }

    #[test]
    fn read_instructions_dmulu() {
        let file_string = "dmulu $t1, $t2, $t3".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001010010110100100010011101
        );
    }

    #[test]
    fn read_instructions_ddivu() {
        let file_string = "ddivu $t1, $t2".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001001010100000000000011111
        );
    }

    #[test]
    fn read_instructions_slt() {
        let file_string = "slt $t1, $t2, $s6".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001010101100100100000101010
        );
    }

    #[test]
    fn read_instructions_sltu() {
        let file_string = "sltu $t1, $t2, $s6".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001010101100100100000101011
        );
    }

    #[test]
    fn read_instructions_swc1() {
        let file_string = "swc1 $f9, 43690($t2)".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b11100101010010011010101010101010
        );
    }

    #[test]
    fn read_instructions_lwc1() {
        let file_string = "lwc1 $f9, 43690($t2)".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b11000101010010011010101010101010
        );
    }

    #[test]
    fn read_instructions_mtc1() {
        let file_string = "mtc1 $t1, $f22".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000100100010011011000000000000
        );
    }

    #[test]
    fn read_instructions_dmtc1() {
        let file_string = "dmtc1 $t1, $f22".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000100101010011011000000000000
        );
    }

    #[test]
    fn read_instructions_mfc1() {
        let file_string = "mfc1 $t1, $f22".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000100000010011011000000000000
        );
    }

    #[test]
    fn read_instructions_dmfc1() {
        let file_string = "dmfc1 $t1, $f22".to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000100001010011011000000000000
        );
    }

    #[test]
    fn read_instructions_j() {
        let file_string =
            "Add $t1, $t2, $t3\nAddress: add $t1, #t2, $t3\nlw $t1, 400($t2)\nj Address"
                .to_string();

        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[3].binary,
            0b00001000000000000000000000000001
        )
    }

    #[test]
    fn read_instructions_beq() {
        let file_string = "Add $t1, $t2, $t3\nAddress: add $t1, #t2, $t3\nlw $t1, 400($t2)\nbeq $t1, $t2, address".to_string();
        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[3].binary,
            0b00010001001010101111111111111101
        )
    }

    #[test]
    fn read_instructions_bne() {
        let file_string = "Add $t1, $t2, $t3\nAddress: add $t1, #t2, $t3\nlw $t1, 400($t2)\nbne $t1, $t2, address".to_string();
        let instruction_list = instruction_parser(file_string);

        assert_eq!(
            instruction_list[3].binary,
            0b00010101001010101111111111111101
        )
    }

    #[test]
    fn read_instructions_c_eq_s() {
        let instruction_list = instruction_parser("c.eq.s $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110000101100100100000111010
        )
    }

    #[test]
    fn read_instructions_c_eq_d() {
        let instruction_list = instruction_parser("c.eq.d $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110001101100100100000111010
        )
    }

    #[test]
    fn read_instructions_c_lt_s() {
        let instruction_list = instruction_parser("c.lt.s $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110000101100100100000111100
        )
    }

    #[test]
    fn read_instructions_c_lt_d() {
        let instruction_list = instruction_parser("c.lt.d $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110001101100100100000111100
        )
    }

    #[test]
    fn read_instructions_c_le_s() {
        let instruction_list = instruction_parser("c.le.s $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110000101100100100000111110
        )
    }

    #[test]
    fn read_instructions_c_le_d() {
        let instruction_list = instruction_parser("c.le.d $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110001101100100100000111110
        )
    }

    #[test]
    fn read_instructions_c_ngt_s() {
        let instruction_list = instruction_parser("c.ngt.s $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110000101100100100000111111
        )
    }

    #[test]
    fn read_instructions_c_ngt_d() {
        let instruction_list = instruction_parser("c.ngt.d $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110001101100100100000111111
        )
    }

    #[test]
    fn read_instructions_c_nge_s() {
        let instruction_list = instruction_parser("c.nge.s $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110000101100100100000111101
        )
    }

    #[test]
    fn read_instructions_c_nge_d() {
        let instruction_list = instruction_parser("c.nge.d $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110001101100100100000111101
        )
    }

    #[test]
    fn read_instruction_bc1t() {
        let instruction_list =
            instruction_parser("instruction: add $t1, $t2, $t3\nbc1t instruction".to_string());

        assert_eq!(
            instruction_list[1].binary,
            0b01000101000000011111111111111110
        );
    }

    #[test]
    fn read_instruction_bc1f() {
        let instruction_list =
            instruction_parser("instruction: add $t1, $t2, $t3\nbc1f instruction".to_string());

        assert_eq!(
            instruction_list[1].binary,
            0b01000101000000001111111111111110
        );
    }
}

use crate::parser::assembling::assemble_data_binary;
use crate::parser::parser_assembler_main::{
    create_binary_vec, parser, place_binary_in_middle_of_another, read_instructions,
};
use crate::parser::parser_structs_and_enums::instruction_tokenization::ErrorType::UnsupportedInstruction;
use crate::parser::parser_structs_and_enums::instruction_tokenization::ProgramInfo;
use crate::parser::parsing::{create_label_map, separate_data_and_text, tokenize_program};
use crate::parser::pseudo_instruction_parsing::{
    complete_lw_sw_pseudo_instructions, expand_pseudo_instructions_and_assign_instruction_numbers,
};
use std::collections::HashMap;

#[test]
fn place_binary_in_middle_of_another_works() {
    let result = place_binary_in_middle_of_another(0b11, 0b0, 1, 0);
    assert_eq!(result, 0b101);
}
#[test]
fn place_binary_in_middle_of_another_works_2() {
    let result = place_binary_in_middle_of_another(0b1001, 0b111, 3, 1);
    assert_eq!(result, 0b1011101);
}
#[test]
fn place_binary_in_middle_of_another_works_3() {
    let result = place_binary_in_middle_of_another(0b10100101, 0b11011, 5, 3);
    assert_eq!(result, 0b1010110110101);
}

#[test]
fn place_binary_works_dahi() {
    let result = place_binary_in_middle_of_another(0b000001010011010101010101010, 0b00110, 5, 15);
    assert_eq!(result, 0b00000101001001101010101010101010);
}

mod helper_functions {
    use crate::parser::assembling::assemble_data_binary;
    use crate::parser::parser_assembler_main::read_instructions;
    use crate::parser::parser_structs_and_enums::instruction_tokenization::Instruction;
    use crate::parser::parsing::{create_label_map, separate_data_and_text, tokenize_program};
    use crate::parser::pseudo_instruction_parsing::expand_pseudo_instructions_and_assign_instruction_numbers;
    use std::collections::HashMap;

    pub fn instruction_parser(mut file_string: String) -> Vec<Instruction> {
        file_string = file_string.to_lowercase();

        let (lines, mut updated_monaco_strings, mut monaco_line_info_vec) =
            tokenize_program(file_string);
        let (mut instruction_list, mut data) = separate_data_and_text(lines);
        expand_pseudo_instructions_and_assign_instruction_numbers(
            &mut instruction_list,
            &data,
            &mut updated_monaco_strings,
            &mut monaco_line_info_vec,
        );
        assemble_data_binary(&mut data);

        let labels: HashMap<String, u32> = create_label_map(&mut instruction_list, &mut data);

        read_instructions(&mut instruction_list, &labels);

        instruction_list
    }
}

#[test]
fn create_binary_vec_works_with_data() {
    let mut program_info = ProgramInfo::default();
    let file_string =
        ".data\nlabel: .ascii \"this is a string\"\n.text\nlw $t1, label".to_lowercase();
    let (lines, mut updated_monaco_strings, mut monaco_line_info_vec) =
        tokenize_program(file_string);
    (program_info.instructions, program_info.data) = separate_data_and_text(lines);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut updated_monaco_strings,
        &mut monaco_line_info_vec,
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

    let result = create_binary_vec(program_info.instructions.clone(), vec_of_data);

    assert_eq!(result[2], 0b01110100011010000110100101110011);
    assert_eq!(result[3], 0b00100000011010010111001100100000);
    assert_eq!(result[4], 0b01100001001000000111001101110100);
    assert_eq!(result[5], 0b01110010011010010110111001100111);
}

#[test]
fn read_instructions_recognizes_valid_but_unsupported_instructions() {
    let mut program_info = ProgramInfo::default();
    let file_string = "jalr $t1, $t2\ndsrav $t1, $t2, $t3".to_lowercase();
    let (lines, mut updated_monaco_strings, mut monaco_line_info_vec) =
        tokenize_program(file_string);
    (program_info.instructions, program_info.data) = separate_data_and_text(lines);
    expand_pseudo_instructions_and_assign_instruction_numbers(
        &mut program_info.instructions,
        &program_info.data,
        &mut updated_monaco_strings,
        &mut monaco_line_info_vec,
    );

    let labels: HashMap<String, u32> =
        create_label_map(&mut program_info.instructions, &mut program_info.data);
    complete_lw_sw_pseudo_instructions(
        &mut program_info.instructions,
        &labels,
        &mut updated_monaco_strings,
    );
    read_instructions(&mut program_info.instructions, &labels);

    assert_eq!(
        program_info.instructions[0].errors[0].error_name,
        UnsupportedInstruction
    );
    assert_eq!(
        program_info.instructions[1].errors[0].error_name,
        UnsupportedInstruction
    );
}

#[test]
fn console_output_post_assembly_works_with_errors() {
    let result = parser(
        ".text\nadd $t1, $t2, 1235\n.data\nlabel: .ascii 100\n.text\nlw t1, address".to_string(),
    )
    .0
    .console_out_post_assembly;

    assert_eq!(result, "UnrecognizedGPRegister on line 1 with token \"1235\"\nGP register is not recognized. A valid, similar register is: r23.\n\nUnrecognizedGPRegister on line 5 with token \"t1\"\nGP register is not recognized. A valid, similar register is: $t1.\n\nInvalidMemorySyntax on line 5 with token \"address\"\nThe given string for memory does not match syntax of \"offset(base)\" or \"label\".\n\nImproperlyFormattedASCII on line 3 with token \"100\"\nToken recognized as ASCII does not start and or end with double quotes (\").\n\n")
}

#[test]
fn console_output_post_assembly_works_with_no_errors() {
    let result = parser(
        ".text\nadd $t1, $t2, $t3\n.data\nlabel: .ascii \"string\"\n.text\nlw $t1, 40($t1)"
            .to_string(),
    )
    .0
    .console_out_post_assembly;

    assert_eq!(result, "Program assembled successfully!".to_string());
}

// #[test]
// fn parser_outputs_line_hover_info_for_pseudo_instructions() {
//     let result = parser("subi: subi $t1, $t2, 100\nlw $t1, subi".to_string())
//         .0
//         .monaco_line_info;
//
//     assert_eq!(result[0].mouse_hover_string, "subi $regA, $regB, immediate is a pseudo-instruction.\nsubi $regA, $regB, immediate =>\n\tori $at, $zero, immediate\n\tsub $regA, $regB, $at");
//     assert_eq!(result[1].mouse_hover_string, "lw $regA, label is a pseudo-instruction.\nlw $regA, label =>\n\tlui $at, label\n\tlw $regA, lower16($at)\n\twhere lower16 is the lower 16 bits of the labelled address.");
// }
