//! Generic 'microcode' implementations of the operations.

use instruction::{Instruction};
use instruction::execution::{Execute};
#[allow(unused_imports)]
use config::Config;
use types::*;

/// Add src to dst and store in dst.
pub fn op_add(inst: &Instruction, core: &mut Execute) {
    let lhs = core.load(inst.dest());
    let rhs = core.load(inst.src());
    let total = lhs + rhs;
    core.set_flag(total > 0xFF);  //set vF if result overflows
    core.store(inst.dest(), total);
}

/// Subtract src from dest, store in dest. Set flag if NOT borrow
pub fn op_sub(inst: &Instruction, core: &mut Execute) {
    let lhs = core.load(inst.dest());
    let rhs = core.load(inst.src());
    let total = (lhs - rhs) & 0xFF;

    core.set_flag(lhs > rhs);  //set vF if NOT borrow
    core.store(inst.dest(), total);
}

/// Subtract dest from src, store in dest. Set flag if NOT borrow
pub fn op_subn(inst: &Instruction, core: &mut Execute) {
    let lhs = core.load(inst.src());
    let rhs = core.load(inst.dest());
    let total = (lhs - rhs) & 0xFF;

    core.set_flag(lhs > rhs);  //set vF if NOT borrow
    core.store(inst.dest(), total);
}

/// Or src with dest and store in dest.
pub fn op_or(inst: &Instruction, core: &mut Execute) {
    let lhs = core.load(inst.dest());
    let rhs = core.load(inst.src());
    let result = lhs | rhs;
    core.store(inst.dest(), result);
}

/// And src with dest and store in dest.
pub fn op_and(inst: &Instruction, core: &mut Execute) {
    let lhs = core.load(inst.dest());
    let rhs = core.load(inst.src());
    let result = lhs & rhs;
    core.store(inst.dest(), result);
}

/// Xor src with dest and store in dest.
pub fn op_xor(inst: &Instruction, core: &mut Execute) {
    let lhs = core.load(inst.dest());
    let rhs = core.load(inst.src());
    let result = lhs ^ rhs;
    core.store(inst.dest(), result);
}

/// Shifts the source right 1 bit, and stores in dest. vF set to old LSB
pub fn op_shr(inst: &Instruction, core: &mut Execute) {
    let val = core.load(inst.src());
    let carry = (val & 1) == 1;
    let result = val >> 1;
    core.store(inst.dest(), result);
    core.set_flag(carry);
}

/// Shifts the source left 1 bit, and stores in dest. vF set to old MSB
pub fn op_shl(inst: &Instruction, core: &mut Execute) {
    let val = core.load(inst.src());
    let carry = (val & 0x80) == 0x80;
    let result = (val << 1) & 0xFF;
    core.store(inst.dest(), result);
    core.set_flag(carry);
}

/// Copy src to dest.
pub fn op_load(inst: &Instruction, core: &mut Execute) {
    let data = core.load(inst.src());
    core.store(inst.dest(), data);
}

/// Set I to the first byte of the glyph specified in the system font.
pub fn op_font(inst: &Instruction, core: &mut Execute) {
    let addr = core.config().addr_font + core.load(inst.src()) * 5;
    core.store(inst.dest(), addr);
}

/// Set I[0...2] to the BCD representation of src.
pub fn op_bcd(inst: &Instruction, core: &mut Execute) {
    let mut val = core.load(inst.src());
    let hundreds = val / 100;
    val -= hundreds * 100;
    let tens = val / 10;
    let ones = val - tens * 10;

    assert!(hundreds < 10);
    assert!(tens < 10);
    assert!(ones < 10);

    let inital_i = core.load(Operand::I);      //store I so it can be restored
    core.store(inst.dest(), hundreds);
    core.store(Operand::I, inital_i + 1);
    core.store(inst.dest(), tens);
    core.store(Operand::I, inital_i + 2);
    core.store(inst.dest(), ones);
    core.store(Operand::I, inital_i);
}

/// Set dest to a src(a random number) masked with aux.
pub fn op_rand(inst: &Instruction, core: &mut Execute) {
    let mask = core.load(inst.src());
    let data = core.load(inst.aux()) & mask;
    core.store(inst.dest(), data);
}

/// Skips the next instruction if src == dest.
pub fn op_skipeq(inst: &Instruction, core: &mut Execute) {
    let lhs = core.load(inst.dest());
    let rhs = core.load(inst.src());
    if lhs == rhs {
        core.advance_pc();
    }
}

/// Skips the next instruction if src != dest.
pub fn op_skipneq(inst: &Instruction, core: &mut Execute) {
    let lhs = core.load(inst.dest());
    let rhs = core.load(inst.src());
    if lhs != rhs {
        core.advance_pc();
    }
}

