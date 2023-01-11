#[cfg(test)]
mod parser_main_function_tests {
    use crate::parser::parser_main::*;

    #[test]
    fn parser_takes_string_and_returns_vec_of_instructions() {
        let results = parser("lw $t1, 4($t2)\nadd $t1, $t1, $zero\naddi r8, $s0, -512".to_string());
        assert_eq!(results[0].int_representation, 2370371588);
        assert_eq!(results[1].int_representation, 608288);
        assert_eq!(results[2].int_representation, 571014656);
    }
}

mod read_instruction_tests {
    use crate::parser::parser_instruction_tokenization::instruction_tokenization::Instruction;
    use crate::parser::parser_main::*;

    #[test]
    fn read_instruction_read_lui() {
        let mut instruction = Instruction {
            tokens: vec!["lui".to_string(), "$t1".to_string(), "256".to_string()],
            ..Default::default()
        };
        instruction = read_instruction(instruction);
        assert_eq!(
            instruction.binary_representation,
            "00111100000010010000000100000000"
        );
    }
}
