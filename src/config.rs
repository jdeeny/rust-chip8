//! Configuration of the Chip8 machine.
//!
//! Possible future inclusions:
//!  - Processor speed (ops/tick or /sec)
//!
use std::fmt::{self, Debug};
use fonts::Font4x5;
use instruction::Set;
pub use self::presets::*;

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
    pub vram_size: usize,
    /// Sets the base address where the program will be loaded.
    pub addr_program: usize,
    /// Sets the base address where the system font will be loaded.
    pub addr_font: usize,
    /// Sets the base address where the system large font will be loaded.
    pub addr_font_big: usize,
    /// Sets the small font.
    pub font_small: &'static Font4x5,
    /// Sets the big font.
    pub font_big: &'static Font4x5,
    /// When true, shifts modify vx in place and ignore vy.
    pub quirk_shift: bool,

    pub isa_chip8: bool,

    pub isa_superchip: bool,

    pub isa_xochip: bool,
}

impl Config {
    /// Returns a new Config.
    pub fn new() -> Config {
        Self::default()
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



mod presets {
    use config::Config;
    use fonts::*;
    /// The default configuration.
    pub const DEFAULT: Config = COSMAC_VIP;

    /// Configuration of a stock COSMAC VIP
    ///
    /// Reference: https://en.wikipedia.org/wiki/COSMAC_VIP
    pub const COSMAC_VIP: Config = Config {
        ram_bytes: 2048,
        stack_size: 12,
        vram_size: 64 * 32,
        addr_program: 0x0200,
        addr_font: 0x0000,
        addr_font_big: 0x0050,
        font_small: &FONT_4X5_SMOOTH, // &FONT_4X5_CHIP8,
        font_big: &FONT_4X5_SMOOTH, // &FONT_4X5_CHIP8,
        quirk_shift: false,
        isa_chip8: true,
        isa_superchip: false,
        isa_xochip: false,
    };

    pub const COSMAC_VIP_UPGRADED: Config = Config { ram_bytes: 4096, ..DEFAULT };
    /// Configuration of a stock TELMAC 1800
    ///
    /// Reference: https://en.wikipedia.org/wiki/Telmac_1800
    pub const _TELMAC_1800: Config = DEFAULT;
}
