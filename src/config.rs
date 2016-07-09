//! Configuration of the Chip8 machine.
use std::fmt::{self, Debug};
use fonts::*;
use instructions::InstructionSet;

/// Defines the configuration of the chip8 system being used.
///
/// These settings account for various historical implementation oddities and also allow for
/// modern instruction set enhancements.
#[derive(Copy, Clone)]
pub struct Config {
    /// Sets the size of ram in bytes.
    pub ram_bytes: usize,
    /// Sets the number of addresses that can be placed on the stack.
    pub stack_size: usize,
    /// Sets the number of pixels in vram.
    pub vram_bytes: usize,
    /// Sets the base address where the program will be loaded.
    pub addr_program: usize,
    /// Sets the base address where the system font will be loaded.
    pub addr_font: usize,
    /// Sets the small font.
    pub font_small: &'static Font4x5,
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
}

impl Default for Config {
    fn default() -> Config {
        DEFAULT
    }
}

impl Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Config {{}}")
    }
}

/// The default configuration.
pub const DEFAULT: Config = Config {
    ram_bytes: 0x2000,
    stack_size: 12,
    vram_bytes: 64*32,
    addr_program: 0x0200,
    addr_font: 0x0000,
    font_small: &FONT_4X5_SMOOTH,//&FONT_4X5_CHIP8,
    quirk_shift: false,
};

/// A sample configuration with large RAM.
pub const BIG: Config = Config {
    ram_bytes: 0xFFFF,
    vram_bytes: 64*32,
    stack_size: 1000,
    addr_program: 0x0200,
    addr_font: 0x000,
    font_small: &FONT_4X5_CHIP8,
    quirk_shift: false,
};
