use std::fmt;

use types::*;
use instruction::{ Operation, OperationKind, DestKind, SrcKind };

/// Type to hold instruction word pattern
pub type Pattern = [Coding; 4];

/// Used to define the coding of each instruction type
#[derive(Clone,Copy,Debug)]
pub enum Coding {
    /// Nibble applies to an argument.
    A(usize),
    /// A literal value
    C(u8),
    /// Don't care
    X,
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
#[derive(Copy,Clone)]
pub struct Definition {
    /// The operation that will be performed when this type of instruction is executed.
    pub op: OperationKind,
    pub pattern: Pattern,
    //pub mnemonic: String,
}
impl Definition {
    /// Returns a new Definition.
    pub fn new(op: OperationKind, pattern: Pattern) -> Definition {
        Definition {
            op: op,
            pattern: pattern,
    //        mnemonic: mnemonic,
        }
    }

    pub fn specify(&self, codeword: Codeword) -> Operation {
        let mut data = [0usize; 4];
        let mut w = codeword;
        for (i, coding) in self.pattern.iter().enumerate() {
            let nibble = (w as usize & 0xF000) >> 12;
            w <<= 4;
            match *coding {
                Coding::A(n) => {
                                    for i in 0..4 {
                                        if n & (1 << i) != 0 {
                                            let idx = i ;
                                            data[idx] = (data[idx] << 4) | nibble;
                                        }
                                    }
                                },
                Coding::C(n) => {},
                Coding::X => {},
            }

        }

        print!("Codeword: {:X} data: ", codeword);
        for d in data.iter() { print!("{:X} ", d)}
        println!("");

        match self.op {
            OperationKind::NoOp => Operation::NoOp,
            OperationKind::Cls => Operation::Cls,
            OperationKind::Load(d, s) => Operation::Load(d.specify(data[0]), s.specify(data[1])),
            OperationKind::Stash(first, last) => Operation::Stash(first.specify(data[0]), last.specify(data[1])),
            OperationKind::Fetch(first, last) => Operation::Fetch(first.specify(data[0]), last.specify(data[1])),
            OperationKind::Jump(addr) => Operation::Jump(addr.specify(data[0])),
            OperationKind::JumpV0(addr) => Operation::JumpV0(addr.specify(data[0])),
            OperationKind::Call(addr) => Operation::Call(addr.specify(data[0])),
            OperationKind::Ret => Operation::Ret,
            OperationKind::SkipEq(a, b) => Operation::SkipEq(a.specify(data[0]), b.specify(data[1])),
            OperationKind::SkipNotEq(a, b) => Operation::SkipNotEq(a.specify(data[0]), b.specify(data[1])),
            OperationKind::SkipKey(n) => Operation::SkipKey(n.specify(data[0])),
            OperationKind::SkipNotKey(n) => Operation::SkipNotKey(n.specify(data[0])),
            OperationKind::Add(d, a, b) => Operation::Add(d.specify(data[0]), a.specify(data[1]), b.specify(data[2])),
            OperationKind::Sub(d, a, b) => Operation::Sub(d.specify(data[0]), a.specify(data[1]), b.specify(data[2])),
            OperationKind::Or(d, a, b) => Operation::Or(d.specify(data[0]), a.specify(data[1]), b.specify(data[2])),
            OperationKind::And(d, a, b) => Operation::And(d.specify(data[0]), a.specify(data[1]), b.specify(data[2])),
            OperationKind::Xor(d, a, b) => Operation::Xor(d.specify(data[0]), a.specify(data[1]), b.specify(data[2])),
            OperationKind::Shr(d, s) => Operation::Shr(d.specify(data[0]), s.specify(data[1])),
            OperationKind::Shl(d, s) => Operation::Shl(d.specify(data[0]), s.specify(data[1])),
            OperationKind::Rand(d, s, m) => Operation::Rand(d.specify(data[0]), s.specify(data[1]), m.specify(data[2])),
            OperationKind::Sprite(x, y, n) => Operation::Sprite(x.specify(data[0]), y.specify(data[1]), n.specify(data[2])),
            OperationKind::Font(glyph, font) => Operation::Font(glyph.specify(data[0]), font.specify(data[1])),
            OperationKind::Bcd(n) => Operation::Bcd(n.specify(data[0])),
            OperationKind::WaitKey(d, n) => Operation::WaitKey(d.specify(data[0]), n.specify(data[1])),
        }

    }
}
