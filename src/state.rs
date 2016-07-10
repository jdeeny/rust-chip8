//! Defines the state of the Chip8 virtual machine.

use std::iter::{FromIterator, repeat};
use rand::{Rng, thread_rng};
pub use types::*;
use config::Config;
use instruction::{Execute, Dest, Src};


/// A struct that contains a Chip8 `Config` and the machine state.
///
/// The machine state includes the RAM, registers, program counter, stack, timers, and the
/// state of the IO subsystems.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Chip8 {
    /// Sets the configuration of the machine.
    pub config: Config,
    /// The ram.
    pub ram: Vec<MemoryCell>,
    /// The general purpose registers, v0-vF.
    pub v: [Register8; 16],
    /// The I register.
    pub i: Register16,
    /// The program counter.
    pub pc: Address,
    /// The call stack.
    pub stack: Vec<Address>,
    /// The sound timer.
    pub st: Timer,
    /// The delay timer.
    pub dt: Timer,
    /// The video ram, containing the state of the video output.
    pub vram: Vec<Pixel>,
    /// The state of the keyboard.
    pub keys: Keyboard,
    /// The state of the audio output.
    pub audio: Audio,
}

impl Chip8 {
    /// Create a new Chip8 using the supplied Config.
    pub fn new(config: &Config) -> Chip8 {
        Chip8 {
            config: *config,
            ram: Vec::from_iter(repeat(0).take(config.ram_bytes)),
            v: [0; 16],
            i: 0,
            st: 0,
            dt: 0,
            pc: 0,
            stack: Vec::with_capacity(config.stack_size),
            vram: Vec::from_iter(repeat(Pixel::default()).take(config.vram_size)),
            keys: [false;16],
            audio: false,
        }
    }
}

impl Execute for Chip8 {

/*    /// Gets the value stored at an address in RAM.
    fn ram(&self, addr: Address) -> MemoryCell {
        self.ram[addr]
    }

    /// Sets the value stored at an address in RAM.
    fn set_ram(&mut self, addr: Address, data: MemoryCell) {
        self.ram[addr] = data;
    }*/

    fn config(&self) -> Config {
        self.config
    }

    fn load(&mut self, src: Src) -> usize {
        match src {
            Src::Register(r) => self.v[r] as usize,
            Src::Address12(a) => self.ram[a] as usize,
            Src::I => self.i as usize,
            Src::IndirectI => self.ram[self.i as usize] as usize,
            Src::Literal12(n12) => n12,
            Src::Literal8(n8) => n8,
            Src::Literal4(n4) => n4,
            Src::SoundTimer => self.st as usize,
            Src::DelayTimer => self.dt as usize,
            Src::Random => (thread_rng().next_u32() & 0xFF) as usize,
        }
    }

    fn store(&mut self, dest: Dest, data: usize) {
        match dest {
            Dest::Register(r) => {
                self.v[r] = (data & 0xFF) as MemoryCell;
            }
            Dest::Address12(a) => {
                self.ram[a] = (data & 0xFF) as MemoryCell;
            }
            Dest::I => {
                self.i = (data & 0xFFFF) as Register16;
            }
            Dest::IndirectI => {
                self.ram[self.i as usize] = data as MemoryCell;
            }
            Dest::SoundTimer => {
                self.st = data as Timer;
            }
            Dest::DelayTimer => {
                self.dt = data as Timer;
            }
        }
    }

    fn set_flag(&mut self, flag: bool) {
        self.v[0xF] = if flag {
            1
        } else {
            0
        };
    }

    fn stack_pop(&mut self) -> Option<Address> {
        self.stack.pop()
    }

    fn stack_push(&mut self, address: Address) {
        self.stack.push(address);
    }

    fn pc(&self) -> Address {
        self.pc
    }

    fn advance_pc(&mut self) {
        self.pc += 2;
    }

    fn jump(&mut self, addr: Address) {
        self.pc = addr;
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new(&Config::default())
    }
}
/*impl Debug for Chip8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Simulator {{}}")
    }
}*/
