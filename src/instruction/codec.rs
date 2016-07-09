use types::*;
use config::Config;
use instruction::{self, Coding, Pattern, Definition};

#[derive(Copy,Clone)]
pub struct Encoder {

}

impl Encoder {
    pub fn new(config: &Config, isa: &instruction::Set) -> Encoder {
        Encoder {}
    }
}

#[derive(Copy,Clone)]
pub struct Decoder {
}
impl Decoder {
    pub fn new(config: &Config, isa: &instruction::Set) -> Decoder {
        Decoder {}
    }
}
