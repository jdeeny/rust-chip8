use chip8::{ Operand, OperandKind };
use operations::*;


pub const CODEWORD_BITS: usize = 16;
pub const NUM_CODEWORDS: usize = (1 << CODEWORD_BITS) + 1;

pub type Word = u16;

/// Type to hold instruction word pattern
pub type Pattern = [Coding; 4];

#[derive(Clone,Copy)]
pub enum Coding {
    C(u8),
    D,
    S,
    A,
}


pub struct InstructionTable {
    table: Vec<InstructionDef>,
}
impl InstructionTable {
    pub fn new() -> InstructionTable {
        use chip8::OperandKind::*;
        use self::Coding::*;

        let itable: Vec<InstructionDef> = vec!(
            InstructionDef::new(op_cls,      Unused,     Unused,     Unused,     [C(0x0), C(0x0), C(0xE), C(0x0)], "Cls"),
            InstructionDef::new(op_ret,      Unused,     Unused,     Unused,     [C(0x0), C(0x0), C(0xE), C(0xE)], "Ret"),
            InstructionDef::new(op_jump,     Literal12,  Unused,     Unused,     [C(0x1), D,      D,      D     ], "Jump {d}"),
            InstructionDef::new(op_call,     Literal12,  Unused,     Unused,     [C(0x2), D,      D,      D     ], "Call {d}"),
            InstructionDef::new(op_skipeq,   Register,   Literal8,   Unused,     [C(0x3), D,      S,      S     ], "SkipEq {d}, {s}"),
            InstructionDef::new(op_skipneq,  Register,   Literal8,   Unused,     [C(0x4), D,      S,      S     ], "SkipNeq {d}, {s}"),
            InstructionDef::new(op_skipeq,   Register,   Register,   Unused,     [C(0x5), D,      S,      C(0x0)], "SkipEq {d}, {s}"),
            InstructionDef::new(op_load,     Register,   Literal8,   Unused,     [C(0x6), D,      S,      S     ], "Load {d}, {s}"),
            InstructionDef::new(op_add,      Register,   Literal8,   Unused,     [C(0x7), D,      S,      S     ], "Add {d}, {s}"),
            InstructionDef::new(op_load,     Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x0)], "Load {d}, {s}"),
            InstructionDef::new(op_or,       Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x1)], "Or {d}, {s}"),
            InstructionDef::new(op_and,      Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x2)], "And {d}, {s}"),
            InstructionDef::new(op_xor,      Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x3)], "Xor {d}, {s}"),
            InstructionDef::new(op_add,      Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x4)], "Add {d}, {s}"),
            InstructionDef::new(op_sub,      Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x5)], "Sub {d}, {s}"),
            InstructionDef::new(op_shr,      Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x6)], "ShR {d}, {s}"),
            InstructionDef::new(op_subn,     Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x7)], "SubN {d}, {s}"),
            InstructionDef::new(op_shl,      Register,   Register,   Unused,     [C(0x8), D,      S,      C(0xE)], "ShL {d}, {s}"),
            InstructionDef::new(op_skipneq,  Register,   Register,   Unused,     [C(0x9), D,      S,      C(0x0)], "SkipNeq {d}, {s}"),
            InstructionDef::new(op_load,     I,          Literal12,  Unused,     [C(0xA), S,      S,      S     ], "Load {d}, {s}"),
            InstructionDef::new(op_jumpv0,   Literal12,  Unused,     Unused,     [C(0xB), D,      D,      D     ], "JumpV0 {d}"),
            InstructionDef::new(op_rand,     Register,   Literal8,   Unused,     [C(0xC), D,      S,      S     ], "Rand {d}, {s}"),
            InstructionDef::new(op_sprite,   Register,   Register,   Literal4,   [C(0xD), D,      S,      A     ], "Sprite {d}, {s}, {a}"),
            InstructionDef::new(op_skipkey,  Register,   Unused,     Unused,     [C(0xE), D,      C(0x9), C(0xE)], "SkipKey {d}"),
            InstructionDef::new(op_skipnkey, Register,   Unused,     Unused,     [C(0xE), D,      C(0xA), C(0x1)], "SkipNKey {d}"),
            InstructionDef::new(op_load,     Register,   DelayTimer, Unused,     [C(0xF), D,      C(0x0), C(0x7)], "Load {d}, {s}"),
            InstructionDef::new(op_waitkey,  Register,   Unused,     Unused,     [C(0xF), D,      C(0x0), C(0xA)], "WaitKey {d}"),
            InstructionDef::new(op_load,     DelayTimer, Register,   Unused,     [C(0xF), S,      C(0x1), C(0x5)], "Load {d}, {s}"),
            InstructionDef::new(op_load,     SoundTimer, Register,   Unused,     [C(0xF), S,      C(0x1), C(0x8)], "Load {d}, {s}"),
            InstructionDef::new(op_add,      I,          Register,   Unused,     [C(0xF), S,      C(0x1), C(0xE)], "Add {d}, {s}"),
            InstructionDef::new(op_font,     I,          Register,   Unused,     [C(0xF), S,      C(0x2), C(0x9)], "Font {d}, {s}"),
            InstructionDef::new(op_bcd,      IndirectI,  Register,   Unused,     [C(0xF), S,      C(0x3), C(0x3)], "BCD {d}, {s}"),
            InstructionDef::new(op_stash,    IndirectI,  Register,   Unused,     [C(0xF), S,      C(0x5), C(0x5)], "Stash {s}"),
            InstructionDef::new(op_fetch,    Register,   IndirectI,  Unused,     [C(0xF), D,      C(0x6), C(0x5)], "Fetch {d}"),
        );

        InstructionTable { table: itable }
    }
}

