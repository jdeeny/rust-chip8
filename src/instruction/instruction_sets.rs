use instruction::{Definition, SrcKind, DestKind};
use instruction::Coding::*;
use instruction::OperationKind::*;
use fonts;

// Possible instruction set extensions to add:
//   http://www.mattmik.com/files/chip8/extensions/CHIP8ExtensionsReference.pdf
// Unlikely:
//   Chip-8C (limited info), Chip-8I (adds hardware IO), Chip-8 II (more hardware IO, for ASCII keyboard), Chip-8III (capability of Chip-8I&II but
//   maintains compatibility with original Chip8), Chip-8E (14 new instructions and hardware IO), Chip-8Y (hardware IO and compatibility)
// Might just be machine config changes
//   Chip-10 (Expanded resolution of 128x64)
//   Hi-Res Chip-8 (128x64 resolution, faster),
// Interesting:
//   MegaChip8 - Adds color
//   Chip-8X - Big update from RCA with color, sound, 2nd keypad
//   Chip-8M (Add morse code; http://www.mattmik.com/files/viper/Volume4Issue05.pdf)
// Already added:
//   Original Chip8, SuperChip, XOChip

const A1:usize = 1;
const A2:usize = 2;
const A3:usize = 4;

pub const CHIP8: &'static [Definition] = &[
    Definition { pattern: [C(0x0), C(0x0),   C(0x0),   C(0x0)], op: NoOp },
    Definition { pattern: [C(0x0), C(0x0),   C(0xE),   C(0x0)], op: Cls },
    Definition { pattern: [C(0x0), C(0x0),   C(0xE),   C(0xE)], op: Ret },
    Definition { pattern: [C(0x1), A(A1),    A(A1),    A(A1) ], op: Jump(SrcKind::Address12) },
    Definition { pattern: [C(0x2), A(A1),    A(A1),    A(A1) ], op: Call(SrcKind::Address12) },
    Definition { pattern: [C(0x3), A(A1),    A(A2),    A(A2) ], op: SkipEq(SrcKind::Register, SrcKind::Literal8) },
    Definition { pattern: [C(0x4), A(A1),    A(A2),    A(A3) ], op: SkipNotEq(SrcKind::Register, SrcKind::Literal8) },
    Definition { pattern: [C(0x5), A(A1),    A(A2),    C(0x0)], op: SkipEq(SrcKind::Register, SrcKind::Register) },
    Definition { pattern: [C(0x6), A(A1),    A(A2),    A(A2) ], op: Load(DestKind::Register, SrcKind::Literal8) },
    Definition { pattern: [C(0x7), A(A1|A2), A(A3),    A(A3) ], op: Add(DestKind::Register, SrcKind::Register, SrcKind::Literal8) },
    Definition { pattern: [C(0x8), A(A1),    A(A2),    C(0x0)], op: Load(DestKind::Register, SrcKind::Register) },
    Definition { pattern: [C(0x8), A(A1|A2), A(A3),    C(0x1)], op: Or(DestKind::Register, SrcKind::Register, SrcKind::Register) },
    Definition { pattern: [C(0x8), A(A1|A2), A(A3),    C(0x2)], op: And(DestKind::Register, SrcKind::Register, SrcKind::Register) },
    Definition { pattern: [C(0x8), A(A1|A2), A(A3),    C(0x3)], op: Xor(DestKind::Register, SrcKind::Register, SrcKind::Register) },
    Definition { pattern: [C(0x8), A(A1|A3), A(A2),    C(0x4)], op: Add(DestKind::Register, SrcKind::Register, SrcKind::Register) },
    Definition { pattern: [C(0x8), A(A1|A2), A(A3),    C(0x5)], op: Sub(DestKind::Register, SrcKind::Register, SrcKind::Register) },
    Definition { pattern: [C(0x8), A(A1),    A(A2),    C(0x6)], op: Shr(DestKind::Register, SrcKind::Register) },
    Definition { pattern: [C(0x8), A(A1|A3), A(A2),    C(0x7)], op: Sub(DestKind::Register, SrcKind::Register, SrcKind::Register) },
    Definition { pattern: [C(0x8), A(A1),    A(A2),    C(0xE)], op: Shl(DestKind::Register, SrcKind::Register) },
    Definition { pattern: [C(0x9), A(A1),    A(A2),    C(0x0)], op: SkipNotEq(SrcKind::Register, SrcKind::Register) },
    Definition { pattern: [C(0xA), A(A2),    A(A2),    A(A2) ], op: Load(DestKind::I, SrcKind::Literal12 ) },
    Definition { pattern: [C(0xB), A(A1),    A(A1),    A(A1) ], op: JumpV0(SrcKind::Address12 ) },
    Definition { pattern: [C(0xC), A(A1),    A(A3),    A(A3) ], op: Rand(DestKind::Register, SrcKind::Random, SrcKind::Literal8) },
    Definition { pattern: [C(0xD), A(A1),    A(A2),    A(A3) ], op: Sprite(SrcKind::Register, SrcKind::Register, SrcKind::Literal4) },
    Definition { pattern: [C(0xE), A(A1),    C(0x9),   C(0xE)], op: SkipKey(SrcKind::Register) },
    Definition { pattern: [C(0xE), A(A1),    C(0xA),   C(0x1)], op: SkipNotKey(SrcKind::Register) },
    Definition { pattern: [C(0xF), A(A1),    C(0x0),   C(0x7)], op: Load(DestKind::Register, SrcKind::DelayTimer) },
    Definition { pattern: [C(0xF), A(A1),    C(0x0),   C(0xA)], op: WaitKey(DestKind::Register, SrcKind::Literal4) },
    Definition { pattern: [C(0xF), A(A2),    C(0x1),   C(0x5)], op: Load(DestKind::DelayTimer, SrcKind::Register) },
    Definition { pattern: [C(0xF), A(A2),    C(0x1),   C(0x8)], op: Load(DestKind::SoundTimer, SrcKind::Register) },
    Definition { pattern: [C(0xF), A(A3),    C(0x1),   C(0xE)], op: Add(DestKind::I, SrcKind::I, SrcKind::Register) },
    Definition { pattern: [C(0xF), A(A1),    C(0x2),   C(0x9)], op: Font(SrcKind::Register, SrcKind::Const(fonts::CODE_SMALL)) },
    Definition { pattern: [C(0xF), A(A1),    C(0x3),   C(0x3)], op: Bcd(SrcKind::Register) },
    Definition { pattern: [C(0xF), A(A2),    C(0x5),   C(0x5)], op: Stash(SrcKind::Const(0), SrcKind::Register, SrcKind::Const(1)) },
    Definition { pattern: [C(0xF), A(A2),    C(0x6),   C(0x5)], op: Fetch(SrcKind::Const(0), SrcKind::Register, SrcKind::Const(1)) },
];

