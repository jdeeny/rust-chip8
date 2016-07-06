//! Simulates a chip8 cpu.

use std::fmt;
use std::sync::{Arc, RwLock};
use rand::{Rng, thread_rng};

use config::{ Config, Configured };

use instruction;
use instruction::{Instruction, Operand, Word};
use instruction::Operand::*;

use fonts::FONT_CHIP8_4X5;

/// Manages the state of a chip8 cpu.
pub struct Simulator {
    /// The chip8 machine configuration.
    pub config: Config,
    /// The state that is shared with the UI.
    pub state: UiState,
    gp_reg: [u8; 16],
    i: usize,
    pc: usize,
    delay_timer: u8,
    sound_timer: u8,
    ram: Vec<u8>,
    /// The call stack.
    pub stack: Vec<usize>,
    rng: Box<Rng>,
    itable: instruction::Table,
}

impl Simulator {
    /// Returns a new Simulator.
    pub fn new(config: Config) -> Simulator {
        let font = &FONT_CHIP8_4X5;
        let mut ram: Vec<u8> = vec![0; config.sys_ram_bytes];

        // copy font to beginning of RAM, into the 0-0x200 area
        let font_start = config.sys_font_addr;
        let font_end = font_start + font.len();
        println!("font addr {:X} - {:X}", font_start, font_end);
        ram[font_start..font_end].copy_from_slice(font);

        Simulator {
            config: config,
            state: UiState::new(),
            gp_reg: [0; 16],
            i: 0,
            pc: 0,
            delay_timer: 0,
            sound_timer: 0,
            ram: ram,
            stack: vec![],
            rng: Box::new(thread_rng()),
            itable: instruction::Table::new(config),
        }
    }

    /// Loads bytes into RAM starting at the given address.
    pub fn load_bytes(&mut self, bytes: &[u8], addr: usize) {
        let mut i = addr;
        for b in bytes {
            self.ram[i] = *b;
            i += 1;
        }
    }

    /// Decodes an instruction. TODO: Move to ::instruction
    pub fn decode_instruction(&self, codeword: Word) -> Instruction {
        self.itable.decode(codeword)
    }

    /// Decodes the instruction stored in RAM at the given address.
    pub fn decode_at_addr(&self, addr: usize) -> Instruction {
        let hi = self.ram[addr];
        let lo = self.ram[addr + 1];
        let word = (hi as u16) << 8 | lo as u16;

        self.itable.decode(word)
    }

    /// Get the 16-bit word stored at the location pointed to by the program counter.
    pub fn current_codeword(&self) -> Word {
        let hi = self.ram[self.pc] as Word;
        let lo = self.ram[self.pc + 1] as Word;
        (hi << 8) | lo
    }

    /// Get the value of a register.
    pub fn reg(&mut self, reg: usize) -> u8 {
        self.gp_reg[reg]
    }

    /// Set the value of a register.
    pub fn set_reg(&mut self, reg: usize, val: u8) {
        self.gp_reg[reg] = val;
    }

    /// Clear vF to 0.
    pub fn vf_clear(&mut self) {
        self.gp_reg[0xF] = 0;
    }

    /// Set vF to 1.
    pub fn vf_set(&mut self) {
        self.gp_reg[0xF] = 1;
    }

    /// Store a flag in vF.
    pub fn vf_store(&mut self, flag: bool) {
        self.gp_reg[0xF] = if flag {
            1
        } else {
            0
        };
    }

    /// Gets the value of the program counter.
    pub fn pc(&self) -> usize {
        self.pc
    }

    /// Advances the program counter one instruction.
    pub fn advance_pc(&mut self) {
        self.pc += 2;
    }

    /// Jumps the program counter to a given address.
    pub fn jump_pc(&mut self, addr: usize) {
        self.pc = addr;
    }

    /// Gets the value stored at an address in RAM.
    pub fn ram(&self, addr: usize) -> u8 {
        self.ram[addr]
    }

    /// Sets the value stored at an address in RAM.
    pub fn set_ram(&mut self, addr: usize, data: u8) {
        self.ram[addr] = data;
    }

    /// Gets the I register.
    pub fn i(&self) -> usize {
        self.i
    }

    /// Sets the I register.
    pub fn set_i(&mut self, val: usize) {
        self.i = val;
    }

