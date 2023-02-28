pub mod instruction_tokenization {
    use std::default::Default;

    #[derive(Default, Debug, Clone, PartialEq, Eq)]
    ///Wrapper for all information gathered in the Parser/Assembler about the written program.
    pub struct ProgramInfo {
        pub monaco_line_info: Vec<MonacoLineInfo>,
        pub address_to_line_number: Vec<(u32, u32)>,
        pub instructions: Vec<Instruction>,
        pub data: Vec<Data>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Eq)]
    pub struct MonacoLineInfo {
        pub mouse_hover_string: String,
        pub error_start_end_columns: Vec<(u32, u32)>,
        pub monaco_updated_string: String,
    }

    ///A collection of all relevant information found about an instruction in the Parser/Assembler
    #[derive(Default, Debug, Clone, PartialEq, Eq)]
    pub struct Instruction {
        pub operator: Token,
        pub operands: Vec<Token>,
        pub binary: u32,
        pub instruction_number: u32,
        pub line_number: u32,
        pub errors: Vec<Error>,
        pub label: Option<(Token, u32)>,
    }

    ///A collection of all relevant information found about a variable in the Parser/Assembler
    #[derive(Default, Debug, Clone, PartialEq, Eq)]
    pub struct Data {
        pub data_number: u32,
        pub line_number: u32,
        pub errors: Vec<Error>,
        pub label: Token,
        pub data_type: Token,
        pub data_entries_and_values: Vec<(Token, u32)>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Eq)]
    pub struct Token {
        pub token_name: String,
        pub starting_column: u32,
        pub token_type: TokenType,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Line {
        pub line_number: u32,
        pub tokens: Vec<Token>,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Error {
        pub error_name: ErrorType,
        pub operand_number: Option<u8>,
        pub message: String,
    }

    #[derive(Default, Debug, PartialEq, Eq, Clone)]
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

    #[derive(Debug, PartialEq, Eq, Clone)]
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
    #[derive(PartialEq, Eq)]
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
