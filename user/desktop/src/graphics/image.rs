use alloc::vec::Vec;

use super::geometry::Rect;
use super::painter::{Color, Painter};

const MAX_DIMENSION: u32 = 4096;
const MAX_PIXELS: usize = 16 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageError {
    UnsupportedFormat,
    InvalidHeader,
    InvalidDimensions,
    InvalidSample,
    Truncated,
    TooLarge,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bitmap {
    width: u32,
    height: u32,
    pixels: Vec<Color>,
}

impl Bitmap {
    pub fn parse_ppm(bytes: &[u8]) -> Result<Self, ImageError> {
        let mut parser = TokenParser::new(bytes);
        let magic = parser.next().ok_or(ImageError::InvalidHeader)?;
        let ascii = match magic {
            b"P3" => true,
            b"P6" => false,
            _ => return Err(ImageError::UnsupportedFormat),
        };
        let width = parse_u32(parser.next())?;
        let height = parse_u32(parser.next())?;
        let maximum = parse_u32(parser.next())?;
        if width == 0 || height == 0 || maximum == 0 || maximum > 255 {
            return Err(ImageError::InvalidDimensions);
        }
        if width > MAX_DIMENSION || height > MAX_DIMENSION {
            return Err(ImageError::TooLarge);
        }
        let count = (width as usize)
            .checked_mul(height as usize)
            .ok_or(ImageError::TooLarge)?;
        if count > MAX_PIXELS {
            return Err(ImageError::TooLarge);
        }
        let mut pixels = Vec::with_capacity(count);
        if ascii {
            for _ in 0..count {
                let r = parse_sample(parser.next(), maximum)?;
                let g = parse_sample(parser.next(), maximum)?;
                let b = parse_sample(parser.next(), maximum)?;
                pixels.push(Color::rgb(r, g, b));
            }
        } else {
            let data = parser.binary_data().ok_or(ImageError::Truncated)?;
            let expected = count.checked_mul(3).ok_or(ImageError::TooLarge)?;
            if data.len() < expected {
                return Err(ImageError::Truncated);
            }
            for sample in data[..expected].chunks_exact(3) {
                pixels.push(Color::rgb(
                    scale_sample(sample[0] as u32, maximum),
                    scale_sample(sample[1] as u32, maximum),
                    scale_sample(sample[2] as u32, maximum),
                ));
            }
        }
        Ok(Self {
            width,
            height,
            pixels,
        })
    }

    pub const fn width(&self) -> u32 {
        self.width
    }

    pub const fn height(&self) -> u32 {
        self.height
    }

    pub fn pixel(&self, x: u32, y: u32) -> Option<Color> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.pixels
            .get(y as usize * self.width as usize + x as usize)
            .copied()
    }

    pub fn draw_fit(&self, painter: &mut Painter<'_>, viewport: Rect, zoom_percent: u32) -> Rect {
        if viewport.is_empty() {
            return Rect::default();
        }
        let fit_scale_x = viewport.width.saturating_mul(100) / self.width.max(1);
        let fit_scale_y = viewport.height.saturating_mul(100) / self.height.max(1);
        let base = fit_scale_x.min(fit_scale_y).max(1);
        let effective = base.saturating_mul(zoom_percent.clamp(25, 400)) / 100;
        let width = self.width.saturating_mul(effective).div_ceil(100).max(1);
        let height = self.height.saturating_mul(effective).div_ceil(100).max(1);
        let target = Rect::new(
            viewport.x + (viewport.width as i32 - width as i32) / 2,
            viewport.y + (viewport.height as i32 - height as i32) / 2,
            width,
            height,
        );
        let Some(visible) = target.intersection(viewport) else {
            return target;
        };
        for y in visible.y..visible.bottom() {
            let source_y = ((y - target.y) as u64 * self.height as u64 / target.height as u64)
                .min(self.height.saturating_sub(1) as u64) as u32;
            for x in visible.x..visible.right() {
                let source_x = ((x - target.x) as u64 * self.width as u64 / target.width as u64)
                    .min(self.width.saturating_sub(1) as u64) as u32;
                if let Some(color) = self.pixel(source_x, source_y) {
                    painter.fill_rect(Rect::new(x, y, 1, 1), color);
                }
            }
        }
        target
    }
}

fn parse_u32(token: Option<&[u8]>) -> Result<u32, ImageError> {
    let token = token.ok_or(ImageError::Truncated)?;
    if token.is_empty() || token.iter().any(|byte| !byte.is_ascii_digit()) {
        return Err(ImageError::InvalidHeader);
    }
    let mut value = 0u32;
    for byte in token {
        value = value
            .checked_mul(10)
            .and_then(|current| current.checked_add((byte - b'0') as u32))
            .ok_or(ImageError::TooLarge)?;
    }
    Ok(value)
}

fn parse_sample(token: Option<&[u8]>, maximum: u32) -> Result<u8, ImageError> {
    let value = parse_u32(token)?;
    if value > maximum {
        return Err(ImageError::InvalidSample);
    }
    Ok(scale_sample(value, maximum))
}

fn scale_sample(value: u32, maximum: u32) -> u8 {
    ((value * 255 + maximum / 2) / maximum) as u8
}

struct TokenParser<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> TokenParser<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn skip_space_and_comments(&mut self) {
        loop {
            while self
                .bytes
                .get(self.position)
                .is_some_and(u8::is_ascii_whitespace)
            {
                self.position += 1;
            }
            if self.bytes.get(self.position) != Some(&b'#') {
                break;
            }
            while self
                .bytes
                .get(self.position)
                .is_some_and(|byte| *byte != b'\n')
            {
                self.position += 1;
            }
        }
    }

    fn next(&mut self) -> Option<&'a [u8]> {
        self.skip_space_and_comments();
        let start = self.position;
        while self
            .bytes
            .get(self.position)
            .is_some_and(|byte| !byte.is_ascii_whitespace() && *byte != b'#')
        {
            self.position += 1;
        }
        (start != self.position).then_some(&self.bytes[start..self.position])
    }

    fn binary_data(&mut self) -> Option<&'a [u8]> {
        let byte = *self.bytes.get(self.position)?;
        if !byte.is_ascii_whitespace() {
            return None;
        }
        self.position += if byte == b'\r' && self.bytes.get(self.position + 1) == Some(&b'\n') {
            2
        } else {
            1
        };
        Some(&self.bytes[self.position..])
    }
}
