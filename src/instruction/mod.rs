//! Decodes and encodes chip8 instructions.

use std::fmt;

pub mod instruction_sets;
mod matching;
mod codec;
mod definition;
mod operations;
mod operands;
mod instruction;
mod set;

//pub use self::codec::{Decoder,Encoder};
pub use self::definition::{Definition, Coding, Pattern};
pub use self::operations::{Operation, OperationKind};
pub use self::instruction::{Instruction};
pub use self::operands::{SrcKind, DestKind, Dest, Src};
pub use self::set::Set;
