//! Defines the data types used to describe the Chip8 and associated peripherals.

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

/// A set of errors that could be returned.
#[derive(Debug)]
pub enum Chip8Error {
    OutOfBoundsAt(usize),
    OutOfBounds,
    PopEmptyStack,
    InvalidOperand,
}
/// The result type used throughout the library.
pub type Chip8Result<T> = Result<T, Chip8Error>;
