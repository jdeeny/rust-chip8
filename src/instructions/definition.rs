use std::fmt;
use instructions::{Operand, OperandKind};
use instructions::operations::Operation;

/// A 16-bit chip8 codeword.
pub type Codeword = u16;

/// Type to hold instruction word pattern
pub type Pattern = [Coding; 4];

/// Used to define the coding of each instruction type
#[derive(Clone,Copy,Debug)]
pub enum Coding {
    /// A constant nibble (literal)
    C(u8),
    /// Destination
    D,
    /// Source
    S,
    /// Auxillary
    A,
}


struct CodewordMatcher {
    code: Codeword,
    mask: Codeword,
}

impl CodewordMatcher {
    fn new(pattern: Pattern) -> CodewordMatcher {
        let mut code: Codeword = 0;
        let mut mask: Codeword = 0;
        for coding in &pattern {
            code <<= 4;
            mask <<= 4;
            if let Coding::C(n) = *coding {
                code |= n as Codeword;
                mask |= 0xF;
            };
        }
        CodewordMatcher {
            code: code,
            mask: mask,
        }
    }
    /// Returns true if a given codeword matches this definition.
    pub fn is_match(&self, codeword: Codeword) -> bool {
        (codeword & self.mask) == (self.code & self.mask)
    }

}

/// Defines the structure of a specific instruction
///
/// It has a unique signature: the kind of operation and the kinds of the locations dest, src, aux
///
/// pattern defines how the instruction is decoded:
///     C(n) is a constant nibble. The codeword must match for this instruction to be valid.
///     D, S, and A are value markers that indicate which nibbles represent which operand's value.
///     If more than one nibble is used for the same operand, the leftmost nibble is most significant
///     and the rightmost is least significant.
///     D indicates dest, S src, and A aux.
pub struct Definition {
    /// The operation that will be performed when this type of instruction is executed.
    pub operation: Operation,
    dest_kind: OperandKind,
    src_kind: OperandKind,
    aux_kind: OperandKind,
    pattern: Pattern,
    mnemonic: String,
}
impl Definition {
    /// Returns a new Definition.
    pub fn new(operation: Operation,
               dest: OperandKind,
               src: OperandKind,
               aux: OperandKind,
               pattern: Pattern,
               mnemonic: String)
               -> Definition {
        Definition {
            operation: operation,
            dest_kind: dest,
            src_kind: src,
            aux_kind: aux,
            pattern: pattern,
            mnemonic: mnemonic,
        }
    }
}


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
    pub fn new(def: Definition, codeword: Codeword) -> Instruction {
        let mut dest_data: usize = 0;
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
        self.operation
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
