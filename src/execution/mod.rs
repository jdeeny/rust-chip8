mod execute;
pub mod implementations;

pub use self::execute::Execute;

use instructions::Instruction;
use self::implementations::*;

/// Each operation is able to perform the action of a certain instruction or group of instructions.
pub type Microprogram = fn(&Instruction, &mut Execute);

pub fn execute_microcode(inst: &Instruction, executor: &mut Execute) {
    let op = inst.operation();
    match op {
        Operation::OpAdd => op_add(inst, executor),
        //Operation::OpSub => op_sub(inst, executor),
        _ => panic!("not implemented"),
    }
}

#[derive(Copy,Clone)]
pub enum Operation {
    OpCls,
    OpRet,
    OpJump,
    OpJumpV0,
    OpCall,
    OpSkipEq,
    OpSkipNeq,
    OpLoad,
    OpAdd,
    OpSub,
    OpSubn,
    OpOr,
    OpAnd,
    OpXor,
    OpShr,
    OpShl,
    OpRand,
    OpSprite,
    OpFont,
    OpBcd,
    OpStash,
    OpFetch,
}