pub const SUPERCHIP: &'static [Definition] = &[
    Definition { pattern: [C(0x0), C(0x0),   C(0xC),   A(A1)],  op: NoOp /*ScrollDown(SrcKind::Literal4)*/ },
    Definition { pattern: [C(0x0), C(0x0),   C(0xF),   C(0xB)], op: NoOp /*ScrollRight*/ },
    Definition { pattern: [C(0x0), C(0x0),   C(0xF),   C(0xC)], op: NoOp /*ScrollLeft*/ },
    Definition { pattern: [C(0x0), C(0x0),   C(0xF),   C(0xD)], op: NoOp /*Exit*/ },
    Definition { pattern: [C(0x0), C(0x0),   C(0xF),   C(0xE)], op: NoOp /*LowRes*/ },
    Definition { pattern: [C(0x0), C(0x0),   C(0xF),   C(0xF)], op: NoOp /*HighRes*/ },
    Definition { pattern: [C(0xD), A(A1),    C(0x3),   C(0x0)], op: Font(SrcKind::Register, SrcKind::Const(fonts::CODE_BIG)) },
    Definition { pattern: [C(0xF), A(A1),    C(0x7),   C(0x5)], op: NoOp /*SaveFlags(SrcKind::Register)*/},
    Definition { pattern: [C(0xF), A(A1),    C(0x8),   C(0x5)], op: NoOp /*LoadFlags(SrcKind::Register)*/},
];

pub const XOCHIP: &'static [Definition] = &[
    Definition { pattern: [C(0x5), A(A1),   A(A2),     C(0x2)], op: Stash(SrcKind::Register, SrcKind::Register, SrcKind::Const(0)) },
    Definition { pattern: [C(0x5), A(A1),   A(A2),     C(0x3)], op: Fetch(SrcKind::Register, SrcKind::Register, SrcKind::Const(0)) },
    Definition { pattern: [C(0xF), C(0x0),  C(0x0),    C(0x0)], op: NoOp /*LoadI16*/ },
    Definition { pattern: [C(0xF), A(A1),   C(0x0),    C(0x1)], op: NoOp /*SelectDrawPlane(SrcKind::Literal4)*/ },
    Definition { pattern: [C(0xF), A(A1),   C(0x0),    C(0x2)], op: NoOp /*StoreAudio*/ },
    Definition { pattern: [C(0x0), C(0x0),  C(0xD),    A(A1)],  op: NoOp /*ScrollUp(SrcKind::Literal4)*/ },
];
