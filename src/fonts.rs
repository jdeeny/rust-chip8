//! Chip8 fonts

/// Each glyph is 5 bytes. The most significant 4 bits are used.
pub type Font4x5 = [u8; 5 * 16];

/// I believe this is the 'official' chip8 font.
pub const FONT_CHIP8_4X5: Font4x5 = [// 0
                                     0b_1111_0000,
                                     0b_1001_0000,
                                     0b_1001_0000,
                                     0b_1001_0000,
                                     0b_1111_0000,
                                     // 1
                                     0b_0010_0000,
                                     0b_0110_0000,
                                     0b_0010_0000,
                                     0b_0010_0000,
                                     0b_0111_0000,
                                     // 2
                                     0b_1111_0000,
                                     0b_0001_0000,
                                     0b_1111_0000,
                                     0b_1000_0000,
                                     0b_1111_0000,
                                     // 3
                                     0b_1111_0000,
                                     0b_0001_0000,
                                     0b_1111_0000,
                                     0b_0001_0000,
                                     0b_1111_0000,
                                     // 4
                                     0b_1001_0000,
                                     0b_1001_0000,
                                     0b_1111_0000,
                                     0b_0001_0000,
                                     0b_0001_0000,
                                     // 5
                                     0b_1111_0000,
                                     0b_1000_0000,
                                     0b_1111_0000,
                                     0b_0001_0000,
                                     0b_1111_0000,
                                     // 6
                                     0b_1111_0000,
                                     0b_1000_0000,
                                     0b_1111_0000,
                                     0b_1001_0000,
                                     0b_1111_0000,
                                     // 7
                                     0b_1111_0000,
                                     0b_0001_0000,
                                     0b_0010_0000,
                                     0b_0100_0000,
                                     0b_0100_0000,
                                     // 8
                                     0b_1111_0000,
                                     0b_1001_0000,
                                     0b_1111_0000,
                                     0b_1001_0000,
                                     0b_1111_0000,
                                     // 9
                                     0b_1111_0000,
                                     0b_1001_0000,
                                     0b_1111_0000,
                                     0b_0001_0000,
                                     0b_1111_0000,
                                     // A
                                     0b_1111_0000,
                                     0b_1001_0000,
                                     0b_1111_0000,
                                     0b_1001_0000,
                                     0b_1001_0000,
                                     // B
                                     0b_1110_0000,
                                     0b_1001_0000,
                                     0b_1110_0000,
                                     0b_1001_0000,
                                     0b_1110_0000,
                                     // C
                                     0b_1111_0000,
                                     0b_1000_0000,
                                     0b_1000_0000,
                                     0b_1000_0000,
                                     0b_1111_0000,
                                     // D
                                     0b_1110_0000,
                                     0b_1001_0000,
                                     0b_1001_0000,
                                     0b_1001_0000,
                                     0b_1110_0000,
                                     // E
                                     0b_1111_0000,
                                     0b_1000_0000,
                                     0b_1111_0000,
                                     0b_1000_0000,
                                     0b_1111_0000,
                                     // F
                                     0b_1111_0000,
                                     0b_1000_0000,
                                     0b_1111_0000,
                                     0b_1000_0000,
                                     0b_1000_0000];

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
        let ref_glyph = CHIP8_REFERENCE_FONT[i];
        println!("Glyph {:X}.{}: {:08b} ?= {:08b}",
                 i / 5,
                 i % 5,
                 glyph,
                 ref_glyph);
        assert_eq!(*glyph, ref_glyph);
    }
}
