//! Defines the data types used to describe the Chip8 and associated peripherals.

use config::Config;
use instruction::{Dest, Src};


/// One byte in RAM.
pub type MemoryCell = u8;
/// An 8-bit register.
pub type Register8 = u8;
/// A 16-bit register.
pub type Register16 = u16;
/// An 8-bit timer.
pub type Timer = u8;
/// An address pointing to a location in CHIP-8 memory.
pub type Address = u16;
/// A 16-bit CHIP-8 codeword.
pub type Codeword = u16;
/// A single pixel.
pub type Pixel = u8;
/// The state of the keyboard.
pub type Keyboard = [bool; 16];
/// The state of the buzzer.
pub type Buzzer = bool;
/// The audio buffer used for XOCHIP.
pub type Audio = [u8; 16];
/// Vram
pub type Vram = Vec<Pixel>;

/// Errors that could be returned.
#[derive(Debug, Copy, Clone)]
pub enum Chip8Error {
    /// Attempt to access outside bounds -- TODO split into more specific errors
    OutOfBoundsAt(usize),
    /// Attempt to access outside bounds with unknown location -- TODO is this ever needed?
    OutOfBounds,
    /// Attempt to pop an address from an empty stack.
    PopEmptyStack,
    /// Attempt to execute an instruciton wtih an invalid type of operand
    InvalidOperand,
    /// A failure occured while trying to read from a channel.
    ChannelRxFailure,
    /// A failure occured while trying to write to a channel.
    ChannelTxFailure,
    /// An instruction was not recognized.
    InvalidInstruction(Codeword),
    /// Mutex error.
    MutexError,
}
/// The result type used throughout the library.
pub type Chip8Result<T> = Result<T, Chip8Error>;

/// Implementations of `Execute` can manipulate the machine state.
///
/// Each of the operations uses these fucntions to alter the state of the machine as instructions
/// are executed. Currently, there is only one implementation: `Chip8`.
pub trait Execute {
    /// Returns the Config being used by the Execute
    fn config(&self) -> Config;
    /// Loads a value from the source Operand.
    fn load(&mut self, src: Src) -> Chip8Result<usize>;
    /// Stores a value into the destination Operand.
    fn store(&mut self, dest: Dest, data: usize) -> Chip8Result<()>;
    /// Pops an item off the stack
    fn stack_pop(&mut self) -> Option<Address>;
    /// Pops an item off the stack
    fn stack_push(&mut self, address: Address);
    /// Returns the current address pointed to by the program counter
    fn pc(&self) -> Address;
    /// Advances the program counter one instruction.
    fn advance_pc(&mut self);
    /// Jumps the program counter to a given address.
    fn jump(&mut self, addr: Address) -> Chip8Result<()>;
    /// Store a flag in vF.
    fn set_flag(&mut self, state: bool);
    /// Set a pixel directly.
    fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel) -> Chip8Result<()>;
    /// Set a pixel by XORing it. Returns true if the pixel was already set.
    fn xor_pixel(&mut self, x: usize, y: usize, pixel: Pixel) -> Chip8Result<bool>;
    /// Returns the keyboard state.
    fn set_keyboard(&mut self, keys: &Keyboard) -> Chip8Result<()>;
    /// Returns the keyboard.
    fn keyboard(&self) -> Chip8Result<Keyboard>;
    /// Returns a reference to video buffer.
    fn vram(&self) -> Chip8Result<Vram>;
    /// Returns the buzzer state.
    fn buzzer(&self) -> Chip8Result<Buzzer>;
    /// Returns a mut reference to the audio buffer.
    fn audio(&self) -> Chip8Result<Audio>;
}
