
use simulator::{Simulator, Simulate, SimulatorTask};
use instruction::{Src};
use config::{Config, COSMAC_VIP};
use types::Address;


#[test]
fn test_jump() {
    let config = COSMAC_VIP;
    let mut s = Simulator::new(&config, None);

    let prog = [0x60, 0x55, 0x12, 0x00];    //LD V0, 0x55; Jump 0x200
    s.load_program(&prog);

    assert_eq!(s.load(Src::Address12(config.addr_program)).unwrap(), 0x60);
    assert_eq!(s.load(Src::Address12(config.addr_program + 1)).unwrap(), 0x55);
    //s.jump(config.addr_program as Address).unwrap();
    s.step();
    assert_eq!(s.load(Src::PC).unwrap(), 0x202);
    assert_eq!(s.load(Src::Register(0)).unwrap(), 0x55);
    s.step();
    assert_eq!(s.load(Src::PC).unwrap(), 0x200);
}


#[test]
fn test_fetch_equality_or() {
    // Octo equivalent:
    //: data 0x55 0xAA 0xA9
    //: main
    //    v3 := 0xFF
    //    i := data
    //    load v2
    //    v1 |= v0
    //    v2 |= v0
    //    if v3 == v1 then vA := 1
    //    if v3 != v1 then vB := 1
    //    if v3 == v2 then vC := 1
    //    if v3 != v2 then vD := 1
    let prog = [0x12, 0x05, 0x55, 0xAA, 0xA9, 0x63, 0xFF, 0xA2, 0x02, 0xF2, 0x65, 0x81, 0x01, 0x82, 0x01, 0x93, 0x10, 0x6A, 0x01, 0x53, 0x10, 0x6B, 0x01, 0x93, 0x20, 0x6C, 0x01, 0x53, 0x20, 0x6D, 0x01];
    let mut s = Simulator::new(&COSMAC_VIP, None);
    s.load_program(&prog);

    s.step();
    assert_eq!(s.load(Src::PC).unwrap(), 0x205);
    s.step();
    assert_eq!(s.load(Src::Register(3)).unwrap(), 0xFF);
    s.step();
    assert_eq!(s.load(Src::I).unwrap(), 0x202);
    s.step();
    assert_eq!(s.load(Src::Register(0)).unwrap(), 0x55);
    assert_eq!(s.load(Src::Register(1)).unwrap(), 0xAA);
    assert_eq!(s.load(Src::Register(2)).unwrap(), 0xA9);
    s.step();
    assert_eq!(s.load(Src::Register(1)).unwrap(), 0xFF);
    s.step();
    assert_eq!(s.load(Src::Register(2)).unwrap(), 0xFD);
    s.step_n(6);
    assert_eq!(s.load(Src::Register(0xA)).unwrap(), 1);
    assert_eq!(s.load(Src::Register(0xB)).unwrap(), 0);
    assert_eq!(s.load(Src::Register(0xC)).unwrap(), 0);
    assert_eq!(s.load(Src::Register(0xD)).unwrap(), 1);
}

#[test]
fn test_jump_threaded() {
    let config = COSMAC_VIP;
    let mut s = SimulatorTask::spawn(config);

    let prog = [0x60, 0x55, 0x12, 0x00];    //LD V0, 0x55; Jump 0x200
    s.load_program(&prog);

    assert_eq!(s.load(Src::Address12(config.addr_program)).unwrap(), 0x60);
    assert_eq!(s.load(Src::Address12(config.addr_program + 1)).unwrap(), 0x55);
    //s.jump(config.addr_program as Address).unwrap();
    s.step();
    assert_eq!(s.load(Src::PC).unwrap(), 0x202);
    assert_eq!(s.load(Src::Register(0)).unwrap(), 0x55);
    s.step();
    assert_eq!(s.load(Src::PC).unwrap(), 0x200);
}

#[test]
fn test_add() {
    let config = COSMAC_VIP;
    let mut s = SimulatorTask::spawn(config);

    let prog = [0x64, 0x32, 0x67, 0xC8, 0x84, 0x74, 0x84, 0x74];    //v4 := 50, v7 := 200, v4 += v7, v4 += v7
    s.load_program(&prog);

    s.step_n(3);
    assert_eq!(s.load(Src::Register(4)).unwrap(), 250);
    assert_eq!(s.load(Src::Register(0xF)).unwrap(), 0);
    s.step();
    assert_eq!(s.load(Src::Register(4)).unwrap(), 0xC2);
    assert_eq!(s.load(Src::Register(0xF)).unwrap(), 1);
}

