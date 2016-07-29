//! Decodes and encodes chip8 instructions.

pub mod instruction_sets;
mod matching;
mod definition;
mod operations;
mod operands;
mod set;

pub use self::definition::{Coding, Definition, Pattern};
pub use self::operations::{Operation, OperationKind};
pub use self::operands::{Dest, DestKind, Src, SrcKind};
pub use self::set::Set;
