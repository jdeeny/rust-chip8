//! Simulates a chip8 cpu and provides a thread-safe interface to control execution and state.

use std::fmt;
use rand::{Rng, thread_rng};

use config::Config;
use types::*;

use chip8::Chip8;
use instructions;
use instructions::{Executor, Instruction, Operand, Word};


/// Manages the state of a chip8 cpu.
pub struct Simulator {
    core: Chip8,
    itable: instructions::Table,
}

impl Simulator {
    /// Returns a new Simulator.
    pub fn new(config: Config) -> Simulator {
        let mut s = Simulator {
                        core: Chip8::new(config),
                        itable: instructions::Table::new(config),
                    };
        s.load_bytes(config.font_small, config.addr_font as Address);
        s
    }

    /// Loads bytes into RAM starting at the given address.
    pub fn load_bytes(&mut self, bytes: &[u8], addr: Address) {
        let mut i = addr as usize;
        for b in bytes {
            self.core.ram[i] = *b;
            i += 1;
        }
    }

    /// Decodes an instruction. TODO: Move to ::instruction
    pub fn decode_instruction(&self, codeword: Word) -> Instruction {
        self.itable.decode(codeword)
    }

    /// Decodes the instruction stored in RAM at the given address.
    pub fn decode_at_addr(&self, addr: Address) -> Instruction {
        let a = addr as usize;
        let hi = (self.core.ram[a] as Word) << 8;
        let lo = self.core.ram[a + 1] as Word;
        let codeword = hi | lo;

        self.itable.decode(codeword)
    }

    /// Get the 16-bit word stored at the location pointed to by the program counter.
    pub fn current_codeword(&self) -> Word {
        let pc = self.core.pc as usize;
        let hi = self.core.ram[pc] as Word;
        let lo = self.core.ram[pc + 1] as Word;
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

/*    /// Gets the I register.
    pub fn i(&self) -> usize {
        self.core.i
    }

    /// Sets the I register.
    pub fn set_i(&mut self, val: Register16) {
        self.core.i = val;
    }*/

    /// Decrements the delay and sound timer.
    pub fn decrement_timers(&mut self) {
        if self.core.dt > 0 {
            self.core.dt -= 1;
        }
        if self.core.st > 0 {
            self.core.st -= 1;
        }
    }

    fn step(&mut self) {
        let i = self.decode_at_addr(self.pc());
        self.advance_pc();
        self.execute(&i);
    }


    /// Executes an instruction.
    pub fn execute(&mut self, instruction: &Instruction) {
        instruction.operation()(instruction, self);
    }

    /*/// Returns the `UiState`.
    pub fn state(&self) -> UiState {
        self.state.clone()
    }*/

}

impl Executor for Simulator {

/*    /// Gets the value stored at an address in RAM.
    fn ram(&self, addr: Address) -> MemoryCell {
        self.core.ram[addr]
    }

    /// Sets the value stored at an address in RAM.
    fn set_ram(&mut self, addr: Address, data: MemoryCell) {
        self.core.ram[addr] = data;
    }*/

    fn config(&self) -> Config {
        self.core.config
    }

    fn load(&mut self, src: Operand) -> usize {
        match src {
            Operand::Register(r) => self.core.v[r] as usize,
            Operand::Address12(a) => self.core.ram[a] as usize,
            Operand::I => self.core.i as usize,
            Operand::IndirectI => self.core.ram[self.core.i as usize] as usize,
            Operand::Literal12(n12) => n12,
            Operand::Literal8(n8) => n8,
            Operand::Literal4(n4) => n4,
            Operand::SoundTimer => self.core.st as usize,
            Operand::DelayTimer => self.core.dt as usize,
            Operand::Random => (thread_rng().next_u32() & 0xFF) as usize,
            _ => 0,
            // Operand::Nowhere   => panic!("Cannot load nothing"),
        }
    }

    fn store(&mut self, dest: Operand, data: usize) {
        match dest {
            Operand::Register(r) => {
                self.core.v[r] = (data & 0xFF) as MemoryCell;
            }
            Operand::Address12(a) => {
                self.core.ram[a] = (data & 0xFF) as MemoryCell;
            }
            Operand::I => {
                self.core.i = (data & 0xFFFF) as Register16;
            }
            Operand::IndirectI => {
                self.core.ram[self.core.i as usize] = data as MemoryCell;
            }
            Operand::SoundTimer => {
                self.core.st = data as Timer;
            }
            Operand::DelayTimer => {
                self.core.dt = data as Timer;
            }
            Operand::Literal12(_) | Operand::Literal8(_) | Operand::Literal4(_) |
            Operand::Random | Operand::Nowhere => {
                panic!("Cannot store");
            }
        }
    }

    fn set_flag(&mut self, flag: bool) {
        self.core.v[0xF] = if flag {
            1
        } else {
            0
        };
    }

    fn stack_pop(&mut self) -> Option<Address> {
        self.core.stack.pop()
    }

    fn stack_push(&mut self, address: Address) {
        self.core.stack.push(address);
    }

    fn pc(&self) -> Address {
        self.core.pc
    }

    fn advance_pc(&mut self) {
        self.core.pc += 2;
    }

    fn jump(&mut self, addr: Address) {
        self.core.pc = addr;
    }

}


impl fmt::Debug for Simulator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Simulator {{}}")
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use instructions::{Executor, Operand};
    use config::Config;
    use types::Address;
    #[test]
    fn test_sim_jump() {
        let config = Config::default();
        let mut s = Simulator::new(config);
        let prog = [0x60, 0x55, 0x12, 0x00];    //LD V0, 0x55; Jump 0x200
        s.load_bytes(&prog, config.addr_program as Address);
        s.jump(config.addr_program as Address);
        s.step();
        assert_eq!(s.pc(), 0x202);
        assert_eq!(s.load(Operand::Register(0)), 0x55);
        s.step();
        assert_eq!(s.pc(), 0x200);

    }
}
