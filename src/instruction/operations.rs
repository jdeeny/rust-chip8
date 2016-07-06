use simulator::Simulator;
use instruction::{Instruction, Operand};

pub type Operation = fn(&Instruction, &mut Simulator);

pub fn op_add(inst: &Instruction, core: &mut Simulator) {
    let lhs = core.load(inst.dest());
    let rhs = core.load(inst.src());
    let total = lhs + rhs;
    core.vf_store(total > 0xFF);  //set vF if result overflows
    core.store(inst.dest(), total);
}

/// Subtract src from dest, store in dest. Set flag if NOT borrow
pub fn op_sub(inst: &Instruction, core: &mut Simulator) {
    let lhs = core.load(inst.dest()) as i32;
    let rhs = core.load(inst.src()) as i32;
    let total = ((lhs - rhs) as u32) & 0xFF;

    core.vf_store(lhs > rhs);  //set vF if NOT borrow
    core.store(inst.dest(), total);
}

/// Subtract dest from src, store in dest. Set flag if NOT borrow
pub fn op_subn(inst: &Instruction, core: &mut Simulator) {
    let lhs = core.load(inst.src()) as i32;
    let rhs = core.load(inst.dest()) as i32;
    let total = ((lhs - rhs) as u32) & 0xFF;

    core.vf_store(lhs > rhs);  //set vF if NOT borrow
    core.store(inst.dest(), total);
}

pub fn op_or(inst: &Instruction, core: &mut Simulator) {
    let lhs = core.load(inst.dest());
    let rhs = core.load(inst.src());
    let result = lhs | rhs;
    core.store(inst.dest(), result);
}

pub fn op_and(inst: &Instruction, core: &mut Simulator) {
    let lhs = core.load(inst.dest());
    let rhs = core.load(inst.src());
    let result = lhs & rhs;
    core.store(inst.dest(), result);
}

pub fn op_xor(inst: &Instruction, core: &mut Simulator) {
    let lhs = core.load(inst.dest());
    let rhs = core.load(inst.src());
    let result = lhs ^ rhs;
    core.store(inst.dest(), result);
}

/// Shifts the source right 1 bit, and stores in dest. vF set to old LSB
pub fn op_shr(inst: &Instruction, core: &mut Simulator) {
    let val = core.load(inst.src());
    let carry = (val & 1) == 1;
    let result = val >> 1;
    core.store(inst.dest(), result);
    core.vf_store(carry);
}

/// Shifts the source left 1 bit, and stores in dest. vF set to old MSB
pub fn op_shl(inst: &Instruction, core: &mut Simulator) {
    let val = core.load(inst.src());
    let carry = (val & 0x80) == 0x80;
    let result = (val << 1) & 0xFF;
    core.store(inst.dest(), result);
    core.vf_store(carry);
}

pub fn op_load(inst: &Instruction, core: &mut Simulator) {
    let data = core.load(inst.src());
    core.store(inst.dest(), data);
}

pub fn op_font(inst: &Instruction, core: &mut Simulator) {
    let addr = core.config.sys_font_addr as u32 + core.load(inst.src()) * 5;
    core.store(inst.dest(), addr);
}

pub fn op_bcd(inst: &Instruction, core: &mut Simulator) {
    let mut val = core.load(inst.src());
    let hundreds = val / 100;
    val -= hundreds * 100;
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


pub fn op_rand(inst: &Instruction, core: &mut Simulator) {
    let mask = core.load(inst.src());
    let data = core.load(inst.aux()) & mask;
    core.store(inst.dest(), data);
}



pub fn op_skipeq(inst: &Instruction, core: &mut Simulator) {
    let lhs = core.load(inst.dest());
    let rhs = core.load(inst.src());
    if lhs == rhs {
        core.advance_pc();
    }
}

pub fn op_skipneq(inst: &Instruction, core: &mut Simulator) {
    let lhs = core.load(inst.dest());
    let rhs = core.load(inst.src());
    if lhs != rhs {
        core.advance_pc();
    }
}

pub fn op_skipkey(inst: &Instruction, core: &mut Simulator) {
    let key = core.load(inst.dest()) as usize;
    let mut key_state = false;
    if let Ok(keys) = core.state.keys.read() {
        key_state = keys.is_down(key);
        drop(keys)
    } else {
        // TODO: log this
    }
    if key_state {
        core.advance_pc();
    }
}

pub fn op_skipnkey(inst: &Instruction, core: &mut Simulator) {
    let key = core.load(inst.dest()) as usize;
    let mut key_state = false;
    if let Ok(keys) = core.state.keys.read() {
        key_state = keys.is_down(key);
        drop(keys)
    } else {
        // TODO: log this
    }
    if !key_state {
        core.advance_pc();
    }
}

#[allow(unused_variables)]
pub fn op_waitkey(inst: &Instruction, core: &mut Simulator) {
    panic!("WaitKey Unimplemented")
}

/// Jump to address
pub fn op_jump(inst: &Instruction, core: &mut Simulator) {
    let addr = core.load(inst.dest()) as usize;
    core.jump_pc(addr);
}

/// Jump to address + V0
pub fn op_jumpv0(inst: &Instruction, core: &mut Simulator) {
    let mut addr = core.load(inst.dest()) as usize;
    addr += core.reg(0) as usize;
    core.jump_pc(addr);
}

pub fn op_call(inst: &Instruction, core: &mut Simulator) {
    let addr = core.load(inst.dest()) as usize;
    let pc = core.pc();
    core.stack.push(pc);
    core.jump_pc(addr);
}

#[allow(unused_variables)]
pub fn op_ret(inst: &Instruction, core: &mut Simulator) {
    if let Some(addr) = core.stack.pop() {
        core.jump_pc(addr);
    } else {
        //TODO: There should probably be some kind of log/output
    }
}


#[allow(unused_variables)]
pub fn op_cls(inst: &Instruction, core: &mut Simulator) {
    if let Ok(mut vram) = core.state.vram.write() {
        vram.pixels = [[0; 32]; 64];
        drop(vram);
    } else {
        // TODO: log this or?
    }
}

pub fn op_sprite(inst: &Instruction, core: &mut Simulator) {

    let x = core.load(inst.dest());
    let mut y = core.load(inst.src());
    let n = core.load(inst.aux());

    let mut i = core.i();

    let mut pixels: [[u8;32]; 64];

    if let Ok(vram) = core.state.vram.read() {
        pixels = vram.pixels;
        drop(vram);
    } else {
        pixels = [[0; 32]; 64];
        //TODO: log this
    }

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

    if let Ok(mut vram) = core.state.vram.write() {
        vram.pixels = pixels;
        drop(vram);
    } else {
        //TODO: log this
    }

}

pub fn op_stash(inst: &Instruction, core: &mut Simulator) {
    let last = if let Operand::Register(r) = inst.src() {
        r
    } else {
        panic!("Fetch only works with a register");
    };
    let i = core.i();
    for r in 0...last {
        let value = core.reg(r);
        core.set_ram(i + r, value);
    }
    core.set_i(i + last + 1);
}

pub fn op_fetch(inst: &Instruction, core: &mut Simulator) {
    let last = if let Operand::Register(r) = inst.dest() {
        r
    } else {
        panic!("Fetch only works with a register");
    };
    let i = core.i();
    for r in 0...last {
        let v = core.ram(i + r);
        core.set_reg(r, v);
    }
    core.set_i(i + last + 1);
}
