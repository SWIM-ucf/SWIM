
#[cfg(test)]
mod parser_main_function_tests{
    use crate::parser::parser::*;

    #[test]
    fn parser_takes_string_and_returns_vec_of_instructions(){
        let results = parser("lw $t1, 4($t2)\nadd $t1, $t1, $zero\naddi r8, $s0, -512".to_string());
        assert_eq!(results[0].int_representation, 2370371588);
        assert_eq!(results[1].int_representation, 608288);
        assert_eq!(results[2].int_representation, 571014656);

    }
}
