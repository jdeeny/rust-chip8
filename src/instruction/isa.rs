//! A set of instructions.
//!
//! # Examples
//! ```
//! use chip8::instruction::{Set};
//! use chip8::config::Config;
//! let set = Set::new(&Config::default());
//! let word = 0x1234;
//! let inst = set.decode(word);
//! assert_eq!( word, set.encode(inst) );
//! ```

use std::fmt;

use types::*;
use config::Config;
use instruction::{Definition, Instruction };
use instruction::matching::{InstructionMatcher, CodewordMatcher};
use instruction::sets;

/// A Chip8 instruction set based on a particular configuration. Translates between machine code
/// and `Instruction`s.
///
/// An ` Set` is created with a particular configuration, which governs the
/// instructions that are included in the ISA.
///
/// A 16-bit codeword can be decoded into a generic `Instruction`, which can then be processed
/// by application logic, e.g. a disassembler. An `Instruction` can be encoded into a 16-bit
/// codeword. The `Instruction` is created by application logic, e.g. an assembler.
//#[derive(Debug)]
pub struct Set {
    table: Vec<InstructionMatcher>,
}

impl Set {
    /// Creates a new  Set using the given configuration.
    pub fn new(config: &Config) ->  Set {

         Set {
        //    config: config,
            table: Vec::new(),
            //table.push(sets::CHIP8.)
        }
    }

    /// Encodes a given chip8 instruction into a 16-bit codeword.
    #[allow(unused_variables)]
    pub fn encode(&self, inst: Instruction) -> Codeword {
        unimplemented!()
    }

    /// Decodes a 16-bit codeword into an Instruction.
    pub fn decode(&self, codeword: Codeword) -> Instruction {
        unimplemented!()
    }

/*    /// Returns the configuration that was used to create this ` Set`
    pub fn config(&self) -> Config {
        self.config
    }*/

}
