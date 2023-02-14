use crate::parser::assembling::assemble_data_binary;
use crate::parser::parsing::{separate_data_and_text, tokenize_program};

#[cfg(test)]

mod convert_to_u32_tests {
    use crate::parser::assembling::_convert_to_u32;

    #[test]
    fn convert_to_u32_returns_correct_value_on_zeros() {
        let result = _convert_to_u32("00000".to_string());
        assert_eq!(result, 0);
    }

    #[test]
    fn convert_to_u32_returns_correct_value_on_32_bit_long_string() {
        let result = _convert_to_u32("11111111111111111111111111111111".to_string());
        assert_eq!(result, 4294967295);
    }

    #[test]
    fn convert_to_u32_returns_correct_value_for_an_actual_instruction() {
        let result = _convert_to_u32("10001101010010010000000000000100".to_string());
        assert_eq!(result, 2370371588);
    }
}
mod read_register_tests {
    use crate::parser::assembling::read_register;
    use crate::parser::parser_structs_and_enums::instruction_tokenization::ErrorType::{
        IncorrectRegisterType, UnrecognizedGPRegister,
    };
    use crate::parser::parser_structs_and_enums::instruction_tokenization::RegisterType::{
        FloatingPoint, GeneralPurpose,
    };

    #[test]
    fn read_register_returns_correct_binary_on_valid_register_name() {
        let results = read_register("$t1", 1, GeneralPurpose);
        assert_eq!(results.0, 0b01001);
    }

    #[test]
    fn read_register_returns_correct_binary_on_valid_register_number() {
        let results = read_register("r12", 1, GeneralPurpose);
        assert_eq!(results.0, 0b01100);
    }

    #[test]
    fn read_register_returns_error_option_on_unrecognized_register() {
        let results = read_register("hello_world", 1, GeneralPurpose);
        assert_eq!(results.1.unwrap().error_name, UnrecognizedGPRegister);
    }

    #[test]
    fn read_register_returns_error_fp_when_needs_gp() {
        let results = read_register("$t1", 1, FloatingPoint);
        assert_eq!(results.1.unwrap().error_name, IncorrectRegisterType);
    }

    #[test]
    fn read_register_returns_error_gp_when_needs_fp() {
        let results = read_register("$f1", 1, GeneralPurpose);
        assert_eq!(results.1.unwrap().error_name, IncorrectRegisterType);
    }
}

mod immediate_tests {
    use crate::parser::assembling::read_immediate;
    use crate::parser::parser_structs_and_enums::instruction_tokenization::ErrorType::{
        ImmediateOutOfBounds, NonIntImmediate,
    };

    #[test]
    fn read_immediate_returns_error_on_non_int_string() {
        let results = read_immediate("Non_Int", 1, 16);
        assert_eq!(results.1.unwrap().error_name, NonIntImmediate);
    }

    #[test]
    fn read_immediate_returns_error_on_immediate_too_large() {
        let results = read_immediate("300", 1, 8);
        assert_eq!(results.1.unwrap().error_name, ImmediateOutOfBounds);
    }

    #[test]
    fn read_immediate_returns_error_on_immediate_too_small() {
        let results = read_immediate("-1000", 1, 8);
        assert_eq!(results.1.unwrap().error_name, ImmediateOutOfBounds);
    }

    #[test]
    fn read_immediate_returns_correct_positive_value() {
        let results = read_immediate("255", 1, 16);
        assert_eq!(results.0, 0b0000000011111111);
    }

    #[test]
    fn read_immediate_returns_correct_negative_value() {
        let results = read_immediate("-5", 1, 12);
        assert_eq!(results.0, 0b11111111111111111111111111111011)
    }
}

mod memory_address_tests {
    use crate::parser::assembling::read_memory_address;
    use crate::parser::parser_structs_and_enums::instruction_tokenization::ErrorType::{
        ImmediateOutOfBounds, InvalidMemorySyntax, NonIntImmediate, UnrecognizedGPRegister,
    };

    #[test]
    fn missing_open_parenthesis_returns_error() {
        let results = read_memory_address("4$t1)", 0);
        assert_eq!(results.2.unwrap()[0].error_name, InvalidMemorySyntax);
    }

