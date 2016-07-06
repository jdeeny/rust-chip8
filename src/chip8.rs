use config::Config;
use instruction::Table;

/// Holds the configuration for a chip8 implementation.
pub struct Chip8 {
    config: Config,
    table: Table
}

impl Chip8 {
    /// Creates a new Chip8 using the given configuration.
    pub fn new(config: Config) -> Chip8 {
        Chip8 {
            config: config,
            table: Table::new(config),
        }
    }

    /// Encodes a given chip8 instruction into a 16-bit codeword.
    pub fn encode(&self, inst: instruction) -> Word {
        0
    }

    /// Decodes a 16-bit codeword into an Instruction.
    pub fn decode(&self, codeword: Word) -> Instruction {
        self.table.decode(codeword: Word)
    }

}
