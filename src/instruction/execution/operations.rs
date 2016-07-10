use instruction::{Dest, DestKind, Src, SrcKind};

#[derive(Copy,Clone,Eq,PartialEq)]
pub enum OperationKind {
    NoOp,
    Load(DestKind, SrcKind),
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
    Sprite(SrcKind, SrcKind, SrcKind),
    Font,
    Bcd,
}



#[derive(Copy,Clone,Eq,PartialEq)]
pub enum Operation {
    NoOp,
    Load(Dest, Src),
    Stash(Src),
    Fetch(Dest),

    Jump(Src),
    JumpV0(Src),
    Call(Src),
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
    Sprite(Src, Src, Dest),
    Font,
    Bcd,
}

impl Operation {
    pub fn kind(&self) -> OperationKind {
        match *self {
            Operation::NoOp => OperationKind::NoOp,
            _ => panic!("cannot specify"),
        }
    }
}
