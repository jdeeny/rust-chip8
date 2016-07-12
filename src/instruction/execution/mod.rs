//! Enumerates the types of operations and their operands.
//!
//! An `OperationKind` can be converted to an Operation by

use std::boxed::FnBox;

mod execute;
mod implementations;

pub use self::execute::Execute;

use types::*;
use instruction::{Dest, Src, DestKind, SrcKind };
// use self::implementations::*;

#[derive(Copy,Clone,Eq,PartialEq)]
pub enum Operation {
    NoOp,
    Load(Dest, Src),
    Stash(Src),
    Fetch(Src),

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
    Font(Src),
    Bcd(Src),
    WaitKey(Src),
}

impl Operation {
    pub fn kind(&self) -> OperationKind {
        match *self {
            Operation::NoOp => OperationKind::NoOp,
            _ => panic!("cannot specify"),
        }
    }

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
            Operation::Font(glyph)              => { implementations::font(exec, glyph) },
            Operation::Bcd(value)               => { implementations::bcd(exec, value) },
            Operation::SkipEq(lhs, rhs)         => { implementations::skip_eq(exec, lhs, rhs) },
            Operation::SkipNotEq(lhs, rhs)      => { implementations::skip_not_eq(exec, lhs, rhs) },
            Operation::SkipKey(key)             => { implementations::skip_key_pressed(exec, key) },
            Operation::SkipNotKey(key)          => { implementations::skip_key_not_pressed(exec, key) },
            Operation::WaitKey(key)             => { implementations::wait_key(exec, key) },
            Operation::Cls                      => { implementations::clear_screen(exec) },
            Operation::Stash(last)              => { implementations::stash(exec, last) },
            Operation::Fetch(last)              => { implementations::fetch(exec, last) },
            Operation::Rand(dest, src, mask)    => { implementations::random(exec, dest, src, mask) },
            Operation::Sprite(x, y, n)          => { implementations::sprite(exec, x, y, n) },
        }
    }
}

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

    SkipEq(SrcKind, SrcKind),
    SkipNotEq(SrcKind, SrcKind),
    SkipKey(SrcKind),
    SkipNotKey(SrcKind),

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
    Font(SrcKind),
    Bcd(SrcKind),
    WaitKey(SrcKind),
}
