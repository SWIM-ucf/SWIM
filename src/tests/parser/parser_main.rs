#[cfg(test)]
mod parser_main_function_tests {
    use crate::parser::parser_main::*;
    use crate::parser::parser_structs_and_enums::instruction_tokenization::print_instruction_struct_contents;

    #[test]
    fn parser_takes_string_and_returns_vec_of_instructions() {
        let results =
            parser("lw $t1, 512($t1)\nadd $t1, $s6, $t2\naddi $t1, $t2, 43690".to_string());

        let length = results.0.len();

        for i in 0..length {
            print_instruction_struct_contents(results.0.get(i).unwrap());
        }
        assert_eq!(results.0[0].binary, 0b10001101001010010000001000000000);
        assert_eq!(results.0[1].binary, 0b00000010110010100100100000100000);
        assert_eq!(results.0[2].binary, 0b00100001010010011010101010101010);
    }
}

mod read_instructions_tests {
    use crate::tests::parser::parser_main::helper_functions::simulate_parser;

    #[test]
    fn read_instructions_add() {
        let file_string = "add $t1, $s6, $t2".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000010110010100100100000100000
        );
    }

    #[test]
    fn read_instructions_sub() {
        let file_string = "sub $t1, $s6, $t2".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000010110010100100100000100010
        );
    }

    #[test]
    fn read_instructions_mul() {
        let file_string = "mul $t1, $s6, $t2".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01110010110010100100100000000010
        );
    }

    #[test]
    fn read_instructions_div() {
        let file_string = "div $t1, $s6".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001001101100000000000011010
        );
    }

    #[test]
    fn read_instructions_lw() {
        let file_string = "lw $t1, 512($t1)".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b10001101001010010000001000000000
        );
    }

    #[test]
    fn read_instructions_sw() {
        let file_string = "sw $t1, 512($t1)".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b10101101001010010000001000000000
        );
    }

    #[test]
    fn read_instructions_lui() {
        let file_string = "lui $t1, 43690".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00111100000010011010101010101010
        );
    }

    #[test]
    fn read_instructions_addi() {
        let file_string = "addi $t1, $t2, 43690".to_string();

        let instruction_list = simulate_parser(file_string);
        assert_eq!(
            instruction_list[0].binary,
            0b00100001010010011010101010101010
        );
    }

    #[test]
    fn read_instructions_and() {
        let file_string = "and $t1, $s6, $t2".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000010110010100100100000100100
        );
    }

    #[test]
    fn read_instructions_or() {
        let file_string = "or $t1, $s6, $t2".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000010110010100100100000100101
        );
    }

    #[test]
    fn read_instructions_ori() {
        let file_string = "ori $t1, $t2, 43690".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00110101010010011010101010101010
        );
    }

    #[test]
    fn read_instructions_andi() {
        let file_string = "andi $t1, $t2, 43690".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00110001010010011010101010101010
        );
    }

    #[test]
    fn read_instructions_dadd() {
        let file_string = "dadd $t1, $t2, $s6".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001010101100100100000101100
        );
    }

    #[test]
    fn read_instructions_dsub() {
        let file_string = "dsub $t1, $t2, $s6".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001010101100100100000101110
        );
    }

    #[test]
    fn read_instructions_dmul() {
        let file_string = "dmul $t1, $t2, $s6".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001010101100100100010011100
        );
    }

    #[test]
    fn read_instructions_ddiv() {
        let file_string = "ddiv $t1, $t2".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001001010100000000000011110
        );
    }

    #[test]
    fn read_instructions_add_s() {
        let file_string = "add.s $f9, $f10, $f22".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000110000101100101001001000000
        );
    }

    #[test]
    fn read_instructions_add_d() {
        let file_string = "add.d $f9, $f10, $f22".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000110001101100101001001000000
        );
    }

    #[test]
    fn read_instructions_sub_s() {
        let file_string = "sub.s $f9, $f10, $f22".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000110000101100101001001000001
        );
    }

    #[test]
    fn read_instructions_sub_d() {
        let file_string = "sub.d $f9, $f10, $f22".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000110001101100101001001000001
        );
    }

    #[test]
    fn read_instructions_mul_s() {
        let file_string = "mul.s $f9, $f10, $f22".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000110000101100101001001000010
        );
    }

    #[test]
    fn read_instructions_mul_d() {
        let file_string = "mul.d $f9, $f10, $f22".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000110001101100101001001000010
        );
    }

    #[test]
    fn read_instructions_div_s() {
        let file_string = "div.s $f9, $f10, $f22".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000110000101100101001001000011
        );
    }

    #[test]
    fn read_instructions_div_d() {
        let file_string = "div.d $f9, $f10, $f22".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000110001101100101001001000011
        );
    }

    #[test]
    fn read_instructions_dahi() {
        let file_string = "dahi $t1, 43690".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000101001001101010101010101010
        );
    }

    #[test]
    fn read_instructions_dati() {
        let file_string = "dati $t1, 43690".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000101001111101010101010101010
        );
    }

    #[test]
    fn read_instructions_daddiu() {
        let file_string = "daddiu $t1, $t2, 43690".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01100101010010011010101010101010
        );
    }

    #[test]
    fn read_instructions_slt() {
        let file_string = "slt $t1, $t2, $s6".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001010101100100100000101010
        );
    }

    #[test]
    fn read_instructions_sltu() {
        let file_string = "sltu $t1, $t2, $s6".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b00000001010101100100100000101011
        );
    }

    #[test]
    fn read_instructions_swc1() {
        let file_string = "swc1 $f9, 43690($t2)".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b11100101010010011010101010101010
        );
    }

    #[test]
    fn read_instructions_lwc1() {
        let file_string = "lwc1 $f9, 43690($t2)".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b11000101010010011010101010101010
        );
    }

    #[test]
    fn read_instructions_mtc1() {
        let file_string = "mtc1 $t1, $f22".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000100100010011011000000000000
        );
    }

    #[test]
    fn read_instructions_dmtc1() {
        let file_string = "dmtc1 $t1, $f22".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000100101010011011000000000000
        );
    }

    #[test]
    fn read_instructions_mfc1() {
        let file_string = "mfc1 $t1, $f22".to_string();

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[0].binary,
            0b01000100000010011011000000000000
        );
    }

    #[test]
    fn read_instructions_dmfc1() {
        let file_string = "dmfc1 $t1, $f22".to_string();

        let instruction_list = simulate_parser(file_string);

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

        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[3].binary,
            0b00001000000000000000000000000001
        )
    }

    #[test]
    fn read_instructions_beq() {
        let file_string = "Add $t1, $t2, $t3\nAddress: add $t1, #t2, $t3\nlw $t1, 400($t2)\nbeq $t1, $t2, address".to_string();
        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[3].binary,
            0b00010001001010101111111111111101
        )
    }

    #[test]
    fn read_instructions_bne() {
        let file_string = "Add $t1, $t2, $t3\nAddress: add $t1, #t2, $t3\nlw $t1, 400($t2)\nbne $t1, $t2, address".to_string();
        let instruction_list = simulate_parser(file_string);

        assert_eq!(
            instruction_list[3].binary,
            0b00010101001010101111111111111101
        )
    }

    #[test]
    fn read_instructions_c_eq_s() {
        let instruction_list = simulate_parser("c.eq.s $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110000101100100100000111010
        )
    }

    #[test]
    fn read_instructions_c_eq_d() {
        let instruction_list = simulate_parser("c.eq.d $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110001101100100100000111010
        )
    }

    #[test]
    fn read_instructions_c_lt_s() {
        let instruction_list = simulate_parser("c.lt.s $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110000101100100100000111100
        )
    }

    #[test]
    fn read_instructions_c_lt_d() {
        let instruction_list = simulate_parser("c.lt.d $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110001101100100100000111100
        )
    }

    #[test]
    fn read_instructions_c_le_s() {
        let instruction_list = simulate_parser("c.le.s $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110000101100100100000111110
        )
    }

    #[test]
    fn read_instructions_c_le_d() {
        let instruction_list = simulate_parser("c.le.d $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110001101100100100000111110
        )
    }

    #[test]
    fn read_instructions_c_ngt_s() {
        let instruction_list = simulate_parser("c.ngt.s $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110000101100100100000111111
        )
    }

    #[test]
    fn read_instructions_c_ngt_d() {
        let instruction_list = simulate_parser("c.ngt.d $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110001101100100100000111111
        )
    }

    #[test]
    fn read_instructions_c_nge_s() {
        let instruction_list = simulate_parser("c.nge.s $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110000101100100100000111101
        )
    }

    #[test]
    fn read_instructions_c_nge_d() {
        let instruction_list = simulate_parser("c.nge.d $f9, $f22".to_string());

        assert_eq!(
            instruction_list[0].binary,
            0b01000110001101100100100000111101
        )
    }
}
use crate::parser::parser_main::place_binary_in_middle_of_another;
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
    use crate::parser::parser_main::read_instructions;
    use crate::parser::parser_structs_and_enums::instruction_tokenization::Instruction;
    use crate::parser::preprocessing::{
        assign_instruction_numbers, build_instruction_list_from_lines, confirm_operand_commas,
        create_label_map, expand_pseudo_instruction, tokenize_instructions,
    };
    use std::collections::HashMap;

    pub fn simulate_parser(mut file_string: String) -> Vec<Instruction> {
        file_string = file_string.to_lowercase();

        let lines = tokenize_instructions(file_string);
        let mut instruction_list: Vec<Instruction> = build_instruction_list_from_lines(lines);
        confirm_operand_commas(&mut instruction_list);
        expand_pseudo_instruction(&mut instruction_list);
        assign_instruction_numbers(&mut instruction_list);

        let labels: HashMap<String, u32> = create_label_map(&mut instruction_list);

        read_instructions(&mut instruction_list, labels);

        instruction_list
    }
}
