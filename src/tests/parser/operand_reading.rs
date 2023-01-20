#[cfg(test)]

mod convert_to_u32_tests {
    use crate::parser::operand_reading::_convert_to_u32;

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
    use crate::parser::operand_reading::read_register;
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
    use crate::parser::operand_reading::read_immediate;
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
    use crate::parser::operand_reading::read_memory_address;
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
    use crate::parser::parser_main::append_binary;

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
    fn append_binary_works_with_negative_numbers(){
        let negative = -3;
        let result = append_binary(0b1111, negative as u32, 4);
        assert_eq!(result, 0b11111101);
    }
}

mod read_label_absolute_tests {
    use crate::parser::operand_reading::read_label_absolute;
    use crate::parser::parser_structs_and_enums::instruction_tokenization::ErrorType::LabelNotFound;
    use crate::parser::parser_structs_and_enums::instruction_tokenization::Instruction;
    use crate::parser::preprocessing::{
        assign_instruction_numbers, build_instruction_list_from_lines, confirm_operand_commas,
        create_label_map, expand_pseudo_instruction, tokenize_instructions,
    };
    use std::collections::HashMap;

    #[test]
    fn read_label_absolute_returns_address_of_instruction() {
        let lines = tokenize_instructions("add $t1, $t2, $t3\nload_from_memory: lw $t1 400($t2)\nadd $t1, #t2, $t3\nsw $t1, 400($t2)\naddi $t1, $t2, 400".to_string());
        let mut instruction_list: Vec<Instruction> = build_instruction_list_from_lines(lines);
        confirm_operand_commas(&mut instruction_list);
        expand_pseudo_instruction(&mut instruction_list);
        assign_instruction_numbers(&mut instruction_list);
        let labels: HashMap<String, u32> = create_label_map(&mut instruction_list);

        let results = read_label_absolute(&*"load_from_memory", 2, labels);

        assert!(results.1.is_none());
        assert_eq!(results.0, 1);
    }

    #[test]
    fn read_label_absolute_returns_error_if_label_cannot_be_found() {
        let lines = tokenize_instructions("add $t1, $t2, $t3\nload_from_memory: lw $t1 400($t2)\nadd $t1, #t2, $t3\nsave_to_memory: sw $t1, 400($t2)\naddi $t1, $t2, 400".to_string());
        let mut instruction_list: Vec<Instruction> = build_instruction_list_from_lines(lines);
        confirm_operand_commas(&mut instruction_list);
        expand_pseudo_instruction(&mut instruction_list);
        assign_instruction_numbers(&mut instruction_list);
        let labels: HashMap<String, u32> = create_label_map(&mut instruction_list);

        let results = read_label_absolute(&*"label_not_found:", 2, labels);

        assert_eq!(results.1.unwrap().error_name, LabelNotFound);
    }
}

mod read_label_relative_tests {
    use crate::parser::operand_reading::read_label_relative;
    use crate::parser::parser_structs_and_enums::instruction_tokenization::Instruction;
    use crate::parser::preprocessing::{
        assign_instruction_numbers, build_instruction_list_from_lines, confirm_operand_commas,
        create_label_map, expand_pseudo_instruction, tokenize_instructions,
    };
    use std::collections::HashMap;

    #[test]
    fn read_label_relative_returns_correct_value_for_instruction_above_current() {
        let lines = tokenize_instructions("add $t1, $t2, $t3\nload_from_memory: lw $t1 400($t2)\nadd $t1, #t2, $t3\nsw $t1, 400($t2)\naddi $t1, $t2, 400".to_string());
        let mut instruction_list: Vec<Instruction> = build_instruction_list_from_lines(lines);
        confirm_operand_commas(&mut instruction_list);
        expand_pseudo_instruction(&mut instruction_list);
        assign_instruction_numbers(&mut instruction_list);
        let labels: HashMap<String, u32> = create_label_map(&mut instruction_list);

        let result = read_label_relative(&*"load_from_memory", 0, 4, labels);

        let correct = -4;
        assert_eq!(result.0, correct as u32);
    }

    #[test]
    fn read_label_relative_returns_correct_value_for_instruction_below_current() {
        let lines = tokenize_instructions("add $t1, $t2, $t3\nload_from_memory: lw $t1 400($t2)\nadd $t1, #t2, $t3\nstore_in_memory: sw $t1, 400($t2)\naddi $t1, $t2, 400".to_string());
        let mut instruction_list: Vec<Instruction> = build_instruction_list_from_lines(lines);
        confirm_operand_commas(&mut instruction_list);
        expand_pseudo_instruction(&mut instruction_list);
        assign_instruction_numbers(&mut instruction_list);
        let labels: HashMap<String, u32> = create_label_map(&mut instruction_list);

        let result = read_label_relative(&*"store_in_memory", 0, 1, labels);

        assert_eq!(result.0, 1);
    }
}
