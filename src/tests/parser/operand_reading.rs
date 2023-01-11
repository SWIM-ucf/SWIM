#[cfg(test)]

mod convert_to_u32_tests {
    use crate::parser::parser_main::convert_to_u32;

    #[test]
    fn convert_to_u32_returns_correct_value_on_zeros() {
        let result = convert_to_u32("00000".to_string());
        assert_eq!(result, 0);
    }

    #[test]
    fn convert_to_u32_returns_correct_value_on_32_bit_long_string() {
        let result = convert_to_u32("11111111111111111111111111111111".to_string());
        assert_eq!(result, 4294967295);
    }

    #[test]
    fn convert_to_u32_returns_correct_value_for_an_actual_instruction() {
        let result = convert_to_u32("10001101010010010000000000000100".to_string());
        assert_eq!(result, 2370371588);
    }
}

mod read_register_tests {
    use crate::parser::parser_main::read_register;
    use crate::parser::parser_instruction_tokenization::instruction_tokenization::ErrorType::UnrecognizedRegister;

    #[test]
    fn read_register_returns_correct_binary_on_valid_register_name() {
        let results = read_register("$t1", 1);
        assert_eq!(results.0, "01001");
    }

    #[test]
    fn read_register_returns_correct_binary_on_valid_register_number() {
        let results = read_register("r12", 1);
        assert_eq!(results.0, "01100");
    }

    #[test]
    fn read_register_returns_error_option_on_unrecognized_register() {
        let results = read_register("hello_world", 1);
        assert_eq!(results.1.unwrap().error_name, UnrecognizedRegister);
    }
}

mod immediate_tests {
    use crate::parser::parser_main::*;
    use crate::parser::parser_instruction_tokenization::instruction_tokenization::ErrorType::{
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
        assert_eq!(results.0, "0000000011111111");
    }

    #[test]
    fn read_immediate_returns_correct_negative_value() {
        let results = read_immediate("-5", 1, 12);
        assert_eq!(results.0, "111111111011")
    }
}

mod memory_address_tests {
    use crate::parser::parser_main::read_memory_address;
    use crate::parser::parser_instruction_tokenization::instruction_tokenization::ErrorType::{
        ImmediateOutOfBounds, InvalidMemorySyntax, NonIntImmediate, UnrecognizedRegister,
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
        assert_eq!(results.2.unwrap()[0].error_name, UnrecognizedRegister);
    }

    #[test]
    fn invalid_base_and_offset_returns_multiple_errors() {
        let results = read_memory_address("sad($wrong)", 0).2.unwrap();
        assert_eq!(results[0].error_name, NonIntImmediate);
        assert_eq!(results[1].error_name, UnrecognizedRegister);
    }

    #[test]
    fn memory_address_can_be_correctly_read() {
        let results = read_memory_address("4($t1)", 0);
        assert!(results.2.is_none());
        assert_eq!(results.0, "0000000000000100");
        assert_eq!(results.1, "01001");
    }
}

mod tokenize_instruction_tests {
    use crate::parser::parser_instruction_tokenization::instruction_tokenization::*;

    #[test]
    fn tokenize_instruction_returns_struct_with_tokens() {
        let correct_instruction = Instruction {
            tokens: vec![
                "ADD".to_string(),
                "T1".to_string(),
                "T2".to_string(),
                "T2".to_string(),
            ],
            binary_representation: String::new(),
            int_representation: 0,
            // instruction_number: 0,
            errors: vec![],
        };
        let received_instruction = tokenize_instruction("ADD T1 T2 T2");
        assert_eq!(received_instruction.tokens, correct_instruction.tokens);
    }
}

mod confirm_commas_tests {
    use crate::parser::parser_instruction_tokenization::instruction_tokenization::ErrorType::MissingComma;
    use crate::parser::parser_instruction_tokenization::instruction_tokenization::{
        confirm_commas_in_instruction, Instruction,
    };

    #[test]
    fn confirm_comma_generates_error_when_a_middle_token_is_missing_a_comma() {
        let mut instruction = Instruction::default();
        instruction.tokens = vec![
            "add".to_string(),
            "$t1,".to_string(),
            "$t1".to_string(),
            "$t1".to_string(),
        ];
        instruction = confirm_commas_in_instruction(instruction);
        assert_eq!(instruction.errors[0].error_name, MissingComma);
        assert_eq!(instruction.errors[0].token_number_giving_error, 2);
    }

    #[test]
    fn confirm_comma_can_generate_multiple_errors_if_multiple_commas_are_missing() {
        let mut instruction = Instruction::default();
        instruction.tokens = vec![
            "add".to_string(),
            "$t1".to_string(),
            "$zero".to_string(),
            "$t1".to_string(),
        ];
        instruction = confirm_commas_in_instruction(instruction);
        assert_eq!(instruction.errors[0].error_name, MissingComma);
        assert_eq!(instruction.errors[0].token_number_giving_error, 1);
        assert_eq!(instruction.errors[1].error_name, MissingComma);
        assert_eq!(instruction.errors[1].token_number_giving_error, 2);
    }

    #[test]
    fn confirm_comma_does_not_generate_errors_given_proper_syntax() {
        let mut instruction = Instruction::default();
        instruction.tokens = vec![
            "add".to_string(),
            "$t1,".to_string(),
            "$zero,".to_string(),
            "$t1".to_string(),
        ];
        instruction = confirm_commas_in_instruction(instruction);
        assert_eq!(instruction.errors.len(), 0);
    }
}

mod create_vector_of_instructions_tests {
    use crate::parser::parser_instruction_tokenization::instruction_tokenization::{
        create_vector_of_instructions, Instruction,
    };

    #[test]
    fn create_vector_of_instructions_builds_the_correct_number_of_instructions() {
        let original_string = "add $t1, $t2, $zero\nsub $t2, $t2, $t2\nlw r8, 52($s0)".to_string();
        let instructions: Vec<Instruction> = create_vector_of_instructions(original_string);
        assert_eq!(instructions.len(), 3);
    }

    #[test]
    fn create_vector_of_instructions_separates_instructions_at_correct_spot() {
        let original_string = "add $t1, $t2, $zero\nsub $t2, $t2, $t2\nlw r8, 52($s0)".to_string();
        let instructions: Vec<Instruction> = create_vector_of_instructions(original_string);
        assert_eq!(instructions[0].tokens, vec!["add", "$t1,", "$t2,", "$zero"]);
        assert_eq!(instructions[1].tokens, vec!["sub", "$t2,", "$t2,", "$t2"]);
        assert_eq!(instructions[2].tokens, vec!["lw", "r8,", "52($s0)"]);
    }
}
