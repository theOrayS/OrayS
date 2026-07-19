use alloc::vec;
use alloc::vec::Vec;

use super::geometry::Rect;
use super::painter::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceError {
    Empty,
    InvalidStride,
    SizeOverflow,
    BufferTooSmall,
}

#[derive(Clone)]
pub struct Surface {
    width: u32,
    height: u32,
    stride_pixels: usize,
    pixels: Vec<u32>,
}

impl Surface {
    pub fn new(width: u32, height: u32, stride_pixels: u32) -> Result<Self, SurfaceError> {
        if width == 0 || height == 0 {
            return Err(SurfaceError::Empty);
        }
        if stride_pixels < width {
            return Err(SurfaceError::InvalidStride);
        }
        let len = (height as usize)
            .checked_mul(stride_pixels as usize)
            .ok_or(SurfaceError::SizeOverflow)?;
        Ok(Self {
            width,
            height,
            stride_pixels: stride_pixels as usize,
            pixels: vec![0; len],
        })
    }

    pub const fn width(&self) -> u32 {
        self.width
    }

    pub const fn height(&self) -> u32 {
        self.height
    }

    pub const fn stride_pixels(&self) -> usize {
        self.stride_pixels
    }

    pub const fn bounds(&self) -> Rect {
        Rect::new(0, 0, self.width, self.height)
    }

    pub fn pixels(&self) -> &[u32] {
        &self.pixels
    }

    pub fn get(&self, x: u32, y: u32) -> Option<Color> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(Color::from_argb(
            self.pixels[y as usize * self.stride_pixels + x as usize],
        ))
    }

    pub(crate) fn set(&mut self, x: u32, y: u32, color: Color) {
        if x < self.width && y < self.height {
            self.pixels[y as usize * self.stride_pixels + x as usize] = color.argb();
        }
    }

    pub fn copy_bgra8888_to(
        &self,
        destination: &mut [u8],
        destination_stride: usize,
    ) -> Result<(), SurfaceError> {
        self.copy_bgra8888_regions_to(destination, destination_stride, &[self.bounds()])
    }

    pub fn copy_bgra8888_regions_to(
        &self,
        destination: &mut [u8],
        destination_stride: usize,
        regions: &[Rect],
    ) -> Result<(), SurfaceError> {
        let row_bytes = (self.width as usize)
            .checked_mul(4)
            .ok_or(SurfaceError::SizeOverflow)?;
        if destination_stride < row_bytes {
            return Err(SurfaceError::InvalidStride);
        }
        let required = (self.height as usize)
            .checked_mul(destination_stride)
            .ok_or(SurfaceError::SizeOverflow)?;
        if destination.len() < required {
            return Err(SurfaceError::BufferTooSmall);
        }

        for region in regions {
            let Some(region) = region.intersection(self.bounds()) else {
                continue;
            };
            let x_start = region.x as usize;
            let x_end = x_start + region.width as usize;
            let y_start = region.y as usize;
            let y_end = y_start + region.height as usize;
            for y in y_start..y_end {
                let source_row = y * self.stride_pixels;
                let destination_row = y * destination_stride;
                for x in x_start..x_end {
                    let bytes = self.pixels[source_row + x].to_le_bytes();
                    let offset = destination_row + x * 4;
                    destination[offset..offset + 4].copy_from_slice(&bytes);
                }
            }
        }
        Ok(())
    }

    pub fn checksum64(&self) -> u64 {
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for y in 0..self.height as usize {
            let row = y * self.stride_pixels;
            for pixel in &self.pixels[row..row + self.width as usize] {
                for byte in pixel.to_le_bytes() {
                    hash ^= byte as u64;
                    hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
                }
            }
        }
        hash
    }
}
