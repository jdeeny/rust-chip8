#![feature(inclusive_range_syntax)]
extern crate rand;
extern crate strfmt;

mod emulator;
mod config;
mod instruction;
mod operand;
mod operations;
mod font;
mod state;

pub use emulator::Emulator;
pub use operand::Operand;
pub use instruction::Instruction;
pub use state::SharedState;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
