#![feature(inclusive_range_syntax)]
extern crate rand;
extern crate strfmt;

mod simulator;
mod config;
mod instruction;
mod operand;
mod operations;
mod font;
mod state;

pub use simulator::Simulator;
pub use operand::Operand;
pub use instruction::Instruction;
pub use state::SharedState;
pub use config::Config;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
