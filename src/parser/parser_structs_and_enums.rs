use std::default::Default;
use std::fmt;
use std::fmt::Formatter;
use std::string::ToString;

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstructionDescription {
    pub syntax: String,
    pub description: String,
}
impl fmt::Display for InstructionDescription {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "**Syntax:** `{}`\n\n", self.syntax)?;
        write!(f, "{}", self.description)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PseudoDescription {
    pub name: String,
    pub syntax: String,
    pub translation_lines: Vec<String>,
}
impl fmt::Display for PseudoDescription {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "`{}` is a pseudo-instruction.\n\n", self.name)?;
        write!(f, "```\n{} =>\n", self.syntax)?;
        for line in &self.translation_lines {
            writeln!(f, "{}", line)?;
        }
        write!(f, "\n```\n\n",)?;
        Ok(())
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
            recreated_string.push_str(&format!("{}: ", self.label.clone().unwrap().0.token_name));
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
    JALRRDRegisterZero, //The destination address for JALR cannot be the zero register
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
    ShiftAmount,
}

pub static GP_REGISTERS: &[GPRegister; 32] = &[
    GPRegister {
        names: ["$zero", "r0", "$0"],
        binary: 0b00000,
    },
    GPRegister {
        names: ["$at", "r1", "$1"],
        binary: 0b00001,
    },
    GPRegister {
        names: ["$v0", "r2", "$2"],
        binary: 0b00010,
    },
    GPRegister {
        names: ["$v1", "r3", "$3"],
        binary: 0b00011,
    },
    GPRegister {
        names: ["$a0", "r4", "$4"],
        binary: 0b00100,
    },
    GPRegister {
        names: ["$a1", "r5", "$5"],
        binary: 0b00101,
    },
    GPRegister {
        names: ["$a2", "r6", "$6"],
        binary: 0b00110,
    },
    GPRegister {
        names: ["$a3", "r7", "$7"],
        binary: 0b00111,
    },
    GPRegister {
        names: ["$t0", "r8", "$8"],
        binary: 0b01000,
    },
    GPRegister {
        names: ["$t1", "r9", "$9"],
        binary: 0b01001,
    },
    GPRegister {
        names: ["$t2", "r10", "$10"],
        binary: 0b01010,
    },
    GPRegister {
        names: ["$t3", "r11", "$11"],
        binary: 0b01011,
    },
    GPRegister {
        names: ["$t4", "r12", "$12"],
        binary: 0b01100,
    },
    GPRegister {
        names: ["$t5", "r13", "$13"],
        binary: 0b01101,
    },
    GPRegister {
        names: ["$t6", "r14", "$14"],
        binary: 0b01110,
    },
    GPRegister {
        names: ["$t7", "r15", "$15"],
        binary: 0b01111,
    },
    GPRegister {
        names: ["$s0", "r16", "$16"],
        binary: 0b10000,
    },
    GPRegister {
        names: ["$s1", "r17", "$17"],
        binary: 0b10001,
    },
    GPRegister {
        names: ["$s2", "r18", "$18"],
        binary: 0b10010,
    },
    GPRegister {
        names: ["$s3", "r19", "$19"],
        binary: 0b10011,
    },
    GPRegister {
        names: ["$s4", "r20", "$20"],
        binary: 0b10100,
    },
    GPRegister {
        names: ["$s5", "r21", "$21"],
        binary: 0b10101,
    },
    GPRegister {
        names: ["$s6", "r22", "$22"],
        binary: 0b10110,
    },
    GPRegister {
        names: ["$s7", "r23", "$23"],
        binary: 0b10111,
    },
    GPRegister {
        names: ["$t8", "r24", "$24"],
        binary: 0b11000,
    },
    GPRegister {
        names: ["$t9", "r25", "$25"],
        binary: 0b11001,
    },
    GPRegister {
        names: ["$k0", "r26", "$26"],
        binary: 0b11010,
    },
    GPRegister {
        names: ["$k1", "r27", "$27"],
        binary: 0b11011,
    },
    GPRegister {
        names: ["$gp", "r28", "$28"],
        binary: 0b11100,
    },
    GPRegister {
        names: ["$sp", "r29", "$29"],
        binary: 0b11101,
    },
    GPRegister {
        names: ["$fp", "r30", "$30"],
        binary: 0b11110,
    },
    GPRegister {
        names: ["$ra", "r31", "$31"],
        binary: 0b11111,
    },
];
pub struct GPRegister<'a> {
    pub names: [&'a str; 3],
    pub binary: u8,
}

pub static FP_REGISTERS: &[FPRegister] = &[
    FPRegister {
        name: "$f0",
        binary: 0b00000,
    },
    FPRegister {
        name: "$f1",
        binary: 0b00001,
    },
    FPRegister {
        name: "$f2",
        binary: 0b00010,
    },
    FPRegister {
        name: "$f3",
        binary: 0b00011,
    },
    FPRegister {
        name: "$f4",
        binary: 0b00100,
    },
    FPRegister {
        name: "$f5",
        binary: 0b00101,
    },
    FPRegister {
        name: "$f6",
        binary: 0b00110,
    },
    FPRegister {
        name: "$f7",
        binary: 0b00111,
    },
    FPRegister {
        name: "$f8",
        binary: 0b01000,
    },
    FPRegister {
        name: "$f9",
        binary: 0b01001,
    },
    FPRegister {
        name: "$f10",
        binary: 0b01010,
    },
    FPRegister {
        name: "$f11",
        binary: 0b01011,
    },
    FPRegister {
        name: "$f12",
        binary: 0b01100,
    },
    FPRegister {
        name: "$f13",
        binary: 0b01101,
    },
    FPRegister {
        name: "$f14",
        binary: 0b01110,
    },
    FPRegister {
        name: "$f15",
        binary: 0b01111,
    },
    FPRegister {
        name: "$f16",
        binary: 0b10000,
    },
    FPRegister {
        name: "$f17",
        binary: 0b10001,
    },
    FPRegister {
        name: "$f18",
        binary: 0b10010,
    },
    FPRegister {
        name: "$f19",
        binary: 0b10011,
    },
    FPRegister {
        name: "$f20",
        binary: 0b10100,
    },
    FPRegister {
        name: "$f21",
        binary: 0b10101,
    },
    FPRegister {
        name: "$f22",
        binary: 0b10110,
    },
    FPRegister {
        name: "$f23",
        binary: 0b10111,
    },
    FPRegister {
        name: "$f24",
        binary: 0b11000,
    },
    FPRegister {
        name: "$f25",
        binary: 0b11001,
    },
    FPRegister {
        name: "$f26",
        binary: 0b11010,
    },
    FPRegister {
        name: "$f27",
        binary: 0b11011,
    },
    FPRegister {
        name: "$f28",
        binary: 0b11100,
    },
    FPRegister {
        name: "$f29",
        binary: 0b11101,
    },
    FPRegister {
        name: "$f30",
        binary: 0b11110,
    },
    FPRegister {
        name: "$f31",
        binary: 0b11111,
    },
];

pub struct FPRegister<'a> {
    pub name: &'a str,
    pub binary: u8,
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
