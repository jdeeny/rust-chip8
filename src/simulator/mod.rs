//! Simulates a chip8 cpu and provides a thread-safe interface to control execution and state.

#[cfg(test)]
mod tests;
//mod controller;

//pub use self::controller::Controller;

use std::fmt;
use std::sync::{Arc, RwLock};
use rand::{Rng, ThreadRng, thread_rng};

use types::*;
use Chip8;
use config::Config;
use instruction::{Dest, Instruction, Src};

enum RunState {
    RESET,
    IDLE,
    RUN
}

pub trait Simulate {
    fn run(&mut self, cycles_per_tick: Option<usize>);
    fn halt(&mut self);
    fn step(&mut self) -> Chip8Result<()>;
    fn step_n(&mut self, number_of_steps: usize) -> Chip8Result<()>;
    fn timer_tick(&mut self);

    fn load_bytes(&mut self, bytes: &[u8], addr: Address) -> Chip8Result<()>;
    fn load_program(&mut self, bytes: &[u8]) -> Chip8Result<()>;
    fn load(&mut self, src: Src) -> Chip8Result<usize>;
    fn store(&mut self, dest: Dest, value: usize) -> Chip8Result<()>;
}



/// Manages the state of a chip8 cpu.
pub struct Simulator<'a> {
    core: Chip8<'a>,
    execution_state: RunState,
}

impl<'a> Simulate for Simulator<'a> {
    /// Loads bytes into RAM starting at the given address.
    fn load_bytes(&mut self, bytes: &[u8], addr: Address) -> Chip8Result<()>  {
        self.core.load_bytes(bytes, addr)
    }

    fn load_program(&mut self, bytes: &[u8]) -> Chip8Result<()> {
        let address = self.core.config.addr_program;
        self.load_bytes(bytes, address as Address)
    }

    /// Decrements the delay and sound timer.
    fn timer_tick(&mut self) {
        if self.core.dt > 0 {
            self.core.dt -= 1;
        }
        if self.core.st > 0 {
            self.core.st -= 1;
        }
    }

    fn load(&mut self, src: Src) -> Chip8Result<usize> {
        self.core.load(src)
    }
    fn store(&mut self, dest: Dest, value: usize) -> Chip8Result<()> {
        self.core.store(dest, value)
    }
    fn run(&mut self, cycles_per_tick: Option<usize>) {

    }
    fn halt(&mut self) {

    }
    fn step(&mut self) -> Chip8Result<()> {
        Ok(())
    }
    fn step_n(&mut self, number_of_steps: usize) -> Chip8Result<()> {
        for _ in 0..number_of_steps {
            self.step();
        }
        Ok(())
    }

}

impl<'a> Simulator<'a> {
    /// Returns a new Simulator.
    pub fn new(config: &Config, rand_iterator: Option<&'a mut Iterator<Item=MemoryCell>>) -> Simulator<'a> {
        let core = Chip8::new(config, rand_iterator);
        let mut s = Simulator {
            core: core,
            execution_state: RunState::RESET,
        };
        s.load_bytes(config.font_small, config.addr_font as Address);
        s
    }

    pub fn default() -> Simulator<'a> {
        Self::new(&Config::default(), None)
    }

    /// Decodes an instruction. TODO: Move to ::instruction
    pub fn decode_instruction(&self, codeword: Codeword) -> Instruction {
        //        self.itable.decode(codeword)
        Instruction::default()
    }

    /// Decodes the instruction stored in RAM at the given address.
    pub fn decode_at_addr(&self, addr: Address) -> Instruction {
        let a = addr as usize;
        let hi = (self.core.ram[a] as Codeword) << 8;
        let lo = self.core.ram[a + 1] as Codeword;
        let codeword = hi | lo;

        // self.itable.decode(codeword)
        Instruction::default()
    }

    /// Get the 16-bit word stored at the location pointed to by the program counter.
    pub fn current_codeword(&self) -> Codeword {
        let pc = self.core.pc as usize;
        let hi = self.core.ram[pc] as Codeword;
        let lo = self.core.ram[pc + 1] as Codeword;
        (hi << 8) | lo
    }




    /// Fetch the current instruction, advance the PC, and execute the instruction.
    pub fn step(&mut self) {
        let i = self.decode_at_addr(self.core.pc());
        self.core.advance_pc();
        self.execute(&i);
    }


    /// Executes an instruction.
    pub fn execute(&mut self, instruction: &Instruction) {
        //instruction.operation()(instruction, self);
    }

}
