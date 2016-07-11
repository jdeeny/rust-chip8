//! Simulates a chip8 cpu and provides a thread-safe interface to control execution and state.

#[cfg(test)]
mod tests;
mod controller;

pub use self::controller::Controller;

use std::fmt;
use std::sync::{Arc, RwLock};
use rand::{Rng, thread_rng};

use types::*;
use Chip8;
use config::Config;
use instruction::{Dest, Execute, Instruction, Src};

pub trait Simulate {
    fn load_bytes(&mut self, bytes: &[u8], addr: Address);
    fn ram(&self, addr: Address) -> MemoryCell;
    fn set_ram(&mut self, addr: Address, data: MemoryCell);
}



/// Manages the state of a chip8 cpu.
pub struct Simulator {
    core: Chip8,
}

impl Simulate for Simulator {
    /// Loads bytes into RAM starting at the given address.
    fn load_bytes(&mut self, bytes: &[u8], addr: Address) {
        let mut i = addr as usize;
        for b in bytes {
            self.core.ram[i] = *b;
            i += 1;
        }
    }
    fn ram(&self, addr: Address) -> MemoryCell {
        self.core.ram[addr as usize]
    }
    fn set_ram(&mut self, addr: Address, data: MemoryCell) {
        self.core.ram[addr as usize] = data;
    }

}

impl Simulator {
    /// Returns a new Simulator.
    pub fn new(config: &Config) -> Simulator {
        let core = Chip8::new(config);
        let mut s = Simulator { core: core };
        s.load_bytes(config.font_small, config.addr_font as Address);
        s
    }



    pub fn ram(&self, addr: Address) -> MemoryCell {
        self.core.ram[addr as usize]
    }

    pub fn set_ram(&mut self, data: MemoryCell, addr: Address) {
        self.core.ram[addr as usize] = data;
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

    /// Get the value of a register.
    pub fn reg(&mut self, reg: usize) -> u8 {
        self.core.v[reg]
    }

    /// Set the value of a register.
    pub fn set_reg(&mut self, reg: usize, val: u8) {
        self.core.v[reg] = val;
    }

    /// Clear vF to 0.
    pub fn vf_clear(&mut self) {
        self.core.v[0xF] = 0;
    }

    /// Set vF to 1.
    pub fn vf_set(&mut self) {
        self.core.v[0xF] = 1;
    }

    //    /// Gets the I register.
    // pub fn i(&self) -> usize {
    // self.core.i
    // }
    //
    // Sets the I register.
    // pub fn set_i(&mut self, val: Register16) {
    // self.core.i = val;
    // }

    /// Decrements the delay and sound timer.
    pub fn decrement_timers(&mut self) {
        if self.core.dt > 0 {
            self.core.dt -= 1;
        }
        if self.core.st > 0 {
            self.core.st -= 1;
        }
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



impl fmt::Debug for Simulator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Simulator {{}}")
    }
}


// Contains the data that is shared between the simulator and the UI.
// #[derive(Debug)]
// pub struct UiState {
// The video ram state.
// pub vram: Arc<RwLock<Vram>>,
// The keyboard state.
// pub keys: Arc<RwLock<Keyboard>>,
// The audio state.
// pub audio: Arc<RwLock<Audio>>,
// }
//
// impl UiState {
// Returns a a new UiState.
// pub fn new() -> UiState {
// Self::default()
// }
// }
//
// impl Default for UiState {
// fn default() -> UiState {
// UiState {
// vram: Arc::new(RwLock::new(Vram::new())),
// keys: Arc::new(RwLock::new(Keyboard::new())),
// audio: Arc::new(RwLock::new(Audio::new())),
// }
// }
// }
//
// impl Clone for UiState {
// fn clone(&self) -> UiState {
// UiState {
// vram: self.vram.clone(),
// keys: self.keys.clone(),
// audio: self.audio.clone(),
// }
// }
// }
//
// Holds the state of the video ram of the simulator.
// #[derive(Copy)]
// pub struct Vram {
// Holds the state of the pixels.
// pub pixels: [[u8; 32]; 64],
// }
// impl Vram {
// Returns a new Vram.
// pub fn new() -> Vram {
// Self::default()
// }
// }
// impl Default for Vram {
// fn default() -> Vram {
// Vram { pixels: [[0; 32]; 64] }
// }
// }
// impl fmt::Debug for Vram {
// fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// write!(f, "Vram {{}}")
// }
// }
// impl Clone for Vram {
// fn clone(&self) -> Self {
// self
// let mut p: [[u8;32];64] = [[0;32];64];
// for (i, row) in self.pixels.iter().enumerate() {
//    p[i] = *row;
// }
// /Vram { pixels: p }
// }
// }
//
// Holds the state of the keyboard of the simulator.
// #[derive(Copy,Clone,Debug)]
// pub struct Keyboard {
// The state of the keyboard. True indicates that the key is pressed.
// pub state: [bool; 16],
// }
// impl Keyboard {
// Returns a new Keyboard.
// pub fn new() -> Keyboard {
// Self::default()
// }
// Returns true if the requested key is currently pressed.
// pub fn is_down(&self, key: usize) -> bool {
// assert!(key <= 16);
// self.state[key]
// }
// }
// impl Default for Keyboard {
// fn default() -> Keyboard {
// Keyboard { state: [false; 16] }
// }
// }
//
// Holds the state of the audio output of the simulator.
// #[derive(Copy,Clone,Debug)]
// pub struct Audio {
//
// }
// impl Audio {
// Returns a new Audio.
// pub fn new() -> Audio {
// Self::default()
// }
// }
// impl Default for Audio {
// fn default() -> Audio {
// Audio {}
// }
// }
//




// use config::Config;
// use state::Chip8;
// use instruction::Set;
//
// pub struct Chip8 {
// pub config: Config,
// pub state: Chip8,
// pub instruction_set: Set,
// }
//
// impl Chip8 {
// pub fn new(config: Config) -> Chip8 {
// Chip8 {
// config: config,
// state: Chip8::new(config),
// instruction_set: Set::new(),
// }
// }
// }
//
