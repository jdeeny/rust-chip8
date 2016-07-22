//! Defines the state of the Chip8 virtual machine.
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::panic;
use std::iter::{FromIterator, Iterator, repeat};
use std::collections::VecDeque;
use rand::{Rng, ThreadRng, thread_rng};
pub use types::*;
use config::Config;
use instruction::{Dest, Src};


pub type RandomBytes = VecDeque<u8>;

// #[derive(Debug, Clone)]
// pub struct Locked<T>(pub Arc<RwLock<T>>);
//
// impl<T> Locked<T> {
// pub fn new(t: T) -> Locked<T> {
// Locked(Arc::new(RwLock::new(t)))
// }
// pub fn try_read(&self) -> RwLockReadGuard<T> {
// self.0.try_read().unwrap()
// }
// pub fn try_write(&mut self) -> RwLockWriteGuard<T> {
// self.0.try_write().unwrap()s
// }
// pub fn clone_lock(&mut self) -> Arc<RwLock<T>> {
// self.0.clone()
// }
// }

/// A struct that contains a Chip8 `Config` and the machine state.
///
/// The machine state includes the RAM, registers, program counter, stack, timers, and the
/// state of the IO subsystems.
#[allow(dead_code)]
pub struct Chip8 {
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
    pub vram: Arc<RwLock<Vram>>,
    /// The state of the keyboard.
    pub keys: Arc<RwLock<Keyboard>>,
    /// The state of the chip8 buzzer.
    pub buzzer: Arc<RwLock<Buzzer>>,
    /// The state of the audio buffer used with XOCHIP.
    pub audio: Arc<RwLock<Audio>>,
    /// Optional user-provided random data for replay.
    pub random: Option<RandomBytes>,
    /// System random number generator.
    thread_rng: ThreadRng,
}

impl Chip8 {
    /// Create a new Chip8 using the supplied Config.
    pub fn new(config: &Config, random: Option<RandomBytes>) -> Chip8 {
        Chip8 {
            config: *config,
            ram: Vec::from_iter(repeat(0).take(config.ram_bytes)),
            v: [0; 16],
            i: 0,
            st: 0,
            dt: 0,
            pc: 0,
            stack: Vec::with_capacity(config.stack_size),
            vram: Arc::new(RwLock::new(Vec::from_iter(repeat(Pixel::default())
                .take(config.vram_size)))),
            keys: Arc::new(RwLock::new([false; 16])),
            buzzer: Arc::new(RwLock::new(false)),
            audio: Arc::new(RwLock::new([0; 16])),
            random: random,
            thread_rng: thread_rng(),
        }

    }

    pub fn vram_clone(&self) -> Arc<RwLock<Vram>> {
        self.vram.clone()
    }
    pub fn keys_clone(&self) -> Arc<RwLock<Keyboard>> {
        self.keys.clone()
    }
    pub fn buzzer_clone(&self) -> Arc<RwLock<Buzzer>> {
        self.buzzer.clone()
    }
    pub fn audio_clone(&self) -> Arc<RwLock<Audio>> {
        self.audio.clone()
    }

    fn next_random(&mut self) -> MemoryCell {
        if let Some(ref mut r) = self.random {
            r.pop_front().unwrap_or(0)
        } else {
            self.thread_rng.gen()
        }
    }

    pub fn set_random(&mut self, iter: Option<RandomBytes>) {
        self.random = iter;
    }

    pub fn load_bytes(&mut self, bytes: &[u8], address: Address) -> Chip8Result<()> {
        let last_byte = address as usize + bytes.len();
        if last_byte > self.config.ram_bytes {
            return Err(Chip8Error::OutOfBoundsAt(address as usize));
        }
        let mut i = address as usize;
        for b in bytes {
            self.ram[i] = *b;
            i += 1;
        }
        Ok(())
    }
}

impl Chip8 {
    pub fn reset(&mut self) {
        self.ram = Vec::from_iter(repeat(0).take(self.config.ram_bytes));
        self.v = [0; 16];
        self.i = 0;
        self.st = 0;
        self.dt = 0;
        self.pc = 0;
        self.stack = Vec::with_capacity(self.config.stack_size);
        *self.vram.try_write().unwrap() = Vec::from_iter(repeat(Pixel::default())
            .take(self.config.vram_size));
        *self.keys.try_write().unwrap() = [false; 16];
        *self.buzzer.try_write().unwrap() = false;
        *self.audio.try_write().unwrap() = [0; 16];
    }
    pub fn step(&mut self) -> Chip8Result<()> {
        Ok(())
    }
    pub fn step_n(&mut self, number_of_steps: usize) -> Chip8Result<()> {
        for _ in 0..number_of_steps {
            try!(self.step());
        }
        Ok(())
    }
    pub fn keyboard_lock(&mut self) -> Arc<RwLock<Keyboard>> {
        self.keys.clone()
    }

