#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32, // Alpha channel for transparency (0.0 = fully transparent, 1.0 = fully opaque)
}

impl Color {
    // ------------------------
    // Neutral Colors
    // ------------------------
    pub const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const LIGHT_GRAY: Color = Color {
        r: 0.75,
        g: 0.75,
        b: 0.75,
        a: 1.0,
    };
    pub const GRAY: Color = Color {
        r: 0.5,
        g: 0.5,
        b: 0.5,
        a: 1.0,
    };
    pub const DARK_GRAY: Color = Color {
        r: 0.25,
        g: 0.25,
        b: 0.25,
        a: 1.0,
    };
    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    pub const IVORY: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const BEIGE: Color = Color {
        r: 0.96,
        g: 0.96,
        b: 0.86,
        a: 1.0,
    };
    pub const TAUPE: Color = Color {
        r: 0.275,
        g: 0.235,
        b: 0.196,
        a: 1.0,
    };
    pub const CHARCOAL: Color = Color {
        r: 0.212,
        g: 0.271,
        b: 0.31,
        a: 1.0,
    };
    pub const SLATE: Color = Color {
        r: 0.439,
        g: 0.5,
        b: 0.565,
        a: 1.0,
    };

    // ------------------------
    // Basic Colors
    // ------------------------
    pub const RED: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const GREEN: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const BLUE: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    pub const YELLOW: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const CYAN: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const MAGENTA: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    pub const ORANGE: Color = Color {
        r: 1.0,
        g: 0.674,
        b: 0.0,
        a: 1.0,
    };

