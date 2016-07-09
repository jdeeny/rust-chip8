//! Decodes and encodes chip8 instructions.

use std::fmt;

mod codec;
mod definition;
mod operands;
pub mod sets;
pub mod matching;
mod isa;
mod instruction;

pub use self::codec::{Decoder,Encoder};
pub use self::definition::Definition;
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
}
