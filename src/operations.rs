use emulator::Emulator;
use operand::Operand;
use instruction::Instruction;


pub type Operation = fn(&Instruction, &mut Emulator);

pub fn op_add(inst: &Instruction, core: &mut Emulator) {
    let lhs = core.load(inst.dest());
    let rhs = core.load(inst.src());
    let total = lhs + rhs;
    core.vf_store(total > 0xFF);  //set vF if result overflows
    core.store(inst.dest(), total);
}

/// Subtract src from dest, store in dest. Set flag if NOT borrow
pub fn op_sub(inst: &Instruction, core: &mut Emulator) {
    let lhs = core.load(inst.dest()) as i32;
    let rhs = core.load(inst.src()) as i32;
    let total = ((lhs - rhs) as u32) & 0xFF;

    core.vf_store(lhs > rhs);  //set vF if NOT borrow
    core.store(inst.dest(), total);
}

/// Subtract dest from src, store in dest. Set flag if NOT borrow
pub fn op_subn(inst: &Instruction, core: &mut Emulator) {
    let lhs = core.load(inst.src()) as i32;
    let rhs = core.load(inst.dest()) as i32;
    let total = ((lhs - rhs) as u32) & 0xFF;

    core.vf_store(lhs > rhs);  //set vF if NOT borrow
    core.store(inst.dest(), total);
}


pub fn op_or(inst: &Instruction, core: &mut Emulator) {
    let lhs = core.load(inst.dest());
    let rhs = core.load(inst.src());
    let result = lhs | rhs;
    core.store(inst.dest(), result);
}

pub fn op_and(inst: &Instruction, core: &mut Emulator) {
    let lhs = core.load(inst.dest());
    let rhs = core.load(inst.src());
    let result = lhs & rhs;
    core.store(inst.dest(), result);
}

pub fn op_xor(inst: &Instruction, core: &mut Emulator) {
    let lhs = core.load(inst.dest());
    let rhs = core.load(inst.src());
    let result = lhs ^ rhs;
    core.store(inst.dest(), result);
}

/// Shifts the source right 1 bit, and stores in dest. vF set to old LSB
pub fn op_shr(inst: &Instruction, core: &mut Emulator) {
    let val = core.load(inst.src());
    let carry = (val & 1) == 1;
    let result = val >> 1;
    core.store(inst.dest(), result);
    core.vf_store(carry);
}

/// Shifts the source left 1 bit, and stores in dest. vF set to old MSB
pub fn op_shl(inst: &Instruction, core: &mut Emulator) {
    let val = core.load(inst.src());
    let carry = (val & 0x80) == 0x80;
    let result = (val << 1) & 0xFF;
    core.store(inst.dest(), result);
    core.vf_store(carry);
}



pub fn op_load(inst: &Instruction, core: &mut Emulator) {
    let data = core.load(inst.src());
    core.store(inst.dest(), data);
}

pub fn op_font(inst: &Instruction, core: &mut Emulator) {
    let addr = core.config.font_addr as u32 + core.load(inst.src()) * 5;
    core.store(inst.dest(), addr);
}

pub fn op_bcd(inst: &Instruction, core: &mut Emulator) {
    let val = core.load(inst.src());
    let hundreds = val / 100;
    let val = val - hundreds * 100;
    let tens = val / 10;
    let ones = val - tens * 10;

    assert!(hundreds < 10);
    assert!(tens < 10);
    assert!(ones < 10);

    let inital_i = core.i();      //store I so it can be restored
    core.store(inst.dest(), hundreds);
    core.set_i(inital_i + 1);
    core.store(inst.dest(), tens);
    core.set_i(inital_i + 2);
    core.store(inst.dest(), ones);
    core.set_i(inital_i);
}


pub fn op_rand(inst: &Instruction, core: &mut Emulator) {
    let mask = core.load(inst.src());
    let data = core.rng.next_u32() & mask;
    core.store(inst.dest(), data);
}