impl InstructionTable {
    pub fn decode(&self, codeword: Word) -> Instruction {
        for def in self.table.iter() {
            if def.is_match(codeword) {
                return Instruction::new(def.clone(), codeword);
            }
        }
        panic!("Unknown Instruction")
    }
}

///Defines a specific kind of instructions
///
///It has a unique signature: the kind of operation and the kinds of the locations dest, src, aux
///
///pattern defines how the instruction is decoded:
///     C(n) is a constant nibble. The codeword must match for this instruction to be valid.
///     D, S, and A are value markers that indicate which nibbles represent which operand's value.
///     If more than one nibble is used for the same operand, the leftmost nibble is most significant
///     and the rightmost is least significant.
///     D indicates dest, S src, and A aux.

pub struct InstructionDef {
  pub operation: Operation,
  dest_kind: OperandKind,
  src_kind: OperandKind,
  aux_kind: OperandKind,
  pattern: Pattern,
  mnemonic: String,
  code: Word,
  mask: Word,
}
impl InstructionDef {
    pub fn new(operation: Operation, dest: OperandKind, src: OperandKind, aux:OperandKind, pattern: Pattern, mnemonic: &str) -> InstructionDef {
        let mut code: Word = 0;
        let mut mask: Word = 0;
        for coding in pattern.iter() {
            code <<= 4;
            mask <<= 4;
            match *coding {
                Coding::C(n) => {
                                    code |= n as Word;
                                    mask |= 0xF;
                                },
                _ => {}
            };
        }
        InstructionDef {
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
    pub fn is_match(&self, codeword: Word) -> bool {
        (codeword & self.mask) == (self.code & self.mask)
    }
    pub fn clone(&self) -> InstructionDef {

        InstructionDef {
            operation: self.operation,
            dest_kind: self.dest_kind,
            src_kind: self.src_kind,
            aux_kind: self.aux_kind,
            pattern: self.pattern.clone(),
            mnemonic: self.mnemonic.clone(),
            code: self.code,
            mask: self.mask,
        }
    }
}

pub struct Instruction {
  pub def: InstructionDef,
  codeword: Word,
  dest: Operand,
  src: Operand,
  aux: Operand,
}

impl Instruction {
    pub fn new(def: InstructionDef, codeword: Word) -> Instruction {
        let mut dest_data: usize = 0;
        let mut src_data: usize = 0;
        let mut aux_data: usize = 0;

        let mut word = codeword as usize;
        for coding in def.pattern.iter() {
            let nibble = (word & 0xF000) >> 12;
            word <<= 4;
            //println!("nibble: {:?}", nibble);
            match *coding {
                self::Coding::C(_) => {},
                self::Coding::D => { dest_data = (dest_data << 4) | nibble; },
                self::Coding::S => { src_data = (src_data << 4) | nibble; },
                self::Coding::A => { aux_data = (aux_data << 4) | nibble; },
            }
        }


        let mut dest: Operand;
        let mut src: Operand;
        let mut aux: Operand;
        {
            dest = Self::specify_operand(&def.dest_kind, dest_data);
            src = Self::specify_operand(&def.src_kind, src_data);
            aux = Self::specify_operand(&def.aux_kind, aux_data);
        }

        //println!("{:?} {:?} {:?} / {:?} {:?} {:?} ", dest.to_string(), src.to_string(), aux.to_string(), dest_data, src_data, aux_data);

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
            OperandKind::Unused => Operand::Nowhere,
        }
    }

    pub fn dest(&self) -> Operand {
        self.dest
    }

    pub fn src(&self) -> Operand {
        self.src
    }

    pub fn aux(&self) -> Operand {
        self.aux
    }

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
