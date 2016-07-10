use std::boxed::FnBox;

mod execute;
// mod implementations;
mod operations;

pub use self::execute::Execute;
pub use self::operations::{Operation, OperationKind};

use types::*;
use instruction::{Instruction, Src, Dest};
// use self::implementations::*;

#[allow(non_snake_case)]
pub fn execute_microcode(inst: &Instruction, exec: &mut Execute) {
    let op = inst.operation();
    let prog: Box<FnBox()> = match op {

        Operation::NoOp => Box::new(move || assert!(true)),

        Operation::Add(A, B, D) => Box::new(move || {
            let lhs = exec.load(A);
            let rhs = exec.load(B);
            let total = lhs + rhs;
            exec.set_flag(total > 0xFF);  //set vF if result overflows
            exec.store(D, total);
        }),
        /// Jump to address
        Operation::Jump(S) => Box::new(move || {
            let addr: Address = exec.load(S) as Address;
            exec.jump(addr);
        }),
        /// Jump to address + V0
        Operation::JumpV0(S) => Box::new(move || {
            let mut addr = exec.load(S) as Address;
            addr += exec.load(Src::Register(0)) as Address;
            exec.jump(addr);
        }),
        /// Add current program counter to the stack and jump to address.
        Operation::Call(S) => Box::new(move || {
            let addr = exec.load(S) as Address;
            let pc = exec.pc();
            exec.stack_push(pc);
            exec.jump(addr);
        }),

        Operation::Load(D, S) => Box::new(move || assert!(true)),

        Operation::Cls => Box::new(move || assert!(true)),
        Operation::Sprite(X, Y, N) => Box::new(move || assert!(true)),

        _ => Box::new(|| panic!("not implemented")),
    };
    prog();
}