#[test]
fn test_sprite() {
    // Octo equivalent:
    // : main
    //   i := the_sprite
    //   v0 := 62
    //   v1 := 30
    //   sprite v0 v1 4
    //   v0 := 0
    //   v1 := 0
    //   sprite v0 v1 4
    // : the_sprite 0x50 0xA0 0x50 0xA0
    let prog = [0xA2, 0x0E, 0x60, 0x3E, 0x61, 0x1E, 0xD0, 0x14, 0x60, 0x00, 0x61, 0x00, 0xD0, 0x14, 0x50, 0xA0, 0x50, 0xA0];
    let mut s = Simulator::new(&COSMAC_VIP, None);
    s.load_program(&prog);

    s.step_n(4);
    let vram = s.vram().unwrap();
    assert_eq!(vram[0 * 64 + 0], 0);
    assert_eq!(vram[0 * 64 + 1], 1);
    assert_eq!(vram[1 * 64 + 0], 1);
    assert_eq!(vram[31 * 64 + 62], 1);
    assert_eq!(vram[31 * 64 + 63], 0);
    assert_eq!(s.load(Src::Register(0xF)).unwrap(), 0);

    s.step_n(3);
    let vram = s.vram().unwrap();
    assert_eq!(vram[0 * 64 + 0], 0);
    assert_eq!(vram[0 * 64 + 1], 0);
    assert_eq!(vram[1 * 64 + 0], 0);
    assert_eq!(vram[1 * 64 + 2], 1);
    assert_eq!(vram[2 * 64 + 1], 1);
    assert_eq!(vram[2 * 64 + 2], 0);
    assert_eq!(vram[31 * 64 + 62], 1);
    assert_eq!(vram[31 * 64 + 63], 0);

    assert_eq!(s.load(Src::Register(0xF)).unwrap(), 1);
}

#[test]
fn test_sub() {
    let config = COSMAC_VIP;
    let mut s = SimulatorTask::spawn(config);

    // : main
    //v0 := 0x20
    //v1 := 0x10
    //v0 -= v1
    //v0 -= v1
    //v0 -= v1
    //vA := 0x20
    //vB := 0x40
    //vA =- vB
    //vB =- vA
    let prog = [0x60, 0x20, 0x61, 0x10, 0x80, 0x15, 0x80, 0x15, 0x80, 0x15, 0x6A, 0x20, 0x6B, 0x40, 0x8A, 0xB7, 0x8B, 0xA7];
    s.load_program(&prog);

    s.step_n(3);
    assert_eq!(s.load(Src::Register(0)).unwrap(), 0x10);
    assert_eq!(s.load(Src::Register(1)).unwrap(), 0x10);
    assert_eq!(s.load(Src::Register(0xF)).unwrap(), 1);
    s.step();
    assert_eq!(s.load(Src::Register(0)).unwrap(), 0x00);
    assert_eq!(s.load(Src::Register(1)).unwrap(), 0x10);
    assert_eq!(s.load(Src::Register(0xF)).unwrap(), 1);
    s.step();
    assert_eq!(s.load(Src::Register(0)).unwrap(), 0xF0);
    assert_eq!(s.load(Src::Register(1)).unwrap(), 0x10);
    assert_eq!(s.load(Src::Register(0xF)).unwrap(), 0);
    s.step_n(3);
    assert_eq!(s.load(Src::Register(0xA)).unwrap(), 0x20);
    assert_eq!(s.load(Src::Register(0xB)).unwrap(), 0x40);
    assert_eq!(s.load(Src::Register(0xF)).unwrap(), 1);
    s.step();
    assert_eq!(s.load(Src::Register(0xA)).unwrap(), 0x20);
    assert_eq!(s.load(Src::Register(0xB)).unwrap(), 0xE0);
    assert_eq!(s.load(Src::Register(0xF)).unwrap(), 0);
}


