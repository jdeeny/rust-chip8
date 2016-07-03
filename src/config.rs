#[derive(Copy, Clone)]
pub struct Config {
    pub ram_size: usize,
    pub font_addr: usize,

    pub print_instructions: bool,
}

impl Config {
    pub fn new() -> Config {
        Self::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ram_size: 0x2000,
            font_addr: 0x0000,
            print_instructions: false,
        }
    }
}
