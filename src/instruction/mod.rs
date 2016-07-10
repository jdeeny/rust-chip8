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
pub use self::definition::{Definition, Coding, Pattern};
pub use self::execution::{Operation, Execute};
pub use self::instruction::Instruction;
pub use self::operands::{OperandKind, Dest, Src, Aux};
pub use self::isa::Set;
