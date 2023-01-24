pub mod instruction_tokenization {
    use std::default::Default;

    #[derive(Default, Debug, Clone, PartialEq, Eq)]
    pub struct Instruction {
        pub tokens: Vec<String>,
        pub operator: Token,
        pub operands: Vec<Token>,
        pub binary: u32,
        pub instruction_number: u32,
        pub line_number: u32,
        pub errors: Vec<Error>,
        pub label: Option<(Token, i32)>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Eq)]
    pub struct Token {
        pub token_name: String,
        pub starting_column: i32,
        pub token_type: TokenType,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Line {
        pub line_number: i32,
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
        JALRRDRegisterZero,
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
            print!("{:?}", error);
            if error.operand_number.is_some() {
                println!(" on operand {}.", error.operand_number.unwrap());
            } else {
                println!();
            }
        }
        println!();
    }
}
