use instruction::{SrcKind,DestKind,Src,Dest};

#[derive(Copy,Clone,Eq,PartialEq)]
pub enum OperationKind {
    NoOp,
    Load(SrcKind),
    Store(DestKind),
    Stash(SrcKind),
    Fetch(DestKind),

    Jump(DestKind),
    JumpV0(DestKind),
    Call(DestKind),
    Ret,

    SkipEq(SrcKind),
    SkipNeq(SrcKind),

    Add(SrcKind, SrcKind, DestKind),
    Sub(DestKind, SrcKind),
    Subn(DestKind, SrcKind),

    Or(DestKind, SrcKind),
    And(DestKind, SrcKind),
    Xor(DestKind, SrcKind),
    Shr(DestKind, SrcKind),
    Shl(DestKind, SrcKind),

    Rand(DestKind),

    Cls,
    Sprite,
    Font,
    Bcd,
}

impl OperationKind {
    pub fn specify(&self, data: usize) -> Operation {
        match *self {
            OperationKind::NoOp => Operation::NoOp,
            _ => panic!("cannot specify")
        }
    }
}

#[derive(Copy,Clone,Eq,PartialEq)]
pub enum Operation {
    NoOp,
    Load(Src),
    Store(Dest),
    Stash(Src),
    Fetch(Dest),

    Jump(Dest),
    JumpV0(Dest),
    Call(Dest),
    Ret,

    SkipEq(Src),
    SkipNeq(Src),

    Add(Src, Src, Dest),
    Sub(Dest, Src),
    Subn(Dest, Src),

    Or(Dest, Src),
    And(Dest, Src),
    Xor(Dest, Src),
    Shr(Dest, Src),
    Shl(Dest, Src),

    Rand(DestKind),

    Cls,
    Sprite,
    Font,
    Bcd,
}

impl Operation {
    pub fn kind(&self) -> OperationKind {
        match *self {
            Operation::NoOp => OperationKind::NoOp,
            _ => panic!("cannot specify")
        }
    }
}
