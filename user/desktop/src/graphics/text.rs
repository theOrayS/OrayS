use super::geometry::Rect;
use super::painter::{Color, Painter};

pub fn draw_text(
    painter: &mut Painter<'_>,
    mut x: i32,
    y: i32,
    text: &str,
    color: Color,
    scale: u32,
) {
    let scale = scale.max(1);
    for byte in text.bytes() {
        if byte == b' ' {
            x = x.saturating_add((4 * scale) as i32);
            continue;
        }
        let glyph = glyph(byte.to_ascii_uppercase());
        for (row, bits) in glyph.iter().copied().enumerate() {
            for column in 0..5 {
                if bits & (1 << (4 - column)) != 0 {
                    painter.fill_rect(
                        Rect::new(
                            x + (column * scale) as i32,
                            y + (row as u32 * scale) as i32,
                            scale,
                            scale,
                        ),
                        color,
                    );
                }
            }
        }
        x = x.saturating_add((6 * scale) as i32);
    }
}

const DEFAULT_GLYPH: [u8; 7] = [31, 17, 2, 4, 0, 4, 0];

pub(crate) const fn glyph_pattern(byte: u8) -> [u8; 7] {
    match byte {
        b'A' => [14, 17, 17, 31, 17, 17, 17],
        b'B' => [30, 17, 17, 30, 17, 17, 30],
        b'C' => [14, 17, 16, 16, 16, 17, 14],
        b'D' => [30, 17, 17, 17, 17, 17, 30],
        b'E' => [31, 16, 16, 30, 16, 16, 31],
        b'F' => [31, 16, 16, 30, 16, 16, 16],
        b'G' => [14, 17, 16, 23, 17, 17, 14],
        b'H' => [17, 17, 17, 31, 17, 17, 17],
        b'I' => [31, 4, 4, 4, 4, 4, 31],
        b'J' => [1, 1, 1, 1, 17, 17, 14],
        b'K' => [17, 18, 20, 24, 20, 18, 17],
        b'L' => [16, 16, 16, 16, 16, 16, 31],
        b'M' => [17, 27, 21, 21, 17, 17, 17],
        b'N' => [17, 25, 21, 19, 17, 17, 17],
        b'O' => [14, 17, 17, 17, 17, 17, 14],
        b'P' => [30, 17, 17, 30, 16, 16, 16],
        b'Q' => [14, 17, 17, 17, 21, 18, 13],
        b'R' => [30, 17, 17, 30, 20, 18, 17],
        b'S' => [15, 16, 16, 14, 1, 1, 30],
        b'T' => [31, 4, 4, 4, 4, 4, 4],
        b'U' => [17, 17, 17, 17, 17, 17, 14],
        b'V' => [17, 17, 17, 17, 17, 10, 4],
        b'W' => [17, 17, 17, 21, 21, 21, 10],
        b'X' => [17, 17, 10, 4, 10, 17, 17],
        b'Y' => [17, 17, 10, 4, 4, 4, 4],
        b'Z' => [31, 1, 2, 4, 8, 16, 31],
        b'0' => [14, 17, 19, 21, 25, 17, 14],
        b'1' => [4, 12, 4, 4, 4, 4, 14],
        b'2' => [14, 17, 1, 2, 4, 8, 31],
        b'3' => [30, 1, 1, 14, 1, 1, 30],
        b'4' => [2, 6, 10, 18, 31, 2, 2],
        b'5' => [31, 16, 16, 30, 1, 1, 30],
        b'6' => [14, 16, 16, 30, 17, 17, 14],
        b'7' => [31, 1, 2, 4, 8, 8, 8],
        b'8' => [14, 17, 17, 14, 17, 17, 14],
        b'9' => [14, 17, 17, 15, 1, 1, 14],
        b'+' => [0, 4, 4, 31, 4, 4, 0],
        b'-' => [0, 0, 0, 31, 0, 0, 0],
        b'/' => [1, 1, 2, 4, 8, 16, 16],
        b'>' => [16, 8, 4, 2, 4, 8, 16],
        b'<' => [1, 2, 4, 8, 4, 2, 1],
        b'_' => [0, 0, 0, 0, 0, 0, 31],
        b':' => [0, 4, 4, 0, 4, 4, 0],
        b'.' => [0, 0, 0, 0, 0, 12, 12],
        _ => DEFAULT_GLYPH,
    }
}

const fn build_glyph_atlas() -> [[u8; 7]; 128] {
    let mut atlas = [DEFAULT_GLYPH; 128];
    let mut byte = 0usize;
    while byte < atlas.len() {
        atlas[byte] = glyph_pattern(byte as u8);
        byte += 1;
    }
    atlas
}

static GLYPH_ATLAS: [[u8; 7]; 128] = build_glyph_atlas();

fn glyph(byte: u8) -> &'static [u8; 7] {
    let index = if byte.is_ascii() { byte as usize } else { 0 };
    &GLYPH_ATLAS[index]
}

#[cfg(test)]
mod tests {
    use super::{glyph, glyph_pattern};

    #[test]
    fn glyphs_are_reused_from_the_static_atlas() {
        let first = glyph(b'A');
        let second = glyph(b'A');
        assert!(core::ptr::eq(first, second));
        assert_eq!(*first, glyph_pattern(b'A'));
        assert_eq!(*glyph(0xff), glyph_pattern(0xff));
    }
}