pub fn op_skipeq(inst: &Instruction, core: &mut Emulator) {
    let lhs = core.load(inst.dest());
    let rhs = core.load(inst.src());
    if lhs == rhs {
        core.advance_pc();
    }
}

pub fn op_skipneq(inst: &Instruction, core: &mut Emulator) {
    let lhs = core.load(inst.dest());
    let rhs = core.load(inst.src());
    if lhs != rhs {
        core.advance_pc();
    }
}

pub fn op_skipkey(inst: &Instruction, core: &mut Emulator) {
    let key = core.load(inst.dest()) as usize;
    let mut key_state = false;
    {
        let keys = core.state.keys.read().unwrap();
        key_state = keys.is_down(key);
    }
    if key_state {
        core.advance_pc();
    }
}

pub fn op_skipnkey(inst: &Instruction, core: &mut Emulator) {
    let key = core.load(inst.dest()) as usize;
    let mut key_state = false;
    {
        let keys = core.state.keys.read().unwrap();
        key_state = keys.is_down(key);
    }
    if !key_state {
        core.advance_pc();
    }
}

#[allow(unused_variables)]
pub fn op_waitkey(inst: &Instruction, core: &mut Emulator) {
    panic!("WaitKey Unimplemented")
}


/// Jump to address
pub fn op_jump(inst: &Instruction, core: &mut Emulator) {
    let addr = core.load(inst.dest()) as usize;
    core.jump_pc(addr);
}

/// Jump to address + V0
pub fn op_jumpv0(inst: &Instruction, core: &mut Emulator) {
    let mut addr = core.load(inst.dest()) as usize;
    addr += core.reg(0) as usize;
    core.jump_pc(addr);
}


pub fn op_call(inst: &Instruction, core: &mut Emulator) {
    let addr = core.load(inst.dest()) as usize;
    let pc = core.pc();
    core.stack.push(pc);
    core.jump_pc(addr);
}

#[allow(unused_variables)]
pub fn op_ret(inst: &Instruction, core: &mut Emulator) {
    let addr = core.stack.pop().unwrap();
    core.jump_pc(addr);
}


#[allow(unused_variables)]
pub fn op_cls(inst: &Instruction, core: &mut Emulator) {
    let mut vram = core.state.vram.write().unwrap();

    vram.pixels = [[0; 32]; 64];
}

pub fn op_sprite(inst: &Instruction, core: &mut Emulator) {

    let x = core.load(inst.dest());
    let mut y = core.load(inst.src());
    let n = core.load(inst.aux());

    let mut i = core.i();

    let mut pixels = core.state.vram.read().unwrap().pixels;

    // println!("sprite: x{} y{} n{} i{:X}", x, y, n, i );
    core.vf_clear();
    for _ in 0..n {
        let byte = core.ram(i);
        for bit in 0..8 {
            let pixel = if byte & (0x80 >> bit) == 0 {
                0
            } else {
                1
            };
            let x_loc = ((x + bit) & 63) as usize;
            let y_loc = (y & 31) as usize;
            pixels[x_loc][y_loc] ^= pixel;
            if pixels[x_loc][y_loc] == 0 && pixel == 1 {
                core.vf_set();
            }
        }
        i += 1;
        y += 1;
    }

    let mut vram = core.state.vram.write().unwrap();
    vram.pixels = pixels;

}

pub fn op_stash(inst: &Instruction, core: &mut Emulator) {
    let last = match inst.src() {
        Operand::Register(r) => r,
        _ => {
            panic!("Fetch only works with a register");
        }
    };
    let i = core.i();
    for r in 0...last {
        let value = core.reg(r) as u8;
        core.set_ram(i + r, value);
    }
    core.set_i(i + last + 1);
}

pub fn op_fetch(inst: &Instruction, core: &mut Emulator) {
    let last = match inst.dest() {
        Operand::Register(r) => r,
        _ => {
            panic!("Fetch only works with a register");
        }
    };
    let i = core.i();
    for r in 0...last {
        let v = core.ram(i + r);
        core.set_reg(r, v);
    }
    core.set_i(i + last + 1);
}
