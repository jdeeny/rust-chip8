//! Generic 'microcode' implementations of the operations.

use instruction::{Dest, Instruction, Src};
use instruction::Execute;
#[allow(unused_imports)]
use config::Config;
use types::*;

pub fn add(exec: &mut Execute, dest: Dest, lhs: Src, rhs: Src) -> Chip8Result<()> {
    let l = try!(exec.load(lhs));
    let r = try!(exec.load(rhs));
    let total = l + r;
    exec.set_flag(total > 0xFF);  //set vF if result overflows
    exec.store(dest, total)
}

pub fn sub(exec: &mut Execute, dest: Dest, lhs: Src, rhs: Src) -> Chip8Result<()> {
    let l = try!(exec.load(lhs));
    let r = try!(exec.load(rhs));
    let total = (l - r) & 0xFF;
    //set vF if NOT borrow
    exec.set_flag(l > r);
    exec.store(dest, total)
}

pub fn load(exec: &mut Execute, dest: Dest, src: Src) -> Chip8Result<()> {
    let data = try!(exec.load(src));
    exec.store(dest, data)
}

// Stores registers v0.. to ram[I]
pub fn stash(exec: &mut Execute, last: Src) -> Chip8Result<()> {
    let last_reg = if let Src::Register(r) = last {
        r
    } else {
        return Err(Chip8Error::InvalidOperand)
    };

    let i = try!(exec.load(Src::I));
    for r in 0...last_reg {
        let value = try!(exec.load(Src::Register(r)));
        try!(exec.store(Dest::Address12(i + r), value));
    }
    exec.store(Dest::I, i + last_reg + 1)
}

// Fetches several bytes, pointed to by I, into v0..
pub fn fetch(exec: &mut Execute, last: Src) -> Chip8Result<()> {
    let last_reg = if let Src::Register(r) = last {
        r
    } else {
        return Err(Chip8Error::InvalidOperand)
    };
    let i = try!(exec.load(Src::I));
    for r in 0...last_reg {
        let value = try!(exec.load(Src::Address12(i + r)));
        try!(exec.store(Dest::Register(r), value));
    }
    exec.store(Dest::I, i + last_reg + 1)
}


pub fn jump(exec: &mut Execute, addr: Src) -> Chip8Result<()> {
    let a = try!(exec.load(addr)) as Address;
    exec.jump(a);
    Ok(())
}

pub fn jump_v0(exec: &mut Execute, addr: Src) -> Chip8Result<()> {
    let mut a = try!(exec.load(addr)) as Address;
    a += try!(exec.load(Src::Register(0))) as Address;
    exec.jump(a);
    Ok(())
}

pub fn call(exec: &mut Execute, addr: Src) -> Chip8Result<()> {
    let a = try!(exec.load(addr)) as Address;
    let pc = exec.pc();
    exec.stack_push(pc);
    exec.jump(a);
    Ok(())
}

pub fn ret(exec: &mut Execute) -> Chip8Result<()> {
    if let Some(a) = exec.stack_pop() {
        exec.jump(a);
        Ok(())
    } else {
        Err(Chip8Error::PopEmptyStack)
    }
}


pub fn or(exec: &mut Execute, dest: Dest, lhs: Src, rhs: Src) -> Chip8Result<()> {
    let lhs = exec.load(lhs)?;
    let rhs = exec.load(rhs)?;
    let result = lhs | rhs;
    exec.store(dest, result)
}


pub fn and(exec: &mut Execute, dest: Dest, lhs: Src, rhs: Src) -> Chip8Result<()> {
    let lhs = exec.load(lhs)?;
    let rhs = exec.load(rhs)?;
    let result = lhs & rhs;
    exec.store(dest, result)
}

pub fn xor(exec: &mut Execute, dest: Dest, lhs: Src, rhs: Src) -> Chip8Result<()> {
    let lhs = exec.load(lhs)?;
    let rhs = exec.load(rhs)?;
    let result = lhs ^ rhs;
    exec.store(dest, result)
}

