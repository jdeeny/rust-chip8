//! Chip8 fonts
use types::MemoryCell;

/// Each glyph is 5 bytes. The most significant 4 bits are used.
pub type Font4x5 = [MemoryCell; 5 * 16];

use self::draw::*;

/// I believe this is the 'official' chip8 font.
pub const FONT_CHIP8_4X5: Font4x5 = [
                                            // 0
                                            draw_u8!(XXXX),
                                            draw_u8!(X__X),
                                            draw_u8!(X__X),
                                            draw_u8!(X__X),
                                            draw_u8!(XXXX),
                                            // 1
                                            draw_u8!(__X_),
                                            draw_u8!(_XX_),
                                            draw_u8!(__X_),
                                            draw_u8!(__X_),
                                            draw_u8!(_XXX),
                                            // 2
                                            draw_u8!(XXXX),
                                            draw_u8!(___X),
                                            draw_u8!(XXXX),
                                            draw_u8!(X___),
                                            draw_u8!(XXXX),
                                            // 3
                                            draw_u8!(XXXX),
                                            draw_u8!(___X),
                                            draw_u8!(XXXX),
                                            draw_u8!(___X),
                                            draw_u8!(XXXX),
                                            // 4
                                            draw_u8!(X__X),
                                            draw_u8!(X__X),
                                            draw_u8!(XXXX),
                                            draw_u8!(___X),
                                            draw_u8!(___X),
                                            // 5
                                            draw_u8!(XXXX),
                                            draw_u8!(X___),
                                            draw_u8!(XXXX),
                                            draw_u8!(___X),
                                            draw_u8!(XXXX),
                                            // 6
                                            draw_u8!(XXXX),
                                            draw_u8!(X___),
                                            draw_u8!(XXXX),
                                            draw_u8!(X__X),
                                            draw_u8!(XXXX),
                                            // 7
                                            draw_u8!(XXXX),
                                            draw_u8!(___X),
                                            draw_u8!(__X_),
                                            draw_u8!(_X__),
                                            draw_u8!(_X__),
                                            // 8
                                            draw_u8!(XXXX),
                                            draw_u8!(X__X),
                                            draw_u8!(XXXX),
                                            draw_u8!(X__X),
                                            draw_u8!(XXXX),
                                            // 9
                                            draw_u8!(XXXX),
                                            draw_u8!(X__X),
                                            draw_u8!(XXXX),
                                            draw_u8!(___X),
                                            draw_u8!(XXXX),
                                            // A
                                            draw_u8!(XXXX),
                                            draw_u8!(X__X),
                                            draw_u8!(XXXX),
                                            draw_u8!(X__X),
                                            draw_u8!(X__X),
                                            // B
                                            draw_u8!(XXX_),
                                            draw_u8!(X__X),
                                            draw_u8!(XXX_),
                                            draw_u8!(X__X),
                                            draw_u8!(XXX_),
                                            // C
                                            draw_u8!(XXXX),
                                            draw_u8!(X___),
                                            draw_u8!(X___),
                                            draw_u8!(X___),
                                            draw_u8!(XXXX),
                                            // D
                                            draw_u8!(XXX_),
                                            draw_u8!(X__X),
                                            draw_u8!(X__X),
                                            draw_u8!(X__X),
                                            draw_u8!(XXX_),
                                            // E
                                            draw_u8!(XXXX),
                                            draw_u8!(X___),
                                            draw_u8!(XXXX),
                                            draw_u8!(X___),
                                            draw_u8!(XXXX),
                                            // F
                                            draw_u8!(XXXX),
                                            draw_u8!(X___),
                                            draw_u8!(XXXX),
                                            draw_u8!(X___),
                                            X___
                                        ];

#[test]
fn test_font4x5() {
    const CHIP8_REFERENCE_FONT: Font4x5 =
        [0xF0, 0x90, 0x90, 0x90, 0xF0 /* 0 */, 0x20, 0x60, 0x20, 0x20, 0x70 /* 1 */,
         0xF0, 0x10, 0xF0, 0x80, 0xF0 /* 2 */, 0xF0, 0x10, 0xF0, 0x10, 0xF0 /* 3 */,
         0x90, 0x90, 0xF0, 0x10, 0x10 /* 4 */, 0xF0, 0x80, 0xF0, 0x10, 0xF0 /* 5 */,
         0xF0, 0x80, 0xF0, 0x90, 0xF0 /* 6 */, 0xF0, 0x10, 0x20, 0x40, 0x40 /* 7 */,
         0xF0, 0x90, 0xF0, 0x90, 0xF0 /* 8 */, 0xF0, 0x90, 0xF0, 0x10, 0xF0 /* 9 */,
         0xF0, 0x90, 0xF0, 0x90, 0x90 /* A */, 0xE0, 0x90, 0xE0, 0x90, 0xE0 /* B */,
         0xF0, 0x80, 0x80, 0x80, 0xF0 /* C */, 0xE0, 0x90, 0x90, 0x90, 0xE0 /* D */,
         0xF0, 0x80, 0xF0, 0x80, 0xF0 /* E */, 0xF0, 0x80, 0xF0, 0x80, 0x80 /* F */];

    for (i, glyph) in FONT_CHIP8_4X5.iter().enumerate() {
        let re_glyph = CHIP8_REFERENCE_FONT[i];
        println!("Glyph {:X}.{}: {:08b} ?= {:08b}",
                 i / 5,
                 i % 5,
                 glyph,
                 re_glyph);
        assert_eq!(*glyph, re_glyph);
    }
}


#[allow(dead_code)]
mod draw {
    use types::MemoryCell;
    pub const ____: MemoryCell = 0x00;
    pub const ___X: MemoryCell = 0x10;
    pub const __X_: MemoryCell = 0x20;
    pub const __XX: MemoryCell = 0x30;
    pub const _X__: MemoryCell = 0x40;
    pub const _X_X: MemoryCell = 0x50;
    pub const _XX_: MemoryCell = 0x60;
    pub const _XXX: MemoryCell = 0x70;
    pub const X___: MemoryCell = 0x80;
    pub const X__X: MemoryCell = 0x90;
    pub const X_X_: MemoryCell = 0xA0;
    pub const X_XX: MemoryCell = 0xB0;
    pub const XX__: MemoryCell = 0xC0;
    pub const XX_X: MemoryCell = 0xD0;
    pub const XXX_: MemoryCell = 0xE0;
    pub const XXXX: MemoryCell = 0xF0;
}

#[test]
fn test_rn() {
    assert_eq!(draw_u8!(X___), 0x80);
    assert_eq!(draw_u8!(____), 0x00);
}
