//! Data types used to describe the data that makes up the Chip8.

/// One byte in RAM.
pub type MemoryCell = u8;
/// A value in an 8-bit register.
pub type Register8 = u8;
/// A value in a 16-bit register.
pub type Register16 = u16;
/// An 8-bit timer.
pub type Timer = u8;
/// An Address.
pub type Address = u16;
/// A contiguous set of MemoryCells.
pub type Memory = Vec<MemoryCell>;
/// A 16-bit chip8 codeword.
pub type Codeword = u16;
/// A single pixel. 
pub type Pixel = u8;
/// The state of the keyboard.
pub type Keyboard = [bool; 16];
/// The state of the audio output.
pub type Audio = bool;
