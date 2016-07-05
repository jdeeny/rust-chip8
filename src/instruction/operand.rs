/// A kind of operand
///
/// This is used to define a kind of operand. It is not specified, so the exact register,
/// value, or address is not known.
#[derive(Copy, Clone, Debug)]
pub enum OperandKind {
    /// A register, v0-vF.
    Register,
    /// A 12-bit address.
    Address12,
    /// The I register.
    I,
    /// The memory location pointed to by I.
    IndirectI,
    /// A 12-bit literal.
    Literal12,
    /// An 8-bit literal.
    Literal8,
    /// A 4-bit literal.
    Literal4,
    /// The delay timer.
    DelayTimer,
    /// The sound timer.
    SoundTimer,
    /// A random value.
    Random,
    /// No value.
    Unused,
}

/// An operand
///
/// This is a fully specified operand, including the particular register, address, or value.
#[derive(Copy, Clone, Debug)]
pub enum Operand {
    /// A register, v0-vF.
    Register(usize),
    /// A 12-bit address.
    Address12(usize),
    /// The I register.
    I,
    /// The memory location pointed to by I.
    IndirectI,
    /// A 12-bit literal.
    Literal12(usize),
    /// An 8-bit literal.
    Literal8(usize),
    /// A 4-bit literal.
    Literal4(usize),
    /// The delay timer.
    DelayTimer,
    /// The sound timer.
    SoundTimer,
    /// A random value.
    Random,
    /// No value.
    Nowhere,
}

impl Operand {
    /// Returns a string describing the operand.
    pub fn to_string(&self) -> String {
        match *self {
            Operand::Register(r) => format!("v{:X}", r),
            Operand::Address12(a) => format!("@0x{:X}", a),
            Operand::Literal12(n) => format!("0x{:03X}", n),
            Operand::Literal8(n) => format!("0x{:02X}", n),
            Operand::Literal4(n) => format!("0x{:01X}", n),
            Operand::I => "I".to_string(),
            Operand::IndirectI => "Indirect".to_string(),
            Operand::SoundTimer => "ST".to_string(),
            Operand::DelayTimer => "DT".to_string(),
            Operand::Random => "Random".to_string(),
            Operand::Nowhere => "none".to_string(),
        }

    }
}
