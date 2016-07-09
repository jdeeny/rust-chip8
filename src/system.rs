use config::Config;
use state::State;
use instructions::Isa;

pub struct Chip8 {
    pub config: Config,
    pub state: State,
    pub instruction_set: Isa,
}

impl Chip8 {
    pub fn new(config: Config) -> Chip8 {
        Chip8 {
            config: config,
            state: State::new(config),
            instruction_set: Isa::new(),
        }
    }
}
