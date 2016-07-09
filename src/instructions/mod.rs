//! Decodes and encodes chip8 instructions.

use std::fmt;

pub mod operations;

mod definition;
mod operand;
mod isa;
mod set;
mod execution;

pub use self::definition::{Codeword, Coding, Definition, Instruction};

pub use self::execution::Executor;
pub use self::set::InstructionSet;
pub use self::operand::{Operand, OperandKind};
pub use self::isa::*;

use types::*;
use self::operations::*;
use config::Config;
