pub mod instruction_tokenization {
    pub struct Instruction {
        pub tokens: Vec<String>,
        pub binary: u32,
        //instruction_number is not yet being tracked
        // pub instruction_number: u32,
        pub errors: Vec<Error>,
    }

    pub struct Error {
        pub error_name: ErrorType,
        pub token_number_giving_error: u8,
    }

    impl Default for Instruction {
        fn default() -> Instruction {
            Instruction {
                tokens: vec![],
                binary: 0,
                //instruction_number: 0,
                errors: vec![],
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum ErrorType {
        UnrecognizedGPRegister,
        UnrecognizedFPRegister,
        UnrecognizedInstruction,
        IncorrectRegisterType,
        MissingComma,
        ImmediateOutOfBounds,
        NonIntImmediate,
        InvalidMemorySyntax,
        IncorrectNumberOfOperands,
    }

    //this enum is used for the fn read_operands to choose the types of operands expected for an instruction type
    pub enum OperandType {
        RegisterGP,
        RegisterFP,
        Immediate,
        MemoryAddress,
        //Label
    }

    //This enum is just for the read_register_function to determine which register type it should expect
    #[derive(PartialEq, Eq)]
    pub enum RegisterType {
        GeneralPurpose,
        FloatingPoint,
    }

    //takes the string representation of a line of MIPS code and breaks it up into tokens delimited by space characters
    pub fn tokenize_instruction(line: &str) -> Instruction {
        //breaks up line into a vector delimited by space characters
        let mut contents: Vec<String> = Vec::new();
        for token in line.split(' ') {
            contents.push(token.parse().unwrap());
        }

        //creates an instruction from the vector
        Instruction {
            tokens: contents,
            ..Default::default()
        }
    }

    //takes the string of the MIPS program after comments, extra spaces, and label names have been removed
    //and turns each line into an Instruction and returns the vec of these Instructions with the contents as tokens
    pub fn create_vector_of_instructions(file_string: String) -> Vec<Instruction> {
        let mut instructions: Vec<Instruction> = Vec::new();
        for line in file_string.lines() {
            instructions.push(tokenize_instruction(line));
        }
        instructions
    }

    //this function takes an instruction as its argument and checks that every token within it except the first and the last (ie all but the last operand) ends with the ',' character
    //for each of these that does end in a comma, the comma is removed. Any instance that this isn't the case generates a missingComma error that is added to the error list for that instruction.
    //the updated version of the instruction is then returned
    pub fn confirm_commas_in_instruction(mut instruction: Instruction) -> Instruction {
        //for loop goes through all but the first and last tokens
        for i in 1..(instruction.tokens.len() - 1) {
            let last_char = instruction.tokens.get(i).unwrap().chars().last().unwrap();

            if last_char == ',' {
                //this chunk of code removes the last char of the string if it is a ','
                //due to mutability issues, instruction.tokens.get(i).pop() does not work so instead we create a new string without the comma and replace the token instead
                let mut token_as_chars: Vec<char> =
                    instruction.tokens.get(i).unwrap().chars().collect();
                token_as_chars.remove(token_as_chars.len() - 1);
                instruction
                    .tokens
                    .push(token_as_chars.into_iter().collect());
                let length = instruction.tokens.len() - 1;
                instruction.tokens.swap(i, length);
                instruction.tokens.pop();
            } else {
                //if the last char of the token is not ',', an error is pushed to the list
                instruction.errors.push(Error {
                    error_name: ErrorType::MissingComma,
                    token_number_giving_error: i as u8,
                })
            }
        }

        instruction
    }

    pub fn print_instruction_struct_contents(instruction: &Instruction) {
        print!("Tokens:");
        for token in instruction.tokens.clone() {
            print!(" {}", token);
        }
        println!();

        println!("Binary representation: {:b}", instruction.binary);
        println!("Int representation: {}", instruction.binary);

        for error in &instruction.errors {
            println!(
                "Error: {:?} on token number {}",
                error.error_name, error.token_number_giving_error
            );
        }
        println!();
    }
}
