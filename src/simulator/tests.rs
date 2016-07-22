use std::collections::VecDeque;

use simulator::{Simulate, Simulator, SimulatorTask};
use instruction::Src;
use config::{COSMAC_VIP, Config};
use types::*;

#[test]
fn test_jump() {
    let config = COSMAC_VIP;
    let mut s = Simulator::new(&config, None);

    let prog = [0x60, 0x55, 0x12, 0x00];    //LD V0, 0x55; Jump 0x200
    s.load_program(&prog).unwrap();

    assert_eq!(s.load(Src::Address12(config.addr_program)).unwrap(), 0x60);
    assert_eq!(s.load(Src::Address12(config.addr_program + 1)).unwrap(),
               0x55);
    // s.jump(config.addr_program as Address).unwrap();
    s.step().unwrap();
    assert_eq!(s.load(Src::PC).unwrap(), 0x202);
    assert_eq!(s.load(Src::Register(0)).unwrap(), 0x55);
    s.step().unwrap();
    assert_eq!(s.load(Src::PC).unwrap(), 0x200);
}


#[test]
fn test_fetch_equality_or() {
    // Octo equivalent:
    // : data 0x55 0xAA 0xA9
    // : main
    //    v3 := 0xFF
    //    i := data
    //    load v2
    //    v1 |= v0
    //    v2 |= v0
    //    if v3 == v1 then vA := 1
    //    if v3 != v1 then vB := 1
    //    if v3 == v2 then vC := 1
    //    if v3 != v2 then vD := 1
    let prog = [0x12, 0x05, 0x55, 0xAA, 0xA9, 0x63, 0xFF, 0xA2, 0x02, 0xF2, 0x65, 0x81, 0x01,
                0x82, 0x01, 0x93, 0x10, 0x6A, 0x01, 0x53, 0x10, 0x6B, 0x01, 0x93, 0x20, 0x6C,
                0x01, 0x53, 0x20, 0x6D, 0x01];
    let mut s = Simulator::new(&COSMAC_VIP, None);
    s.load_program(&prog).unwrap();

    s.step().unwrap();
    assert_eq!(s.load(Src::PC).unwrap(), 0x205);
    s.step().unwrap();
    assert_eq!(s.load(Src::Register(3)).unwrap(), 0xFF);
    s.step().unwrap();
    assert_eq!(s.load(Src::I).unwrap(), 0x202);
    s.step().unwrap();
    assert_eq!(s.load(Src::Register(0)).unwrap(), 0x55);
    assert_eq!(s.load(Src::Register(1)).unwrap(), 0xAA);
    assert_eq!(s.load(Src::Register(2)).unwrap(), 0xA9);
    s.step().unwrap();
    assert_eq!(s.load(Src::Register(1)).unwrap(), 0xFF);
    s.step().unwrap();
    assert_eq!(s.load(Src::Register(2)).unwrap(), 0xFD);
    s.step_n(6).unwrap();
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
    s.load_program(&prog).unwrap();

    assert_eq!(s.load(Src::Address12(config.addr_program)).unwrap(), 0x60);
    assert_eq!(s.load(Src::Address12(config.addr_program + 1)).unwrap(),
               0x55);
    // s.jump(config.addr_program as Address).unwrap();
    s.step().unwrap();
    assert_eq!(s.load(Src::PC).unwrap(), 0x202);
    assert_eq!(s.load(Src::Register(0)).unwrap(), 0x55);
    s.step().unwrap();
    assert_eq!(s.load(Src::PC).unwrap(), 0x200);
}

#[test]
fn test_add() {
    let config = COSMAC_VIP;
    let mut s = SimulatorTask::spawn(config);

    let prog = [0x64, 0x32, 0x67, 0xC8, 0x84, 0x74, 0x84, 0x74];    //v4 := 50, v7 := 200, v4 += v7, v4 += v7
    s.load_program(&prog).unwrap();

    s.step_n(3).unwrap();
    assert_eq!(s.load(Src::Register(4)).unwrap(), 250);
    assert_eq!(s.load(Src::Register(0xF)).unwrap(), 0);
    s.step().unwrap();
    assert_eq!(s.load(Src::Register(4)).unwrap(), 0xC2);
    assert_eq!(s.load(Src::Register(0xF)).unwrap(), 1);
}

