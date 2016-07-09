use types::Address;
use config::Config;
use instructions::Operand;

/// Implementations of `Execute` can manipulate the machine state.
///
/// `Microprograms` make use of these functions to update the core as instructions are
/// executed. Currently, there is only one implementation: `Simulator`.
pub trait Execute {
    /// Returns the Config being used by the Execute
    fn config(&self) -> Config;
    /// Loads a value from the source Operand.
    fn store(&mut self, dest: Operand, data: usize);
    /// Stores a value into the destination Operand.
    fn load(&mut self, src: Operand) -> usize;
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
}