    /// Decrements the delay and sound timer.
    pub fn decrement_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    /// Loads a value from the source Operand.
    pub fn load(&mut self, src: Operand) -> u32 {
        match src {
            Register(r) => self.reg(r) as u32,
            Address12(a) => self.ram[a] as u32,
            I => self.i as u32,
            IndirectI => self.ram[self.i] as u32,
            Literal12(n12) => n12 as u32,
            Literal8(n8) => n8 as u32,
            Literal4(n4) => n4 as u32,
            SoundTimer => self.sound_timer as u32,
            DelayTimer => self.delay_timer as u32,
            Random => self.rng.next_u32(),
            _ => 0,
            // Operand::Nowhere   => panic!("Cannot load nothing"),
        }
    }

    /// Stores a value into the destination Operand.
    pub fn store(&mut self, dest: Operand, val: u32) {
        match dest {
            Register(r) => {
                self.gp_reg[r] = (val & 0xFF) as u8;
            }
            Address12(a) => {
                self.ram[a] = (val & 0xFF) as u8;
            }
            I => {
                self.i = (val & 0xFFFF) as usize;
            }
            IndirectI => {
                self.ram[self.i] = val as u8;
            }
            SoundTimer => {
                self.sound_timer = val as u8;
            }
            DelayTimer => {
                self.delay_timer = val as u8;
            }
            Literal12(_) | Literal8(_) | Literal4(_) | Random | Nowhere => {
                panic!("Cannot store");
            }
        }
    }

    /// Returns the current config.
    pub fn config(&self) -> Config {
        self.config
    }

    /// Executes an instruction.
    pub fn execute(&mut self, instruction: &Instruction) {
        instruction.operation()(instruction, self);
    }
}

impl Configured for Simulator {
    fn config(&self) -> Config {
        self.config
    }
}

impl fmt::Debug for Simulator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Simulator {{}}")
    }
}


/// Contains the data that is shared between the simulator and the UI.
#[derive(Debug)]
pub struct UiState {
    /// The video ram state.
    pub vram: Arc<RwLock<Vram>>,
    /// The keyboard state.
    pub keys: Arc<RwLock<Keyboard>>,
    /// The audio state.
    pub audio: Arc<RwLock<Audio>>,
}

impl UiState {
    /// Returns a a new UiState.
    pub fn new() -> UiState {
        Self::default()
    }
}

impl Default for UiState {
    fn default() -> UiState {
        UiState {
            vram: Arc::new(RwLock::new(Vram::new())),
            keys: Arc::new(RwLock::new(Keyboard::new())),
            audio: Arc::new(RwLock::new(Audio::new())),
        }
    }
}

impl Clone for UiState {
    fn clone(&self) -> UiState {
        UiState {
            vram: self.vram.clone(),
            keys: self.keys.clone(),
            audio: self.audio.clone(),
        }
    }
}

/// Holds the state of the video ram of the simulator.
#[derive(Copy)]
pub struct Vram {
    ///Holds the state of the pixels.
    pub pixels: [[u8; 32]; 64],
}
impl Vram {
    /// Returns a new Vram.
    pub fn new() -> Vram {
        Self::default()
    }
}
impl Default for Vram {
    fn default() -> Vram {
        Vram { pixels: [[0; 32]; 64] }
    }
}
impl fmt::Debug for Vram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vram {{}}")
    }
}
impl Clone for Vram {
    fn clone(&self) -> Self {
        let mut p: [[u8;32];64] = [[0;32];64];
        for (i, row) in self.pixels.iter().enumerate() {
            p[i] = *row;
        }
        Vram { pixels: p }
    }
}

/// Holds the state of the keyboard of the simulator.
#[derive(Copy,Clone,Debug)]
pub struct Keyboard {
    /// The state of the keyboard. True indicates that the key is pressed.
    pub state: [bool; 16],
}
impl Keyboard {
    /// Returns a new Keyboard.
    pub fn new() -> Keyboard {
        Self::default()
    }
    /// Returns true if the requested key is currently pressed.
    pub fn is_down(&self, key: usize) -> bool {
        assert!(key <= 16);
        self.state[key]
    }
}
impl Default for Keyboard {
    fn default() -> Keyboard {
        Keyboard { state: [false; 16] }
    }
}

/// Holds the state of the audio output of the simulator.
#[derive(Copy,Clone,Debug)]
pub struct Audio {

}
impl Audio {
    /// Returns a new Audio.
    pub fn new() -> Audio {
        Self::default()
    }
}
impl Default for Audio {
    fn default() -> Audio {
        Audio {}
    }
}
