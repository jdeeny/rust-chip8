//! A set of instructions.
//!
//! # Examples
//! ` ``
//! use chip8::instruction::{Set};
//! use chip8::config::Config;
//! let set = Set::new(&Config::default());
//! let word = 0x4234;
//! let inst = set.decode(word).unwrap();
//! assert_eq!( word, set.encode(inst) );
//! ```

use types::*;
use config::Config;
use instruction::{Definition, Operation, instruction_sets};
use instruction::matching::{CodewordMatcher, InstructionMatcher};

/// A Chip8 instruction set based on a particular configuration. Translates between machine code
/// and `Operation`s.
///
/// A `Set` is created with a particular configuration, which governs the
/// instructions that are included in the ISA.
///
/// A 16-bit codeword can be decoded into an `Operation`, which can then be processed
/// by application logic, e.g. a disassembler. An `Operation` can be encoded into a 16-bit
/// codeword. In this case, the `Operation` is created by application logic, e.g. an assembler.
#[derive(Debug)]
pub struct Set {
    table: Vec<DefMatcher>,
}

#[derive(Debug)]
struct DefMatcher {
    pub definition: Definition,
    pub code_matcher: CodewordMatcher,
    pub inst_matcher: InstructionMatcher,
}



impl Set {
    /// Creates a new  Set using the given configuration.
    pub fn new(config: &Config) -> Set {
        let mut set = Set { table: Vec::new() };

        if config.isa_chip8 {
            set.append(instruction_sets::CHIP8);
        }
        if config.isa_superchip {
            set.append(instruction_sets::SUPERCHIP);
        }
        if config.isa_xochip {
            set.append(instruction_sets::XOCHIP);
        }

        set
    }

    /// Add a slice of definitions to the set.
    pub fn append(&mut self, set: &[Definition]) {
        for d in set.iter() {
            self.push(*d);
        }
    }

    /// Add a definition to the set.
    pub fn push(&mut self, definition: Definition) {
        self.table.push(DefMatcher {
            definition: definition,
            code_matcher: CodewordMatcher::new(definition.pattern),
            inst_matcher: InstructionMatcher::new(&definition),
        });
    }


    /// Encodes a given chip8 instruction into a 16-bit codeword.
    #[allow(unused_variables)]
    pub fn encode(&self, op: Operation) -> Option<Codeword> {
        for i in &self.table {
            if i.inst_matcher.is_match(&op) {
                return Some(0);
            }
        }
        None
    }

    /// Decodes a 16-bit codeword into an Instruction.
    pub fn decode(&self, codeword: Codeword) -> Option<Operation> {
        for i in &self.table {
            if i.code_matcher.is_match(codeword) {
                return Some(i.definition.specify(codeword));
            }
        }
        None
    }

    /// Returns true if the codeword is in the set.
    pub fn codeword_exists(&self, codeword: Codeword) -> bool {
        let mut count = 0;
        for dm in &self.table {
            if dm.code_matcher.is_match(codeword) {
                count += 1;
            }
        }
        match count {
            0 => false,
            1 => true,
            _ => panic!("should never match more than once"),

        }
    }

    //    /// Returns the configuration that was used to create this ` Set`
    // pub fn config(&self) -> Config {
    // self.config
    // }
}
