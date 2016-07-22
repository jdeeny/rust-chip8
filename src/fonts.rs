//! Chip8 fonts
use types::MemoryCell;


pub const CODE_SMALL: usize = 0;
pub const CODE_BIG: usize = 1;

/// Each glyph is 5 bytes. The most significant 4 bits are used.
pub type Font4x5 = [MemoryCell; 5 * 16];

/// I believe this is the 'official' chip8 font.
pub const FONT_4X5_CHIP8: Font4x5 = [// 0
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
                                     draw_u8!(X___)];

/// Attempt at creating a rounded 4x5 font. Mostly to test that different fonts can be used.
pub const FONT_4X5_SMOOTH: Font4x5 = [// 0
                                      draw_u8!(_XX_),
                                      draw_u8!(X__X),
                                      draw_u8!(X__X),
                                      draw_u8!(X__X),
                                      draw_u8!(_XX_),
                                      // 1
                                      draw_u8!(___X),
                                      draw_u8!(__XX),
                                      draw_u8!(___X),
                                      draw_u8!(___X),
                                      draw_u8!(___X),
                                      // 2
                                      draw_u8!(XXXX),
                                      draw_u8!(___X),
                                      draw_u8!(XXXX),
                                      draw_u8!(X___),
                                      draw_u8!(XXXX),
                                      // 3
                                      draw_u8!(XXXX),
                                      draw_u8!(___X),
                                      draw_u8!(_XXX),
                                      draw_u8!(___X),
                                      draw_u8!(XXXX),
                                      // 4
                                      draw_u8!(X___),
                                      draw_u8!(X__X),
                                      draw_u8!(XXXX),
                                      draw_u8!(___X),
                                      draw_u8!(___X),
                                      // 5
                                      draw_u8!(XXXX),
                                      draw_u8!(X___),
                                      draw_u8!(XXX_),
                                      draw_u8!(___X),
                                      draw_u8!(XXX_),
                                      // 6
                                      draw_u8!(__X_),
                                      draw_u8!(_X__),
                                      draw_u8!(XXX_),
                                      draw_u8!(X__X),
                                      draw_u8!(_XX_),
                                      // 7
                                      draw_u8!(_XXX),
                                      draw_u8!(___X),
                                      draw_u8!(___X),
                                      draw_u8!(___X),
                                      draw_u8!(___X),
                                      // 8
                                      draw_u8!(_XX_),
                                      draw_u8!(X__X),
                                      draw_u8!(_XX_),
                                      draw_u8!(X__X),
                                      draw_u8!(_XX_),
                                      // 9
                                      draw_u8!(_XX_),
                                      draw_u8!(X__X),
                                      draw_u8!(_XXX),
                                      draw_u8!(__X_),
                                      draw_u8!(_X__),
                                      // A
                                      draw_u8!(_XX_),
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
                                      draw_u8!(_XXX),
                                      draw_u8!(X___),
                                      draw_u8!(X___),
                                      draw_u8!(X___),
                                      draw_u8!(_XXX),
                                      // D
                                      draw_u8!(XXX_),
                                      draw_u8!(X__X),
                                      draw_u8!(X__X),
                                      draw_u8!(X__X),
                                      draw_u8!(XXX_),
                                      // E
                                      draw_u8!(XXXX),
                                      draw_u8!(X___),
                                      draw_u8!(XXX_),
                                      draw_u8!(X___),
                                      draw_u8!(XXXX),
                                      // F
                                      draw_u8!(XXXX),
                                      draw_u8!(X___),
                                      draw_u8!(XXXX),
                                      draw_u8!(X___),
                                      draw_u8!(X___)];

#[cfg(test)]
mod tests {
    use super::*;

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

        for (i, glyph) in FONT_4X5_CHIP8.iter().enumerate() {
            let re_glyph = CHIP8_REFERENCE_FONT[i];
            println!("Glyph {:X}.{}: {:08b} ?= {:08b}",
                     i / 5,
                     i % 5,
                     glyph,
                     re_glyph);
            assert_eq!(*glyph, re_glyph);
        }
    }

    #[test]
    fn test_draw() {
        assert_eq!(draw_u8!(X___), 0x80);
        assert_eq!(draw_u8!(____), 0x00);
    }
}
