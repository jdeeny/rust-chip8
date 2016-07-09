//! A set of instructions.
//!
//! # Examples
//! ```
//! let set = Isa::new(Config::default());
//! let word: Codeword = 0x1234;
//! let inst: = set.decode(word);
//! assert_eq!( word, set.encode(inst) );
//! ```

use std::fmt;

use config::Config;
use instructions::{Definition, Instruction, Codeword, isa_chip8};


/// A Chip8 instruction set based on a particular configuration. Translates between machine code
/// and `Instruction`s.
///
/// An ` Isa` is created with a particular configuration, which governs the
/// instructions that are included in the ISA.
///
/// A 16-bit codeword can be decoded into a generic `Instruction`, which can then be processed
/// by application logic, e.g. a disassembler. An `Instruction` can be encoded into a 16-bit
/// codeword. The `Instruction` is created by application logic, e.g. an assembler.
//#[derive(Debug)]
pub struct Isa {
    table: Vec<InstructionMatcher>,
}

impl Isa {
    /// Creates a new  Isa using the given configuration.
    pub fn new(config: Config) ->  Isa {
         Isa {
        //    config: config,
            table: Vec::new(),
        }
    }

    /// Encodes a given chip8 instruction into a 16-bit codeword.
    #[allow(unused_variables)]
    pub fn encode(&self, inst: Instruction) -> Codeword {
        self.table.encode(inst)
    }

    /// Decodes a 16-bit codeword into an Instruction.
    pub fn decode(&self, codeword: Codeword) -> Instruction {
        for matcher in self.table {
            if matcher.is_match(codeword) {
                
            }
        }
        self.table.decode(codeword)
    }

/*    /// Returns the configuration that was used to create this ` Isa`
    pub fn config(&self) -> Config {
        self.config
    }*/

}

/// A table of all the chip8 instructions. TODO: Move to new file
pub struct Table {
    table: Vec<Definition>,
}
impl Table {
    /// Returns a new instruction table.
    #[allow(unused_variables)]
    pub fn new(config: Config) -> Table {

        let mut table: Vec<Definition> = Vec::new();
        for d in isa_chip8 {
            //table.push(*d);
        }

        Table { table: table }
    }
}

impl Table {
    /// Decode a codeword by finding a match in the table.
    pub fn decode(&self, codeword: Codeword) -> Instruction {
/*        for def in &self.table {
            if def.is_match(codeword) {
                return Instruction::new(def.clone(), codeword);
            }
        }*/
        panic!("Unknown Instruction")
    }

    /// Encode an `Instruction` into a codeword.
    #[allow(unused_variables)]
    pub fn encode(&self, inst: Instruction) -> Codeword {
        0
        /*for def in &self.table {
            if def.operation == inst.operation {

            }
        }*/
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Table {{}}")
    }
}


struct CodewordMatcher {
    code: Codeword,
    mask: Codeword,
}

impl CodewordMatcher {
    fn new(pattern: Pattern) -> CodewordMatcher {
        let mut code: Codeword = 0;
        let mut mask: Codeword = 0;
        for coding in &pattern {
            code <<= 4;
            mask <<= 4;
            if let Coding::C(n) = *coding {
                code |= n as Codeword;
                mask |= 0xF;
            };
        }
        CodewordMatcher {
            code: code,
            mask: mask,
        }
    }
    /// Returns true if a given codeword matches this definition.
    pub fn is_match(&self, codeword: Codeword) -> bool {
        (codeword & self.mask) == (self.code & self.mask)
    }

}

struct InstructionMatcher {
        definition: Definition,
        matcher: CodewordMatcher,
}
impl InstructionMatcher {
    pub fn is_match(&self, codeword: Codeword) -> bool {
        self.matcher.is_match(codeword)
    }
}
