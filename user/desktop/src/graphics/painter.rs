use super::geometry::Rect;
use super::surface::Surface;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn from_argb(value: u32) -> Self {
        let bytes = value.to_be_bytes();
        Self {
            a: bytes[0],
            r: bytes[1],
            g: bytes[2],
            b: bytes[3],
        }
    }

    pub const fn argb(self) -> u32 {
        u32::from_be_bytes([self.a, self.r, self.g, self.b])
    }

    pub fn over(self, background: Self) -> Self {
        if self.a == 255 {
            return self;
        }
        if self.a == 0 {
            return background;
        }
        let alpha = self.a as u32;
        let inverse = 255 - alpha;
        let blend = |foreground: u8, behind: u8| -> u8 {
            ((foreground as u32 * alpha + behind as u32 * inverse + 127) / 255) as u8
        };
        Self::rgb(
            blend(self.r, background.r),
            blend(self.g, background.g),
            blend(self.b, background.b),
        )
    }
}

pub struct Painter<'a> {
    surface: &'a mut Surface,
    clip: Rect,
}

impl<'a> Painter<'a> {
    pub fn new(surface: &'a mut Surface, clip: Rect) -> Self {
        let clip = clip.intersection(surface.bounds()).unwrap_or_default();
        Self { surface, clip }
    }

    pub const fn clip(&self) -> Rect {
        self.clip
    }

    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        let Some(rect) = rect.intersection(self.clip) else {
            return;
        };
        for y in rect.y as u32..rect.bottom() as u32 {
            for x in rect.x as u32..rect.right() as u32 {
                self.surface.set(x, y, color);
            }
        }
    }

    pub fn blend_rect(&mut self, rect: Rect, color: Color) {
        let Some(rect) = rect.intersection(self.clip) else {
            return;
        };
        for y in rect.y as u32..rect.bottom() as u32 {
            for x in rect.x as u32..rect.right() as u32 {
                let background = self.surface.get(x, y).unwrap_or_default();
                self.surface.set(x, y, color.over(background));
            }
        }
    }

    pub fn fill_rounded_rect(&mut self, rect: Rect, radius: u32, color: Color) {
        self.rounded_rect_rows(rect, radius, |painter, row| {
            painter.fill_rect(row, color);
        });
    }

    pub fn blend_rounded_rect(&mut self, rect: Rect, radius: u32, color: Color) {
        self.rounded_rect_rows(rect, radius, |painter, row| {
            painter.blend_rect(row, color);
        });
    }

    pub fn stroke_rect(&mut self, rect: Rect, thickness: u32, color: Color) {
        if thickness == 0 || rect.is_empty() {
            return;
        }
        self.fill_rect(
            Rect::new(rect.x, rect.y, rect.width, thickness.min(rect.height)),
            color,
        );
        self.fill_rect(
            Rect::new(
                rect.x,
                rect.bottom().saturating_sub(thickness as i32),
                rect.width,
                thickness.min(rect.height),
            ),
            color,
        );
        self.fill_rect(
            Rect::new(rect.x, rect.y, thickness.min(rect.width), rect.height),
            color,
        );
        self.fill_rect(
            Rect::new(
                rect.right().saturating_sub(thickness as i32),
                rect.y,
                thickness.min(rect.width),
                rect.height,
            ),
            color,
        );
    }

    pub fn fill_vertical_gradient(&mut self, rect: Rect, top: Color, bottom: Color) {
        let Some(rect) = rect.intersection(self.clip) else {
            return;
        };
        let denominator = rect.height.saturating_sub(1).max(1);
        for row in 0..rect.height {
            let mix = |a: u8, b: u8| -> u8 {
                let value = a as u32 * (denominator - row) + b as u32 * row;
                (value / denominator) as u8
            };
            let color = Color::rgb(
                mix(top.r, bottom.r),
                mix(top.g, bottom.g),
                mix(top.b, bottom.b),
            );
            self.fill_rect(Rect::new(rect.x, rect.y + row as i32, rect.width, 1), color);
        }
    }

    fn rounded_rect_rows(
        &mut self,
        rect: Rect,
        radius: u32,
        mut draw: impl FnMut(&mut Self, Rect),
    ) {
        if rect.is_empty() {
            return;
        }
        let radius = radius.min(rect.width / 2).min(rect.height / 2);
        if radius == 0 {
            draw(self, rect);
            return;
        }
        for row in 0..rect.height {
            let edge_distance = row.min(rect.height - 1 - row);
            let inset = if edge_distance >= radius {
                0
            } else {
                circle_inset(radius, edge_distance)
            };
            draw(
                self,
                Rect::new(
                    rect.x + inset as i32,
                    rect.y + row as i32,
                    rect.width.saturating_sub(inset * 2),
                    1,
                ),
            );
        }
    }
}

fn circle_inset(radius: u32, edge_distance: u32) -> u32 {
    let center = radius.saturating_sub(1);
    let y = center.saturating_sub(edge_distance) as u64;
    let radius_squared = radius as u64 * radius as u64;
    let mut x = 0u32;
    while x < radius {
        let distance = radius.saturating_sub(x) as u64;
        if distance * distance + y * y <= radius_squared {
            return x;
        }
        x += 1;
    }
    radius
}
