use crate::text::Text;
use crate::constants::{R_INSTRUCTION_OPCODE};

pub enum InstructionFormat {
    REGISTER,
    IMMEDIATE,
    JUMP,
    PSEUDO,
}

pub struct Instruction {
    pub name: &'static str,
    pub opcode: i32,
    pub opx: i32,
}

impl Instruction {
    pub const fn new(name: &'static str, opcode: i32, opx: i32) -> Self {
        Self {
            name,
            opcode,
            opx,
        }
    }

    pub fn is_conditional_branch(&self) -> bool {
        // bne, blt, beq, bge
        self.opcode == 0x26 || self.opcode == 0x1E || self.opcode == 0x0E || self.opcode == 0x16
    }

    pub fn is_relative_branch(&self) -> bool {
        // br
        self.opcode == 0x6
    }

    pub fn is_register_jump(&self) -> bool {
        // ret
        self.opx == 0x5
    }

    pub fn to_register_format_text(&self, ra: i32, rb: i32, rc: i32, shamt: i32) -> Text {
        Text::new(ra, rb, rc, shamt, self.opx, R_INSTRUCTION_OPCODE, 0, 0)
    }

    pub fn to_jump_format_text(&self, address: i32) -> Text {
        Text::new(0, 0, 0, 0, self.opx, self.opcode, 0, address)
    }

    pub fn to_immediate_format_text(&self, ra: i32, rb: i32, immediate: i32) -> Text {
        Text::new(ra, rb, 0, 0, self.opx, self.opcode, immediate, 0)
    }
}

pub fn convert_opcode_to_format(opcode: i32) -> InstructionFormat {
    match opcode {
        R_INSTRUCTION_OPCODE => InstructionFormat::REGISTER,
        0 | 1 => InstructionFormat::JUMP,
        -1 => InstructionFormat::PSEUDO,
        _ => InstructionFormat::IMMEDIATE,
    }
}