    #[test]
    fn missing_close_parenthesis_returns_error() {
        let results = read_memory_address("4($t1", 0);
        assert_eq!(results.2.unwrap()[0].error_name, InvalidMemorySyntax);
    }

    #[test]
    fn invalid_parentheses_order_returns_error() {
        let results = read_memory_address("4)$t1(", 0);
        assert_eq!(results.2.unwrap()[0].error_name, InvalidMemorySyntax);
    }

    #[test]
    fn character_after_close_parenthesis_returns_error() {
        let results = read_memory_address("4($t1)char", 0);
        assert_eq!(results.2.unwrap()[0].error_name, InvalidMemorySyntax);
    }

    #[test]
    fn non_int_offset_returns_error() {
        let results = read_memory_address("characters($t1)", 0);
        assert_eq!(results.2.unwrap()[0].error_name, NonIntImmediate);
    }

    #[test]
    fn offset_over_16_bits_returns_error() {
        let results = read_memory_address("9999999($t1)", 0);
        assert_eq!(results.2.unwrap()[0].error_name, ImmediateOutOfBounds);
    }

    #[test]
    fn base_not_valid_register_returns_error() {
        let results = read_memory_address("0($wrong)", 0);
        assert_eq!(results.2.unwrap()[0].error_name, UnrecognizedGPRegister);
    }

    #[test]
    fn invalid_base_and_offset_returns_multiple_errors() {
        let results = read_memory_address("sad($wrong)", 0).2.unwrap();
        assert_eq!(results[0].error_name, NonIntImmediate);
        assert_eq!(results[1].error_name, UnrecognizedGPRegister);
    }

    #[test]
    fn memory_address_can_be_correctly_read() {
        let results = read_memory_address("4($t1)", 0);
        assert!(results.2.is_none());
        assert_eq!(results.0, 0b0000000000000100);
        assert_eq!(results.1, 0b01001);
    }
}
mod append_instruction_component_tests {
    use crate::parser::parser_assembler_main::append_binary;

    #[test]
    fn append_binary_works() {
        let result = append_binary(15, 3, 2);
        assert_eq!(result, 63);
    }

    #[test]
    fn append_binary_accepts_binary() {
        let result = append_binary(0b1111, 0b10, 2);
        assert_eq!(result, 0b111110);
        assert_eq!(result, 62);
    }

    #[test]
    fn append_binary_still_works_past_32_bits() {
        let result = append_binary(4294967295, 0b00, 2);
        assert_eq!(result, 4294967292);
    }

    #[test]
    fn append_binary_works_with_negative_numbers() {
        let negative = -3;
        let result = append_binary(0b1111, negative as u32, 4);
        assert_eq!(result, 0b11111101);
    }
}

mod read_label_absolute_tests {
    use crate::parser::assembling::read_label_absolute;
    use crate::parser::parser_structs_and_enums::instruction_tokenization::ErrorType::LabelNotFound;
    use crate::parser::parsing::{
        assign_instruction_numbers, create_label_map, expand_pseudo_instruction,
        separate_data_and_text, tokenize_program,
    };
    use std::collections::HashMap;

    #[test]
    fn read_label_absolute_returns_address_of_instruction() {
        let (lines, _comments) = tokenize_program("add $t1, $t2, $t3\nload_from_memory: lw $t1 400($t2)\nadd $t1, #t2, $t3\nsw $t1, 400($t2)\naddi $t1, $t2, 400".to_string());
        let (mut instruction_list, mut data) = separate_data_and_text(lines);
        expand_pseudo_instruction(&mut instruction_list);
        assign_instruction_numbers(&mut instruction_list);
        let labels: HashMap<String, u32> = create_label_map(&mut instruction_list, &mut data);

        let results = read_label_absolute("load_from_memory", 2, labels);

        assert!(results.1.is_none());
        assert_eq!(results.0, 1);
    }

    #[test]
    fn read_label_absolute_returns_error_if_label_cannot_be_found() {
        let (lines, _comments) = tokenize_program("add $t1, $t2, $t3\nload_from_memory: lw $t1, 400($t2)\nadd $t1, #t2, $t3\nsave_to_memory: sw $t1, 400($t2)\naddi $t1, $t2, 400".to_string());
        let (mut instruction_list, mut data) = separate_data_and_text(lines);
        expand_pseudo_instruction(&mut instruction_list);
        assign_instruction_numbers(&mut instruction_list);
        let labels: HashMap<String, u32> = create_label_map(&mut instruction_list, &mut data);

        let results = read_label_absolute("label_not_found:", 2, labels);

        assert_eq!(results.1.unwrap().error_name, LabelNotFound);
    }
}

