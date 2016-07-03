#![feature(inclusive_range_syntax)]
extern crate rand;
extern crate strfmt;

mod chip8;
mod config;
mod instruction;
mod operand;
mod operations;
mod font;
mod state;

pub use chip8::*;
pub use config::*;
pub use state::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
