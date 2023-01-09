pub struct Instruction {
    pub tokens : Vec<String>,
    pub binary_representation: String,
    pub int_representation: u32,
    //instruction_number is not yet being tracked
    // pub instruction_number: u32,
    pub errors: Vec<Error>
}

pub struct Error {
    pub error_name: ErrorType,
    pub token_number_giving_error: u8
}

impl Default for Instruction{
    fn default() -> Instruction{
        Instruction{
            tokens: vec![],
            binary_representation: "".to_string(),
            int_representation: 0,
            //instruction_number: 0,
            errors: vec![]
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ErrorType {
    UnrecognizedRegister,
    MissingComma,
    ImmediateOutOfBounds,
    NonIntImmediate,
    InvalidMemorySyntax,
    IncorrectNumberOfOperands
}

//this enum is used for the fn read_operands to choose the types of operands expected for an instruction type
pub enum OperandType {
    RegisterGp,
    //  RegisterFp,
    Immediate,
    MemoryAddress,
//    Label
}

//takes the string representation of a line of MIPS code and breaks it up into tokens delimited by space characters
pub fn tokenize_instruction (line: &str) -> Instruction {

    //breaks up line into a vector delimited by space characters
    let mut contents:Vec<String> = Vec::new();
    for token in line.split(" "){
        contents.push(token.parse().unwrap());
    }

    //creates an instruction from the vector
    let mut current_instruction = Instruction::default();
    current_instruction.tokens = contents;
    return current_instruction;
}

//takes the string of the MIPS program after comments, extra spaces, and label names have been removed
//and turns each line into an Instruction and returns the vec of these Instructions with the contents as tokens
pub fn create_vector_of_instructions(file_string: String) -> Vec<Instruction>{
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in file_string.lines(){
        instructions.push( tokenize_instruction(line));
    }
    return instructions;
}

//this function takes an instruction as its argument and checks that every token within it except the first and the last (ie all but the last operand) ends with the ',' character
//for each of these that does end in a comma, the comma is removed. Any instance that this isn't the case generates a missingComma error that is added to the error list for that instruction.
//the updated version of the instruction is then returned
pub fn confirm_commas_in_instruction(mut instruction: Instruction) -> Instruction{

    //for loop goes through all but the first and last tokens
    for i in 1..(instruction.tokens.len() - 1){

        let last_char = instruction.tokens.get(i).unwrap().chars().last().unwrap();

        if last_char == ','{
            //this chunk of code removes the last char of the string if it is a ','
            //due to mutability issues, instruction.tokens.get(i).pop() does not work so instead we create a new string without the comma and replace the token instead
            let mut token_as_chars: Vec<char> = instruction.tokens.get(i).unwrap().chars().collect();
            token_as_chars.remove(token_as_chars.len() - 1);
            instruction.tokens.push(token_as_chars.into_iter().collect());
            let length = instruction.tokens.len() - 1;
            instruction.tokens.swap(i, length);
            instruction.tokens.pop();

        }else{//if the last char of the token is not ',', an error is pushed to the list
            instruction.errors.push(Error{
                error_name: ErrorType::MissingComma,
                token_number_giving_error: i as u8
            })
        }
    }

    return instruction;
}

pub fn print_instruction_struct_contents(instruction: Instruction){

    print!("Tokens:");
    for token in instruction.tokens{
        print!(" {}", token);

    }
    println!();

    println!("Binary representation: {}", instruction.binary_representation);
    println!("Int representation: {}", instruction.int_representation);

    for error in instruction.errors{
        println!("Error: {:?} on token number {}", error.error_name, error.token_number_giving_error );
    }
    println!();
}

#[cfg(test)]
mod tokenize_instruction_tests{
    use crate::parser::parser_instruction_tokenization::*;

    #[test]
    fn tokenize_instruction_returns_struct_with_tokens(){
        let   correct_instruction = Instruction{
            tokens: vec!["ADD".to_string(), "T1".to_string(), "T2".to_string(), "T2".to_string() ],
            binary_representation: String::new(),
            int_representation: 0,
            // instruction_number: 0,
            errors: vec![]
        };
        let received_instruction = tokenize_instruction("ADD T1 T2 T2");
        assert_eq!(received_instruction.tokens, correct_instruction.tokens);
    }
}

mod confirm_commas_tests {
    use crate::parser::parser_instruction_tokenization::*;
    use crate::parser::parser_instruction_tokenization::ErrorType::*;

    #[test]
    fn confirm_comma_generates_error_when_a_middle_token_is_missing_a_comma() {
        let mut instruction = Instruction::default();
        instruction.tokens = vec!["add".to_string(), "$t1,".to_string(), "$t1".to_string(), "$t1".to_string()];
        instruction = confirm_commas_in_instruction(instruction);
        assert_eq!(instruction.errors[0].error_name, MissingComma);
        assert_eq!(instruction.errors[0].token_number_giving_error, 2);
    }

    #[test]
    fn confirm_comma_can_generate_multiple_errors_if_multiple_commas_are_missing(){
        let mut instruction = Instruction::default();
        instruction.tokens = vec!["add".to_string(), "$t1".to_string(), "$zero".to_string(), "$t1".to_string()];
        instruction = confirm_commas_in_instruction(instruction);
        assert_eq!(instruction.errors[0].error_name, MissingComma);
        assert_eq!(instruction.errors[0].token_number_giving_error, 1);
        assert_eq!(instruction.errors[1].error_name, MissingComma);
        assert_eq!(instruction.errors[1].token_number_giving_error, 2);
    }

    #[test]
    fn confirm_comma_does_not_generate_errors_given_proper_syntax(){
        let mut instruction = Instruction::default();
        instruction.tokens = vec!["add".to_string(), "$t1,".to_string(), "$zero,".to_string(), "$t1".to_string()];
        instruction = confirm_commas_in_instruction(instruction);
        assert_eq!(instruction.errors.len(), 0);
    }
}

mod create_vector_of_instructions_tests{
    use crate::parser::parser_instruction_tokenization::{create_vector_of_instructions, Instruction};

    #[test]
    fn create_vector_of_instructions_builds_the_correct_number_of_instructions(){
        let original_string = "add $t1, $t2, $zero\nsub $t2, $t2, $t2\nlw r8, 52($s0)".to_string();
        let instructions: Vec<Instruction> = create_vector_of_instructions(original_string);
        assert_eq!(instructions.len(), 3);
    }

    #[test]
    fn create_vector_of_instructions_separates_instructions_at_correct_spot(){
        let original_string = "add $t1, $t2, $zero\nsub $t2, $t2, $t2\nlw r8, 52($s0)".to_string();
        let instructions: Vec<Instruction> = create_vector_of_instructions(original_string);
        assert_eq!(instructions[0].tokens, vec!["add", "$t1,", "$t2,", "$zero"]);
        assert_eq!(instructions[1].tokens, vec!["sub", "$t2,", "$t2,", "$t2"]);
        assert_eq!(instructions[2].tokens, vec!["lw", "r8,", "52($s0)"]);
    }
}