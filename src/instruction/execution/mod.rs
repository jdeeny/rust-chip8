use std::boxed::FnBox;

mod execute;
//mod implementations;
mod operations;

pub use self::execute::Execute;
pub use self::operations::{Operation,OperationKind};

use instruction::Instruction;
//use self::implementations::*;

#[allow(non_snake_case)]
pub fn execute_microcode(inst: &Instruction, exec: &mut Execute) {
    let op = inst.operation();
    let prog: Box<FnBox()> = match op {
        Operation::Add(A, B, D) => Box::new(move ||  {
                let lhs = exec.load(A);
                let rhs = exec.load(B);
                let total = lhs + rhs;
                exec.set_flag(total > 0xFF);  //set vF if result overflows
                exec.store(D, total);
            }),
        //Operation::OpSub => op_sub(inst, executor),
        _ => Box::new(|| { panic!("not implemented") }),
    };
    prog();
}
