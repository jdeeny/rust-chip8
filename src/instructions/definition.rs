use std::fmt;

use instructions::{Operand, OperandKind, Pattern};
use execution::{ Operation };


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
    pub operation: Operation,
    pub dest_kind: OperandKind,
    pub src_kind: OperandKind,
    pub aux_kind: OperandKind,
    pub pattern: Pattern,
    //pub mnemonic: String,
}
impl Definition {
    /// Returns a new Definition.
    pub fn new(operation: Operation,
               dest: OperandKind,
               src: OperandKind,
               aux: OperandKind,
               pattern: Pattern,
           /*mnemonic: String*/)
               -> Definition {
        Definition {
            operation: operation,
            dest_kind: dest,
            src_kind: src,
            aux_kind: aux,
            pattern: pattern,
    //        mnemonic: mnemonic,
        }
    }
}
