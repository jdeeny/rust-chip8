//! Decodes and encodes chip8 instructions.

use std::fmt;

mod codec;
mod definition;
mod operands;
pub mod sets;
mod instruction;

pub use self::codec::{Decoder,Encoder};
pub use self::definition::Definition;
pub use self::instruction::Instruction;
pub use self::operands::{Operand, OperandKind};

pub type Isa = Vec<Definition>;
pub type Codeword = u16;

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
