use std::thread::{self, JoinHandle};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use types::*;
use simulator::{Simulate, Simulator};
use instruction::{Dest, Src};
use Config;

enum Command {
    Load(Sender<Chip8Result<usize>>, Src),
    Store(Sender<Chip8Result<()>>, Dest, usize),
    Step(Sender<Chip8Result<()>>),
    Tick(Sender<Chip8Result<()>>),
    LoadBytes(Sender<Chip8Result<()>>, Vec<u8>, Address),
    LoadProgram(Sender<Chip8Result<()>>, Vec<u8>),
    VramLock(Sender<Chip8Result<Arc<RwLock<Vram>>>>),
    KeyboardLock(Sender<Chip8Result<Arc<RwLock<Keyboard>>>>),
    BuzzerLock(Sender<Chip8Result<Arc<RwLock<Buzzer>>>>),
    AudioLock(Sender<Chip8Result<Arc<RwLock<Audio>>>>),
}


struct Manager {
    rx_chan: Receiver<Command>,
    sim: Simulator,
    keyboard_lock: Arc<RwLock<Keyboard>>,
    vram_lock: Arc<RwLock<Vram>>,
    buzzer_lock: Arc<RwLock<Buzzer>>,
    audio_lock: Arc<RwLock<Audio>>,
}

impl Manager {
    pub fn new(config: Config, rx_chan: Receiver<Command>) -> Manager {
        let mut simulator = Simulator::new(&config, None);
        let keyboard_lock = simulator.keyboard_lock().unwrap();
        let vram_lock = simulator.vram_lock().unwrap();
        let buzzer_lock = simulator.buzzer_lock().unwrap();
        let audio_lock = simulator.audio_lock().unwrap();

        Manager {
            rx_chan: rx_chan,
            sim: simulator,
            keyboard_lock: keyboard_lock,
            vram_lock: vram_lock,
            buzzer_lock: buzzer_lock,
            audio_lock: audio_lock,
        }
    }

    pub fn run(&mut self) {
        while true {
            if let Ok(command) = self.rx_chan.recv() {
                match command {
                    Command::Load(tx_chan, src) => {
                        tx_chan.send(self.sim.load(src)).unwrap();
                    },
                    Command::Store(tx_chan, dest, value) => {
                        tx_chan.send(self.sim.store(dest, value)).unwrap();
                    },
                    Command::Step(tx_chan) => {
                        tx_chan.send(self.sim.step()).unwrap();
                    },
                    Command::Tick(tx_chan) => {
                        tx_chan.send(self.sim.timer_tick()).unwrap();
                    },
                    Command::LoadBytes(tx_chan, bytes, addr) => {
                        tx_chan.send(self.sim.load_bytes(&bytes, addr)).unwrap();
                    },
                    Command::LoadProgram(tx_chan, bytes) => {
                        tx_chan.send(self.sim.load_program(&bytes)).unwrap();
                    },
                    Command::KeyboardLock(tx_chan) => {
                        tx_chan.send(self.sim.keyboard_lock()).unwrap();
                    },
                    Command::VramLock(tx_chan) => {
                        tx_chan.send(self.sim.vram_lock()).unwrap();
                    },
                    Command::BuzzerLock(tx_chan) => {
                        tx_chan.send(self.sim.buzzer_lock()).unwrap();
                    },
                    Command::AudioLock(tx_chan) => {
                        tx_chan.send(self.sim.audio_lock()).unwrap();
                    },
                }
            } else {
                return;
            }
        }
    }
}

pub struct SimulatorTask {
    child: JoinHandle<()>,
    tx_chan: Sender<Command>,
    keyboard_lock: Arc<RwLock<Keyboard>>,
    vram_lock: Arc<RwLock<Vram>>,
    buzzer_lock: Arc<RwLock<Buzzer>>,
    audio_lock: Arc<RwLock<Audio>>,
}

