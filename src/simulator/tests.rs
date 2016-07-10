
use simulator::Simulator;
use instruction::{Execute, Src};
use config::Config;
use types::Address;


#[test]
fn test_sim_jump() {
    let config = Config::default();
    let mut s = Simulator::new(&config);
    let prog = [0x60, 0x55, 0x12, 0x00];    //LD V0, 0x55; Jump 0x200
    s.load_bytes(&prog, config.addr_program as Address);
    assert_eq!(s.ram(config.addr_program as Address), 0x60);
    assert_eq!(s.ram(config.addr_program as Address + 1), 0x55);
    s.jump(config.addr_program as Address);
    s.step();
    assert_eq!(s.pc(), 0x202);
    assert_eq!(s.load(Src::Register(0)), 0x55);
    s.step();
    assert_eq!(s.pc(), 0x200);

}
