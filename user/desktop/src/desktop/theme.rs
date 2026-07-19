use crate::graphics::painter::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeKind {
    Dark,
    Light,
}

impl ThemeKind {
    pub const fn toggled(self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub kind: ThemeKind,
    pub wallpaper_top: Color,
    pub wallpaper_bottom: Color,
    pub panel: Color,
    pub panel_border: Color,
    pub text: Color,
    pub muted_text: Color,
    pub accent: Color,
    pub window: Color,
    pub title_active: Color,
    pub title_inactive: Color,
}

impl Theme {
    pub const fn for_kind(kind: ThemeKind) -> Self {
        match kind {
            ThemeKind::Dark => Self {
                kind,
                wallpaper_top: Color::rgb(14, 23, 43),
                wallpaper_bottom: Color::rgb(42, 59, 94),
                panel: Color::rgba(12, 18, 31, 222),
                panel_border: Color::rgba(163, 187, 232, 90),
                text: Color::rgb(243, 247, 255),
                muted_text: Color::rgb(180, 195, 221),
                accent: Color::rgb(92, 142, 239),
                window: Color::rgb(232, 237, 246),
                title_active: Color::rgb(39, 63, 108),
                title_inactive: Color::rgb(66, 76, 96),
            },
            ThemeKind::Light => Self {
                kind,
                wallpaper_top: Color::rgb(185, 211, 242),
                wallpaper_bottom: Color::rgb(102, 142, 198),
                panel: Color::rgba(243, 247, 253, 224),
                panel_border: Color::rgba(65, 88, 125, 80),
                text: Color::rgb(29, 40, 59),
                muted_text: Color::rgb(72, 88, 112),
                accent: Color::rgb(51, 104, 208),
                window: Color::rgb(250, 252, 255),
                title_active: Color::rgb(55, 104, 192),
                title_inactive: Color::rgb(130, 145, 168),
            },
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::for_kind(ThemeKind::Dark)
    }
}
