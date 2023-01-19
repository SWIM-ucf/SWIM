#[cfg(test)]
mod parser_main_function_tests {
    use crate::parser::parser_main::*;
    use crate::parser::parser_structs_and_enums::instruction_tokenization::print_instruction_struct_contents;

    #[test]
    fn parser_takes_string_and_returns_vec_of_instructions() {
        let results =
            parser("lw $t1, 512($t1)\nadd $t1, $s6, $t2\naddi $t1, $t2, 43690".to_string());

        let length = results.len();

        for i in 0..length {
            print_instruction_struct_contents(results.get(i).unwrap());
        }
        assert_eq!(results[0].binary, 0b10001101001010010000001000000000);
        assert_eq!(results[1].binary, 0b00000010110010100100100000100000);
        assert_eq!(results[2].binary, 0b00100001010010011010101010101010);
    }
}

mod read_instructions_tests {
    use crate::parser::parser_main::*;
    use crate::parser::parser_preprocessing::{
        build_instruction_list_from_lines, tokenize_instructions,
    };

    #[test]
    fn read_instructions_add() {
        let line = tokenize_instructions("add $t1 $s6 $t2".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b00000010110010100100100000100000);
    }

    #[test]
    fn read_instructions_sub() {
        let line = tokenize_instructions("sub $t1 $s6 $t2".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b00000010110010100100100000100010);
    }

    #[test]
    fn read_instructions_mul() {
        let line = tokenize_instructions("mul $t1 $s6 $t2".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b01110010110010100100100000000010);
    }

    #[test]
    fn read_instructions_div() {
        let line = tokenize_instructions("div $t1 $s6".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b00000001001101100000000000011010);
    }

    #[test]
    fn read_instructions_lw() {
        let line = tokenize_instructions("lw $t1 512($t1)".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b10001101001010010000001000000000);
    }

    #[test]
    fn read_instructions_sw() {
        let line = tokenize_instructions("sw $t1 512($t1)".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b10101101001010010000001000000000);
    }

    #[test]
    fn read_instructions_lui() {
        let line = tokenize_instructions("lui $t1 43690".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b00111100000010011010101010101010);
    }

    #[test]
    fn read_instructions_addi() {
        let line = tokenize_instructions("addi $t1 $t2 43690".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b00100001010010011010101010101010);
    }

    #[test]
    fn read_instructions_and() {
        let line = tokenize_instructions("and $t1 $s6 $t2".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b00000010110010100100100000100100);
    }

    #[test]
    fn read_instructions_or() {
        let line = tokenize_instructions("or $t1 $s6 $t2".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b00000010110010100100100000100101);
    }

    #[test]
    fn read_instructions_ori() {
        let line = tokenize_instructions("ori $t1 $t2 43690".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b00110101010010011010101010101010);
    }

    #[test]
    fn read_instructions_andi() {
        let line = tokenize_instructions("andi $t1 $t2 43690".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b00110001010010011010101010101010);
    }

    #[test]
    fn read_instructions_dadd() {
        let line = tokenize_instructions("dadd $t1 $t2 $s6".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b00000001010101100100100000101100);
    }

    #[test]
    fn read_instructions_dsub() {
        let line = tokenize_instructions("dsub $t1 $t2 $s6".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b00000001010101100100100000101110);
    }

    #[test]
    fn read_instructions_dmul() {
        let line = tokenize_instructions("dmul $t1 $t2 $s6".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b00000001010101100100100010011100);
    }

    #[test]
    fn read_instructions_ddiv() {
        let line = tokenize_instructions("ddiv $t1 $t2".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b00000001001010100000000000011110);
    }

    #[test]
    fn read_instructions_add_s() {
        let line = tokenize_instructions("add.s $f9 $f10 $f22".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b01000110000101100101001001000000);
    }

    #[test]
    fn read_instructions_add_d() {
        let line = tokenize_instructions("add.d $f9 $f10 $f22".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b01000110001101100101001001000000);
    }

    #[test]
    fn read_instructions_sub_s() {
        let line = tokenize_instructions("sub.s $f9 $f10 $f22".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b01000110000101100101001001000001);
    }

    #[test]
    fn read_instructions_sub_d() {
        let line = tokenize_instructions("sub.d $f9 $f10 $f22".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b01000110001101100101001001000001);
    }

    #[test]
    fn read_instructions_mul_s() {
        let line = tokenize_instructions("mul.s $f9 $f10 $f22".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b01000110000101100101001001000010);
    }

    #[test]
    fn read_instructions_mul_d() {
        let line = tokenize_instructions("mul.d $f9 $f10 $f22".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b01000110001101100101001001000010);
    }

    #[test]
    fn read_instructions_div_s() {
        let line = tokenize_instructions("div.s $f9 $f10 $f22".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b01000110000101100101001001000011);
    }

    #[test]
    fn read_instructions_div_d() {
        let line = tokenize_instructions("div.d $f9 $f10 $f22".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b01000110001101100101001001000011);
    }

    #[test]
    fn read_instructions_dahi() {
        let line = tokenize_instructions("dahi $t1 43690".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b00000101001001101010101010101010);
    }

    #[test]
    fn read_instructions_dati() {
        let line = tokenize_instructions("dati $t1 43690".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b00000101001111101010101010101010);
    }

    #[test]
    fn read_instructions_daddiu() {
        let line = tokenize_instructions("daddiu $t1 $t2 43690".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b01100101010010011010101010101010);
    }

    #[test]
    fn read_instructions_slt() {
        let line = tokenize_instructions("slt $t1 $t2 $s6".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b00000001010101100100100000101010);
    }

    #[test]
    fn read_instructions_sltu() {
        let line = tokenize_instructions("sltu $t1 $t2 $s6".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b00000001010101100100100000101011);
    }

    #[test]
    fn read_instructions_swc1() {
        let line = tokenize_instructions("swc1 $f9 43690($t2)".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b11100101010010011010101010101010);
    }

    #[test]
    fn read_instructions_lwc1() {
        let line = tokenize_instructions("lwc1 $f9 43690($t2)".to_string());
        let mut instructions = build_instruction_list_from_lines(line);
        read_instructions(&mut instructions, Default::default());
        let instruction = instructions[0].clone();

        assert_eq!(instruction.binary, 0b11000101010010011010101010101010);
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