/// Skips the next instruction if the key in dest is currently pressed.
#[allow(unused_variables)]
pub fn op_skipkey(inst: &Instruction, core: &mut Execute) {
    /*let key = core.load(inst.dest());
    let mut key_state = false;
    if let Ok(keys) = core.state().keys.read() {
        key_state = keys.is_down(key);
        drop(keys)
    } else {
        // TODO: log this
    }
    if key_state {
        core.advance_pc();
    }
    */
}

#[allow(unused_variables)]
/// Skips the next instruction if the key in dest is currently not pressed.
pub fn op_skipnkey(inst: &Instruction, core: &mut Execute) {
    /*let key = core.load(inst.dest()) ;
    let mut key_state = false;
    if let Ok(keys) = core.state().keys.read() {
        key_state = keys.is_down(key);
        drop(keys)
    } else {
        // TODO: log this
    }
    if !key_state {
        core.advance_pc();
    }*/
}

/// Halt execution until a key is pressed.
#[allow(unused_variables)]
pub fn op_waitkey(inst: &Instruction, core: &mut Execute) {
    panic!("WaitKey Unimplemented")
}



/// Return
#[allow(unused_variables)]
pub fn op_ret(inst: &Instruction, core: &mut Execute) {
    if let Some(addr) = core.stack_pop() {
        core.jump(addr);
    } else {
        //TODO: There should probably be some kind of log/output
    }
}


/// Clear the screen.
#[allow(unused_variables)]
pub fn op_cls(inst: &Instruction, core: &mut Execute) {
    /*if let Ok(mut vram) = core.state().vram.write() {
        vram.pixels = [[0; 32]; 64];
        drop(vram);
    } else {
        // TODO: log this or?
    }*/
}

#[allow(unused_variables)]
/// Draw a sprite.
pub fn op_sprite(inst: &Instruction, core: &mut Execute) {
    /*
    let x = core.load(inst.dest());
    let mut y = core.load(inst.src());
    let n = core.load(inst.aux());

    let mut i = core.load(Operand::I)();

    let mut pixels: [[u8;32]; 64];
    pixels = [[0; 32]; 64];
    if let Ok(vram) = core.state().vram.read() {
        local_vram = vram.clone();//pixels = vram.pixels;
        drop(vram);
    } else {
        local_vram = Vram::default();
        //pixels = [[0; 32]; 64];
        //TODO: log this
    }

    core.vf_clear();
    for byte in 0..n {
        core.ram(i);
        for bit in 0..8 {
            let pixel = if byte & (0x80 >> bit) == 0 {
                0
            } else {
                1
            };
            let x_loc = ((x + bit) & 63) ;
            let y_loc = (y & 31) ;
            pixels[x_loc][y_loc] ^= pixel;
            if pixels[x_loc][y_loc] == 0 && pixel == 1 {
                core.vf_set();
            }
        }
        i += 1;
        y += 1;
    }

    let mut vram: Vram;
    if let Ok(mut vram) = core.state().vram.write() {
        vram = local_vram;//.clone();
        drop(vram);
    } else {
        //TODO: log this
    }
    */
}

/// Stores registers v0.. to ram[I]
pub fn op_stash(inst: &Instruction, core: &mut Execute) {
    let last = if let Operand::Register(r) = inst.src() {
        r
    } else {
        panic!("Fetch only works with a register");
    };
    let i = core.load(Operand::I);
    for r in 0...last {
        let value = core.load(Operand::Register(r));
        core.store(Operand::Address12(i + r), value);
    }
    core.store(Operand::I, i + last + 1);
}

/// Fetches several bytes, pointed to by I, into v0..
pub fn op_fetch(inst: &Instruction, core: &mut Execute) {
    let last = if let Operand::Register(r) = inst.dest() {
        r
    } else {
        panic!("Fetch only works with a register");
    };
    let i = core.load(Operand::I);
    for r in 0...last {
        let v = core.load(Operand::Address12(i + r));
        core.store(Operand::Register(r), v);
    }
    core.store(Operand::I, i + last + 1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use Chip8;
    use Config;
    use instruction::{Execute, Instruction, Operand};
    use instruction::Operation::*;
    use instruction::execution::execute_microcode;
    #[test]
    fn test_add_reg() {
        let mut core = Chip8::default();
        let inst = Instruction::new(Add, Operand::Register(0), Operand::Register(1), Operand::Nowhere);

        core.store(Operand::Register(0), 5);
        core.store(Operand::Register(1), 10);
        execute_microcode(&inst, &mut core);
        assert_eq!(core.load(Operand::Register(0)), 15);
        assert_eq!(core.load(Operand::Register(1)), 10);
        assert_eq!(core.load(Operand::Register(0xF)), 0);

        //with overflow
        core.store(Operand::Register(0), 0xFF);
        op_add(&inst, &mut core);
        assert_eq!(core.load(Operand::Register(0)), 9);
        assert_eq!(core.load(Operand::Register(1)), 10);
        assert_eq!(core.load(Operand::Register(0xF)), 1);
    }
}
