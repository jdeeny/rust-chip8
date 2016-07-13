//! Defines the state of the Chip8 virtual machine.
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::panic;
use std::iter::{Iterator, FromIterator, repeat};
use rand::{Rng, thread_rng, ThreadRng};
pub use types::*;
use config::Config;
use instruction::{Execute, Dest, Src};


pub type ByteIter<'a> =  &'a mut Iterator<Item=MemoryCell>;

#[derive(Debug, Clone)]
pub struct Locked<T>(pub Arc<RwLock<T>>);

impl<T> Locked<T> {
    pub fn new(t: T) -> Locked<T> {
        Locked(Arc::new(RwLock::new(t)))
    }
    pub fn try_read(&self) -> RwLockReadGuard<T> {
        self.0.try_read().unwrap()
    }
    pub fn try_write(&mut self) -> RwLockWriteGuard<T> {
        self.0.try_write().unwrap()
    }
}

/// A struct that contains a Chip8 `Config` and the machine state.
///
/// The machine state includes the RAM, registers, program counter, stack, timers, and the
/// state of the IO subsystems.
#[allow(dead_code)]
pub struct Chip8<'a> {
    /// Sets the configuration of the machine.
    pub config: Config,
    /// The ram.
    pub ram: Vec<MemoryCell>,
    /// The general purpose registers, v0-vF.
    pub v: [Register8; 16],
    /// The I register.
    pub i: Register16,
    /// The program counter.
    pub pc: Address,
    /// The call stack.
    pub stack: Vec<Address>,
    /// The sound timer.
    pub st: Timer,
    /// The delay timer.
    pub dt: Timer,
    /// The video ram, containing the state of the video output.
    pub vram: Locked<Vram>,
    /// The state of the keyboard.
    pub keys: Locked<Keyboard>,
    /// The state of the chip8 buzzer.
    pub buzzer: Locked<Buzzer>,
    /// The state of the audio buffer used with XOCHIP.
    pub audio: Locked<Audio>,
    ///
    pub random: Option<ByteIter<'a>>,
    thread_rng: ThreadRng,
}

pub trait StateControl {
    /// Reset the state of the machine.
    fn reset(&mut self);
    /// Execute the current instruction.
    fn step(&mut self) -> Result<(),()>;
}

impl<'a> Chip8<'a> {
    /// Create a new Chip8 using the supplied Config.
    pub fn new(config: &Config, rand_iterator: Option<&'a mut Iterator<Item=MemoryCell>>) -> Chip8<'a> {
        Chip8 {
            config: *config,
            ram: Vec::from_iter(repeat(0).take(config.ram_bytes)),
            v: [0; 16],
            i: 0,
            st: 0,
            dt: 0,
            pc: 0,
            stack: Vec::with_capacity(config.stack_size),
            vram: Locked::new(Vec::from_iter(repeat(Pixel::default()).take(config.vram_size))),
            keys: Locked::new([false; 16]),
            buzzer: Locked::new(false),
            audio: Locked::new([0; 16]),
            random: rand_iterator,
            thread_rng: thread_rng(),
        }

    }

    pub fn vram_clone(&self) -> Locked<Vram> {
        self.vram.clone()
    }
    pub fn keys_clone(&self) -> Locked<Keyboard> {
        self.keys.clone()
    }
    pub fn buzzer_clone(&self) -> Locked<Buzzer> {
        self.buzzer.clone()
    }
    pub fn audio_clone(&self) -> Locked<Audio> {
        self.audio.clone()
    }

    pub fn next_random(&mut self) -> MemoryCell {
        if let Some(ref mut r) = self.random {
            r.next().unwrap_or(0)
        } else {
            self.thread_rng.gen()
        }
    }

    pub fn set_random(&mut self, iter: Option<ByteIter<'a>>) {
        self.random = iter;
    }
}

impl<'a> StateControl for Chip8<'a> {
        fn reset(&mut self) {
            self.ram = Vec::from_iter(repeat(0).take(self.config.ram_bytes));
            self.v = [0; 16];
            self.i = 0;
            self.st = 0;
            self.dt = 0;
            self.pc = 0;
            self.stack = Vec::with_capacity(self.config.stack_size);
            *self.vram.try_write() = Vec::from_iter(repeat(Pixel::default()).take(self.config.vram_size));
            *self.keys.try_write() = [false; 16];
            *self.buzzer.try_write() = false;
            *self.audio.try_write() = [0; 16];
        }
        fn step(&mut self) -> Result<(),()> {
            Ok(())
        }
}

impl<'a> Execute for Chip8<'a> {

    fn config(&self) -> Config {
        self.config
    }

    fn load(&mut self, src: Src) -> Chip8Result<usize> {
        match src {
            Src::Const(n)       => Ok(n),
            Src::Register(r)    => self.v.get(r).map(|reg| *reg as usize).ok_or(Chip8Error::OutOfBoundsAt(r)),
            Src::Address12(a)   => self.ram.get(a).map(|cell| *cell as usize).ok_or(Chip8Error::OutOfBoundsAt(a)),
            Src::I              => Ok(self.i as usize),
            Src::IndirectI      => self.ram.get(self.i as usize).and_then(|addr| self.ram.get(*addr as usize)).map(|v| *v as usize).ok_or(Chip8Error::OutOfBounds),
            Src::Literal12(n12) => Ok(n12),
            Src::Literal8(n8)   => Ok(n8),
            Src::Literal4(n4)   => Ok(n4),
            Src::SoundTimer     => Ok(self.st as usize),
            Src::DelayTimer     => Ok(self.dt as usize),
            Src::Random         => Ok(self.next_random() as usize),
            Src::PC             => Ok(self.pc as usize),
        }
    }

    fn store(&mut self, dest: Dest, data: usize) -> Chip8Result<()> {
        match dest {
            Dest::Register(r) => self.v.get_mut(r).map(|reg| {*reg = data as Register8 & 0xFF; () }).ok_or(Chip8Error::OutOfBoundsAt(r)),
            Dest::Address12(a) => self.ram.get_mut(a).map(|cell| {*cell = data as MemoryCell; () }).ok_or(Chip8Error::OutOfBoundsAt(a)),
            Dest::I => { self.i = (data & 0xFFFF) as Register16; Ok(()) },
            Dest::IndirectI => self.load(Src::I).and_then(|addr| self.store(Dest::Address12(addr), data)),
            Dest::SoundTimer => { self.st = data as Timer; Ok(()) },
            Dest::DelayTimer => { self.dt = data as Timer; Ok(())},
            Dest::PC => { self.pc = data as Address; Ok(()) },
        }
    }

    fn set_flag(&mut self, flag: bool) {
        self.v[0xF] = if flag {
            1
        } else {
            0
        };
    }

    fn stack_pop(&mut self) -> Option<Address> {
        self.stack.pop()
    }

    fn stack_push(&mut self, address: Address) {
        self.stack.push(address);
    }

    fn pc(&self) -> Address {
        self.pc
    }

    fn advance_pc(&mut self) {
        self.pc += 2;
    }

    fn jump(&mut self, addr: Address) {
        self.pc = addr;
    }

    fn keyboard(&self) -> Keyboard {
        *self.keys.try_read()
    }

    fn vram_mut(&mut self) -> RwLockWriteGuard<Vram> {
        self.vram.try_write()
    }

    fn audio_mut(&mut self) -> RwLockWriteGuard<Audio> {
        self.audio.try_write()
    }
}

impl<'a> Default for Chip8<'a> {
    fn default() -> Self {
        Self::new(&Config::default(), None)
    }
}
