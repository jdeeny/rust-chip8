use config::{ Config, Configured };
use instruction::{Instruction, Table, Word};
use simulator::{Simulator,UiState};


/// A InstructionSet instruction coder/decoder.
///
/// A InstructionSet object is created with a particular configuration, which governs the
/// instructions that are included in the ISA.
///
/// A 16-bit codeword can be decoded into a generic Instruction, which can then be processed
/// by application logic, e.g. a disassembler. An Instruction can be encoded into a 16-bit
/// codeword. The Instruction is created by application logic, e.g. an assembler.
#[derive(Debug)]
pub struct InstructionSet {
    config: Config,
    table: Table
}

impl InstructionSet {
    /// Creates a new InstructionSet using the given configuration.
    pub fn new(config: Config) -> InstructionSet {
        InstructionSet {
            config: config,
            table: Table::new(config),
        }
    }

    /// Encodes a given chip8 instruction into a 16-bit codeword.
    #[allow(unused_variables)]
    pub fn encode(&self, inst: Instruction) -> Word {
        0
    }

    /// Decodes a 16-bit codeword into an Instruction.
    pub fn decode(&self, codeword: Word) -> Instruction {
        self.table.decode(codeword)
    }
}

impl Configured for InstructionSet {
    fn config(&self) -> Config {
        self.config
    }
}
