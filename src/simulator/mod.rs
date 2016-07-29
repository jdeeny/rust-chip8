//! Simulates a chip8 cpu and provides a thread-safe interface to control execution and state.

mod threaded;
#[cfg(test)]
mod tests;

use std::sync::{Arc, RwLock};

use types::*;
use Chip8;
use config::Config;
use instruction::{self, Dest, Operation, Src};
pub use self::threaded::SimulatorTask;
use state::RandomBytes;

/// An object that can simulate a chip8 machine.
pub trait Simulate {
    /// Fetch the current instruction, advance the PC, and execute the instruction.
    fn step(&mut self) -> Chip8Result<()>;
    /// Execute multiple instructions.
    fn step_n(&mut self, number_of_steps: usize) -> Chip8Result<()>;
    /// Advance the sound and delay timers.
    fn timer_tick(&mut self) -> Chip8Result<()>;
    /// Load bytes into ram.
    fn load_bytes(&mut self, bytes: &[u8], addr: Address) -> Chip8Result<()>;
    /// Load a program into ram and the configured base address.
    fn load_program(&mut self, bytes: &[u8]) -> Chip8Result<()>;
    /// Load a value from a Src.
    fn load(&mut self, src: Src) -> Chip8Result<usize>;
    /// Store a value into a Dest.
    fn store(&mut self, dest: Dest, value: usize) -> Chip8Result<()>;
    /// Set the keyboard state.
    fn set_keyboard(&mut self, keys: &Keyboard) -> Chip8Result<()>;
    /// Read the keyboard state.
    fn keyboard(&self) -> Chip8Result<Keyboard>;
    /// Read the Vram state.
    fn vram(&self) -> Chip8Result<Vram>;
    /// Read the buzzer state.
    fn buzzer(&self) -> Chip8Result<Buzzer>;
    /// Read the audio state.
    fn audio(&self) -> Chip8Result<Audio>;
}

/// Manages the state of a chip8 cpu.
#[derive(Debug)]
pub struct Simulator {
    core: Chip8,
    instruction_set: instruction::Set,
}

impl Simulate for Simulator {
    /// Loads bytes into RAM starting at the given address.
    fn load_bytes(&mut self, bytes: &[u8], addr: Address) -> Chip8Result<()> {
        self.core.load_bytes(bytes, addr)
    }

    fn load_program(&mut self, bytes: &[u8]) -> Chip8Result<()> {
        let address = self.core.config.addr_program;
        self.load_bytes(bytes, address as Address)
    }

    /// Decrements the delay and sound timer.
    fn timer_tick(&mut self) -> Chip8Result<()> {
        if self.core.dt > 0 {
            self.core.dt -= 1;
        }
        if self.core.st > 0 {
            self.core.st -= 1;
        }
        Ok(())
    }

    fn load(&mut self, src: Src) -> Chip8Result<usize> {
        self.core.load(src)
    }
    fn store(&mut self, dest: Dest, value: usize) -> Chip8Result<()> {
        self.core.store(dest, value)
    }

    fn step(&mut self) -> Chip8Result<()> {
        let instruction = try!(self.decode_at_addr(self.core.pc()));
        self.core.advance_pc();
        try!(instruction.execute(&mut self.core));
        Ok(())
    }

    fn step_n(&mut self, number_of_steps: usize) -> Chip8Result<()> {
        for _ in 0..number_of_steps {
            try!(self.step())
        }
        Ok(())
    }

    fn set_keyboard(&mut self, keys: &Keyboard) -> Chip8Result<()> {
        self.core.set_keyboard(keys)
    }
    fn keyboard(&self) -> Chip8Result<Keyboard> {
        self.core.keyboard()
    }
    fn vram(&self) -> Chip8Result<Vram> {
        self.core.vram()
    }
    fn buzzer(&self) -> Chip8Result<Buzzer> {
        self.core.buzzer()
    }
    fn audio(&self) -> Chip8Result<Audio> {
        self.core.audio()
    }
}

impl Simulator {
    /// Returns a new Simulator.
    pub fn new(config: &Config, rand_iterator: Option<RandomBytes>) -> Chip8Result<Simulator> {
        let core: Chip8 = Chip8::new(config, rand_iterator);
        let iset = instruction::Set::new(config);
        let mut s = Simulator {
            core: core,
            instruction_set: iset,
        };
        try!(s.load_bytes(config.font_small, config.addr_font as Address));
        try!(s.core.store(Dest::PC, config.addr_program));
        Ok(s)
    }

    /// Returns a default simulator, using the default configuration.
    pub fn default() -> Chip8Result<Simulator> {
        Self::new(&Config::default(), None)
    }

    /// Decodes an instruction. TODO: Move to ::instruction
    pub fn decode_instruction(&self, codeword: Codeword) -> Chip8Result<Operation> {
        self.instruction_set
            .decode(codeword)
            .ok_or_else(|| Chip8Error::InvalidInstruction(codeword))
    }

    /// Decodes the instruction stored in RAM at the given address.
    pub fn decode_at_addr(&self, addr: Address) -> Chip8Result<Operation> {
        let a = addr as usize;
        let hi = (self.core.ram[a] as Codeword) << 8;
        let lo = self.core.ram[a + 1] as Codeword;
        let codeword = hi | lo;

        self.decode_instruction(codeword)
    }

    /// Get the 16-bit word stored at the location pointed to by the program counter.
    pub fn current_codeword(&self) -> Codeword {
        let pc = self.core.pc as usize;
        let hi = self.core.ram[pc] as Codeword;
        let lo = self.core.ram[pc + 1] as Codeword;
        (hi << 8) | lo
    }

    /// Returns a copy of the lock for the keyboard.
    fn keyboard_lock(&mut self) -> Chip8Result<Arc<RwLock<Keyboard>>> {
        Ok(self.core.keyboard_lock())
    }
    /// Returns a copy of the lock for the vram.
    fn vram_lock(&mut self) -> Chip8Result<Arc<RwLock<Vram>>> {
        Ok(self.core.vram_lock())
    }
    /// Returns a copy of the lock for the buzzer.
    fn buzzer_lock(&mut self) -> Chip8Result<Arc<RwLock<Buzzer>>> {
        Ok(self.core.buzzer_lock())
    }
    /// Returns a copy of the lock for the audio.
    fn audio_lock(&mut self) -> Chip8Result<Arc<RwLock<Audio>>> {
        Ok(self.core.audio_lock())
    }
}
