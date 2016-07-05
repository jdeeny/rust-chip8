//! Decodes and encodes chip8 instructions.

use std::fmt;

mod operand;
mod operations;

pub use self::operand::{Operand, OperandKind};
use self::operations::*;

/// A 16-bit chip8 codeword.
pub type Word = u16;

/// Type to hold instruction word pattern
pub type Pattern = [Coding; 4];

/// Used to define the coding of each instruction type
#[derive(Clone,Copy,Debug)]
pub enum Coding {
    /// A constant nibble (literal)
    C(u8),
    /// Destination
    D,
    /// Source
    S,
    /// Auxillary
    A,
}


/// A table of all the chip8 instructions. TODO: Move to new file
pub struct Table {
    table: Vec<Definition>,
}
impl Table {
    /// Returns a new instruction table.
    pub fn new() -> Table {
        use self::operand::OperandKind::*;
        use self::Coding::*;

        let itable: Vec<Definition> = vec!(
            Definition::new(op_cls,      Unused,     Unused,     Unused,     [C(0x0), C(0x0), C(0xE), C(0x0)], "Cls"),
            Definition::new(op_ret,      Unused,     Unused,     Unused,     [C(0x0), C(0x0), C(0xE), C(0xE)], "Ret"),
            Definition::new(op_jump,     Literal12,  Unused,     Unused,     [C(0x1), D,      D,      D     ], "Jump {d}"),
            Definition::new(op_call,     Literal12,  Unused,     Unused,     [C(0x2), D,      D,      D     ], "Call {d}"),
            Definition::new(op_skipeq,   Register,   Literal8,   Unused,     [C(0x3), D,      S,      S     ], "SkipEq {d}, {s}"),
            Definition::new(op_skipneq,  Register,   Literal8,   Unused,     [C(0x4), D,      S,      S     ], "SkipNeq {d}, {s}"),
            Definition::new(op_skipeq,   Register,   Register,   Unused,     [C(0x5), D,      S,      C(0x0)], "SkipEq {d}, {s}"),
            Definition::new(op_load,     Register,   Literal8,   Unused,     [C(0x6), D,      S,      S     ], "Load {d}, {s}"),
            Definition::new(op_add,      Register,   Literal8,   Unused,     [C(0x7), D,      S,      S     ], "Add {d}, {s}"),
            Definition::new(op_load,     Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x0)], "Load {d}, {s}"),
            Definition::new(op_or,       Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x1)], "Or {d}, {s}"),
            Definition::new(op_and,      Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x2)], "And {d}, {s}"),
            Definition::new(op_xor,      Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x3)], "Xor {d}, {s}"),
            Definition::new(op_add,      Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x4)], "Add {d}, {s}"),
            Definition::new(op_sub,      Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x5)], "Sub {d}, {s}"),
            Definition::new(op_shr,      Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x6)], "ShR {d}, {s}"),
            Definition::new(op_subn,     Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x7)], "SubN {d}, {s}"),
            Definition::new(op_shl,      Register,   Register,   Unused,     [C(0x8), D,      S,      C(0xE)], "ShL {d}, {s}"),
            Definition::new(op_skipneq,  Register,   Register,   Unused,     [C(0x9), D,      S,      C(0x0)], "SkipNeq {d}, {s}"),
            Definition::new(op_load,     I,          Literal12,  Unused,     [C(0xA), S,      S,      S     ], "Load {d}, {s}"),
            Definition::new(op_jumpv0,   Literal12,  Unused,     Unused,     [C(0xB), D,      D,      D     ], "JumpV0 {d}"),
            Definition::new(op_rand,     Register,   Literal8,   Random,     [C(0xC), D,      S,      S     ], "Rand {d}, {s}"),
            Definition::new(op_sprite,   Register,   Register,   Literal4,   [C(0xD), D,      S,      A     ], "Sprite {d}, {s}, {a}"),
            Definition::new(op_skipkey,  Register,   Unused,     Unused,     [C(0xE), D,      C(0x9), C(0xE)], "SkipKey {d}"),
            Definition::new(op_skipnkey, Register,   Unused,     Unused,     [C(0xE), D,      C(0xA), C(0x1)], "SkipNKey {d}"),
            Definition::new(op_load,     Register,   DelayTimer, Unused,     [C(0xF), D,      C(0x0), C(0x7)], "Load {d}, {s}"),
            Definition::new(op_waitkey,  Register,   Unused,     Unused,     [C(0xF), D,      C(0x0), C(0xA)], "WaitKey {d}"),
            Definition::new(op_load,     DelayTimer, Register,   Unused,     [C(0xF), S,      C(0x1), C(0x5)], "Load {d}, {s}"),
            Definition::new(op_load,     SoundTimer, Register,   Unused,     [C(0xF), S,      C(0x1), C(0x8)], "Load {d}, {s}"),
            Definition::new(op_add,      I,          Register,   Unused,     [C(0xF), S,      C(0x1), C(0xE)], "Add {d}, {s}"),
            Definition::new(op_font,     I,          Register,   Unused,     [C(0xF), S,      C(0x2), C(0x9)], "Font {d}, {s}"),
            Definition::new(op_bcd,      IndirectI,  Register,   Unused,     [C(0xF), S,      C(0x3), C(0x3)], "BCD {d}, {s}"),
            Definition::new(op_stash,    IndirectI,  Register,   Unused,     [C(0xF), S,      C(0x5), C(0x5)], "Stash {s}"),
            Definition::new(op_fetch,    Register,   IndirectI,  Unused,     [C(0xF), D,      C(0x6), C(0x5)], "Fetch {d}"),
        );

        Table { table: itable }
    }
}

