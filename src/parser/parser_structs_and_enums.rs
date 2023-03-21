pub mod instruction_tokenization {
    use std::default::Default;
    use std::fmt;

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    ///Wrapper for all information gathered in the Parser/Assembler about the written program.
    pub struct ProgramInfo {
        pub monaco_line_info: Vec<MonacoLineInfo>,
        pub address_to_line_number: Vec<usize>,
        pub updated_monaco_string: String,
        pub console_out_post_assembly: String,
        pub instructions: Vec<Instruction>,
        pub data: Vec<Data>,
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct MonacoLineInfo {
        pub mouse_hover_string: String,
        pub updated_monaco_string: String,
        pub tokens: Vec<Token>,
        pub line_number: usize,
        pub error_start_end_columns: Vec<(usize, usize)>,
        pub errors: Vec<Error>,
        pub line_type: LineType,
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub enum LineType {
        #[default]
        Blank,
        Data,
        Text,
        Directive,
    }

    impl MonacoLineInfo {
        pub fn update_pseudo_string(&mut self, expansion: Vec<&mut Instruction>) {
            self.updated_monaco_string
                .insert_str(0, "#Pseudo-Instruction: ");

            for instruction in expansion {
                self.updated_monaco_string.push_str(&format!(
                    "\n{} #Pseudo-Instruction Translation",
                    instruction.recreate_string()
                ));
            }
        }
    }

    ///A collection of all relevant information found about an instruction in the Parser/Assembler
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct Instruction {
        pub operator: Token,
        pub operands: Vec<Token>,
        pub binary: u32,
        pub instruction_number: usize,
        pub line_number: usize,
        pub errors: Vec<Error>,
        pub label: Option<(Token, usize)>, //label.1 refers to the line number the label is on
    }

    impl Instruction {
        ///Takes the operator, operands, and label(optional) associated with an instruction and recreates the string version
        pub fn recreate_string(&self) -> String {
            let mut recreated_string = "".to_string();
            //if the instruction had a label on the same line, start the string with that
            if self.label.is_some() && self.label.clone().unwrap().1 == self.line_number {
                recreated_string
                    .push_str(&format!("{}: ", self.label.clone().unwrap().0.token_name));
            }
            recreated_string.push_str(&self.operator.token_name.to_string());

            for operand in &self.operands {
                recreated_string.push_str(&format!(" {},", operand.token_name.clone()));
            }
            //pop the extra comma
            recreated_string.pop();

            recreated_string
        }
    }

    ///A collection of all relevant information found about a variable in the Parser/Assembler
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct Data {
        pub data_number: usize,
        pub line_number: usize,
        pub errors: Vec<Error>,
        pub label: Token,
        pub data_type: Token,
        pub data_entries_and_values: Vec<(Token, u32)>,
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct Token {
        pub token_name: String,
        pub start_end_columns: (usize, usize),
        pub token_type: TokenType,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Error {
        pub error_name: ErrorType,
        pub token_causing_error: String,
        pub start_end_columns: (usize, usize),
        pub message: String,
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub enum TokenType {
        #[default]
        Unknown,
        Label,
        LabelOperand,
        Immediate,
        MemoryAddress,
        RegisterFP,
        RegisterGP,
        Operator,
        Half,
        Word,
        ASCIIZ,
        ASCII,
        DataType,
        Space,
        Byte,
        Float,
        Double,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum ErrorType {
        UnsupportedInstruction, //valid MIPS64 instruction that is not supported by SWIM
        UnrecognizedGPRegister, //Given string does not match GP Register names
        UnrecognizedFPRegister, //Given string does not match FP Register names
        UnrecognizedInstruction, //Given string does not match any valid MIPs64 instructions or our supported pseudo-instructions
        UnrecognizedDataType,    //Given string does not match data type directives
        IncorrectRegisterTypeFP, //Expected GP Register but received FP
        IncorrectRegisterTypeGP, //Expected FP Register but received GP
        MissingComma,            //Operand expected to end with a comma but does not
        ImmediateOutOfBounds,    //Immediate value given cannot be expressed in given number of bits
        NonIntImmediate,         //Given string cannot be recognized as an integer
        NonFloatImmediate,       //Given string cannot be recognized as a float
        InvalidMemorySyntax, //Given string for memory does not match syntax of "offset(base)" or "label"
        IncorrectNumberOfOperands, //The given number of operands does not match the number expected for an instruction
        LabelAssignmentError, //A label is specified but it is not followed by anything committed to memory
        LabelMultipleDefinition, //The given label name is already used elsewhere in the project
        LabelNotFound,        //The given label operand does not match a given label
        ImproperlyFormattedLabel, //Label assignment recognized but does not end in a colon.
        ImproperlyFormattedData, //Line of data does not contain the proper number of tokens
        ImproperlyFormattedASCII, //Token recognized as ASCII does not start and or end with "
        ImproperlyFormattedChar, //Token recognized as a char does not end with ' or is larger than a single char
        NonASCIIString, //One or multiple characters within the given string cannot be represented in ASCII
        NonASCIIChar,   //The given char cannot be represented in ASCII
    }

    impl fmt::Display for ErrorType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{self:?}")
        }
    }

    //this enum is used for the fn read_operands to choose the types of operands expected for an instruction type
    pub enum OperandType {
        RegisterGP,
        RegisterFP,
        Immediate,
        MemoryAddress,
        LabelAbsolute,
        LabelRelative,
    }

    //This enum is just for the read_register_function to determine which register type it should expect
    #[derive(Eq, PartialEq)]
    pub enum RegisterType {
        GeneralPurpose,
        FloatingPoint,
    }

    pub fn print_vec_of_instructions(instructions: Vec<Instruction>) {
        for instruction in instructions {
            print_instruction_contents(instruction);
            println!();
        }
    }

    pub fn print_vec_of_data(data: Vec<Data>) {
        for data_entry in data {
            print_data_contents(data_entry);
            println!();
        }
    }

    pub fn print_instruction_contents(instruction: Instruction) {
        println!("Operator: {}", instruction.operator.token_name);
        print!("Operands: ");
        for operand in instruction.operands {
            print!("{} ", operand.token_name);
        }
        println!();
        if instruction.label.is_some() {
            println!("Label: {:?}", instruction.label.unwrap().0);
        }
        print!("Errors: ");
        for error in instruction.errors {
            print!("{:?} ", error.error_name);
        }
    }

    pub fn print_data_contents(data: Data) {
        println!("Label: {}", data.label.token_name);
        println!("Data Type: {}", data.data_type.token_name);
        println!("Data Entries:");
        for data_entry in data.data_entries_and_values {
            println!("{:?}", data_entry.0);
        }
        for error in data.errors {
            println!("{:?}", error.error_name);
        }
    }
}