mod read_label_relative_tests {
    use crate::parser::assembling::read_label_relative;
    use crate::parser::parsing::{
        assign_instruction_numbers, create_label_map, expand_pseudo_instruction,
        separate_data_and_text, tokenize_program,
    };
    use std::collections::HashMap;

    #[test]
    fn read_label_relative_returns_correct_value_for_instruction_above_current() {
        let (lines, _comments) = tokenize_program("add $t1, $t2, $t3\nload_from_memory: lw $t1 400($t2)\nadd $t1, #t2, $t3\nsw $t1, 400($t2)\naddi $t1, $t2, 400".to_string());
        let (mut instruction_list, mut data) = separate_data_and_text(lines);
        expand_pseudo_instruction(&mut instruction_list);
        assign_instruction_numbers(&mut instruction_list);
        let labels: HashMap<String, u32> = create_label_map(&mut instruction_list, &mut data);

        let result = read_label_relative("load_from_memory", 0, 4, labels);

        let correct = -4;
        assert_eq!(result.0, correct as u32);
    }

    #[test]
    fn read_label_relative_returns_correct_value_for_instruction_below_current() {
        let (lines, _comments) = tokenize_program("add $t1, $t2, $t3\nload_from_memory: lw $t1 400($t2)\nadd $t1, #t2, $t3\nstore_in_memory: sw $t1, 400($t2)\naddi $t1, $t2, 400".to_string());
        let (mut instruction_list, mut data) = separate_data_and_text(lines);
        expand_pseudo_instruction(&mut instruction_list);
        assign_instruction_numbers(&mut instruction_list);
        let labels: HashMap<String, u32> = create_label_map(&mut instruction_list, &mut data);

        let result = read_label_relative("store_in_memory", 0, 1, labels);

        assert_eq!(result.0, 1);
    }
}

#[test]
fn assemble_data_binary_works_one_word() {
    let lines = tokenize_program(".data\nlabel: .word 200".to_string()).0;
    let mut modified_data = separate_data_and_text(lines).1;
    let result = assemble_data_binary(&mut modified_data);

    assert_eq!(result[0], 0);
    assert_eq!(result[1], 0);
    assert_eq!(result[2], 0);
    assert_eq!(result[3], 200);
}

#[test]
fn assemble_data_binary_works_multiple_words() {
    let lines = tokenize_program(".data\nlabel: .word 200, 45, -12".to_string()).0;
    let mut modified_data = separate_data_and_text(lines).1;
    let result = assemble_data_binary(&mut modified_data);

    assert_eq!(result[0], 0);
    assert_eq!(result[1], 0);
    assert_eq!(result[2], 0);
    assert_eq!(result[3], 200);
    assert_eq!(result[4], 0);
    assert_eq!(result[5], 0);
    assert_eq!(result[6], 0);
    assert_eq!(result[7], 45);
    assert_eq!(result[8], 255);
    assert_eq!(result[9], 255);
    assert_eq!(result[10], 255);
    assert_eq!(result[11], 244);
}

#[test]
fn assemble_data_binary_works_half_words() {
    let lines = tokenize_program(".data\nlabel: .half 200, 45, -12".to_string()).0;
    let mut modified_data = separate_data_and_text(lines).1;
    let result = assemble_data_binary(&mut modified_data);

    assert_eq!(result[0], 0);
    assert_eq!(result[1], 200);
    assert_eq!(result[2], 0);
    assert_eq!(result[3], 45);
    assert_eq!(result[4], 255);
    assert_eq!(result[5], 244);
    assert_eq!(result.len(), 6);
}

#[test]
fn assemble_data_binary_works_for_spaces() {
    let lines = tokenize_program(".data\nlabel: .space 3, 1".to_string()).0;
    let mut modified_data = separate_data_and_text(lines).1;
    let result = assemble_data_binary(&mut modified_data);

    assert_eq!(result[0], 0);
    assert_eq!(result[1], 0);
    assert_eq!(result[2], 0);
    assert_eq!(result[3], 0);
    assert_eq!(result.len(), 4);
}

