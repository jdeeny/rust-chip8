/// Defines the configuration of the chip8 system being used.
///
/// These settings account for various historical implementation oddities and also allow for
/// modern instruction set enhancements.
#[derive(Copy, Clone, Debug)]
pub struct Config {
    /// The size of the system RAM in bytes.
    pub ram_size: usize,
    /// The base address used when loading the system font.
    pub font_addr: usize,
}

impl Config {
    /// Returns a new Config.
    pub fn new() -> Config {
        Self::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ram_size: 0x2000,
            font_addr: 0x0000,
        }
    }
}
