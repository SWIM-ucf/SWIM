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
    fn read_instruction_reads_instruction() {
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
use crate::parser::parser_main:: place_binary_in_middle_of_another;
#[test]
fn place_binary_in_middle_of_another_works(){
    let result = place_binary_in_middle_of_another(0b11, 0b0, 1, 1);
    assert_eq!(result, 0b101);
}
#[test]
fn place_binary_in_middle_of_another_works_2(){
    let result = place_binary_in_middle_of_another(0b1001, 0b111, 3, 2);
    assert_eq!(result, 0b1011101);
}
#[test]
fn place_binary_in_middle_of_another_works_3(){
    let result = place_binary_in_middle_of_another(0b10100101, 0b11011, 5, 4);
    assert_eq!(result, 0b1010110110101);
}