use types::*;
use instruction::{Pattern, Coding, Definition};
pub struct CodewordMatcher {
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

pub struct InstructionMatcher {
        definition: Definition,
        matcher: CodewordMatcher,
}
impl InstructionMatcher {
    pub fn is_match(&self, codeword: Codeword) -> bool {
        self.matcher.is_match(codeword)
    }
}
