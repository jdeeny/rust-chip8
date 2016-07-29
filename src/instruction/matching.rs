use types::*;
use instruction::{Coding, Definition, Pattern};
use instruction::{Operation, OperationKind};


#[derive(Debug)]
pub struct CodewordMatcher {
    code: Codeword,
    mask: Codeword,
}

/// Matches a codeword
impl CodewordMatcher {
    pub fn new(pattern: Pattern) -> CodewordMatcher {
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

/// Matches an instruction
#[derive(Debug)]
pub struct InstructionMatcher {
    op_kind: OperationKind,
}

impl InstructionMatcher {
    pub fn new(definition: &Definition) -> InstructionMatcher {
        InstructionMatcher { op_kind: definition.op }
    }

    #[allow(dead_code)]
    pub fn is_match(&self, operation: &Operation) -> bool {
        self.op_kind == operation.kind()
    }
}
