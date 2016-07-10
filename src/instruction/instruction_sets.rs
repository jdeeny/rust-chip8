use instruction::{Definition, SrcKind, DestKind};
use instruction::Coding::*;
use instruction::execution::OperationKind::*;

pub const CHIP8: &'static [Definition] = &[
    Definition { op: NoOp, pattern: [C(0x0), C(0x0), C(0xE), C(0x0)] },
    //Definition::new(OpCls,      Unused,     Unused,     Unused,     [C(0x0), C(0x0), C(0xE), C(0x0)], /*"Cls"*/),
    /*Definition::new(op_ret,      Unused,     Unused,     Unused,     [C(0x0), C(0x0), C(0xE), C(0xE)], "Ret"),
    Definition::new(op_jump,     Literal12,  Unused,     Unused,     [C(0x1), D,      D,      D     ], "Jump {d}"),
    Definition::new(op_call,     Literal12,  Unused,     Unused,     [C(0x2), D,      D,      D     ], "Call {d}"),
    Definition::new(op_skipeq,   Register,   Literal8,   Unused,     [C(0x3), D,      S,      S     ], "SkipEq {d}, {s}"),
    Definition::new(op_skipneq,  Register,   Literal8,   Unused,     [C(0x4), D,      S,      S     ], "SkipNeq {d}, {s}"),
    Definition::new(op_skipeq,   Register,   Register,   Unused,     [C(0x5), D,      S,      C(0x0)], "SkipEq {d}, {s}"),
    Definition::new(op_load,     Register,   Literal8,   Unused,     [C(0x6), D,      S,      S     ], "Load {d}, {s}"),
    Definition::new(op_add,      Register,   Literal8,   Unused,     [C(0x7), D,      S,      S     ], "Add {d}, {s}"),
    Definition::new(op_load,     Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x0)], "Load {d}, {s}"),
    Definition::new(op_or,       Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x1)], "Or {d}, {s}"),
    Definition::new(op_and,      Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x2)], "And {d}, {s}"),
    Definition::new(op_xor,      Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x3)], "Xor {d}, {s}"),
    Definition::new(op_add,      Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x4)], "Add {d}, {s}"),
    Definition::new(op_sub,      Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x5)], "Sub {d}, {s}"),
    Definition::new(op_shr,      Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x6)], "ShR {d}, {s}"),
    Definition::new(op_subn,     Register,   Register,   Unused,     [C(0x8), D,      S,      C(0x7)], "SubN {d}, {s}"),
    Definition::new(op_shl,      Register,   Register,   Unused,     [C(0x8), D,      S,      C(0xE)], "ShL {d}, {s}"),
    Definition::new(op_skipneq,  Register,   Register,   Unused,     [C(0x9), D,      S,      C(0x0)], "SkipNeq {d}, {s}"),
    Definition::new(op_load,     I,          Literal12,  Unused,     [C(0xA), S,      S,      S     ], "Load {d}, {s}"),
    Definition::new(op_jumpv0,   Literal12,  Unused,     Unused,     [C(0xB), D,      D,      D     ], "JumpV0 {d}"),
    Definition::new(op_rand,     Register,   Literal8,   Random,     [C(0xC), D,      S,      S     ], "Rand {d}, {s}"),
    Definition::new(op_sprite,   Register,   Register,   Literal4,   [C(0xD), D,      S,      A     ], "Sprite {d}, {s}, {a}"),
    Definition::new(op_skipkey,  Register,   Unused,     Unused,     [C(0xE), D,      C(0x9), C(0xE)], "SkipKey {d}"),
    Definition::new(op_skipnkey, Register,   Unused,     Unused,     [C(0xE), D,      C(0xA), C(0x1)], "SkipNKey {d}"),
    Definition::new(op_load,     Register,   DelayTimer, Unused,     [C(0xF), D,      C(0x0), C(0x7)], "Load {d}, {s}"),
    Definition::new(op_waitkey,  Register,   Unused,     Unused,     [C(0xF), D,      C(0x0), C(0xA)], "WaitKey {d}"),
    Definition::new(op_load,     DelayTimer, Register,   Unused,     [C(0xF), S,      C(0x1), C(0x5)], "Load {d}, {s}"),
    Definition::new(op_load,     SoundTimer, Register,   Unused,     [C(0xF), S,      C(0x1), C(0x8)], "Load {d}, {s}"),
    Definition::new(op_add,      I,          Register,   Unused,     [C(0xF), S,      C(0x1), C(0xE)], "Add {d}, {s}"),
    Definition::new(op_font,     I,          Register,   Unused,     [C(0xF), S,      C(0x2), C(0x9)], "Font {d}, {s}"),
    Definition::new(op_bcd,      IndirectI,  Register,   Unused,     [C(0xF), S,      C(0x3), C(0x3)], "BCD {d}, {s}"),
    Definition::new(op_stash,    IndirectI,  Register,   Unused,     [C(0xF), S,      C(0x5), C(0x5)], "Stash {s}"),*/
    //Definition::new(op_fetch,    Register,   IndirectI,  Unused,     [C(0xF), D,      C(0x6), C(0x5)], "Fetch {d}"),
];

pub const SUPERCHIP: &'static [Definition] = &[];

pub const XOCHIP: &'static [Definition] = &[];
