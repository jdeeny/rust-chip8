//! Enumerates the types of operations and their operands.
//!
//! An `OperationKind` can be converted to an Operation by

use std::boxed::FnBox;
use types::*;
use instruction::{Dest, Src, DestKind, SrcKind };

mod implementations;

#[derive(Copy,Clone,Eq,PartialEq)]
pub enum OperationKind {
    NoOp,
    Load(DestKind, SrcKind),
    Stash(SrcKind, SrcKind, SrcKind), //third argument is a flag, 0 = do not increment I, 1 increment I
    Fetch(SrcKind, SrcKind, SrcKind), //third argument is a flag, 0 = do not increment I, 1 increment I
    Jump(SrcKind),
    JumpV0(SrcKind),
    Call(SrcKind),
    Ret,
    SkipEq(SrcKind, SrcKind),
    SkipNotEq(SrcKind, SrcKind),
    SkipKey(SrcKind),
    SkipNotKey(SrcKind),
    Add(DestKind, SrcKind, SrcKind),
    Sub(DestKind, SrcKind, SrcKind),
    Or(DestKind, SrcKind, SrcKind),
    And(DestKind, SrcKind, SrcKind),
    Xor(DestKind, SrcKind, SrcKind),
    Shr(DestKind, SrcKind),
    Shl(DestKind, SrcKind),
    Rand(DestKind, SrcKind, SrcKind),
    Cls,
    Sprite(SrcKind, SrcKind, SrcKind),
    Font(SrcKind, SrcKind),     // Glyph number, font number - 0 is small font, 1 is big font
    Bcd(SrcKind),
    WaitKey(DestKind, SrcKind),
}


#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub enum Operation {
    NoOp,
    Load(Dest, Src),
    Stash(Src, Src, Src),
    Fetch(Src, Src, Src),

    Jump(Src),
    JumpV0(Src),
    Call(Src),
    Ret,

    SkipEq(Src, Src),
    SkipNotEq(Src, Src),
    SkipKey(Src),
    SkipNotKey(Src),

    Add(Dest, Src, Src),
    Sub(Dest, Src, Src),

    Or(Dest, Src, Src),
    And(Dest, Src, Src),
    Xor(Dest, Src, Src),
    Shr(Dest, Src),
    Shl(Dest, Src),

    Rand(Dest, Src, Src),

    Cls,
    Sprite(Src, Src, Src),
    Font(Src, Src),
    Bcd(Src),
    WaitKey(Dest, Src),
}

impl Operation {
    ///
    pub fn kind(&self) -> OperationKind {
        match *self {
            Operation::NoOp => OperationKind::NoOp,
            Operation::Load(d, s) => OperationKind::Load(d.kind(), s.kind()),

            Operation::Stash(first, last, flag) => OperationKind::Stash(first.kind(), last.kind(), flag.kind()),
            Operation::Fetch(first, last, flag) => OperationKind::Fetch(first.kind(), last.kind(), flag.kind()),

            Operation::Jump(d) => OperationKind::Jump(d.kind()),
            Operation::JumpV0(d) => OperationKind::JumpV0(d.kind()),

            Operation::Call(d) => OperationKind::Call(d.kind()),
            Operation::Ret => OperationKind::Ret,

            Operation::SkipEq(a, b) => OperationKind::SkipEq(a.kind(), b.kind()),
            Operation::SkipNotEq(a, b) => OperationKind::SkipNotEq(a.kind(), b.kind()),
            Operation::SkipKey(n) => OperationKind::SkipKey(n.kind()),
            Operation::SkipNotKey(n) => OperationKind::SkipNotKey(n.kind()),

            Operation::Add(d, a, b) => OperationKind::Add(d.kind(), a.kind(), b.kind()),
            Operation::Sub(d, a, b) => OperationKind::Sub(d.kind(), a.kind(), b.kind()),

            Operation::Or(d, a, b) => OperationKind::Or(d.kind(), a.kind(), b.kind()),
            Operation::And(d, a, b) => OperationKind::And(d.kind(), a.kind(), b.kind()),
            Operation::Xor(d, a, b) => OperationKind::Xor(d.kind(), a.kind(), b.kind()),
            Operation::Shr(d, s) => OperationKind::Shr(d.kind(), s.kind()),
            Operation::Shl(d, s) => OperationKind::Shl(d.kind(), s.kind()),

            Operation::Rand(d, s, m) => OperationKind::Rand(d.kind(), s.kind(), m.kind()),

            Operation::Cls => OperationKind::Cls,

            Operation::Sprite(x, y, n) => OperationKind::Sprite(x.kind(), y.kind(), n.kind()),
            Operation::Font(s, c) => OperationKind::Font(s.kind(), c.kind()),
            Operation::Bcd(s) => OperationKind::Bcd(s.kind()),

            Operation::WaitKey(d, n) => OperationKind::WaitKey(d.kind(), n.kind()),
        }
    }

    /// Execute the operation on exec.
    pub fn execute(&self, exec: &mut Execute) -> Chip8Result<()> {
        match *self {
            Operation::NoOp                     => { Ok(()) },
            Operation::Load(dest, src)          => { implementations::load(exec, dest, src) },
            Operation::Add(dest, lhs, rhs)      => { implementations::add(exec, dest, lhs, rhs) },
            Operation::Sub(dest, lhs, rhs)      => { implementations::sub(exec, dest, lhs, rhs) },
            Operation::Jump(addr)               => { implementations::jump(exec, addr) },
            Operation::JumpV0(addr)             => { implementations::jump_v0(exec, addr) },
            Operation::Call(addr)               => { implementations::call(exec, addr) },
            Operation::Ret                      => { implementations::ret(exec) },
            Operation::Or(dest, lhs, rhs)       => { implementations::or(exec, dest, lhs, rhs) },
            Operation::And(dest, lhs, rhs)      => { implementations::and(exec, dest, lhs, rhs) },
            Operation::Xor(dest, lhs, rhs)      => { implementations::xor(exec, dest, lhs, rhs) },
            Operation::Shr(dest, src)           => { implementations::shr(exec, dest, src) },
            Operation::Shl(dest, src)           => { implementations::shl(exec, dest, src) },
            Operation::Font(glyph, font)        => { implementations::font(exec, glyph, font) },
            Operation::Bcd(value)               => { implementations::bcd(exec, value) },
            Operation::SkipEq(lhs, rhs)         => { implementations::skip_eq(exec, lhs, rhs) },
            Operation::SkipNotEq(lhs, rhs)      => { implementations::skip_not_eq(exec, lhs, rhs) },
            Operation::SkipKey(key)             => { implementations::skip_key_pressed(exec, key) },
            Operation::SkipNotKey(key)          => { implementations::skip_key_not_pressed(exec, key) },
            Operation::WaitKey(dest, key)       => { implementations::wait_key(exec, dest, key) },
            Operation::Cls                      => { implementations::clear_screen(exec) },
            Operation::Stash(first, last, flag) => { implementations::stash(exec, first, last, flag) },
            Operation::Fetch(first, last, flag) => { implementations::fetch(exec, first, last, flag) },
            Operation::Rand(dest, src, mask)    => { implementations::random(exec, dest, src, mask) },
            Operation::Sprite(x, y, n)          => { implementations::sprite(exec, x, y, n) },
        }
    }
}
