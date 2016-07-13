

/// A kind of source operand.
///
/// This is used to define a kind of source operand. It is not specified, so the exact register,
/// value, or address is not known.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SrcKind {
    /// A constant value.
    Const(usize),
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
    /// The program counter.
    PC,
}

impl SrcKind {
    /// Specify the operand with the supplied data.
    ///
    /// For example, a Register will change from an SrcKind::Register with no associated
    /// data into an Src::Register(n) where n is the specific register number. The given
    /// data is also used for Addresses and Literals. Other kinds of operands cannot be further
    /// specified.
    pub fn specify(&self, data: usize) -> Src {
        match *self {
            SrcKind::Const(n) => Src::Const(n),
            SrcKind::Register => Src::Register(data),
            SrcKind::I => Src::I,
            SrcKind::Address12 => Src::Address12(data),
            SrcKind::IndirectI => Src::IndirectI,
            SrcKind::Literal12 => Src::Literal12(data),
            SrcKind::Literal8 => Src::Literal8(data),
            SrcKind::Literal4 => Src::Literal4(data),
            SrcKind::DelayTimer => Src::DelayTimer,
            SrcKind::SoundTimer => Src::SoundTimer,
            SrcKind::Random => Src::Random,
            SrcKind::PC => Src::PC,
        }
    }
}



/// An operand.
///
/// This is a fully specified operand, including the particular register, address, or value.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Src {
    /// A constant value from the instruction definition.
    Const(usize),
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
    /// The program counter.
    PC,
}

impl Src {
    pub fn kind(&self) -> SrcKind {
        match *self {
            Src::Const(n) => SrcKind::Const(n),
            Src::Register(_) => SrcKind::Register,
            Src::I => SrcKind::I,
            Src::Address12(_) => SrcKind::Address12,
            Src::IndirectI => SrcKind::IndirectI,
            Src::Literal12(_) => SrcKind::Literal12,
            Src::Literal8(_) => SrcKind::Literal8,
            Src::Literal4(_) => SrcKind::Literal4,
            Src::DelayTimer => SrcKind::DelayTimer,
            Src::SoundTimer => SrcKind::SoundTimer,
            Src::Random => SrcKind::Random,
            Src::PC => SrcKind::PC,
        }
    }


}
