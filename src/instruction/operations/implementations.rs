//! Generic 'microcode' implementations of the operations.

use types::*;
use instruction::{Dest, Src};
use fonts;

pub fn add(exec: &mut Execute, dest: Dest, lhs: Src, rhs: Src) -> Chip8Result<()> {
    let l = try!(exec.load(lhs));
    let r = try!(exec.load(rhs));
    let total = l + r;
    exec.set_flag(total > 0xFF);  //set vF if result overflows
    exec.store(dest, total)
}

pub fn sub(exec: &mut Execute, dest: Dest, lhs: Src, rhs: Src) -> Chip8Result<()> {
    let mut l = try!(exec.load(lhs));
    let r = try!(exec.load(rhs));
    // set vF if NOT borrow
    exec.set_flag(l >= r);
    if r > l {
        l += 256;
    }
    let total = (l - r) & 0xFF;
    exec.store(dest, total)
}

pub fn load(exec: &mut Execute, dest: Dest, src: Src) -> Chip8Result<()> {
    let data = try!(exec.load(src));
    exec.store(dest, data)
}

// Stores registers v0.. to ram[I]
pub fn stash(exec: &mut Execute, first: Src, last: Src, flag: Src) -> Chip8Result<()> {
    let first_reg = match first {
        Src::Register(r) => r,
        Src::Const(n) => n,
        _ => {
            return Err(Chip8Error::InvalidOperand);
        },
    };

    let last_reg = match last {
        Src::Register(r) => r,
        Src::Const(n) => n,
        _ => {
            return Err(Chip8Error::InvalidOperand);
        },
    };

    let flag = try!(exec.load(flag));


    let i = try!(exec.load(Src::I));
    let mut offset = 0;
    for r in first_reg..=last_reg {
        let value = try!(exec.load(Src::Register(r)));
        try!(exec.store(Dest::Address12(i + offset), value));
        offset += 1;
    }
    if flag == 1 {
        try!(exec.store(Dest::I, i + offset));
    }
    Ok(())
}

// Fetches several bytes, pointed to by I, into v0..
pub fn fetch(exec: &mut Execute, first: Src, last: Src, flag: Src) -> Chip8Result<()> {
    let first_reg = match first {
        Src::Register(r) => r,
        Src::Const(n) => n,
        _ => {
            return Err(Chip8Error::InvalidOperand);
        },
    };

    let last_reg = match last {
        Src::Register(r) => r,
        Src::Const(n) => n,
        _ => {
            return Err(Chip8Error::InvalidOperand);
        },
    };

    let flag = try!(exec.load(flag));

    let i = try!(exec.load(Src::I));
    let mut offset = 0;
    for r in first_reg..=last_reg {
        let value = try!(exec.load(Src::Address12(i + offset)));
        try!(exec.store(Dest::Register(r), value));
        offset += 1;
    }
    if flag == 1 {
        try!(exec.store(Dest::I, i + offset));
    }
    Ok(())
}


pub fn jump(exec: &mut Execute, addr: Src) -> Chip8Result<()> {
    if let Src::Address12(a) = addr {
        exec.jump(a as Address)
    } else {
        Err(Chip8Error::InvalidOperand)
    }
}

pub fn jump_v0(exec: &mut Execute, addr: Src) -> Chip8Result<()> {
    let v0 = try!(exec.load(Src::Register(0)));
    if let Src::Address12(a) = addr {
        exec.jump((a + v0) as Address)
    } else {
        Err(Chip8Error::InvalidOperand)
    }
}


pub fn call(exec: &mut Execute, addr: Src) -> Chip8Result<()> {
    if let Src::Address12(a) = addr {
        let pc = exec.pc();
        exec.stack_push(pc);
        exec.jump(a as Address)
    } else {
        Err(Chip8Error::InvalidOperand)
    }
}

pub fn ret(exec: &mut Execute) -> Chip8Result<()> {
    if let Some(a) = exec.stack_pop() {
        exec.jump(a)
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
    try!(exec.store(dest, result));
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

pub fn font(exec: &mut Execute, glyph: Src, font: Src) -> Chip8Result<()> {
    let font_code = try!(exec.load(font));
    let addr = if font_code == fonts::CODE_SMALL {
        exec.config().addr_font + try!(exec.load(glyph)) * 5
    } else {
        exec.config().addr_font_big + try!(exec.load(glyph)) * 10
    };
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
    if l == r {
        exec.advance_pc();
    }
    Ok(())
}

pub fn skip_not_eq(exec: &mut Execute, lhs: Src, rhs: Src) -> Chip8Result<()> {
    let l = try!(exec.load(lhs));
    let r = try!(exec.load(rhs));
    if l != r {
        exec.advance_pc();
    }
    Ok(())
}

pub fn skip_key_pressed(exec: &mut Execute, key: Src) -> Chip8Result<()> {
    let key = try!(exec.load(key));
    let key_state = try!(exec.keyboard())[key];
    if key_state {
        exec.advance_pc();
    }
    Ok(())
}

pub fn skip_key_not_pressed(exec: &mut Execute, key: Src) -> Chip8Result<()> {
    let key = try!(exec.load(key));
    let key_state = try!(exec.keyboard())[key];
    if !key_state {
        exec.advance_pc();
    }
    Ok(())
}

/// Halt execution until a key is pressed.
#[allow(unused_variables)]
pub fn wait_key(exec: &mut Execute, dest: Dest, key: Src) -> Chip8Result<()> {
    unimplemented!()
}

pub fn clear_screen(exec: &mut Execute) -> Chip8Result<()> {
    for x in 0..64 {
        for y in 0..32 {
            try!(exec.set_pixel(x, y, 0));
        }
    }
    Ok(())
}

pub fn sprite(exec: &mut Execute, x: Src, y: Src, n: Src) -> Chip8Result<()> {
    let x = try!(exec.load(x));
    let y = try!(exec.load(y));
    let n = try!(exec.load(n));

    let mut addr = try!(exec.load(Src::I));

    let mut flag = false;

    for y in y..n + y {
        let data = try!(exec.load(Src::Address12(addr)));
        for bit in 0..8 {
            flag |= try!(exec.xor_pixel(x + (7 - bit), y, ((data >> bit) & 1) as Pixel));
        }
        addr += 1;
    }
    exec.set_flag(flag);
    Ok(())
}


pub fn random(exec: &mut Execute, dest: Dest, src: Src, mask: Src) -> Chip8Result<()> {
    let data = try!(exec.load(src));
    let bitmask = try!(exec.load(mask));
    let result = data & bitmask;
    exec.store(dest, result)
}