// Shifts the source right 1 bit, and stores in dest. vF set to old LSB
pub fn shr(exec: &mut Execute, dest: Dest, src: Src) -> Chip8Result<()> {
    let value = try!(exec.load(src));
    let carry = (value & 1) == 1;
    let result = value >> 1;
    exec.store(dest, result);
    exec.set_flag(carry);
    Ok(())
}

// Shifts the source left 1 bit, and stores in dest. vF set to old MSB
pub fn shl(exec: &mut Execute, dest: Dest, src: Src) -> Chip8Result<()> {
    let value = try!(exec.load(src));
    let carry = (value & 0x80) == 0x80;
    let result = value << 1;
    try!(exec.store(dest, result));
    exec.set_flag(carry);
    Ok(())
}

pub fn font(exec: &mut Execute, glyph: Src) -> Chip8Result<()> {
    let addr = exec.config().addr_font + try!(exec.load(glyph)) * 5;
    exec.store(Dest::I, addr)
}

pub fn bcd(exec: &mut Execute, src: Src) -> Chip8Result<()> {
    let mut val = try!(exec.load(src));
    let hundreds = val / 100;
    val -= hundreds * 100;
    let tens = val / 10;
    let ones = val - tens * 10;
    let i = try!(exec.load(Src::I));
    try!(exec.store(Dest::Address12(i), hundreds));
    try!(exec.store(Dest::Address12(i + 1), tens));
    try!(exec.store(Dest::Address12(i + 2), ones));
    Ok(())
}


// Skips the next instruction if src == dest.
pub fn skip_eq(exec: &mut Execute, lhs: Src, rhs: Src) -> Chip8Result<()> {
    let l = try!(exec.load(lhs));
    let r = try!(exec.load(rhs));
    if l == r { exec.advance_pc(); }
    Ok(())
}

pub fn skip_not_eq(exec: &mut Execute, lhs: Src, rhs: Src) -> Chip8Result<()> {
    let l = try!(exec.load(lhs));
    let r = try!(exec.load(rhs));
    if l != r { exec.advance_pc(); }
    Ok(())
}

pub fn skip_key_pressed(exec: &mut Execute, key: Src) -> Chip8Result<()> {
    let key_state = false;
    if key_state { exec.advance_pc(); }
    Ok(())
}

pub fn skip_key_not_pressed(exec: &mut Execute, key: Src) -> Chip8Result<()> {
    let key_state = false;
    if !key_state { exec.advance_pc(); }
    Ok(())
}

/// Halt execution until a key is pressed.
pub fn wait_key(exec: &mut Execute, key: Src) -> Chip8Result<()> {
    Ok(())
}

pub fn clear_screen(exec: &mut Execute) -> Chip8Result<()> {
    Ok(())
}

pub fn sprite(exec: &mut Execute, x: Src, y: Src, n: Src) -> Chip8Result<()> {
    Ok(())
}


pub fn random(exec: &mut Execute, dest: Dest, src: Src, mask: Src) -> Chip8Result<()> {
    let data = try!(exec.load(src));
    let mask = try!(exec.load(mask));
    let result = data & mask;
    exec.store(dest, result)
}



#[cfg(test)]
mod tests {
/*    use super::*;
    use Chip8;
    use Config;
    use instruction::{Execute, Instruction, Operand};
    use instruction::Operation::*;
    use instruction::execution::execute_microcode;
    #[test]
    fn test_add_reg() {
        let mut core = Chip8::default();
        let inst = Instruction::new(Add, Dest::Nowhere, Src::Register(0), Src::Register(1));

        core.store(Operand::Register(0), 5);
        core.store(Operand::Register(1), 10);
        execute_microcode(&inst, &mut core);
        assert_eq!(core.load(Src::Register(0)), 15);
        assert_eq!(core.load(Src::Register(1)), 10);
        assert_eq!(core.load(Src::Register(0xF)), 0);

        // with overflow
        core.store(Operand::Register(0), 0xFF);
        op_add(&inst, &mut core);
        assert_eq!(core.load(Src::Register(0)), 9);
        assert_eq!(core.load(Src::Register(1)), 10);
        assert_eq!(core.load(Src::Register(0xF)), 1);
    }*/
}
