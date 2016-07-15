use std::thread::{self, JoinHandle};
use std::sync::mpsc::{channel, Receiver, Sender};

use types::*;
use simulator::{Simulator, Simulate};
use instruction::{Src, Dest};
use Config;

enum Command {
    Load(Sender<Chip8Result<usize>>, Src),
    Store(Sender<Chip8Result<()>>, Dest, usize),
    Step(Sender<Chip8Result<()>>),
    Tick(Sender<Chip8Result<()>>),
    LoadBytes(Sender<Chip8Result<()>>, Vec<u8>, Address),
    LoadProgram(Sender<Chip8Result<()>>, Vec<u8>),
}


struct Manager<'a> {
    rx_chan: Receiver<Command>,
    sim: Simulator<'a>,
}

impl<'a> Manager<'a> {
    pub fn new(config: Config, rx_chan: Receiver<Command>) -> Manager<'a> {
        let simulator = Simulator::new(&config, None);
        Manager {
            rx_chan: rx_chan,
            sim: simulator,
        }
    }

    pub fn run(&mut self) {
        while true {
            if let Ok(command) = self.rx_chan.recv() {
                match command {
                    Command::Load(tx_chan, src) => { tx_chan.send(self.sim.load(src)).unwrap(); },
                    Command::Store(tx_chan, dest, value) => { tx_chan.send(self.sim.store(dest, value)).unwrap(); },
                    Command::Step(tx_chan) => { tx_chan.send(self.sim.step()).unwrap(); },
                    Command::Tick(tx_chan) => { tx_chan.send(self.sim.timer_tick()).unwrap(); },
                    Command::LoadBytes(tx_chan, bytes, addr) => { tx_chan.send(self.sim.load_bytes(&bytes, addr)).unwrap(); },
                    Command::LoadProgram(tx_chan, bytes) => { tx_chan.send(self.sim.load_program(&bytes)).unwrap(); },
                }
            } else { return; }
        }
    }
}

pub struct SimulatorTask {
    child: JoinHandle<()>,
    tx_chan: Sender<Command>,
}

impl<'a> SimulatorTask {
    pub fn spawn(config: Config) -> SimulatorTask {
        let (tx, rx) = channel();

        let child = thread::spawn(move || {
            Manager::new(config, rx).run();
        });
        SimulatorTask {
            child: child,
            tx_chan: tx,
        }
    }


}


impl Simulate for SimulatorTask {
    fn step(&mut self) -> Chip8Result<()> {
        let (tx, rx) = channel();
        try!(self.tx_chan.send(Command::Step(tx)).map_err(|_| Chip8Error::ChannelTxFailure));
        try!(rx.recv().map_err(|_| Chip8Error::ChannelRxFailure))
    }
    fn step_n(&mut self, number_of_steps: usize) -> Chip8Result<()>{
        for i in 0..number_of_steps {
            try!(self.step());
        }
        Ok(())
    }
    fn timer_tick(&mut self) -> Chip8Result<()>{
        let (tx, rx) = channel();
        try!(self.tx_chan.send(Command::Tick(tx)).map_err(|_| Chip8Error::ChannelTxFailure));
        try!(rx.recv().map_err(|_| Chip8Error::ChannelRxFailure))
    }
    fn load_bytes(&mut self, bytes: &[u8], addr: Address) -> Chip8Result<()>{
        let (tx, rx) = channel();
        try!(self.tx_chan.send(Command::LoadBytes(tx, bytes.to_vec(), addr)).map_err(|_| Chip8Error::ChannelTxFailure));
        try!(rx.recv().map_err(|_| Chip8Error::ChannelRxFailure))
    }
    fn load_program(&mut self, bytes: &[u8]) -> Chip8Result<()>{
        let (tx, rx) = channel();
        try!(self.tx_chan.send(Command::LoadProgram(tx, bytes.to_vec())).map_err(|_| Chip8Error::ChannelTxFailure));
        try!(rx.recv().map_err(|_| Chip8Error::ChannelRxFailure))
    }
    fn load(&mut self, src: Src) -> Chip8Result<usize> {
        let (tx, rx) = channel();
        try!(self.tx_chan.send(Command::Load(tx, src)).map_err(|_| Chip8Error::ChannelTxFailure));
        try!(rx.recv().map_err(|_| Chip8Error::ChannelRxFailure))
    }
    fn store(&mut self, dest: Dest, value: usize) -> Chip8Result<()> {
        let (tx, rx) = channel();
        try!(self.tx_chan.send(Command::Store(tx, dest, value)).map_err(|_| Chip8Error::ChannelTxFailure));
        try!(rx.recv().map_err(|_| Chip8Error::ChannelRxFailure))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use config::COSMAC_VIP;
    use instruction::Src;
    use simulator::{Simulator, Simulate};

    #[test]
    fn test_simtask() {
        let mut task = SimulatorTask::spawn(COSMAC_VIP);

        let t = task.load(Src::Register(0));
        println!("{:?}", t);
        //assert!(false);

    }
}
