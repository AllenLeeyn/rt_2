#[derive(Debug, Clone, Copy, Default)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    // ------------------------
    // Neutral Colors
    // ------------------------

    pub const WHITE:       Color = Color { r: 255, g: 255, b: 255 };
    pub const LIGHT_GRAY:  Color = Color { r: 211, g: 211, b: 211 };
    pub const GRAY:        Color = Color { r: 128, g: 128, b: 128 };
    pub const DARK_GRAY:   Color = Color { r:  64, g:  64, b:  64 };
    pub const BLACK:       Color = Color { r:   0, g:   0, b:   0 };

    pub const IVORY:       Color = Color { r: 255, g: 255, b: 240 };
    pub const BEIGE:       Color = Color { r: 245, g: 245, b: 220 };
    pub const TAUPE:       Color = Color { r:  72, g:  60, b:  50 };
    pub const CHARCOAL:    Color = Color { r:  54, g:  69, b:  79 };
    pub const SLATE:       Color = Color { r: 112, g: 128, b: 144 };

    // ------------------------
    // Basic Colors
    // ------------------------

    pub const RED:         Color = Color { r: 255, g:   0, b:   0 };
    pub const GREEN:       Color = Color { r:   0, g: 255, b:   0 };
    pub const BLUE:        Color = Color { r:   0, g:   0, b: 255 };
    pub const YELLOW:      Color = Color { r: 255, g: 255, b:   0 };
    pub const CYAN:        Color = Color { r:   0, g: 255, b: 255 };
    pub const MAGENTA:     Color = Color { r: 255, g:   0, b: 255 };
    pub const ORANGE:      Color = Color { r: 255, g: 165, b:   0 };

    // ------------------------
    // Light Colors
    // ------------------------

    pub const LIGHT_RED:     Color = Color { r: 255, g: 102, b: 102 };
    pub const LIGHT_GREEN:   Color = Color { r: 144, g: 238, b: 144 };
    pub const LIGHT_BLUE:    Color = Color { r: 173, g: 216, b: 230 };
    pub const LIGHT_YELLOW:  Color = Color { r: 255, g: 255, b: 153 };
    pub const LIGHT_CYAN:    Color = Color { r: 224, g: 255, b: 255 };
    pub const LIGHT_MAGENTA: Color = Color { r: 255, g: 182, b: 255 };
    pub const LIGHT_ORANGE:  Color = Color { r: 255, g: 200, b: 124 };
    pub const LIGHT_PURPLE:  Color = Color { r: 216, g: 191, b: 216 };

    // ------------------------
    // Dark Colors
    // ------------------------

    pub const DARK_RED:     Color = Color { r: 139, g:   0, b:   0 };
    pub const DARK_GREEN:   Color = Color { r:   0, g: 100, b:   0 };
    pub const DARK_BLUE:    Color = Color { r:   0, g:   0, b: 139 };
    pub const DARK_YELLOW:  Color = Color { r: 204, g: 204, b:   0 };
    pub const DARK_CYAN:    Color = Color { r:   0, g: 139, b: 139 };
    pub const DARK_MAGENTA: Color = Color { r: 139, g:   0, b: 139 };
    pub const DARK_ORANGE:  Color = Color { r: 255, g: 140, b:   0 };
    pub const DARK_PURPLE:  Color = Color { r:  75, g:   0, b: 130 };

    // ------------------------
    // Pastel Colors
    // ------------------------

    pub const PASTEL_RED:     Color = Color { r: 255, g: 179, b: 186 };
    pub const PASTEL_GREEN:   Color = Color { r: 186, g: 255, b: 201 };
    pub const PASTEL_BLUE:    Color = Color { r: 186, g: 225, b: 255 };
    pub const PASTEL_YELLOW:  Color = Color { r: 255, g: 255, b: 186 };
    pub const PASTEL_PURPLE:  Color = Color { r: 220, g: 190, b: 255 };
    pub const PASTEL_ORANGE:  Color = Color { r: 255, g: 209, b: 170 };
    pub const PASTEL_PINK:    Color = Color { r: 255, g: 192, b: 203 };
    pub const PASTEL_CYAN:    Color = Color { r: 174, g: 255, b: 252 };
    pub const PASTEL_LIME:    Color = Color { r: 210, g: 255, b: 173 };
    pub const PASTEL_GRAY:    Color = Color { r: 200, g: 200, b: 200 };

    // ------------------------
    // Neon Colors
    // ------------------------

    pub const NEON_RED:       Color = Color { r: 255, g:  20, b:  60 };
    pub const NEON_ORANGE:    Color = Color { r: 255, g: 110, b:   0 };
    pub const NEON_YELLOW:    Color = Color { r: 255, g: 255, b:   0 };
    pub const NEON_GREEN:     Color = Color { r:  57, g: 255, b:  20 };
    pub const NEON_CYAN:      Color = Color { r:   0, g: 255, b: 255 };
    pub const NEON_BLUE:      Color = Color { r:   0, g: 160, b: 255 };
    pub const NEON_PURPLE:    Color = Color { r: 191, g:   0, b: 255 };
    pub const NEON_PINK:      Color = Color { r: 255, g:  20, b: 147 };
    pub const NEON_LIME:      Color = Color { r: 191, g: 255, b:   0 };
    pub const NEON_MAGENTA:   Color = Color { r: 255, g:   0, b: 255 };

    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self{ r, g ,b }
    }

    pub fn new_f32(r: f32, g: f32, b: f32) -> Self {
        Self {
            r: (r * 255.999) as u8,
            g: (g * 255.999) as u8,
            b: (b * 255.999) as u8,
        }
    }

    pub fn set(&mut self, r: u8, g: u8, b: u8) {
        self.r = r;
        self.g = g;
        self.b = b;
    }
    
    pub fn lerp(a: Color, b: Color, t: f32) -> Color {
        fn mix_channel(c1: u8, c2: u8, t: f32) -> u8 {
            let mixed = c1 as f32 + (c2 as f32 - c1 as f32) * t.clamp(0.0, 1.0);
            mixed.round().clamp(0.0, 255.0) as u8
        }

        Color {
            r: mix_channel(a.r, b.r, t),
            g: mix_channel(a.g, b.g, t),
            b: mix_channel(a.b, b.b, t),
        }
    }

    pub fn add(self, other: Color) -> Color {
        Color {
            r: self.r.saturating_add(other.r),
            g: self.g.saturating_add(other.g),
            b: self.b.saturating_add(other.b),
        }
    }

    pub fn mul(self, other: Color) -> Color {
        fn blend(a: u8, b: u8) -> u8 {
            ((a as u16 * b as u16) / 255) as u8
        }

        Color {
            r: blend(self.r, other.r),
            g: blend(self.g, other.g),
            b: blend(self.b, other.b),
        }
    }

    pub fn mul_f32(self, factor: f32) -> Color {
        fn scale(channel: u8, factor: f32) -> u8 {
            (channel as f32 * factor.clamp(0.0, 1.0))
                .round()
                .clamp(0.0, 255.0) as u8
        }

        Color {
            r: scale(self.r, factor),
            g: scale(self.g, factor),
            b: scale(self.b, factor),
        }
    }

    pub fn div_f32(self, factor: f32) -> Color {
        fn scale(channel: u8, factor: f32) -> u8 {
            (channel as f32 / factor)
                .round()
                .clamp(0.0, 255.0) as u8
        }

        Color {
            r: scale(self.r, factor),
            g: scale(self.g, factor),
            b: scale(self.b, factor),
        }
    }

    pub fn to_vec3(&self) -> crate::core::Vec3 {
        crate::core::Vec3::new(self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0)
    }

    pub fn from_vec3(v: crate::core::Vec3) -> Self {
        Self {
            r: (v.x() * 255.999) as u8,
            g: (v.y() * 255.999) as u8,
            b: (v.z() * 255.999) as u8,
        }
    }
}

use std::fmt;
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}