    // ------------------------
    // Light Colors
    // ------------------------
    pub const LIGHT_RED: Color = Color {
        r: 1.0,
        g: 0.4,
        b: 0.4,
        a: 1.0,
    }; // 255, 102, 102
    pub const LIGHT_GREEN: Color = Color {
        r: 0.565,
        g: 0.933,
        b: 0.565,
        a: 1.0,
    }; // 144, 238, 144
    pub const LIGHT_BLUE: Color = Color {
        r: 0.678,
        g: 0.847,
        b: 0.902,
        a: 1.0,
    }; // 173, 216, 230
    pub const LIGHT_YELLOW: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 0.6,
        a: 1.0,
    }; // 255, 255, 153
    pub const LIGHT_CYAN: Color = Color {
        r: 0.878,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    }; // 224, 255, 255
    pub const LIGHT_MAGENTA: Color = Color {
        r: 1.0,
        g: 0.714,
        b: 1.0,
        a: 1.0,
    }; // 255, 182, 255
    pub const LIGHT_ORANGE: Color = Color {
        r: 1.0,
        g: 0.784,
        b: 0.486,
        a: 1.0,
    }; // 255, 200, 124
    pub const LIGHT_PURPLE: Color = Color {
        r: 0.847,
        g: 0.749,
        b: 0.847,
        a: 1.0,
    }; // 216, 191, 216

    // ------------------------
    // Dark Colors
    // ------------------------
    pub const DARK_RED: Color = Color {
        r: 0.545,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    }; // 139, 0, 0
    pub const DARK_GREEN: Color = Color {
        r: 0.0,
        g: 0.392,
        b: 0.0,
        a: 1.0,
    }; // 0, 100, 0
    pub const DARK_BLUE: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.545,
        a: 1.0,
    }; // 0, 0, 139
    pub const DARK_YELLOW: Color = Color {
        r: 0.8,
        g: 0.8,
        b: 0.0,
        a: 1.0,
    }; // 204, 204, 0
    pub const DARK_CYAN: Color = Color {
        r: 0.0,
        g: 0.545,
        b: 0.545,
        a: 1.0,
    }; // 0, 139, 139
    pub const DARK_MAGENTA: Color = Color {
        r: 0.545,
        g: 0.0,
        b: 0.545,
        a: 1.0,
    }; // 139, 0, 139
    pub const DARK_ORANGE: Color = Color {
        r: 1.0,
        g: 0.549,
        b: 0.0,
        a: 1.0,
    }; // 255, 140, 0
    pub const DARK_PURPLE: Color = Color {
        r: 0.294,
        g: 0.0,
        b: 0.51,
        a: 1.0,
    }; // 75, 0, 130

    // ------------------------
    // Pastel Colors
    // ------------------------
    pub const PASTEL_RED: Color = Color {
        r: 1.0,
        g: 0.702,
        b: 0.729,
        a: 1.0,
    }; // 255, 179, 186
    pub const PASTEL_GREEN: Color = Color {
        r: 0.729,
        g: 1.0,
        b: 0.788,
        a: 1.0,
    }; // 186, 255, 201
    pub const PASTEL_BLUE: Color = Color {
        r: 0.729,
        g: 0.882,
        b: 1.0,
        a: 1.0,
    }; // 186, 225, 255
    pub const PASTEL_YELLOW: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 0.729,
        a: 1.0,
    }; // 255, 255, 186
    pub const PASTEL_PURPLE: Color = Color {
        r: 0.863,
        g: 0.745,
        b: 1.0,
        a: 1.0,
    }; // 220, 190, 255
    pub const PASTEL_ORANGE: Color = Color {
        r: 1.0,
        g: 0.82,
        b: 0.667,
        a: 1.0,
    }; // 255, 209, 170
    pub const PASTEL_PINK: Color = Color {
        r: 1.0,
        g: 0.753,
        b: 0.796,
        a: 1.0,
    }; // 255, 192, 203
    pub const PASTEL_CYAN: Color = Color {
        r: 0.682,
        g: 1.0,
        b: 0.988,
        a: 1.0,
    }; // 174, 255, 252
    pub const PASTEL_LIME: Color = Color {
        r: 0.824,
        g: 1.0,
        b: 0.678,
        a: 1.0,
    }; // 210, 255, 173
    pub const PASTEL_GRAY: Color = Color {
        r: 0.784,
        g: 0.784,
        b: 0.784,
        a: 1.0,
    }; // 200, 200, 200

    // ------------------------
    // Neon Colors
    // ------------------------
    pub const NEON_RED: Color = Color {
        r: 1.0,
        g: 0.078,
        b: 0.235,
        a: 1.0,
    }; // 255, 20, 60
    pub const NEON_ORANGE: Color = Color {
        r: 1.0,
        g: 0.431,
        b: 0.0,
        a: 1.0,
    }; // 255, 110, 0
    pub const NEON_YELLOW: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    }; // 255, 255, 0
    pub const NEON_GREEN: Color = Color {
        r: 0.224,
        g: 1.0,
        b: 0.078,
        a: 1.0,
    }; // 57, 255, 20
    pub const NEON_CYAN: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    }; // 0, 255, 255
    pub const NEON_BLUE: Color = Color {
        r: 0.0,
        g: 0.627,
        b: 1.0,
        a: 1.0,
    }; // 0, 160, 255
    pub const NEON_PURPLE: Color = Color {
        r: 0.749,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    }; // 191, 0, 255
    pub const NEON_PINK: Color = Color {
        r: 1.0,
        g: 0.078,
        b: 0.576,
        a: 1.0,
    }; // 255, 20, 147
    pub const NEON_LIME: Color = Color {
        r: 0.749,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    }; // 191, 255, 0
    pub const NEON_MAGENTA: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    }; // 255, 0, 255

    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub fn new_with_alpha(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a: a.clamp(0.0, 1.0) }
    }

    pub fn r(&self) -> f32 {
        self.r
    }

    pub fn g(&self) -> f32 {
        self.g
    }

    pub fn b(&self) -> f32 {
        self.b
    }

    pub fn a(&self) -> f32 {
        self.a
    }

    pub fn from_u8(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: 1.0,
        }
    }

    pub fn from_u8_with_alpha(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    pub fn set(&mut self, r: f32, g: f32, b: f32) {
        self.r = r;
        self.g = g;
        self.b = b;
        self.a = 1.0;
    }

    pub fn set_with_alpha(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.r = r;
        self.g = g;
        self.b = b;
        self.a = a.clamp(0.0, 1.0);
    }

    pub fn lerp(a: Color, b: Color, t: f32) -> Color {
        Color {
            r: a.r + (b.r - a.r) * t.clamp(0.0, 1.0),
            g: a.g + (b.g - a.g) * t.clamp(0.0, 1.0),
            b: a.b + (b.b - a.b) * t.clamp(0.0, 1.0),
            a: a.a + (b.a - a.a) * t.clamp(0.0, 1.0),
        }
    }

    pub fn alpha_blend_over(&self, background: Color) -> Color {
        let alpha = self.a;
        let inv_alpha = 1.0 - alpha;
        
        Color {
            r: self.r * alpha + background.r * inv_alpha,
            g: self.g * alpha + background.g * inv_alpha,
            b: self.b * alpha + background.b * inv_alpha,
            a: alpha + background.a * inv_alpha,
        }
    }

    pub fn with_transparency(&self, transparency: f32) -> Color {
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a * transparency.clamp(0.0, 1.0),
        }
    }

    pub fn luminance(&self) -> f32 {
        // Standard luminance weights for sRGB
        0.2126 * self.r + 0.7152 * self.g + 0.0722 * self.b
    }

    pub fn to_u8_tuple(self, apply_gamma: bool) -> (u8, u8, u8) {
        let gamma_correct = |c: f32| {
            let c = if apply_gamma { c.powf(1.0 / 2.2) } else { c };
            (c.clamp(0.0, 1.0) * 255.0).round() as u8
        };

        (
            gamma_correct(self.r),
            gamma_correct(self.g),
            gamma_correct(self.b),
        )
    }
}

use std::fmt;
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (r, g, b) = self.to_u8_tuple(true); // gamma correct on write
        writeln!(f, "{r} {g} {b}")
    }
}

use std::ops::{Add, Div, Mul};
impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Color {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: (self.a + rhs.a).min(1.0),
        }
    }
}

impl Mul for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Color {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a * rhs.a,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, scalar: f32) -> Color {
        Color {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
            a: self.a,
        }
    }
}

impl Div<f32> for Color {
    type Output = Color;
    fn div(self, scalar: f32) -> Color {
        Color {
            r: self.r / scalar,
            g: self.g / scalar,
            b: self.b / scalar,
            a: self.a,
        }
    }
}
