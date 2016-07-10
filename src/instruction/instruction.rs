use std::fmt;
use std::clone::Clone;

use execution::{Operation, execute_microcode, Execute};
use instruction::{Definition,Operand};
use types::*;

/// A fully specified chip8 instruction.
#[derive(Copy)]
pub struct Instruction {
    /// The definition from the instruction table that matches this instruction.
    //pub def: Definition,
    //#[allow(dead_code)]
    //codeword: Codeword,
    operation: Operation,
    dest: Operand,
    src: Operand,
    aux: Operand,
}

impl Instruction {
    /// Returns a new Instruction.
    pub fn new(def: &Definition, codeword: Codeword) -> Instruction {
        let mut dest_data: usize = 0;
        let mut src_data: usize = 0;
        let mut aux_data: usize = 0;

        let mut word = codeword as usize;
/*        for coding in &def.pattern {
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
        }*/


        let dest: Operand;
        let src: Operand;
        let aux: Operand;
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
        }
    }

    /// Returns the destination operand.
    pub fn dest(&self) -> Operand {
        self.dest
    }

    /// Returns the source operand.
    pub fn src(&self) -> Operand {
        self.src
    }

    /// Returns the auxillary operand.
    pub fn aux(&self) -> Operand {
        self.aux
    }

    /// Returns the operation used by this instruction.
    pub fn operation(&self) -> Operation {
        let op: Operation = self.operation.clone();
        op
    }

    pub fn execute(&self, cpu: &mut Execute) {
        execute_microcode(self, cpu)
    }

    /// Returns a string describing the instruction.
    pub fn to_string(&self) -> String {
        //use strfmt::strfmt;
        use std::collections::HashMap;
        let mut vars = HashMap::new();
        vars.insert("d".to_string(), self.dest.to_string());
        vars.insert("s".to_string(), self.src.to_string());
        vars.insert("a".to_string(), self.aux.to_string());
        "".to_string()
        /*if let Ok(s) = strfmt(&self.def.mnemonic, &vars) {
            s
        } else {
            "".to_string()
            //TODO: log this?
        }*/
    }
}

impl Clone for Instruction {
    fn clone(&self) -> Instruction {
        Instruction {
            operation: self.operation,
            dest: self.dest,
            src: self.src,
            aux: self.aux,
        }
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Instruction")
    }
}
