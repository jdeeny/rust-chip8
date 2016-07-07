use config::Config;
use instructions::{Instruction, Table, Word};


/// A Chip8 instruction set based on a particular configuration. Translates between machine code
/// and `Instruction`s.
///
/// An `InstructionCodec` is created with a particular configuration, which governs the
/// instructions that are included in the ISA.
///
/// A 16-bit codeword can be decoded into a generic `Instruction`, which can then be processed
/// by application logic, e.g. a disassembler. An `Instruction` can be encoded into a 16-bit
/// codeword. The `Instruction` is created by application logic, e.g. an assembler.
#[derive(Debug)]
pub struct InstructionCodec {
    config: Config,
    table: Table
}

impl InstructionCodec {
    /// Creates a new InstructionCodec using the given configuration.
    pub fn new(config: Config) -> InstructionCodec {
        InstructionCodec {
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

    /// Returns the configuration that was used to create this `InstructionCodec`
    pub fn config(&self) -> Config {
        self.config
    }

}
