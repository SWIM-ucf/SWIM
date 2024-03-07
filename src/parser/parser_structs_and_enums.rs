use std::default::Default;
use std::fmt;
use std::fmt::Formatter;
use std::string::ToString;

use gloo_console::log;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
///Wrapper for all information gathered in the Parser/Assembler about the written program.
pub struct ProgramInfo {
    pub monaco_line_info: Vec<MonacoLineInfo>,
    pub address_to_line_number: Vec<usize>,
    pub updated_monaco_string: String,
    pub console_out_post_assembly: String,
    pub instructions: Vec<Instruction>,
    pub data: Vec<Data>,
    pub pc_starting_point: usize,
    pub data_starting_point: usize,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum Architecture {
    #[default]
    MIPS,
    RISCV,
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
}

impl MonacoLineInfo {
    ///This function puts the translation from a pseudo-instruction into the updated monaco string
    pub fn update_pseudo_string(&mut self, expansion: Vec<&mut Instruction>) {
        let (prefix, index) = self.get_tab_space_offset();

        self.updated_monaco_string
            .insert_str(index, "#Pseudo-Instruction: ");

        for instruction in expansion {
            self.updated_monaco_string.push_str(&format!(
                "\n{}{} #Pseudo-Instruction Translation",
                prefix,
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

    ///Returns a string of all the spaces and tabs that are at the beginning of updated_monaco_string and returns the number of characters that is
    pub fn get_tab_space_offset(&self) -> (String, usize) {
        let mut prefix = "".to_string();
        let mut index = 0;
        for (i, char) in self.updated_monaco_string.chars().enumerate() {
            index = i;
            if char == ' ' || char == '\t' {
                prefix.push(char);
            } else {
                break;
            }
        }
        (prefix, index)
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

impl Data {
    ///Takes the operator, operands, and label(optional) associated with an instruction and recreates the string version
    pub fn recreate_string(&self) -> String {
        let mut recreated_string = "".to_string();
        recreated_string.push_str(&format!("{}: ", self.label.clone().token_name));
        recreated_string.push_str(&self.data_type.token_name.to_string());

        for token in &self.data_entries {
            recreated_string.push_str(&format!(" {},", token.token_name.clone()));
        }
        //pop the extra comma
        recreated_string.pop();

        recreated_string
    }
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
    UpperImmediate,
    MemoryAddress,
    LabelAbsolute,
    LabelRelative,
    ShiftAmount,
}

pub const SUPPORTED_INSTRUCTIONS_MIPS: [&str; 64] = [
    // MIPS Instructions
    "add", "add.d", "add.s", "addi", "addiu", "addu", "and", "andi", "aui", "b", "bc1f", "bc1t",
    "beq", "bne", "c.eq.d", "c.eq.s", "c.le.d", "c.le.s", "c.lt.d", "c.lt.s", "c.nge.d", "c.nge.s",
    "c.ngt.d", "c.ngt.s", "dadd", "daddi", "daddiu", "daddu", "dahi", "dati", "ddiv", "ddivu",
    "div", "div.d", "div.s", "dmfc1", "dmtc1", "dmul", "dmulu", "dsub", "dsubu", "j", "jal",
    "jalr", "jr", "lui", "lw", "lwc1", "mfc1", "mtc1", "mul", "mul.d", "mul.s", "nop", "or", "ori",
    "sll", "slt", "sltu", "sub", "sub.d", "sub.s", "sw", "swc1",
];

pub const SUPPORTED_INSTRUCTIONS_RISCV: [&str; 131] = [
    // RV32I
    "lui",
    "auipc",
    "addi",
    "slti",
    "xori",
    "ori",
    "andi",
    "slli",
    "srli",
    "srai",
    "add",
    "sub",
    "sll",
    "slt",
    "sltu",
    "cor",
    "srl",
    "sra",
    "or",
    "and",
    "fence",
    "fence.i",
    "csrrw",
    "csrrs",
    "csrrc",
    "csrrwi",
    "csrrsi",
    "csrrci",
    "ecall",
    "ebreak",
    "uret",
    "sret",
    "mret",
    "wfi",
    "sfence.vma",
    "lb",
    "lh",
    "lw",
    "lbu",
    "lhu",
    "sb",
    "sh",
    "sw",
    "jal",
    "jalr",
    "beq",
    "bne",
    "blt",
    "bge",
    "bltu",
    "bgeu",
    // RV64I
    "addiw",
    "slliw",
    "srliw",
    "addw",
    "subw",
    "sllw",
    "srlw",
    "sraw",
    "lwu",
    "ld",
    "sd",
    // RV32M
    "mul",
    "mulh",
    "mulhsu",
    "mulhu",
    "div",
    "divu",
    "rem",
    "remu",
    // RV64M
    "mulw",
    "divw",
    "divuw",
    "remw",
    "remuw",
    // RV32F
    "fmadd.s",
    "fmsub.s",
    "fnmsub.s",
    "fnmadd.s",
    "fadd.s",
    "fsub.s",
    "fmul.s",
    "fdiv.s",
    "fsqrt.s",
    "fsgnj.s",
    "fsgnjn.s",
    "fsgnjx.s",
    "fmin.s",
    "fmax.s",
    "fcvt.w.s",
    "fcvt.wu.s",
    "fmv.x.w",
    "feq.s",
    "flt.s",
    "fle.s",
    "fclass.s",
    "fcvt.s.w",
    "fcvt.s.wu",
    "fmv.w.x",
    "fmadd.d",
    "fmsub.d",
    "fnmadd.d",
    "fnmsub.d",
    "fadd.d",
    "fsub.d",
    "fmul.d",
    "fdiv.d",
    "fsqrt.d",
    "fsgnj.d",
    "fsgnjn.d",
    "fsgnjx.d",
    "fmin.d",
    "fmax.d",
    "fcvt.s.d",
    "fcvt.d.s",
    "feq.d",
    "flt.d",
    "fle.d",
    "fclass.d",
    "fcvt.w.d",
    "fcvt.wu.d",
    "fcvt.d.w",
    "fcvt.d.wu",
    "flw",
    "fsw",
    "fld",
    "fsd",
    // RV64F
    "fcvt.l.s",
    "fcvt.lu.s",
    "fcvt.s.l",
    "fcvt.s.lu",
];

pub const UNSUPPORTED_INSTRUCTIONS_MIPS: [&str; 408] = [
    // MIPS Instructions
    "abs.d",
    "abs.ps",
    "abs.s",
    "addiupc",
    "align",
    "alnv.ps",
    "aluipc",
    "auipc",
    "bal",
    "balc",
    "bc",
    "bc1eqz",
    "bc1fl",
    "bc1nez",
    "bc1tl",
    "bc2eqz",
    "bc2f",
    "bc2fl",
    "bc2nez",
    "bc2t",
    "bc2tl",
    "beqc",
    "beql",
    "beqzalc",
    "beqzc",
    "bgec",
    "bgeuc",
    "bgez",
    "bgezal",
    "bgezalc",
    "bgezall",
    "bgezc",
    "bgezl",
    "bgtc",
    "bgtuc",
    "bgtz",
    "bgtzalc",
    "bgtzc",
    "bgtzl",
    "bitswap",
    "blec",
    "bleuc",
    "blez",
    "blezalc",
    "blezc",
    "blezl",
    "bltc",
    "bltuc",
    "bltz",
    "bltzal",
    "bltzalc",
    "bltzall",
    "bltzc",
    "bltzl",
    "bnec",
    "bnel",
    "bnezalc",
    "bnezc",
    "bnvc",
    "bovc",
    "break",
    "c.f.d",
    "c.f.s",
    "c.ngl.d",
    "c.ngl.s",
    "c.ngle.d",
    "c.ngle.s",
    "c.ole.d",
    "c.ole.s",
    "c.olt.d",
    "c.olt.s",
    "c.seq.d",
    "c.seq.s",
    "c.sf.d",
    "c.sf.s",
    "c.ueq.d",
    "c.ueq.s",
    "c.ule.d",
    "c.ule.s",
    "c.ult.d",
    "c.ult.s",
    "c.un.d",
    "c.un.s",
    "cache",
    "cachee",
    "ceil.l.d",
    "ceil.l.s",
    "ceil.w.d",
    "ceil.w.s",
    "cfc1",
    "cfc2",
    "class.d",
    "class.s",
    "clo",
    "clz",
    "cmp.eq.d",
    "cmp.eq.s",
    "cmp.f.d",
    "cmp.f.s",
    "cmp.le.d",
    "cmp.le.s",
    "cmp.lt.d",
    "cmp.lt.s",
    "cmp.nge.d",
    "cmp.nge.s",
    "cmp.ngl.d",
    "cmp.ngl.s",
    "cmp.ngle.d",
    "cmp.ngle.s",
    "cmp.ngt.d",
    "cmp.ngt.s",
    "cmp.ole.d",
    "cmp.ole.s",
    "cmp.olt.d",
    "cmp.olt.s",
    "cmp.seq.d",
    "cmp.seq.s",
    "cmp.sf.d",
    "cmp.sf.s",
    "cmp.ueq.d",
    "cmp.ueq.s",
    "cmp.ule.d",
    "cmp.ule.s",
    "cmp.ult.d",
    "cmp.ult.s",
    "cmp.un.d",
    "cmp.un.s",
    "cop2",
    "crc32b",
    "crc32cb",
    "crc32cd",
    "crc32ch",
    "crc32cw",
    "crc32d",
    "crc32h",
    "crc32w",
    "ctc1",
    "ctc2",
    "cvt.d.l",
    "cvt.d.s",
    "cvt.d.w",
    "cvt.l.d",
    "cvt.l.s",
    "cvt.ps.s",
    "cvt.s.d",
    "cvt.s.l",
    "cvt.s.pl",
    "cvt.s.pu",
    "cvt.s.w",
    "cvt.w.d",
    "cvt.w.s",
    "dalign",
    "daui",
    "dbitswap",
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
    "divu",
    "dlsa",
    "dmfc0",
    "dmod",
    "dmodu",
    "dmtc0",
    "dmtc2",
    "dmuh",
    "dmuhu",
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
    "floor.l.d",
    "floor.l.s",
    "floor.w.d",
    "floor.w.s",
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
    "lldp",
    "lle",
    "llwp",
    "llwpe",
    "lsa",
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
    "madd.d",
    "madd.ps",
    "madd.s",
    "maddf.d",
    "maddf.s",
    "maddf.s",
    "maddu",
    "max.d",
    "max.s",
    "maxa.d",
    "maxa.s",
    "mcf0",
    "mcf1",
    "mfc2",
    "mfhi",
    "mflo",
    "min.s",
    "mina.d",
    "mod",
    "modu",
    "mov.d",
    "mov.ps",
    "mov.s",
    "movf",
    "movf.d",
    "movf.ps",
    "movf.s",
    "movn",
    "movn.d",
    "movn.ps",
    "movn.s",
    "movt",
    "movt.d",
    "movt.ps",
    "movt.s",
    "movz",
    "movz.d",
    "movz.ps",
    "movz.s",
    "msub",
    "msub.d",
    "msub.ps",
    "msub.s",
    "msubf.d",
    "msubf.s",
    "msubu",
    "mtc0",
    "mtc2",
    "mthc0",
    "mthc1",
    "mthc2",
    "mthi",
    "mtlo",
    "muh",
    "muhu",
    "mul.ps",
    "mult",
    "multu",
    "mulu",
    "nal",
    "neg.d",
    "neg.ps",
    "neg.s",
    "nmadd.d",
    "nmadd.ps",
    "nmadd.s",
    "nmsub.d",
    "nmsub.ps",
    "nmsub.s",
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
    "recip.d",
    "recip.s",
    "rint.d",
    "rint.s",
    "rotrv",
    "rotzr",
    "round.l.d",
    "round.l.s",
    "round.w.d",
    "round.w.s",
    "rsqrt.d",
    "rsqrt.s",
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
    "sel.d",
    "sel.s",
    "seleqz",
    "seleqz.d",
    "seleqz.s",
    "selneqz.d",
    "selneqz.s",
    "selnez",
    "sh",
    "she",
    "sigrie",
    "sllv",
    "slti",
    "sltiu",
    "sqrt.d",
    "sqrt.s",
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
    "trunc.l.d",
    "trunc.l.s",
    "trunc.w.d",
    "trunc.w.s",
    "wait",
    "wrpgpr",
    "xor",
    "xori",
    // RISC-V Instructions
];

pub const UNSUPPORTED_INSTRUCTIONS_RISCV: [&str; 69] = [
    "lr.w",
    "sc.w",
    "amoswap.w",
    "amoadd.w",
    "amoxor.w",
    "amoand.w",
    "amomin.w",
    "amomax.w",
    "amominu.w",
    "amomaxu.w",
    "lr.d",
    "sc.d",
    "amoswap.d",
    "amoadd.d",
    "amoxor.d",
    "amoand.d",
    "amoor.d",
    "amomin.d",
    "amomax.d",
    "amominu.d",
    "amomaxu.d",
    "fcvt.l.d",
    "scvt.lu.d",
    "fmv.x.d",
    "fcvt.d.l",
    "fcvt.d.lu",
    "fmv.d.x",
    "c.addi4spn",
    "c.fld",
    "c.lw",
    "c.flw",
    "c.ld",
    "c.fsd",
    "c.sw",
    "c.fsw",
    "c.sd",
    "c.nop",
    "c.addi",
    "c.jal",
    "c.addiw",
    "c.li",
    "c.addi16sp",
    "c.lui",
    "c.srli",
    "c.srai",
    "c.andi",
    "c.sub",
    "c.xor",
    "c.or",
    "c.and",
    "c.subw",
    "c.addw",
    "c.j",
    "c.beqz",
    "c.bnez",
    "c.slli",
    "c.fldsp",
    "c.lwsp",
    "c.flwsp",
    "c.ldsp",
    "c.jr",
    "c.mv",
    "c.ebreak",
    "c.jalr",
    "c.add",
    "c.fsdsp",
    "c.swsp",
    "c.fswsp",
    "c.sdsp",
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

pub struct GPRegisterRiscv<'a> {
    pub names: &'a [&'a str],
    pub binary: u8,
}

///Contains every general purpose register's binary value and the various names they are recognized as. Any reference to gp registers throughout the parser/assembler should reference this array
pub const RISCV_GP_REGISTERS: &[GPRegisterRiscv; 32] = &[
    GPRegisterRiscv {
        names: &["x0", "zero"],
        binary: 0b00000,
    },
    GPRegisterRiscv {
        names: &["x1", "ra"],
        binary: 0b00001,
    },
    GPRegisterRiscv {
        names: &["x2", "sp"],
        binary: 0b00010,
    },
    GPRegisterRiscv {
        names: &["x3", "gp"],
        binary: 0b00011,
    },
    GPRegisterRiscv {
        names: &["x4", "tp"],
        binary: 0b00100,
    },
    GPRegisterRiscv {
        names: &["x5", "t0"],
        binary: 0b00101,
    },
    GPRegisterRiscv {
        names: &["x6", "t1"],
        binary: 0b00110,
    },
    GPRegisterRiscv {
        names: &["x7", "t2"],
        binary: 0b00111,
    },
    GPRegisterRiscv {
        names: &["x8", "s0", "fp"],
        binary: 0b01000,
    },
    GPRegisterRiscv {
        names: &["x9", "s1"],
        binary: 0b01001,
    },
    GPRegisterRiscv {
        names: &["x10", "a0"],
        binary: 0b01010,
    },
    GPRegisterRiscv {
        names: &["x11", "a1"],
        binary: 0b01011,
    },
    GPRegisterRiscv {
        names: &["x12", "a2"],
        binary: 0b01100,
    },
    GPRegisterRiscv {
        names: &["x13", "a3"],
        binary: 0b01101,
    },
    GPRegisterRiscv {
        names: &["x14", "a4"],
        binary: 0b01110,
    },
    GPRegisterRiscv {
        names: &["x15", "a5"],
        binary: 0b01111,
    },
    GPRegisterRiscv {
        names: &["x16", "a6"],
        binary: 0b10000,
    },
    GPRegisterRiscv {
        names: &["x17", "a7"],
        binary: 0b10001,
    },
    GPRegisterRiscv {
        names: &["x18", "s2"],
        binary: 0b10010,
    },
    GPRegisterRiscv {
        names: &["x19", "s3"],
        binary: 0b10011,
    },
    GPRegisterRiscv {
        names: &["x20", "s4"],
        binary: 0b10100,
    },
    GPRegisterRiscv {
        names: &["x21", "s5"],
        binary: 0b10101,
    },
    GPRegisterRiscv {
        names: &["x22", "s6"],
        binary: 0b10110,
    },
    GPRegisterRiscv {
        names: &["x23", "s7"],
        binary: 0b10111,
    },
    GPRegisterRiscv {
        names: &["x24", "s8"],
        binary: 0b11000,
    },
    GPRegisterRiscv {
        names: &["x25", "s9"],
        binary: 0b11001,
    },
    GPRegisterRiscv {
        names: &["x26", "s10"],
        binary: 0b11010,
    },
    GPRegisterRiscv {
        names: &["x27", "s11"],
        binary: 0b11011,
    },
    GPRegisterRiscv {
        names: &["x28", "t3"],
        binary: 0b11100,
    },
    GPRegisterRiscv {
        names: &["x29", "t4"],
        binary: 0b11101,
    },
    GPRegisterRiscv {
        names: &["x30", "t5"],
        binary: 0b11110,
    },
    GPRegisterRiscv {
        names: &["x31", "t6"],
        binary: 0b11111,
    },
];

pub struct FPRegisterRiscv<'a> {
    pub names: &'a [&'a str],
    pub binary: u8,
}

///Contains every floating point register name and binary value. Any reference to fp registers throughout the parser/assembler should reference this array
pub const RISCV_FP_REGISTERS: &[FPRegisterRiscv; 32] = &[
    FPRegisterRiscv {
        names: &["f0", "ft0"],
        binary: 0b00000,
    },
    FPRegisterRiscv {
        names: &["f1", "ft1"],
        binary: 0b00001,
    },
    FPRegisterRiscv {
        names: &["f2", "ft2"],
        binary: 0b00010,
    },
    FPRegisterRiscv {
        names: &["f3", "ft3"],
        binary: 0b00011,
    },
    FPRegisterRiscv {
        names: &["f4", "ft4"],
        binary: 0b00100,
    },
    FPRegisterRiscv {
        names: &["f5", "ft5"],
        binary: 0b00101,
    },
    FPRegisterRiscv {
        names: &["f6", "ft6"],
        binary: 0b00110,
    },
    FPRegisterRiscv {
        names: &["f7", "ft7"],
        binary: 0b00111,
    },
    FPRegisterRiscv {
        names: &["f8", "fs0"],
        binary: 0b01000,
    },
    FPRegisterRiscv {
        names: &["f9", "fs1"],
        binary: 0b01001,
    },
    FPRegisterRiscv {
        names: &["f10", "fa0"],
        binary: 0b01010,
    },
    FPRegisterRiscv {
        names: &["f11", "fa1"],
        binary: 0b01011,
    },
    FPRegisterRiscv {
        names: &["f12", "fa2"],
        binary: 0b01100,
    },
    FPRegisterRiscv {
        names: &["f13", "fa3"],
        binary: 0b01101,
    },
    FPRegisterRiscv {
        names: &["f14", "fa4"],
        binary: 0b01110,
    },
    FPRegisterRiscv {
        names: &["f15", "fa5"],
        binary: 0b01111,
    },
    FPRegisterRiscv {
        names: &["f16", "fa6"],
        binary: 0b10000,
    },
    FPRegisterRiscv {
        names: &["f17", "fa7"],
        binary: 0b10001,
    },
    FPRegisterRiscv {
        names: &["f18", "fs2"],
        binary: 0b10010,
    },
    FPRegisterRiscv {
        names: &["f19", "fs3"],
        binary: 0b10011,
    },
    FPRegisterRiscv {
        names: &["f20", "fs4"],
        binary: 0b10100,
    },
    FPRegisterRiscv {
        names: &["f21", "fs5"],
        binary: 0b10101,
    },
    FPRegisterRiscv {
        names: &["f22", "fs6"],
        binary: 0b10110,
    },
    FPRegisterRiscv {
        names: &["f23", "fs7"],
        binary: 0b10111,
    },
    FPRegisterRiscv {
        names: &["f24", "fs8"],
        binary: 0b11000,
    },
    FPRegisterRiscv {
        names: &["f25", "fs9"],
        binary: 0b11001,
    },
    FPRegisterRiscv {
        names: &["f26", "fs10"],
        binary: 0b11010,
    },
    FPRegisterRiscv {
        names: &["f27", "fs11"],
        binary: 0b11011,
    },
    FPRegisterRiscv {
        names: &["f28", "ft8"],
        binary: 0b11100,
    },
    FPRegisterRiscv {
        names: &["f29", "ft9"],
        binary: 0b11101,
    },
    FPRegisterRiscv {
        names: &["f30", "ft10"],
        binary: 0b11110,
    },
    FPRegisterRiscv {
        names: &["f31", "ft11"],
        binary: 0b11111,
    },
];

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
    log!("Operator: ", instruction.operator.token_name);
    log!("Operands: ");
    for operand in instruction.operands {
        log!(operand.token_name);
    }
    log!("");
    for label in instruction.labels {
        log!("Label: ", label.token.token_name);
    }
    log!("Errors: ");
    for error in instruction.errors {
        log!(error.error_name.to_string());
    }

    log!("Binary ", instruction.binary);
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
