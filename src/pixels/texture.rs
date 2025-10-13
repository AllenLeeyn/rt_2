use crate::core::Color;
use crate::pixels::image::Image;

#[derive(Debug, Clone)]
pub enum Texture {
    SolidColor(Color),
    Gradient(Color, Color, f32),
    Checkerboard(Color, Color, f32),
    Image(Image),
}

impl Texture {
    pub fn default() -> Self {
        Texture::SolidColor(Color::BLACK)
    }

    pub fn value_at(&self, u: f32, v: f32) -> Color {
        match self {
            Texture::SolidColor(color) => *color,

            Texture::Gradient(start, end, angle_rad) => {
                let u_centered = u - 0.5;
                let v_centered = v - 0.5;

                let u_rotated = u_centered * angle_rad.cos() - v_centered * angle_rad.sin();
                let t = (u_rotated + 0.5).clamp(0.0, 1.0);
                Color::lerp(*start, *end, t)
            }

            Texture::Checkerboard(c1, c2, frequency) => {
                let s = (u * frequency) as i32;
                let t = (v * frequency) as i32;
                let check = (s + t) % 2 == 0;
                if check { *c1 } else { *c2 }
            }

            Texture::Image(image) => {
                let u = u.clamp(0.0, 1.0);
                let v = v.clamp(0.0, 1.0);

                let x = (u * (image.width as f32 - 1.0)).round() as usize;
                let y = ((1.0 - v) * (image.height as f32 - 1.0)).round() as usize;

                image.get_pixel(x, y)
            }
        }
    }

    pub fn bg_value_at(&self, u: f32, v: f32) -> Color {
        match self {
            Texture::Gradient(start, end, _) => {
                Color::lerp(*start, *end, v)
                //Color::lerp(Color::BLACK, Color::WHITE, v)
            }

            _ => self.value_at(u, v)
        }
    }
}
