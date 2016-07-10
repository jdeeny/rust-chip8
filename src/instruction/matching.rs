use types::*;
use instruction::{Instruction,Pattern, Coding, Definition, SrcKind, DestKind};
use instruction::execution::OperationKind;



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
            if let Coding::L(n) = *coding {
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
    op_kind: OperationKind,
    pattern: Pattern,
}

impl InstructionMatcher {
    pub fn new(definition: &Definition) -> InstructionMatcher {
        InstructionMatcher {
            op_kind: definition.op,
            pattern: definition.pattern,
        }
    }

    pub fn is_match(&self, inst: &Instruction) -> bool {
        self.op_kind == inst.operation().kind()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use instruction::{Definition, Instruction, Operand, SrcKind, Operation};
    use instruction::Coding::*;

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
            operation: Operation::Add,
            dest_kind: SrcKind::Register,
            src_kind: SrcKind::Register,
            aux_kind: SrcKind::Unused,
            pattern: [C(0x2), D, S, C(0)],
        };

        let im = InstructionMatcher::new(&definition);

        let d2 = Definition {
            operation: Operation::Sub,
            dest_kind: SrcKind::Register,
            src_kind: SrcKind::Register,
            aux_kind: SrcKind::Unused,
            pattern: [C(0x2), D, S, C(0)],
        };

        let d3 = Definition {
            operation: Operation::Add,
            dest_kind: SrcKind::Register,
            src_kind: SrcKind::Register,
            aux_kind: SrcKind::Unused,
            pattern: [C(0x2), D, S, C(0xF)],
        };

        let inst = Instruction::from_definition(&definition, 0x2740);
        assert!( im.is_match(&inst) );
        let inst = Instruction::from_definition(&definition, 0x2380);
        assert!( im.is_match(&inst) );
        let inst = Instruction::from_definition(&definition, 0x20D0);
        assert!( im.is_match(&inst) );
        let inst = Instruction::from_definition(&d2, 0x2340);
        assert!( ! im.is_match(&inst) );
        let inst = Instruction::from_definition(&d3, 0x2340);
        //matches because the pattern and codeword are not taken into account.
        assert!( im.is_match(&inst) );


    }

}