#[test]
fn test_stash_fetch() {
    // Octo equivalent:
    //: main
    //v0 := 1
    //v1 := 2
    //v2 := 5
    //i := storage
    //save v2
    //v0 := 10
    //v1 := 20
    //v2 := 30
    //v3 := 40
    //load v1
    //i := storage
    //load v1
    //i := storage
    //load v2
    //vA := 66
    //vB := 77
    //vC := 88
    //i := storage
    //save vA - vC
    //vA := 0
    //vB := 0
    //vC := 0
    //load vA - vC
    //load vB - vB
    //load vB - vB
    //: storage 0 0 0
    let prog = [0x60, 0x01, 0x61, 0x02, 0x62, 0x05, 0xA2, 0x32, 0xF2, 0x55, 0x60, 0x0A, 0x61, 0x14, 0x62, 0x1E, 0x63, 0x28, 0xF1, 0x65, 0xA2, 0x32, 0xF1, 0x65, 0xA2, 0x32, 0xF2, 0x65, 0x6A, 0x42, 0x6B, 0x4D, 0x6C, 0x58, 0xA2, 0x32, 0x5A, 0xC2, 0x6A, 0x00, 0x6B, 0x00, 0x6C, 0x00, 0x5A, 0xC3, 0x5B, 0xB3, 0x5B, 0xB3, 0x00, 0x00, 0x00];
    let mut config = COSMAC_VIP;
    config.isa_xochip = true;
    let mut s = Simulator::new(&config, None);

    s.load_program(&prog);

    s.step_n(10);
    assert_eq!(s.load(Src::Register(0)).unwrap(), 0);
    assert_eq!(s.load(Src::Register(1)).unwrap(), 0);
    assert_eq!(s.load(Src::Register(2)).unwrap(), 0x1E);
    assert_eq!(s.load(Src::Register(3)).unwrap(), 0x28);
    assert_eq!(s.load(Src::I).unwrap(), 0x237);
    s.step_n(2);
    assert_eq!(s.load(Src::Register(0)).unwrap(), 1);
    assert_eq!(s.load(Src::Register(1)).unwrap(), 2);
    assert_eq!(s.load(Src::Register(2)).unwrap(), 0x1E);
    assert_eq!(s.load(Src::Register(3)).unwrap(), 0x28);
    assert_eq!(s.load(Src::I).unwrap(), 0x234);
    s.step_n(2);
    assert_eq!(s.load(Src::Register(0)).unwrap(), 1);
    assert_eq!(s.load(Src::Register(1)).unwrap(), 2);
    assert_eq!(s.load(Src::Register(2)).unwrap(), 5);
    assert_eq!(s.load(Src::Register(3)).unwrap(), 0x28);
    assert_eq!(s.load(Src::I).unwrap(), 0x235);
    s.step_n(9);
    assert_eq!(s.load(Src::Register(0xA)).unwrap(), 0x42);
    assert_eq!(s.load(Src::Register(0xB)).unwrap(), 0x4D);
    assert_eq!(s.load(Src::Register(0xC)).unwrap(), 0x58);
    assert_eq!(s.load(Src::I).unwrap(), 0x232);
    s.step();
    assert_eq!(s.load(Src::Register(0xA)).unwrap(), 0x42);
    assert_eq!(s.load(Src::Register(0xB)).unwrap(), 0x42);
    assert_eq!(s.load(Src::Register(0xC)).unwrap(), 0x58);
    assert_eq!(s.load(Src::I).unwrap(), 0x232);
    s.step();
    assert_eq!(s.load(Src::Register(0xA)).unwrap(), 0x42);
    assert_eq!(s.load(Src::Register(0xB)).unwrap(), 0x42);
    assert_eq!(s.load(Src::Register(0xC)).unwrap(), 0x58);
    assert_eq!(s.load(Src::I).unwrap(), 0x232);
}

#[test]
fn test_and_xor() {
    // Octo equivalent:
    // : main
    // v0 := 0x55
    // v1 := 0xAA
    // v2 := 0x00
    // v2 &= v0
    // v1 &= v0
    // v1 := 0xAA
    // v0 ^= v1
    // v0 ^= v0
    let prog = [0x60, 0x55, 0x61, 0xAA, 0x62, 0x00, 0x82, 0x02, 0x81, 0x02, 0x61, 0xAA, 0x80, 0x13, 0x80, 0x03];
    let mut s = Simulator::new(&COSMAC_VIP, None);
    s.load_program(&prog);

    s.step_n(5);
    assert_eq!(s.load(Src::Register(0)).unwrap(), 0x55);
    assert_eq!(s.load(Src::Register(1)).unwrap(), 0x00);
    assert_eq!(s.load(Src::Register(2)).unwrap(), 0x00);
    s.step_n(2);
    assert_eq!(s.load(Src::Register(0)).unwrap(), 0xFF);
    assert_eq!(s.load(Src::Register(1)).unwrap(), 0xAA);
    s.step();
    assert_eq!(s.load(Src::Register(0)).unwrap(), 0x00);
}
