use types::*;
use instruction::{Instruction,Pattern, Coding, Definition, OperandKind};
use execution::Operation;



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
pub struct InstructionMatcher {
    operation: Operation,
    dest_kind: OperandKind,
    src_kind: OperandKind,
    aux_kind: OperandKind,
    pattern: Pattern,
}

impl InstructionMatcher {
    pub fn new(definition: &Definition) -> InstructionMatcher {
        InstructionMatcher {
            operation: definition.operation,
            dest_kind: definition.dest_kind,
            src_kind: definition.src_kind,
            aux_kind: definition.aux_kind,
            pattern: definition.pattern,
        }
    }

    pub fn is_match(&self, inst: &Instruction) -> bool {
        self.operation == inst.operation() &&
        self.dest_kind == inst.dest().kind() &&
        self.src_kind == inst.src().kind() &&
        self.aux_kind == inst.aux().kind()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use instruction::{Definition, Instruction, Operand, OperandKind};
    use instruction::Coding::*;
    use execution::Operation;

    #[test]
    fn test_code_match() {
        let cm = CodewordMatcher::new([C(0xA), C(0x5), D, C(0x0)]);
        assert!(cm.is_match(0xA500));
        assert!(cm.is_match(0xA5E0));
        assert!(!cm.is_match(0xB580));

        let cm = CodewordMatcher::new([D, S, C(0xA), A]);
        assert!(!cm.is_match(0xA500));
        assert!(cm.is_match(0xA5A7));
        assert!(cm.is_match(0xB5A0));

    }

    #[test]
    fn test_inst_match() {
        let definition = Definition {
            operation: Operation::OpAdd,
            dest_kind: OperandKind::Register,
            src_kind: OperandKind::Register,
            aux_kind: OperandKind::Unused,
            pattern: [C(0x2), D, S, C(0)],
        };

        let im = InstructionMatcher::new(&definition);

        let d2 = Definition {
            operation: Operation::OpSub,
            dest_kind: OperandKind::Register,
            src_kind: OperandKind::Register,
            aux_kind: OperandKind::Unused,
            pattern: [C(0x2), D, S, C(0)],
        };

        let d3 = Definition {
            operation: Operation::OpAdd,
            dest_kind: OperandKind::Register,
            src_kind: OperandKind::Register,
            aux_kind: OperandKind::Unused,
            pattern: [C(0x2), D, S, C(0xF)],
        };

        let inst = Instruction::new(&definition, 0x2740);
        assert!( im.is_match(&inst) );
        let inst = Instruction::new(&definition, 0x2380);
        assert!( im.is_match(&inst) );
        let inst = Instruction::new(&definition, 0x20D0);
        assert!( im.is_match(&inst) );
        let inst = Instruction::new(&d2, 0x2340);
        assert!( ! im.is_match(&inst) );
        let inst = Instruction::new(&d3, 0x2340);
        //matches because the pattern and codeword are not taken into account.
        assert!( im.is_match(&inst) );


    }

}
