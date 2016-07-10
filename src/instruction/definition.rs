use std::fmt;

use instruction::{ Operation, OperationKind, DestKind, SrcKind };

/// Type to hold instruction word pattern
pub type Pattern = [Coding; 4];

/// Used to define the coding of each instruction type
#[derive(Clone,Copy,Debug)]
pub enum Coding {
    A,
    B,
    C,
    D,
    /// A literal value
    L(u8),
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
}
