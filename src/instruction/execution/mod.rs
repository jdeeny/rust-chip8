mod execute;
mod implementations;

pub use self::execute::Execute;

use instruction::Instruction;
use self::implementations::*;

/// Each operation is able to perform the action of a certain instruction or group of instructions.
pub type Microprogram = fn(&Instruction, &mut Execute);

pub fn execute_microcode(inst: &Instruction, executor: &mut Execute) {
    let op = inst.operation();
    match op {
        Operation::Add => op_add(inst, executor),
        //Operation::OpSub => op_sub(inst, executor),
        _ => panic!("not implemented"),
    }
}

#[derive(Copy,Clone,Eq,PartialEq)]
pub enum Operation {
    NoOp,
    Load,
    Stash,
    Fetch,

    Jump,
    JumpV0,
    Call,
    Ret,

    SkipEq,
    SkipNeq,

    Add,
    Sub,
    Subn,

    Or,
    And,
    Xor,
    Shr,
    Shl,

    Rand,

    Cls,
    Sprite,
    Font,
    Bcd,
}
