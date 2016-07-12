use std::sync::{RwLockWriteGuard, RwLockReadGuard};
use types::*;
use config::Config;
use instruction::{Dest, Src};

/// Implementations of `Execute` can manipulate the machine state.
///
/// Each of the operations uses these fucntions to alter the state of the machine as instructions
/// are executed. Currently, there is only one implementation: `Chip8`.
pub trait Execute {
    /// Returns the Config being used by the Execute
    fn config(&self) -> Config;
    /// Loads a value from the source Operand.
    fn load(&mut self, src: Src) -> usize;
    /// Stores a value into the destination Operand.
    fn store(&mut self, dest: Dest, data: usize);
    /// Pops an item off the stack
    fn stack_pop(&mut self) -> Option<Address>;
    /// Pops an item off the stack
    fn stack_push(&mut self, address: Address);
    /// Returns the current address pointed to by the program counter
    fn pc(&self) -> Address;
    /// Advances the program counter one instruction.
    fn advance_pc(&mut self);
    /// Jumps the program counter to a given address.
    fn jump(&mut self, addr: Address);
    /// Store a flag in vF.
    fn set_flag(&mut self, state: bool);
    /// Returns a mut reference to video buffer.
    fn vram_mut(&mut self) -> RwLockWriteGuard<Vram>;
    /// Returns the keyboard state.
    fn keyboard(&self) -> Keyboard;
    /// Returns a mut reference to the audio buffer.
    fn audio_mut(&mut self) -> RwLockWriteGuard<Audio>;
}
