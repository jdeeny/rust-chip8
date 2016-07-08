//! Data types used to describe the data that makes up the Chip8.

/// One byte in RAM.
pub type MemoryCell = u8;
/// A value in an 8-bit register.
pub type Register8 = u8;
/// A value in a 16-bit register.
pub type Register16 = u16;
/// An 8-bit timer.
pub type Timer = u8;
/// An Address.
pub type Address = u16;
/// A single pixel.
#[derive(Debug,Default,Copy,Clone)]
pub struct Pixel {
    state: bool,
}
/// The state of the keyboard.
pub type Keyboard = [bool; 16];
/// The state of the audio output.
pub type Audio = bool;


/*

/// Contains the data that is shared between the simulator and the UI.
#[derive(Debug)]
pub struct UiState {
    /// The video ram state.
    pub vram: Arc<RwLock<Vram>>,
    /// The keyboard state.
    pub keys: Arc<RwLock<Keyboard>>,
    /// The audio state.
    pub audio: Arc<RwLock<Audio>>,
}

impl UiState {
    /// Returns a a new UiState.
    pub fn new() -> UiState {
        Self::default()
    }
}

impl Default for UiState {
    fn default() -> UiState {
        UiState {
            vram: Arc::new(RwLock::new(Vram::new())),
            keys: Arc::new(RwLock::new(Keyboard::new())),
            audio: Arc::new(RwLock::new(Audio::new())),
        }
    }
}

impl Clone for UiState {
    fn clone(&self) -> UiState {
        UiState {
            vram: self.vram.clone(),
            keys: self.keys.clone(),
            audio: self.audio.clone(),
        }
    }
}

/// Holds the state of the video ram of the simulator.
#[derive(Copy)]
pub struct Vram {
    ///Holds the state of the pixels.
    pub pixels: [[u8; 32]; 64],
}
impl Vram {
    /// Returns a new Vram.
    pub fn new() -> Vram {
        Self::default()
    }
}
impl Default for Vram {
    fn default() -> Vram {
        Vram { pixels: [[0; 32]; 64] }
    }
}
impl fmt::Debug for Vram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vram {{}}")
    }
}
impl Clone for Vram {
    fn clone(&self) -> Self {
        *self
        //let mut p: [[u8;32];64] = [[0;32];64];
        //for (i, row) in self.pixels.iter().enumerate() {
        //    p[i] = *row;
        //}
        /Vram { pixels: p }
    }
}

/// Holds the state of the keyboard of the simulator.
#[derive(Copy,Clone,Debug)]
pub struct Keyboard {
    /// The state of the keyboard. True indicates that the key is pressed.
    pub state: [bool; 16],
}
impl Keyboard {
    /// Returns a new Keyboard.
    pub fn new() -> Keyboard {
        Self::default()
    }
    /// Returns true if the requested key is currently pressed.
    pub fn is_down(&self, key: usize) -> bool {
        assert!(key <= 16);
        self.state[key]
    }
}
impl Default for Keyboard {
    fn default() -> Keyboard {
        Keyboard { state: [false; 16] }
    }
}

/// Holds the state of the audio output of the simulator.
#[derive(Copy,Clone,Debug)]
pub struct Audio {

}
impl Audio {
    /// Returns a new Audio.
    pub fn new() -> Audio {
        Self::default()
    }
}
impl Default for Audio {
    fn default() -> Audio {
        Audio {}
    }
}
*/
