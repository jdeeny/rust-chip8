
/// A kind of destination location.
///
/// This is used to define a kind of destination, but it is not specified exactly. So, it
/// might be any register, or any address. It can then be specified by supplying data, to
/// create a `Dest.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DestKind {
    /// A register, v0-vF.
    Register,
    /// A 12-bit address.
    Address12,
    /// The I register.
    I,
    /// The memory location pointed to by I.
    IndirectI,
    /// The delay timer.
    DelayTimer,
    /// The sound timer.
    SoundTimer,
    /// Program Counter
    PC,
}

impl DestKind {
    /// Specify the destination with the supplied data.
    ///
    /// For example, a Register will change from a DestKind::Register with no associated
    /// data into a Dest::Register(n) where n is the specific register number. The given
    /// data is also used for Addresses and Literals. Other kinds of destinations, such
    /// as I or the SoundTimer, cannot be further specified.
    pub fn specify(&self, data: usize) -> Dest {
        match *self {
            DestKind::Register => Dest::Register(data),
            DestKind::I => Dest::I,
            DestKind::Address12 => Dest::Address12(data),
            DestKind::IndirectI => Dest::IndirectI,
            DestKind::DelayTimer => Dest::DelayTimer,
            DestKind::SoundTimer => Dest::SoundTimer,
            DestKind::PC => Dest::PC,
            // DestKind::Unused => Dest::Nowhere,
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
    /// The delay timer.
    DelayTimer,
    /// The sound timer.
    SoundTimer,
    /// The program counter.
    PC,
}

impl Dest {
    /// Returns the kind of Dest.
    pub fn kind(&self) -> DestKind {
        match *self {
            Dest::Register(_) => DestKind::Register,
            Dest::I => DestKind::I,
            Dest::Address12(_) => DestKind::Address12,
            Dest::IndirectI => DestKind::IndirectI,
            Dest::DelayTimer => DestKind::DelayTimer,
            Dest::SoundTimer => DestKind::SoundTimer,
            Dest::PC => DestKind::PC,
        }
    }
}
