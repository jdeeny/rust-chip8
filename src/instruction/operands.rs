/// A kind of operand.
///
/// This is used to define a kind of operand. It is not specified, so the exact register,
/// value, or address is not known.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

pub trait Operand {
    fn kind(&self) -> OperandKind;
}

impl OperandKind {
    /// Specify the operand with the supplied data.
    ///
    /// For example, a Register will change from an OperandKind::Register with no associated
    /// data into an Operand::Register(n) where n is the specific register number. The given
    /// data is also used for Addresses and Literals. Other kinds of operands cannot be further
    /// specified.
    pub fn dest(&self, data: usize) -> Operand {
        match *self {
            OperandKind::Register => Operand::Register(data),
            OperandKind::I => Operand::I,
            OperandKind::Address12 => Operand::Address12(data),
            OperandKind::IndirectI => Operand::IndirectI,
            OperandKind::Literal12 => Operand::Literal12(data),
            OperandKind::Literal8 => Operand::Literal8(data),
            OperandKind::Literal4 => Operand::Literal4(data),
            OperandKind::DelayTimer => Operand::DelayTimer,
            OperandKind::SoundTimer => Operand::SoundTimer,
            OperandKind::Random => Operand::Random,
            OperandKind::Unused => Operand::Nowhere,
        }
    }

    pub fn src(&self, data: usize) -> Operand {
        match *self {
            OperandKind::Register => Operand::Register(data),
            OperandKind::I => Operand::I,
            OperandKind::Address12 => Operand::Address12(data),
            OperandKind::IndirectI => Operand::IndirectI,
            OperandKind::Literal12 => Operand::Literal12(data),
            OperandKind::Literal8 => Operand::Literal8(data),
            OperandKind::Literal4 => Operand::Literal4(data),
            OperandKind::DelayTimer => Operand::DelayTimer,
            OperandKind::SoundTimer => Operand::SoundTimer,
            OperandKind::Random => Operand::Random,
            OperandKind::Unused => Operand::Nowhere,
        }
    }

    pub fn aux(&self, data: usize) -> Operand {
        match *self {
            OperandKind::Register => Operand::Register(data),
            OperandKind::I => Operand::I,
            OperandKind::Address12 => Operand::Address12(data),
            OperandKind::IndirectI => Operand::IndirectI,
            OperandKind::Literal12 => Operand::Literal12(data),
            OperandKind::Literal8 => Operand::Literal8(data),
            OperandKind::Literal4 => Operand::Literal4(data),
            OperandKind::DelayTimer => Operand::DelayTimer,
            OperandKind::SoundTimer => Operand::SoundTimer,
            OperandKind::Random => Operand::Random,
            OperandKind::Unused => Operand::Nowhere,
        }
    }
}

/// An operand.
///
/// This is a fully specified operand, including the particular register, address, or value.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Dest {
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

impl Operand for Dest {
    fn kind(&self) -> Self {
        match *self {
            Operand::Register(_) => OperandKind::Register,
            Operand::I => OperandKind::I,
            Operand::Address12(_) => OperandKind::Address12,
            Operand::IndirectI => OperandKind::IndirectI,
            Operand::Literal12(_) => OperandKind::Literal12,
            Operand::Literal8(_) => OperandKind::Literal8,
            Operand::Literal4(_) => OperandKind::Literal4,
            Operand::DelayTimer => OperandKind::DelayTimer,
            Operand::SoundTimer => OperandKind::SoundTimer,
            Operand::Random => OperandKind::Random,
            Operand::Nowhere => OperandKind::Unused,
        }
    }

}

/// An operand.
///
/// This is a fully specified operand, including the particular register, address, or value.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Src {
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

impl Operand for Src {
    fn kind(&self) -> Self {
        match *self {
            Operand::Register(_) => OperandKind::Register,
            Operand::I => OperandKind::I,
            Operand::Address12(_) => OperandKind::Address12,
            Operand::IndirectI => OperandKind::IndirectI,
            Operand::Literal12(_) => OperandKind::Literal12,
            Operand::Literal8(_) => OperandKind::Literal8,
            Operand::Literal4(_) => OperandKind::Literal4,
            Operand::DelayTimer => OperandKind::DelayTimer,
            Operand::SoundTimer => OperandKind::SoundTimer,
            Operand::Random => OperandKind::Random,
            Operand::Nowhere => OperandKind::Unused,
        }
    }


}

/// An operand.
///
/// This is a fully specified operand, including the particular register, address, or value.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Aux {
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

impl Operand for Aux {
    fn kind(&self) -> Self {
        match *self {
            Operand::Register(_) => OperandKind::Register,
            Operand::I => OperandKind::I,
            Operand::Address12(_) => OperandKind::Address12,
            Operand::IndirectI => OperandKind::IndirectI,
            Operand::Literal12(_) => OperandKind::Literal12,
            Operand::Literal8(_) => OperandKind::Literal8,
            Operand::Literal4(_) => OperandKind::Literal4,
            Operand::DelayTimer => OperandKind::DelayTimer,
            Operand::SoundTimer => OperandKind::SoundTimer,
            Operand::Random => OperandKind::Random,
            Operand::Nowhere => OperandKind::Unused,
        }
    }


}