#[test]
fn test_sprite_clear() {
    // Octo equivalent:
    // : main
    //   i := the_sprite
    //   v0 := 62
    //   v1 := 30
    //   sprite v0 v1 4
    //   v0 := 0
    //   v1 := 0
    //   sprite v0 v1 4
    //   clear
    // : the_sprite 0x50 0xA0 0x50 0xA0
    let prog = [0xA2, 0x10, 0x60, 0x3E, 0x61, 0x1E, 0xD0, 0x14, 0x60, 0x00, 0x61, 0x00, 0xD0,
                0x14, 0x00, 0xE0, 0x50, 0xA0, 0x50, 0xA0];

    let mut s = Simulator::new(&COSMAC_VIP, None);
    s.load_program(&prog).unwrap();

    s.step_n(4).unwrap();
    let vram = s.vram().unwrap();
    assert_eq!(vram[0 * 64 + 0], 0);
    assert_eq!(vram[0 * 64 + 1], 1);
    assert_eq!(vram[1 * 64 + 0], 1);
    assert_eq!(vram[31 * 64 + 62], 1);
    assert_eq!(vram[31 * 64 + 63], 0);
    assert_eq!(s.load(Src::Register(0xF)).unwrap(), 0);

    s.step_n(3).unwrap();
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
    s.step().unwrap();
    let vram = s.vram().unwrap();
    assert_eq!(vram[1 * 64 + 2], 0);
    assert_eq!(vram[2 * 64 + 1], 0);
    assert_eq!(vram[31 * 64 + 62], 0);
}

#[test]
fn test_sub() {
    let config = COSMAC_VIP;
    let mut s = SimulatorTask::spawn(config);

    // : main
    // v0 := 0x20
    // v1 := 0x10
    // v0 -= v1
    // v0 -= v1
    // v0 -= v1
    // vA := 0x20
    // vB := 0x40
    // vA =- vB
    // vB =- vA
    let prog = [0x60, 0x20, 0x61, 0x10, 0x80, 0x15, 0x80, 0x15, 0x80, 0x15, 0x6A, 0x20, 0x6B,
                0x40, 0x8A, 0xB7, 0x8B, 0xA7];
    s.load_program(&prog).unwrap();

    s.step_n(3).unwrap();
    assert_eq!(s.load(Src::Register(0)).unwrap(), 0x10);
    assert_eq!(s.load(Src::Register(1)).unwrap(), 0x10);
    assert_eq!(s.load(Src::Register(0xF)).unwrap(), 1);
    s.step().unwrap();
    assert_eq!(s.load(Src::Register(0)).unwrap(), 0x00);
    assert_eq!(s.load(Src::Register(1)).unwrap(), 0x10);
    assert_eq!(s.load(Src::Register(0xF)).unwrap(), 1);
    s.step().unwrap();
    assert_eq!(s.load(Src::Register(0)).unwrap(), 0xF0);
    assert_eq!(s.load(Src::Register(1)).unwrap(), 0x10);
    assert_eq!(s.load(Src::Register(0xF)).unwrap(), 0);
    s.step_n(3).unwrap();
    assert_eq!(s.load(Src::Register(0xA)).unwrap(), 0x20);
    assert_eq!(s.load(Src::Register(0xB)).unwrap(), 0x40);
    assert_eq!(s.load(Src::Register(0xF)).unwrap(), 1);
    s.step().unwrap();
    assert_eq!(s.load(Src::Register(0xA)).unwrap(), 0x20);
    assert_eq!(s.load(Src::Register(0xB)).unwrap(), 0xE0);
    assert_eq!(s.load(Src::Register(0xF)).unwrap(), 0);
}