impl SimulatorTask {
    pub fn spawn(config: Config) -> SimulatorTask {
        let (tx, rx) = channel();

        let child = thread::spawn(move || {
            Manager::new(config, rx).run();
        });

        let (tx_locks, rx_locks) = channel();
        tx.send(Command::VramLock(tx_locks)).unwrap();
        let vram_lock = rx_locks.recv().unwrap().unwrap();

        let (tx_locks, rx_locks) = channel();
        tx.send(Command::KeyboardLock(tx_locks)).unwrap();
        let keyboard_lock = rx_locks.recv().unwrap().unwrap();

        let (tx_locks, rx_locks) = channel();
        tx.send(Command::BuzzerLock(tx_locks)).unwrap();
        let buzzer_lock = rx_locks.recv().unwrap().unwrap();

        let (tx_locks, rx_locks) = channel();
        tx.send(Command::AudioLock(tx_locks)).unwrap();
        let audio_lock = rx_locks.recv().unwrap().unwrap();

        SimulatorTask {
            child: child,
            tx_chan: tx,
            keyboard_lock: keyboard_lock,
            vram_lock: vram_lock,
            buzzer_lock: buzzer_lock,
            audio_lock: audio_lock,
        }

    }
}


impl Simulate for SimulatorTask {
    fn step(&mut self) -> Chip8Result<()> {
        let (tx, rx) = channel();
        try!(self.tx_chan.send(Command::Step(tx)).map_err(|_| Chip8Error::ChannelTxFailure));
        try!(rx.recv().map_err(|_| Chip8Error::ChannelRxFailure))
    }
    fn step_n(&mut self, number_of_steps: usize) -> Chip8Result<()> {
        for i in 0..number_of_steps {
            try!(self.step());
        }
        Ok(())
    }
    fn timer_tick(&mut self) -> Chip8Result<()> {
        let (tx, rx) = channel();
        try!(self.tx_chan.send(Command::Tick(tx)).map_err(|_| Chip8Error::ChannelTxFailure));
        try!(rx.recv().map_err(|_| Chip8Error::ChannelRxFailure))
    }
    fn load_bytes(&mut self, bytes: &[u8], addr: Address) -> Chip8Result<()> {
        let (tx, rx) = channel();
        try!(self.tx_chan
            .send(Command::LoadBytes(tx, bytes.to_vec(), addr))
            .map_err(|_| Chip8Error::ChannelTxFailure));
        try!(rx.recv().map_err(|_| Chip8Error::ChannelRxFailure))
    }
    fn load_program(&mut self, bytes: &[u8]) -> Chip8Result<()> {
        let (tx, rx) = channel();
        try!(self.tx_chan
            .send(Command::LoadProgram(tx, bytes.to_vec()))
            .map_err(|_| Chip8Error::ChannelTxFailure));
        try!(rx.recv().map_err(|_| Chip8Error::ChannelRxFailure))
    }
    fn load(&mut self, src: Src) -> Chip8Result<usize> {
        let (tx, rx) = channel();
        try!(self.tx_chan.send(Command::Load(tx, src)).map_err(|_| Chip8Error::ChannelTxFailure));
        try!(rx.recv().map_err(|_| Chip8Error::ChannelRxFailure))
    }
    fn store(&mut self, dest: Dest, value: usize) -> Chip8Result<()> {
        let (tx, rx) = channel();
        try!(self.tx_chan
            .send(Command::Store(tx, dest, value))
            .map_err(|_| Chip8Error::ChannelTxFailure));
        try!(rx.recv().map_err(|_| Chip8Error::ChannelRxFailure))
    }

    fn set_keyboard(&mut self, keys: &Keyboard) -> Chip8Result<()> {
        *self.keyboard_lock.write().unwrap() = *keys;
        Ok(())
    }
    fn keyboard(&self) -> Chip8Result<Keyboard> {
        let keyboard_ref = self.keyboard_lock.read().unwrap();
        let keyboard = keyboard_ref.clone();
        Ok(keyboard)
    }
    fn vram(&self) -> Chip8Result<Vram> {
        let vram_ref = self.vram_lock.read().unwrap();
        let vram = vram_ref.clone();
        Ok(vram)
    }
    fn buzzer(&self) -> Chip8Result<Buzzer> {
        Ok(*self.buzzer_lock.read().unwrap())
    }
    fn audio(&self) -> Chip8Result<Audio> {
        Ok(*self.audio_lock.read().unwrap())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use config::COSMAC_VIP;
    use instruction::Src;
    use simulator::{Simulate, Simulator};

    #[test]
    fn test_simtask() {
        let mut task = SimulatorTask::spawn(COSMAC_VIP);

        let t = task.load(Src::Register(0));
        println!("{:?}", t);
        // assert!(false);

    }
}