    pub fn vram_lock(&mut self) -> Arc<RwLock<Vram>> {
        self.vram.clone()
    }

    pub fn buzzer_lock(&mut self) -> Arc<RwLock<Buzzer>> {
        self.buzzer.clone()
    }

    pub fn audio_lock(&mut self) -> Arc<RwLock<Audio>> {
        self.audio.clone()
    }
}

impl Execute for Chip8 {
    fn config(&self) -> Config {
        self.config
    }

    fn load(&mut self, src: Src) -> Chip8Result<usize> {
        print!("load src: {:?} ", src);
        let x = match src {
            Src::Const(n) => Ok(n),
            Src::Register(r) => self.v
                .get(r)
                .map(|reg| *reg as usize)
                .ok_or(Chip8Error::OutOfBoundsAt(r)),
            Src::Address12(a) => self.ram
                .get(a)
                .map(|cell| *cell as usize)
                .ok_or(Chip8Error::OutOfBoundsAt(a)),
            Src::I => Ok(self.i as usize),
            Src::IndirectI => self.ram
                .get(self.i as usize)
                .and_then(|addr| self.ram.get(*addr as usize))
                .map(|v| *v as usize)
                .ok_or(Chip8Error::OutOfBounds),
            Src::Literal12(n12) => Ok(n12),
            Src::Literal8(n8) => Ok(n8),
            Src::Literal4(n4) => Ok(n4),
            Src::SoundTimer => Ok(self.st as usize),
            Src::DelayTimer => Ok(self.dt as usize),
            Src::Random => Ok(self.next_random() as usize),
            Src::PC => Ok(self.pc as usize),
        };
        println!("= {:?}", x);
        x
    }

    fn store(&mut self, dest: Dest, data: usize) -> Chip8Result<()> {
        println!("store {:?} = {:?} ", dest, data);
        match dest {
            Dest::Register(r) => self.v
                .get_mut(r)
                .map(|reg| {
                    *reg = data as Register8 & 0xFF;
                    ()
                })
                .ok_or(Chip8Error::OutOfBoundsAt(r)),
            Dest::Address12(a) => self.ram
                .get_mut(a)
                .map(|cell| {
                    *cell = data as MemoryCell;
                    ()
                })
                .ok_or(Chip8Error::OutOfBoundsAt(a)),
            Dest::I => {
                self.i = (data & 0xFFFF) as Register16;
                Ok(())
            },
            Dest::IndirectI => self.load(Src::I)
                .and_then(|addr| self.store(Dest::Address12(addr), data)),
            Dest::SoundTimer => {
                self.st = data as Timer;
                Ok(())
            },
            Dest::DelayTimer => {
                self.dt = data as Timer;
                Ok(())
            },
            Dest::PC => {
                self.pc = data as Address;
                Ok(())
            },
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

    fn jump(&mut self, addr: Address) -> Chip8Result<()> {
        let address = addr as usize;
        println!("addr: {:?}", addr);
        if address > self.config.ram_bytes - 1 {
            return Err(Chip8Error::OutOfBoundsAt(address));
        }
        self.pc = addr;
        println!("pc: {:?}", self.pc);

        Ok(())
    }

    fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel) -> Chip8Result<bool> {
        let mut vram = self.vram.write().unwrap();
        let x = x % 64;
        let y = y % 32;
        vram[x + y * 64] ^= pixel;
        Ok(vram[x + y * 64] != pixel)
    }


    fn set_keyboard(&mut self, keys: &Keyboard) -> Chip8Result<()> {
        let mut k = self.keys.try_write().unwrap();
        *k = *keys;
        Ok(())

    }

    fn keyboard(&self) -> Chip8Result<Keyboard> {
        Ok(self.keys.try_read().unwrap().clone())
    }

    fn vram(&self) -> Chip8Result<Vram> {
        Ok(self.vram.try_read().unwrap().clone())
    }

    fn buzzer(&self) -> Chip8Result<Buzzer> {
        Ok(*self.buzzer.try_read().unwrap())
    }

    fn audio(&self) -> Chip8Result<Audio> {
        Ok(*self.audio.try_read().unwrap())
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new(&Config::default(), None)
    }
}