#[test]
fn test_stash_fetch() {
    // Octo equivalent:
    // : main
    // v0 := 1
    // v1 := 2
    // v2 := 5
    // i := storage
    // save v2
    // v0 := 10
    // v1 := 20
    // v2 := 30
    // v3 := 40
    // load v1
    // i := storage
    // load v1
    // i := storage
    // load v2
    // vA := 66
    // vB := 77
    // vC := 88
    // i := storage
    // save vA - vC
    // vA := 0
    // vB := 0
    // vC := 0
    // load vA - vC
    // load vB - vB
    // load vB - vB
    // : storage 0 0 0
    let prog = [0x60, 0x01, 0x61, 0x02, 0x62, 0x05, 0xA2, 0x32, 0xF2, 0x55, 0x60, 0x0A, 0x61,
                0x14, 0x62, 0x1E, 0x63, 0x28, 0xF1, 0x65, 0xA2, 0x32, 0xF1, 0x65, 0xA2, 0x32,
                0xF2, 0x65, 0x6A, 0x42, 0x6B, 0x4D, 0x6C, 0x58, 0xA2, 0x32, 0x5A, 0xC2, 0x6A,
                0x00, 0x6B, 0x00, 0x6C, 0x00, 0x5A, 0xC3, 0x5B, 0xB3, 0x5B, 0xB3, 0x00, 0x00, 0x00];
    let mut config = COSMAC_VIP;
    config.isa_xochip = true;
    let mut s = Simulator::new(&config, None);

    s.load_program(&prog).unwrap();

    s.step_n(10).unwrap();
    assert_eq!(s.load(Src::Register(0)).unwrap(), 0);
    assert_eq!(s.load(Src::Register(1)).unwrap(), 0);
    assert_eq!(s.load(Src::Register(2)).unwrap(), 0x1E);
    assert_eq!(s.load(Src::Register(3)).unwrap(), 0x28);
    assert_eq!(s.load(Src::I).unwrap(), 0x237);
    s.step_n(2).unwrap();
    assert_eq!(s.load(Src::Register(0)).unwrap(), 1);
    assert_eq!(s.load(Src::Register(1)).unwrap(), 2);
    assert_eq!(s.load(Src::Register(2)).unwrap(), 0x1E);
    assert_eq!(s.load(Src::Register(3)).unwrap(), 0x28);
    assert_eq!(s.load(Src::I).unwrap(), 0x234);
    s.step_n(2).unwrap();
    assert_eq!(s.load(Src::Register(0)).unwrap(), 1);
    assert_eq!(s.load(Src::Register(1)).unwrap(), 2);
    assert_eq!(s.load(Src::Register(2)).unwrap(), 5);
    assert_eq!(s.load(Src::Register(3)).unwrap(), 0x28);
    assert_eq!(s.load(Src::I).unwrap(), 0x235);
    s.step_n(9).unwrap();
    assert_eq!(s.load(Src::Register(0xA)).unwrap(), 0x42);
    assert_eq!(s.load(Src::Register(0xB)).unwrap(), 0x4D);
    assert_eq!(s.load(Src::Register(0xC)).unwrap(), 0x58);
    assert_eq!(s.load(Src::I).unwrap(), 0x232);
    s.step().unwrap();
    assert_eq!(s.load(Src::Register(0xA)).unwrap(), 0x42);
    assert_eq!(s.load(Src::Register(0xB)).unwrap(), 0x42);
    assert_eq!(s.load(Src::Register(0xC)).unwrap(), 0x58);
    assert_eq!(s.load(Src::I).unwrap(), 0x232);
    s.step().unwrap();
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
    let prog = [0x60, 0x55, 0x61, 0xAA, 0x62, 0x00, 0x82, 0x02, 0x81, 0x02, 0x61, 0xAA, 0x80,
                0x13, 0x80, 0x03];
    let mut s = Simulator::new(&COSMAC_VIP, None);
    s.load_program(&prog).unwrap();

    s.step_n(5).unwrap();
    assert_eq!(s.load(Src::Register(0)).unwrap(), 0x55);
    assert_eq!(s.load(Src::Register(1)).unwrap(), 0x00);
    assert_eq!(s.load(Src::Register(2)).unwrap(), 0x00);
    s.step_n(2).unwrap();
    assert_eq!(s.load(Src::Register(0)).unwrap(), 0xFF);
    assert_eq!(s.load(Src::Register(1)).unwrap(), 0xAA);
    s.step().unwrap();
    assert_eq!(s.load(Src::Register(0)).unwrap(), 0x00);
}

