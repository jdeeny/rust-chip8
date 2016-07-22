//! Simulates a chip8 cpu and provides a thread-safe interface to control execution and state.

#[cfg(test)]
mod tests;
mod threaded;

use std::fmt;
use std::sync::{Arc, RwLock, RwLockWriteGuard, RwLockReadGuard};
use rand::{Rng, ThreadRng, thread_rng};

use types::*;
use Chip8;
use config::Config;
use instruction::{self, Dest, Instruction, Src};

pub use self::threaded::SimulatorTask;

use state::RandomBytes;

pub trait Simulate {
    /// Fetch the current instruction, advance the PC, and execute the instruction.
    fn step(&mut self) -> Chip8Result<()>;
    fn step_n(&mut self, number_of_steps: usize) -> Chip8Result<()>;
    fn timer_tick(&mut self) -> Chip8Result<()>;
    fn load_bytes(&mut self, bytes: &[u8], addr: Address) -> Chip8Result<()>;
    fn load_program(&mut self, bytes: &[u8]) -> Chip8Result<()>;
    fn load(&mut self, src: Src) -> Chip8Result<usize>;
    fn store(&mut self, dest: Dest, value: usize) -> Chip8Result<()>;
    fn set_keyboard(&mut self, keys: &Keyboard) -> Chip8Result<()>;
    fn keyboard(&self) -> Chip8Result<Keyboard>;
    fn vram(&self) -> Chip8Result<Vram>;
    fn buzzer(&self) -> Chip8Result<Buzzer>;
    fn audio(&self) -> Chip8Result<Audio>;
}

/// Manages the state of a chip8 cpu.
pub struct Simulator {
    core: Chip8,
    instruction_set: instruction::Set,
}

impl Simulate for Simulator {
    /// Loads bytes into RAM starting at the given address.
    fn load_bytes(&mut self, bytes: &[u8], addr: Address) -> Chip8Result<()>  {
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
        let instruction = self.decode_at_addr(self.core.pc());
        println!("{:?} @ {:X}", instruction, self.core.pc());
        self.core.advance_pc();
        instruction.execute(&mut self.core);
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
    pub fn new(config: &Config, rand_iterator: Option<RandomBytes>) -> Simulator {
        let core: Chip8 = Chip8::new(config, rand_iterator);
        let iset = instruction::Set::new(config);
        let mut s = Simulator {
            core: core,
            instruction_set: iset,
        };
        s.load_bytes(config.font_small, config.addr_font as Address).unwrap();
        s.core.store(Dest::PC, config.addr_program).unwrap();
        s
    }

    pub fn default() -> Simulator {
        Self::new(&Config::default(), None)
    }

    /// Decodes an instruction. TODO: Move to ::instruction
    pub fn decode_instruction(&self, codeword: Codeword) -> Instruction {
        self.instruction_set.decode(codeword).unwrap()
    }

    /// Decodes the instruction stored in RAM at the given address.
    pub fn decode_at_addr(&self, addr: Address) -> Instruction {
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

    fn keyboard_lock(&mut self) -> Chip8Result<Arc<RwLock<Keyboard>>> {
        Ok(self.core.keyboard_lock())
    }
    fn vram_lock(&mut self) -> Chip8Result<Arc<RwLock<Vram>>> {
        Ok(self.core.vram_lock())
    }
    fn buzzer_lock(&mut self) -> Chip8Result<Arc<RwLock<Buzzer>>> {
        Ok(self.core.buzzer_lock())
    }
    fn audio_lock(&mut self) -> Chip8Result<Arc<RwLock<Audio>>> {
        Ok(self.core.audio_lock())
    }


}
