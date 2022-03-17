use phf::{phf_map, Map};

use crate::instruction::Instruction;

pub const WORD_SIZE: i32 = 1;
pub const ZERO_REGISTER: i32 = 0;
pub const AT_REGISTER: i32 = 1;
pub const SP_REGISTER: i32 = 28;
pub const RA_REGISTER: i32 = 31;
pub const R_INSTRUCTION_OPCODE: i32 = 0x3A;
pub const DATA_SECTION_MIN_ADDRESS: i32 = 0x100;
pub const TEXT_SECTION_MIN_ADDRESS: i32 = 0x0;

// https://www.intel.com/content/dam/www/programmable/us/en/pdfs/literature/hb/nios2/n2cpu_nii51017.pdf
pub const INSTRUCTION_TABLE: Map<&'static str, &'static Instruction> = phf_map! {
 // J-Type
 "call" => &Instruction::new("call", 0x0, 0x0),
 "jmpi" => &Instruction::new("jmpi", 0x1, 0x0),

 // I-Type
 "ldbu" => &Instruction::new("ldbu", 0x3, 0x0),
 "addi" => &Instruction::new("addi", 0x4, 0x0),
 "stb" => &Instruction::new("stb", 0x5, 0x0),
 "br" => &Instruction::new("br", 0x6, 0x0),
 "ldb" => &Instruction::new("ldb", 0x7, 0x0),
 "cmpgei" => &Instruction::new("cmpgei", 0x8, 0x0),
 "ldhu" => &Instruction::new("ldhu", 0x0B, 0x0),
 "andi" => &Instruction::new("andi", 0x0C, 0x0),
 "sth" => &Instruction::new("sth", 0x0D, 0x0),
 "bge" => &Instruction::new("bge", 0x0E, 0x0),
 "ldh" => &Instruction::new("ldh", 0x0F, 0x0),
 "stw" => &Instruction::new("stw", 0x15, 0x0),
 "blt" => &Instruction::new("blt", 0x16, 0x0),
 "ldw" => &Instruction::new("ldw", 0x17, 0x0),
 "cmpnei" => &Instruction::new("cmpnei", 0x17, 0x0),
 "flushda" => &Instruction::new("flushda", 0x1B, 0x0),
 "xori" => &Instruction::new("xori", 0x1C, 0x0),
 "bne" => &Instruction::new("bne", 0x1E, 0x0),
 "stbio" => &Instruction::new("stbio", 0x25, 0x0),
 "beq" => &Instruction::new("beq", 0x26, 0x0),
 "ldbio" => &Instruction::new("ldbio", 0x27, 0x0),
 "cmpgeui" => &Instruction::new("cmpgeui", 0x28, 0x0),
 "ldhuio" => &Instruction::new("ldhuio", 0x2B, 0x0),
 "andhi" => &Instruction::new("andhi", 0x2C, 0x0),
 "sthio" => &Instruction::new("sthio", 0x2D, 0x0),
 "bgeu" => &Instruction::new("bgeu", 0x2E, 0x0),
 "ldhio" => &Instruction::new("bldhio", 0x2F, 0x0),
 "stwio" => &Instruction::new("stwio", 0x35, 0x0),
 "bltu" => &Instruction::new("bltu", 0x36, 0x0),
 "ldwio" => &Instruction::new("ldwio", 0x37, 0x0),
 "rdprs" => &Instruction::new("rdprs", 0x38, 0x0),
 "flushd" => &Instruction::new("flushd", 0x3B, 0x0),
 "xorhi" => &Instruction::new("xorhi", 0x3C, 0x0),

 // R-Type
 "eret" => &Instruction::new("eret", R_INSTRUCTION_OPCODE, 0x01),
 "roli" => &Instruction::new("roli", R_INSTRUCTION_OPCODE, 0x02),
 "rol" => &Instruction::new("rol", R_INSTRUCTION_OPCODE, 0x03),
 "flushp" => &Instruction::new("flushp", R_INSTRUCTION_OPCODE, 0x04),
 "ret" => &Instruction::new("ret", R_INSTRUCTION_OPCODE, 0x05),
 "nor" => &Instruction::new("nor", R_INSTRUCTION_OPCODE, 0x06),
 "mulxuu" => &Instruction::new("mulxuu", R_INSTRUCTION_OPCODE, 0x07),
 "cmpge" => &Instruction::new("cmpge", R_INSTRUCTION_OPCODE, 0x08),
 "bret" => &Instruction::new("bret", R_INSTRUCTION_OPCODE, 0x09),
 "ror" => &Instruction::new("ror", R_INSTRUCTION_OPCODE, 0x0B),
 "flushi" => &Instruction::new("flushi", R_INSTRUCTION_OPCODE, 0x0C),
 "jmp" => &Instruction::new("jmp", R_INSTRUCTION_OPCODE, 0x0D),
 "and" => &Instruction::new("and", R_INSTRUCTION_OPCODE, 0x0E),
 "cmplt" => &Instruction::new("cmplt", R_INSTRUCTION_OPCODE, 0x10),
 "slli" => &Instruction::new("slli", R_INSTRUCTION_OPCODE, 0x12),
 "sll" => &Instruction::new("sll", R_INSTRUCTION_OPCODE, 0x13),
 "wrprs" => &Instruction::new("wrprs", R_INSTRUCTION_OPCODE, 0x14),
 "or" => &Instruction::new("or", R_INSTRUCTION_OPCODE, 0x16),
 "mulxsu" => &Instruction::new("mulxsu", R_INSTRUCTION_OPCODE, 0x17),
 "cmpne" => &Instruction::new("cmpne", R_INSTRUCTION_OPCODE, 0x18),
 "srli" => &Instruction::new("srli", R_INSTRUCTION_OPCODE, 0x1A),
 "srl" => &Instruction::new("srl", R_INSTRUCTION_OPCODE, 0x1B),
 "nextpc" => &Instruction::new("nextpc", R_INSTRUCTION_OPCODE, 0x1C),
 "callr" => &Instruction::new("callr", R_INSTRUCTION_OPCODE, 0x1D),
 "xor" => &Instruction::new("xor", R_INSTRUCTION_OPCODE, 0x1E),
 "mulxss" => &Instruction::new("mulxss", R_INSTRUCTION_OPCODE, 0x1F),
 "cmpeq" => &Instruction::new("cmpeq", R_INSTRUCTION_OPCODE, 0x20),
 "divu" => &Instruction::new("divu", R_INSTRUCTION_OPCODE, 0x24),
 "div" => &Instruction::new("div", R_INSTRUCTION_OPCODE, 0x25),
 "rdctl" => &Instruction::new("rdctl", R_INSTRUCTION_OPCODE, 0x26),
 "mul" => &Instruction::new("mul", R_INSTRUCTION_OPCODE, 0x27),
 "cmpgeu" => &Instruction::new("cmpgeu", R_INSTRUCTION_OPCODE, 0x28),
 "initi" => &Instruction::new("initi", R_INSTRUCTION_OPCODE, 0x29),
 "trap" => &Instruction::new("trap", R_INSTRUCTION_OPCODE, 0x2D),
 "cmpltu" => &Instruction::new("cmpltu", R_INSTRUCTION_OPCODE, 0x30),
 "add" => &Instruction::new("add", R_INSTRUCTION_OPCODE, 0x31),
 "break" => &Instruction::new("break", R_INSTRUCTION_OPCODE, 0x34),
 "sync" => &Instruction::new("sync", R_INSTRUCTION_OPCODE, 0x36),
 "sub" => &Instruction::new("sub", R_INSTRUCTION_OPCODE, 0x39),
 "srai" => &Instruction::new("srai", R_INSTRUCTION_OPCODE, 0x3A),
 "sra" => &Instruction::new("sra", R_INSTRUCTION_OPCODE, 0x3B),
};