#[test]
fn test_call_return_jump_jumpv0() {
    // : main
    // v0 := 1
    // v1 := 3
    // v3 := 2
    // v4 := 7
    // sub1
    // sub2
    // jump0 table
    // : wait-here
    // wait-here
    // : sub1
    // v3 <<= v3
    // return
    // : sub2
    // v0 >>= v3
    // return
    // : table
    // vA := 10
    // vA := 20
    // vA := 30
    // vA := 40
    // jump wait-here
    let prog = [0x60, 0x01, 0x61, 0x03, 0x63, 0x02, 0x64, 0x07, 0x22, 0x10, 0x22, 0x14, 0xB2,
                0x18, 0x22, 0x0E, 0x83, 0x3E, 0x00, 0xEE, 0x80, 0x36, 0x00, 0xEE, 0x6A, 0x0A,
                0x6A, 0x14, 0x6A, 0x1E, 0x6A, 0x28, 0x12, 0x0E];
    let mut s = Simulator::new(&COSMAC_VIP, None);
    s.load_program(&prog).unwrap();

    s.step_n(6).unwrap();
    assert_eq!(s.load(Src::Register(3)).unwrap(), 0x4);
    assert_eq!(s.load(Src::PC).unwrap(), 0x212);
    s.step_n(3).unwrap();
    assert_eq!(s.load(Src::Register(0)).unwrap(), 2);
    assert_eq!(s.load(Src::PC).unwrap(), 0x216);
    s.step_n(2).unwrap();
    assert_eq!(s.load(Src::PC).unwrap(), 0x21A);
    s.step_n(3).unwrap();
    assert_eq!(s.load(Src::PC).unwrap(), 0x220);
    s.step().unwrap();
    assert_eq!(s.load(Src::PC).unwrap(), 0x20E);
}

#[test]
fn test_random_threadrng() {
    // Looping 256 times means each bit has ~10^-78 chance of not being set. Should be good.
    // : main
    // v0 := 0
    // v2 := 0
    // : theloop
    // v1 := random 0x55
    // v0 |= v1
    // v2 += 1
    // if v2 != 0 then jump theloop
    // v3 := 0
    // : theloop2
    // v1 := random 0xFF
    // v3 |= v1
    // v2 += 1
    // if v2 != 0 then jump theloop2
    // v3 ^= v0
    let prog = [0x60, 0x00, 0x62, 0x00, 0xC1, 0x55, 0x80, 0x11, 0x72, 0x01, 0x32, 0x00, 0x12,
                0x04, 0x63, 0x00, 0xC1, 0xFF, 0x83, 0x11, 0x72, 0x01, 0x32, 0x00, 0x12, 0x10,
                0x83, 0x03];
    let mut s = Simulator::new(&COSMAC_VIP, None);
    s.load_program(&prog).unwrap();

    s.step_n(1281).unwrap();
    assert_eq!(s.load(Src::Register(0)).unwrap(), 0x55);
    assert_eq!(s.load(Src::PC).unwrap(), 0x20E);
    s.step_n(1280).unwrap();
    assert_eq!(s.load(Src::Register(3)).unwrap(), 0xFF);
    assert_eq!(s.load(Src::PC).unwrap(), 0x21A);
    s.step().unwrap();
    assert_eq!(s.load(Src::Register(3)).unwrap(), 0xAA);
}

