use std::fmt;
use std::clone::Clone;

use types::*;
use instruction::{Definition, Dest, Src};
use instruction::execution::{Operation, Execute};

/// A fully specified chip8 instruction.
#[derive(Copy,Clone)]
pub struct Instruction {
    /// The definition from the instruction table that matches this instruction.
    operation: Operation,
}

impl Instruction {
    /// Returns a new Instruction.
    pub fn new(op: Operation) -> Instruction {
        Instruction {
            operation: op,
        }
    }
    pub fn default() -> Instruction {
        Instruction {
            operation: Operation::NoOp,
        }
    }
    pub fn from_definition(def: &Definition, codeword: Codeword) -> Instruction {
/*        let mut dest_data: usize = 0;
        let mut src_data: usize = 0;
        let mut aux_data: usize = 0;

        let mut word = codeword as usize;
        for coding in &def.pattern {
            let nibble = (word & 0xF000) >> 12;
            word <<= 4;
            // println!("nibble: {:?}", nibble);
            match *coding {
                self::Coding::C(_) => {}
                self::Coding::D => {
                    dest_data = (dest_data << 4) | nibble;
                }
                self::Coding::S => {
                    src_data = (src_data << 4) | nibble;
                }
                self::Coding::A => {
                    aux_data = (aux_data << 4) | nibble;
                }
            }
        }


        let dest: Dest;
        let src: Src;
        let aux: Src;
        {
            dest = def.dest_kind.specify(dest_data);
            src = def.src_kind.specify(src_data);
            aux = def.aux_kind.specify(aux_data);
        }

        println!("{:?} {:?} {:?}", dest, src, aux);

        Instruction {
            //def: def,
            //codeword: codeword,
            operation: def.operation,
            dest: dest,
            src: src,
            aux: aux,
        }*/
        Instruction::default()
    }

    /*
    /// Returns the destination operand.
    pub fn dest(&self) -> Dest {
        self.dest
    }

    /// Returns the source operand.
    pub fn src(&self) -> Src {
        self.src
    }

    /// Returns the auxillary operand.
    pub fn aux(&self) -> Src {
        self.aux
    }
    */
    /// Returns the operation used by this instruction.
    pub fn operation(&self) -> Operation {
        self.operation.clone()
    }

    pub fn execute(&self, cpu: &mut Execute) {
        self.operation.execute(cpu);
    }

}

/*impl Clone for Instruction {
    fn clone(&self) -> Instruction {
        Instruction {
            operation: self.operation,
            dest: self.dest,
            src: self.src,
            aux: self.aux,
        }
    }
}*/

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Instruction")
    }
}