impl Table {
    /// Decode a codeword by finding a match in the table.
    pub fn decode(&self, codeword: Word) -> Instruction {
        for def in &self.table {
            if def.is_match(codeword) {
                return Instruction::new(def.clone(), codeword);
            }
        }
        panic!("Unknown Instruction")
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Table {{}}")
    }
}

/// Defines the structure of a specific instruction
///
/// It has a unique signature: the kind of operation and the kinds of the locations dest, src, aux
///
/// pattern defines how the instruction is decoded:
///     C(n) is a constant nibble. The codeword must match for this instruction to be valid.
///     D, S, and A are value markers that indicate which nibbles represent which operand's value.
///     If more than one nibble is used for the same operand, the leftmost nibble is most significant
///     and the rightmost is least significant.
///     D indicates dest, S src, and A aux.
pub struct Definition {
    /// The operation that will be performed when this type of instruction is executed.
    pub operation: Operation,
    dest_kind: OperandKind,
    src_kind: OperandKind,
    aux_kind: OperandKind,
    pattern: Pattern,
    mnemonic: String,
    code: Word,
    mask: Word,
}
impl Definition {
    /// Returns a new Definition.
    pub fn new(operation: Operation,
               dest: OperandKind,
               src: OperandKind,
               aux: OperandKind,
               pattern: Pattern,
               mnemonic: &str)
               -> Definition {
        let mut code: Word = 0;
        let mut mask: Word = 0;
        for coding in &pattern {
            code <<= 4;
            mask <<= 4;
            if let Coding::C(n) = *coding {
                code |= n as Word;
                mask |= 0xF;
            };
        }
        Definition {
            operation: operation,
            dest_kind: dest,
            src_kind: src,
            aux_kind: aux,
            pattern: pattern,
            mnemonic: mnemonic.to_string(),
            code: code,
            mask: mask,
        }
    }
    /// Returns true if a given codeword matches this definition.
    pub fn is_match(&self, codeword: Word) -> bool {
        (codeword & self.mask) == (self.code & self.mask)
    }
}

impl Clone for Definition {
    fn clone(&self) -> Definition {

        Definition {
            operation: self.operation,
            dest_kind: self.dest_kind,
            src_kind: self.src_kind,
            aux_kind: self.aux_kind,
            pattern: self.pattern,
            mnemonic: self.mnemonic.clone(),
            code: self.code,
            mask: self.mask,
        }
    }
}
impl fmt::Debug for Definition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Definition {{}}")
    }
}

/// A fully specified chip8 instruction.
#[derive(Debug)]
pub struct Instruction {
    /// The definition from the instruction table that matches this instruction.
    pub def: Definition,
    #[allow(dead_code)]
    codeword: Word,
    dest: Operand,
    src: Operand,
    aux: Operand,
}

impl Instruction {
    /// Returns a new Instruction.
    pub fn new(def: Definition, codeword: Word) -> Instruction {
        let mut dest_data: usize = 0;
        let mut src_data: usize = 0;
        let mut aux_data: usize = 0;

        let mut word = codeword as usize;
        for coding in &def.pattern {
            let nibble = (word & 0xF000) >> 12;
            word <<= 4;
            // println!("nibble: {:?}", nibble);
            match *coding {
                self::Coding::C(_) => {}
                self::Coding::D => {
                    dest_data = (dest_data << 4) | nibble;
                }
                self::Coding::S => {
                    src_data = (src_data << 4) | nibble;
                }
                self::Coding::A => {
                    aux_data = (aux_data << 4) | nibble;
                }
            }
        }


        let dest: Operand;
        let src: Operand;
        let aux: Operand;
        {
            dest = Self::specify_operand(&def.dest_kind, dest_data);
            src = Self::specify_operand(&def.src_kind, src_data);
            aux = Self::specify_operand(&def.aux_kind, aux_data);
        }

        // println!("{:?} {:?} {:?} / {:?} {:?} {:?} ", dest.to_string(), src.to_string(), aux.to_string(), dest_data, src_data, aux_data);

        Instruction {
            def: def,
            codeword: codeword,
            dest: dest,
            src: src,
            aux: aux,
        }
    }
    fn specify_operand(kind: &OperandKind, data: usize) -> Operand {
        match *kind {
            OperandKind::Register => Operand::Register(data),
            OperandKind::I => Operand::I,
            OperandKind::Address12 => Operand::Address12(data),
            OperandKind::IndirectI => Operand::IndirectI,
            OperandKind::Literal12 => Operand::Literal12(data),
            OperandKind::Literal8 => Operand::Literal8(data),
            OperandKind::Literal4 => Operand::Literal4(data),
            OperandKind::DelayTimer => Operand::DelayTimer,
            OperandKind::SoundTimer => Operand::SoundTimer,
            OperandKind::Random => Operand::Random,
            OperandKind::Unused => Operand::Nowhere,
        }
    }

    /// Returns the destination operand.
    pub fn dest(&self) -> Operand {
        self.dest
    }

    /// Returns the source operand.
    pub fn src(&self) -> Operand {
        self.src
    }

    /// Returns the auxillary operand.
    pub fn aux(&self) -> Operand {
        self.aux
    }

    /// Returns a string describing the instruction.
    pub fn to_string(&self) -> String {
        use strfmt::strfmt;
        use std::collections::HashMap;
        let mut vars = HashMap::new();
        vars.insert("d".to_string(), self.dest.to_string());
        vars.insert("s".to_string(), self.src.to_string());
        vars.insert("a".to_string(), self.aux.to_string());
        strfmt(&self.def.mnemonic, &vars).unwrap()
    }
}