#[test]
fn test_random_provided() {
    // : main
    // v0 := random 0x55
    // v1 := random 0xAA
    // v2 := random 0xFF
    // v3 := random 0x00
    // v4 := random 0xFF
    let prog = [0xC0, 0x55, 0xC1, 0xAA, 0xC2, 0xFF, 0xC3, 0x00, 0xC4, 0xFF];
    // let mut random_values: Vec<MemoryCell> = Vec::new();
    // random_values.push(0xF0);
    // random_values.push(0x0F);
    // random_values.push(0x23);
    // random_values.push(0xFF);
    let mut random_values: VecDeque<u8> = VecDeque::new();
    random_values.push_back(0xF0);
    random_values.push_back(0x0F);
    random_values.push_back(0x23);
    random_values.push_back(0xFF);
    let mut s = Simulator::new(&COSMAC_VIP, Some(random_values));
    s.load_program(&prog).unwrap();

    s.step_n(5).unwrap();
    assert_eq!(s.load(Src::Register(0)).unwrap(), 0x50);
    assert_eq!(s.load(Src::Register(1)).unwrap(), 0x0A);
    assert_eq!(s.load(Src::Register(2)).unwrap(), 0x23);
    assert_eq!(s.load(Src::Register(3)).unwrap(), 0x00);
    assert_eq!(s.load(Src::Register(4)).unwrap(), 0x00);

}

#[test]
fn test_skip_key() {
    // : main
    // v1 := 5
    // if v1 key then jump main
    // v3 := 8
    // : notkey
    // if v3 -key then jump notkey
    // : theend
    // jump theend
    let prog = [0x61, 0x05, 0xE1, 0xA1, 0x12, 0x00, 0x63, 0x08, 0xE3, 0x9E, 0x12, 0x08, 0x12, 0x0C];
    let mut s = Simulator::new(&COSMAC_VIP, None);
    let mut keys: Keyboard = [false; 16];
    keys[5] = true;
    s.load_program(&prog).unwrap();
    s.set_keyboard(&keys);
    s.step_n(13).unwrap();
    assert_eq!(s.load(Src::PC).unwrap(), 0x202);
    keys[5] = false;
    s.set_keyboard(&keys);
    s.step_n(13).unwrap();
    assert_eq!(s.load(Src::PC).unwrap(), 0x20A);
    keys[8] = true;
    s.set_keyboard(&keys);
    s.step_n(2).unwrap();
    assert_eq!(s.load(Src::PC).unwrap(), 0x20C);

}

#[test]
fn test_bcd_font() {
    // : digits 0 0 0
    // : main
    // i := digits
    // v4 := 123
    // bcd v4
    // load v2
    // v5 := 0
    // v6 := 0
    // i := hex v0
    // sprite v5 v6 5
    // v5 := 5
    // i := hex v1
    // sprite v5 v6 5
    // v5 := 11
    // i := hex v2
    // sprite v5 v6 5
    // : the_end
    // jump the_end
    let prog = [0x12, 0x05, 0x00, 0x00, 0x00, 0xA2, 0x02, 0x64, 0x7B, 0xF4, 0x33, 0xF2, 0x65, 0x65, 0x00, 0x66, 0x00, 0xF0, 0x29, 0xD5, 0x65, 0x65, 0x05, 0xF1, 0x29, 0xD5, 0x65, 0x65, 0x0B, 0xF2, 0x29, 0xD5, 0x65, 0x12, 0x21];
    let mut s = Simulator::new(&COSMAC_VIP, None);
    s.load_program(&prog).unwrap();
    s.step_n(50);
    let vram = s.vram().unwrap();
    for line in 0..6 {
        println!("{:?}", &vram[line*64..(line+1)*64]);
    }
    assert_eq!(vram[0 * 64 + 0], 0);
    assert_eq!(vram[0 * 64 + 1], 0);
    assert_eq!(vram[0 * 64 + 2], 1);
    assert_eq!(vram[1 * 64 + 0], 0);
    assert_eq!(vram[1 * 64 + 1], 1);
    assert_eq!(vram[1 * 64 + 2], 1);
    assert_eq!(vram[2 * 64 + 0], 0);
    assert_eq!(vram[2 * 64 + 1], 0);
    assert_eq!(vram[2 * 64 + 2], 1);
    assert_eq!(vram[0 * 64 + 13], 1);
    assert_eq!(vram[0 * 64 + 14], 1);
    assert_eq!(vram[1 * 64 + 13], 0);
    assert_eq!(vram[1 * 64 + 14], 1);
    assert_eq!(vram[2 * 64 + 13], 1);
    assert_eq!(vram[2 * 64 + 14], 1);
}
