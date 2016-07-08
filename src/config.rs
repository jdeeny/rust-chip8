//! Configuration of the Chip8 machine.
use instructions::InstructionSet;
use simulator::Simulator;

/// Defines the configuration of the chip8 system being used.
///
/// These settings account for various historical implementation oddities and also allow for
/// modern instruction set enhancements.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Config {
    /// Sets the size of ram in bytes.
    pub sys_ram_bytes: usize,
    /// Sets the number of addresses that can be placed on the stack.
    pub sys_stack_size: usize,
    /// Sets the number of pixels in vram.
    pub sys_vram_bytes: usize,
    /// Sets the base address where the system font will be loaded.
    pub sys_font_addr: usize,
    /// When true, shifts modify vx in place and ignore vy.
    pub quirk_shift: bool,
}

impl Config {
    /// Returns a new Config.
    pub fn new() -> Config {
        Self::default()
    }
    /// Returns an InstructionSet based on the current configuration.
    pub fn instruction_codec(&self) -> InstructionSet {
        InstructionSet::new(*self)
    }
    /// Returns a Simulator based on the current configuration.
    pub fn simulator(&self) -> Simulator {
        Simulator::new(*self)
    }
}

impl Default for Config {
    fn default() -> Self {
        DEFAULT
    }
}

/// The default configuration.
pub const DEFAULT: Config = Config {
    sys_ram_bytes: 0x2000,
    sys_stack_size: 12,
    sys_vram_bytes: 64*32,
    sys_font_addr: 0x0000,
    quirk_shift: false,
};

/// A sample configuration with large RAM.
pub const BIG: Config = Config {
    sys_ram_bytes: 0xFFFF,
    sys_vram_bytes: 64*32,
    sys_stack_size: 1000,
    sys_font_addr: 0x000,
    quirk_shift: false,
};
