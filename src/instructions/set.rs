use std::fmt;

use config::Config;
use instructions::{Definition, Instruction, Codeword, isa_chip8};


/// A Chip8 instruction set based on a particular configuration. Translates between machine code
/// and `Instruction`s.
///
/// An ` InstructionSet` is created with a particular configuration, which governs the
/// instructions that are included in the ISA.
///
/// A 16-bit codeword can be decoded into a generic `Instruction`, which can then be processed
/// by application logic, e.g. a disassembler. An `Instruction` can be encoded into a 16-bit
/// codeword. The `Instruction` is created by application logic, e.g. an assembler.
#[derive(Debug)]
pub struct InstructionSet {
    config: Config,
    table: Table
}

impl InstructionSet {
    /// Creates a new  InstructionSet using the given configuration.
    pub fn new(config: Config) ->  InstructionSet {
         InstructionSet {
            config: config,
            table: Table::new(config),
        }
    }

    /// Encodes a given chip8 instruction into a 16-bit codeword.
    #[allow(unused_variables)]
    pub fn encode(&self, inst: Instruction) -> Codeword {
        self.table.encode(inst)
    }

    /// Decodes a 16-bit codeword into an Instruction.
    pub fn decode(&self, codeword: Codeword) -> Instruction {
        self.table.decode(codeword)
    }

    /// Returns the configuration that was used to create this ` InstructionSet`
    pub fn config(&self) -> Config {
        self.config
    }

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
