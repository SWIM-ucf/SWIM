pub mod instruction_tokenization {
    use std::default::Default;

    #[derive(Default, Debug, Clone, PartialEq, Eq)]
    ///Wrapper for all information gathered in the Parser/Assembler about the written program.
    pub struct ProgramInfo {
        pub instructions: Vec<Instruction>,
        pub data: Vec<Data>,
        pub comments_line_and_column: Vec<[u32; 2]>,
        pub directives: Vec<(Token, u32)>,
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
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
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
        LabelAssignmentError,
        LabelMultipleDefinition,
        LabelNotFound,
        IncorrectlyFormattedLabel,
        IncorrectlyFormattedData,
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

    pub fn print_instruction_struct_contents(instruction: &Instruction) {
        println!("Instruction Number: {}", instruction.instruction_number);
        println!("Line Number: {}", instruction.line_number);

        println!();

        println!("Binary representation: {:b}", instruction.binary);
        println!("Int representation: {}", instruction.binary);

        for error in &instruction.errors {
            print!("{error:?}");
            if error.operand_number.is_some() {
                println!(" on operand {}.", error.operand_number.unwrap());
            } else {
                println!();
            }
        }
        println!();
    }
}
