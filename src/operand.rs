#[derive(Copy, Clone)]
pub enum OperandKind {
  Register,
  Address12,
  I,
  IndirectI,
  Literal12,
  Literal8,
  Literal4,
  DelayTimer,
  SoundTimer,
  Unused,
}

#[derive(Copy, Clone)]
pub enum Operand {
    Register(usize),
    Address12(usize),
    I,
    IndirectI,
    Literal12(usize),
    Literal8(usize),
    Literal4(usize),
    DelayTimer,
    SoundTimer,
    Nowhere
}

impl Operand {
    pub fn to_string(&self) -> String {
        match *self {
            Operand::Register(r)         => format!("v{:X}", r),
            Operand::Address12(a)        => format!("@0x{:X}", a),
            Operand::Literal12(n)        => format!("0x{:03X}", n),
            Operand::Literal8(n)         => format!("0x{:02X}", n),
            Operand::Literal4(n)         => format!("0x{:01X}", n),
            Operand::I                   => format!("I"),
            Operand::IndirectI           => format!("Indirect"),
            Operand::SoundTimer          => format!("ST"),
            Operand::DelayTimer          => format!("DT"),
            Operand::Nowhere             => format!("none"),
        }

    }
}
