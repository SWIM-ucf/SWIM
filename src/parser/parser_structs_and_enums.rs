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
///This struct holds all the information we gather in the parser & assembler about a single line the user wrote
pub struct MonacoLineInfo {
    pub mouse_hover_string: String,
    pub updated_monaco_string: String,
    pub tokens: Vec<Token>,
    pub line_number: usize,
    pub error_start_end_columns: Vec<(usize, usize)>,
    pub errors: Vec<Error>,
    pub line_type: LineType,
}

impl MonacoLineInfo {
    ///This function puts the translation from a pseudo-instruction into the updated monaco string
    pub fn update_pseudo_string(&mut self, expansion: Vec<&mut Instruction>) {
        self.updated_monaco_string
            .insert_str(0, "#Pseudo-Instruction: ");

        for instruction in expansion {
            self.updated_monaco_string.push_str(&format!(
                "\n{} #Pseudo-Instruction Translation",
                instruction.recreate_string()
            ));
        }

        //special case to handle lw label pseudo-instruction being expanded when there isn't a syscall afterward
        if self.updated_monaco_string.contains("\nsyscall") {
            let mut update = self.updated_monaco_string.replace("\nsyscall", "");
            update.push_str("\nsyscall");
            self.updated_monaco_string = update;
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum LineType {
    #[default]
    Blank,
    Unknown(Vec<Token>),
    Instruction(Vec<Instruction>),
    Data(Data),
    Directive(Vec<Token>),
    Label(Vec<Token>)
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
            writeln!(f, "{line}")?;
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
    pub labels: Vec<LabelInstance>,
}

impl Instruction {
    ///Takes the operator, operands, and label(optional) associated with an instruction and recreates the string version
    pub fn recreate_string(&self) -> String {
        let mut recreated_string = "".to_string();
        //if the instruction had a label on the same line, start the string with that
        if !self.labels.is_empty()
            && self.labels.clone().last().unwrap().token_line == self.line_number
        {
            recreated_string.push_str(&format!(
                "{}: ",
                self.labels.clone().last().unwrap().token.token_name
            ));
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
    pub data_entries: Vec<Token>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LabelInstance {
    pub token_line: usize,
    pub token: Token,
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
    Directive,
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
    UnnecessaryComma,        //The given token should not end with a comma
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

pub const SUPPORTED_INSTRUCTIONS: [&str; 62] = [
    "add", "sub", "mul", "div", "lw", "sw", "lui", "aui", "andi", "ori", "addi", "dadd", "dsub",
    "dmul", "ddiv", "or", "and", "add.s", "add.d", "sub.s", "sub.d", "mul.s", "mul.d", "div.s",
    "div.d", "dahi", "dati", "daddi", "daddiu", "slt", "sltu", "swc1", "lwc1", "mtc1", "dmtc1",
    "mfc1", "dmfc1", "j", "beq", "bne", "c.eq.s", "c.eq.d", "c.lt.s", "c.le.s", "c.le.d",
    "c.ngt.s", "c.ngt.d", "c.nge.s", "c.nge.d", "bc1t", "bc1f", "daddu", "dsubu", "ddivu", "dmulu",
    "b", "nop", "sll", "jr", "jalr", "addiu", "jal",
];

pub const UNSUPPORTED_INSTRUCTIONS: [&str; 409] = [
    "abs.s",
    "abs.d",
    "abs.ps",
    "addiupc",
    "addu",
    "align",
    "dalign",
    "alnv.ps",
    "aluipc",
    "daui",
    "auipc",
    "bal",
    "balc",
    "bc",
    "bc1eqz",
    "bc1nez",
    "bc1fl",
    "bc1tl",
    "bc2eqz",
    "bc2nez",
    "bc2f",
    "bc2fl",
    "bc2t",
    "bc2tl",
    "beql",
    "bgez",
    "bgezal",
    "blezalc",
    "bgezalc",
    "bgtzalc",
    "bltzalc",
    "beqzalc",
    "bnezalc",
    "bgezall",
    "beqc",
    "bnec",
    "bltc",
    "bgec",
    "bltuc",
    "bgeuc",
    "bgtc",
    "blec",
    "bgtuc",
    "bleuc",
    "bltzc",
    "blezc",
    "bgezc",
    "bgtzc",
    "beqzc",
    "bnezc",
    "bgezl",
    "bgtz",
    "bgtzl",
    "bitswap",
    "dbitswap",
    "blez",
    "blezl",
    "bltz",
    "bltzal",
    "bltzall",
    "bltzl",
    "bnel",
    "bovc",
    "bnvc",
    "break",
    "c.f.s",
    "c.un.s",
    "c.ueq.s",
    "c.olt.s",
    "c.ult.s",
    "c.ole.s",
    "c.ule.s",
    "c.sf.s",
    "c.ngle.s",
    "c.seq.s",
    "c.ngl.s",
    "c.f.d",
    "c.un.d",
    "c.ueq.d",
    "c.olt.d",
    "c.ult.d",
    "c.ole.d",
    "c.ule.d",
    "c.sf.d",
    "c.ngle.d",
    "c.seq.d",
    "c.ngl.d",
    "cache",
    "cachee",
    "ceil.l.s",
    "ceil.l.d",
    "ceil.w.s",
    "ceil.w.d",
    "cfc1",
    "cfc2",
    "class.s",
    "class.d",
    "clo",
    "clz",
    "cmp.f.s",
    "cmp.un.s",
    "cmp.eq.s",
    "cmp.ueq.s",
    "cmp.olt.s",
    "cmp.ult.s",
    "cmp.ole.s",
    "cmp.ule.s",
    "cmp.sf.s",
    "cmp.ngle.s",
    "cmp.seq.s",
    "cmp.ngl.s",
    "cmp.lt.s",
    "cmp.nge.s",
    "cmp.le.s",
    "cmp.ngt.s",
    "cmp.f.d",
    "cmp.un.d",
    "cmp.eq.d",
    "cmp.ueq.d",
    "cmp.olt.d",
    "cmp.ult.d",
    "cmp.ole.d",
    "cmp.ule.d",
    "cmp.sf.d",
    "cmp.ngle.d",
    "cmp.seq.d",
    "cmp.ngl.d",
    "cmp.lt.d",
    "cmp.nge.d",
    "cmp.le.d",
    "cmp.ngt.d",
    "cop2",
    "crc32b",
    "crc32h",
    "crc32w",
    "crc32d",
    "crc32cb",
    "crc32ch",
    "crc32cw",
    "crc32cd",
    "ctc1",
    "ctc2",
    "cvt.d.s",
    "cvt.d.w",
    "cvt.d.l",
    "cvt.l.s",
    "cvt.l.d",
    "cvt.ps.s",
    "cvt.s.pl",
    "cvt.s.pu",
    "cvt.s.d",
    "cvt.s.w",
    "cvt.s.l",
    "cvt.w.s",
    "cvt.w.d",
    "dclo",
    "dclz",
    "deret",
    "dext",
    "dextm",
    "dextu",
    "di",
    "dins",
    "dinsm",
    "dinsu",
    "mod",
    "divu",
    "modu",
    "dmod",
    "dmodu",
    "dmfc0",
    "dmtc0",
    "dmtc2",
    "dmult",
    "dmultu",
    "drotr",
    "drotr32",
    "drotrv",
    "dsbh",
    "dshd",
    "dsll",
    "dsll32",
    "dsllv",
    "dsra",
    "dsra32",
    "dsrav",
    "dsrl",
    "dsrl32",
    "dsrlv",
    "dvp",
    "ehb",
    "ei",
    "eret",
    "eretnc",
    "evp",
    "ext",
    "floor.l.s",
    "floor.l.d",
    "floor.w.s",
    "floor.w.d",
    "ginvi",
    "ginvt",
    "ins",
    "jalr.hb",
    "jalx",
    "jialc",
    "jic",
    "jr.hb",
    "lb",
    "lbe",
    "lbu",
    "lbue",
    "ldc1",
    "ldc2",
    "ldl",
    "ldpc",
    "ldr",
    "ldxc1",
    "lh",
    "lhe",
    "lhu",
    "lhue",
    "ll",
    "lld",
    "lle",
    "lldp",
    "llwp",
    "llwpe",
    "lsa",
    "dlsa",
    "luxc1",
    "lwc2",
    "lwe",
    "lwl",
    "lwle",
    "lwpc",
    "lwr",
    "lwre",
    "lwu",
    "lwupc",
    "lwxc1",
    "madd",
    "madd.s",
    "madd.d",
    "madd.ps",
    "maddf.s",
    "maddf.d",
    "maddf.s",
    "msubf.s",
    "msubf.d",
    "maddu",
    "max.s",
    "max.d",
    "maxa.s",
    "maxa.d",
    "min.s",
    "mina.d",
    "mcf0",
    "mcf1",
    "mfc2",
    "mfhi",
    "mflo",
    "mov.s",
    "mov.d",
    "mov.ps",
    "movf",
    "movf.s",
    "movf.d",
    "movf.ps",
    "movn",
    "movn.s",
    "movn.d",
    "movn.ps",
    "movt",
    "movt.s",
    "movt.d",
    "movt.ps",
    "movz",
    "movz.s",
    "movz.d",
    "movz.ps",
    "msub",
    "msub.s",
    "msub.d",
    "msub.ps",
    "msubu",
    "mtc0",
    "mtc2",
    "mthc0",
    "mthc1",
    "mthc2",
    "mthi",
    "mtlo",
    "muh",
    "mulu",
    "muhu",
    "dmuh",
    "dmuhu",
    "mul.ps",
    "mult",
    "multu",
    "nal",
    "neg.s",
    "neg.d",
    "neg.ps",
    "nmadd.s",
    "nmadd.d",
    "nmadd.ps",
    "nmsub.s",
    "nmsub.d",
    "nmsub.ps",
    "nor",
    "pause",
    "pll.ps",
    "plu.ps",
    "pref",
    "prefe",
    "prefx",
    "pul.ps",
    "puu.ps",
    "rdhwr",
    "rdpgpr",
    "recip.s",
    "recip.d",
    "rint.s",
    "rint.d",
    "rotzr",
    "rotrv",
    "round.l.s",
    "round.l.d",
    "round.w.s",
    "round.w.d",
    "rsqrt.s",
    "rsqrt.d",
    "sb",
    "sbe",
    "sc",
    "scd",
    "scdp",
    "sce",
    "scwp",
    "scwpe",
    "sdbbp",
    "sdc1",
    "sdc2",
    "sdl",
    "sdr",
    "sdxc1",
    "seb",
    "seh",
    "sel.s",
    "sel.d",
    "seleqz",
    "selnez",
    "seleqz.s",
    "seleqz.d",
    "selneqz.s",
    "selneqz.d",
    "sh",
    "she",
    "sigrie",
    "sllv",
    "slti",
    "sltiu",
    "sqrt.s",
    "sqrt.d",
    "sra",
    "srav",
    "srl",
    "srlv",
    "ssnop",
    "sub.ps",
    "subu",
    "suxc1",
    "swc2",
    "swe",
    "swl",
    "swle",
    "swr",
    "swre",
    "swxc1",
    "sync",
    "synci",
    "teq",
    "teqi",
    "tge",
    "tgei",
    "tgeiu",
    "tgeu",
    "tlbinv",
    "tlbinvf",
    "tlbp",
    "tlbr",
    "tlbwi",
    "tlbwr",
    "tlt",
    "tlti",
    "tltiu",
    "tltu",
    "tne",
    "tnei",
    "trunc.l.s",
    "trunc.l.d",
    "trunc.w.s",
    "trunc.w.d",
    "wait",
    "wrpgpr",
    "xor",
    "xori",
];

///Contains every general purpose register's binary value and the various names they are recognized as. Any reference to gp registers throughout the parser/assembler should reference this array
pub const GP_REGISTERS: &[GPRegister; 32] = &[
    GPRegister {
        names: &["$zero", "r0", "$0", "zero"],
        binary: 0b00000,
    },
    GPRegister {
        names: &["$at", "r1", "$1", "at"],
        binary: 0b00001,
    },
    GPRegister {
        names: &["$v0", "r2", "$2", "v0"],
        binary: 0b00010,
    },
    GPRegister {
        names: &["$v1", "r3", "$3", "v1"],
        binary: 0b00011,
    },
    GPRegister {
        names: &["$a0", "r4", "$4", "a0"],
        binary: 0b00100,
    },
    GPRegister {
        names: &["$a1", "r5", "$5", "a1"],
        binary: 0b00101,
    },
    GPRegister {
        names: &["$a2", "r6", "$6", "a2"],
        binary: 0b00110,
    },
    GPRegister {
        names: &["$a3", "r7", "$7", "a3"],
        binary: 0b00111,
    },
    GPRegister {
        names: &["$t0", "r8", "$8", "t0"],
        binary: 0b01000,
    },
    GPRegister {
        names: &["$t1", "r9", "$9", "t1"],
        binary: 0b01001,
    },
    GPRegister {
        names: &["$t2", "r10", "$10", "t2"],
        binary: 0b01010,
    },
    GPRegister {
        names: &["$t3", "r11", "$11", "t3"],
        binary: 0b01011,
    },
    GPRegister {
        names: &["$t4", "r12", "$12", "t4"],
        binary: 0b01100,
    },
    GPRegister {
        names: &["$t5", "r13", "$13", "t5"],
        binary: 0b01101,
    },
    GPRegister {
        names: &["$t6", "r14", "$14", "t6"],
        binary: 0b01110,
    },
    GPRegister {
        names: &["$t7", "r15", "$15", "t7"],
        binary: 0b01111,
    },
    GPRegister {
        names: &["$s0", "r16", "$16", "s0"],
        binary: 0b10000,
    },
    GPRegister {
        names: &["$s1", "r17", "$17", "s1"],
        binary: 0b10001,
    },
    GPRegister {
        names: &["$s2", "r18", "$18", "s2"],
        binary: 0b10010,
    },
    GPRegister {
        names: &["$s3", "r19", "$19", "s3"],
        binary: 0b10011,
    },
    GPRegister {
        names: &["$s4", "r20", "$20", "s4"],
        binary: 0b10100,
    },
    GPRegister {
        names: &["$s5", "r21", "$21", "s5"],
        binary: 0b10101,
    },
    GPRegister {
        names: &["$s6", "r22", "$22", "s6"],
        binary: 0b10110,
    },
    GPRegister {
        names: &["$s7", "r23", "$23", "s7"],
        binary: 0b10111,
    },
    GPRegister {
        names: &["$t8", "r24", "$24", "t8"],
        binary: 0b11000,
    },
    GPRegister {
        names: &["$t9", "r25", "$25", "t9"],
        binary: 0b11001,
    },
    GPRegister {
        names: &["$k0", "r26", "$26", "k0"],
        binary: 0b11010,
    },
    GPRegister {
        names: &["$k1", "r27", "$27", "k1"],
        binary: 0b11011,
    },
    GPRegister {
        names: &["$gp", "r28", "$28", "gp"],
        binary: 0b11100,
    },
    GPRegister {
        names: &["$sp", "r29", "$29", "sp", "$s8", "s8"],
        binary: 0b11101,
    },
    GPRegister {
        names: &["$fp", "r30", "$30", "fp"],
        binary: 0b11110,
    },
    GPRegister {
        names: &["$ra", "r31", "$31", "ra"],
        binary: 0b11111,
    },
];
pub struct GPRegister<'a> {
    pub names: &'a [&'a str],
    pub binary: u8,
}

///Contains every floating point register name and binary value. Any reference to fp registers throughout the parser/assembler should reference this array
pub const FP_REGISTERS: &[FPRegister] = &[
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
    for label in instruction.labels {
        println!("Label: {}", label.token.token_name);
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
    for data_entry in data.data_entries {
        println!("{:?}", data_entry.token_name);
    }
    for error in data.errors {
        println!("{:?}", error.error_name);
    }
}
