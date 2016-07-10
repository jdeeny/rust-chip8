//! Decodes and encodes chip8 instructions.

use std::fmt;

pub mod sets;
mod matching;
mod codec;
mod definition;
mod execution;
mod operands;
mod isa;
mod instruction;

pub use self::codec::{Decoder,Encoder};
pub use self::definition::Definition;
pub use self::execution::{Operation,Execute};
pub use self::instruction::Instruction;
pub use self::operands::{Operand, OperandKind};
pub use self::isa::Set;

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
    /// Ignore
    X,
}
