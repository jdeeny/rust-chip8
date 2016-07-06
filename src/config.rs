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
}

impl Default for Config {
    fn default() -> Self {
        DEFAULT_CONFIG
    }
}

const DEFAULT_CONFIG: Config = Config {
    sys_ram_bytes: 0x2000,
    sys_font_addr: 0x0000,
    sys_stack_size: 12,
    quirk_shift: false,
};