#[test]
fn assemble_data_binary_works_for_int_bytes() {
    let lines = tokenize_program(".data\nlabel: .byte 255, -128".to_string()).0;
    let mut modified_data = separate_data_and_text(lines).1;
    let result = assemble_data_binary(&mut modified_data);

    assert_eq!(result[0], 255);
    assert_eq!(result[1], 128);
}

#[test]
fn assemble_data_binary_works_for_char_bytes() {
    let lines = tokenize_program(".data\nlabel: .byte 'a', '?'".to_string()).0;
    let mut modified_data = separate_data_and_text(lines).1;
    let result = assemble_data_binary(&mut modified_data);

    assert_eq!(result[0], 97);
    assert_eq!(result[1], 63);
}

#[test]
fn assemble_data_binary_works_for_ascii() {
    let lines = tokenize_program(".data\nlabel: .ascii \"abcde\"".to_string()).0;
    let mut modified_data = separate_data_and_text(lines.clone()).1;
    let result = assemble_data_binary(&mut modified_data);

    assert_eq!(result[0], 97);
    assert_eq!(result[1], 98);
    assert_eq!(result[2], 99);
    assert_eq!(result[3], 100);
    assert_eq!(result[4], 101);
}

#[test]
fn assemble_data_binary_works_for_asciiz() {
    let lines = tokenize_program(".data\nlabel: .asciiz \"abcde\"".to_string()).0;
    let mut modified_data = separate_data_and_text(lines.clone()).1;
    let result = assemble_data_binary(&mut modified_data);

    assert_eq!(result[0], 97);
    assert_eq!(result[1], 98);
    assert_eq!(result[2], 99);
    assert_eq!(result[3], 100);
    assert_eq!(result[4], 101);
    assert_eq!(result[5], 0);
}

#[test]
fn assemble_data_binary_works_for_float() {
    let lines = tokenize_program(".data\nlabel: .float 0.234, -121.8, 20".to_string()).0;
    let mut modified_data = separate_data_and_text(lines.clone()).1;
    let result = assemble_data_binary(&mut modified_data);

    assert_eq!(result[0], 62);
    assert_eq!(result[1], 111);
    assert_eq!(result[2], 157);
    assert_eq!(result[3], 178);
    assert_eq!(result[4], 194);
    assert_eq!(result[5], 243);
    assert_eq!(result[6], 153);
    assert_eq!(result[7], 154);
    assert_eq!(result[8], 65);
    assert_eq!(result[9], 160);
    assert_eq!(result[10], 0);
    assert_eq!(result[11], 0);
}

#[test]
fn assemble_data_binary_works_for_double() {
    let lines = tokenize_program(".data\nlabel: .double 0.234, -121.8, 20".to_string()).0;
    let mut modified_data = separate_data_and_text(lines.clone()).1;
    let result = assemble_data_binary(&mut modified_data);

    assert_eq!(result[0], 0b00111111);
    assert_eq!(result[1], 0b11001101);
    assert_eq!(result[2], 0b11110011);
    assert_eq!(result[3], 0b10110110);
    assert_eq!(result[4], 0b01000101);
    assert_eq!(result[5], 0b10100001);
    assert_eq!(result[6], 0b11001010);
    assert_eq!(result[7], 0b11000001);

    assert_eq!(result[8], 0b11000000);
    assert_eq!(result[9], 0b01011110);
    assert_eq!(result[10], 0b01110011);
    assert_eq!(result[11], 0b00110011);
    assert_eq!(result[12], 0b00110011);
    assert_eq!(result[13], 0b00110011);
    assert_eq!(result[14], 0b00110011);
    assert_eq!(result[15], 0b00110011);

    assert_eq!(result[16], 0b01000000);
    assert_eq!(result[17], 0b00110100);
    assert_eq!(result[18], 0b00000000);
    assert_eq!(result[19], 0b00000000);
    assert_eq!(result[20], 0b00000000);
    assert_eq!(result[21], 0b00000000);
    assert_eq!(result[22], 0b00000000);
    assert_eq!(result[23], 0b00000000);
}
