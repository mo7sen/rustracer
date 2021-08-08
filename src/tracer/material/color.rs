#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn RGB(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
    pub fn RGBA(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}

impl std::ops::Add<Color> for Color {
    type Output = Color;
    fn add(self, other: Color) -> <Self as std::ops::Add<Color>>::Output {
        Color::RGB(
            (self.r as u16 + other.r as u16).min(255).max(0) as u8,
            (self.g as u16 + other.g as u16).min(255).max(0) as u8,
            (self.b as u16 + other.b as u16).min(255).max(0) as u8,
        )
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Color;
    fn mul(self, scale: f32) -> <Self as std::ops::Mul<f32>>::Output {
        let new_r = self.r as f32 * scale;
        let new_g = self.g as f32 * scale;
        let new_b = self.b as f32 * scale;

        let mut color_scale: f32 = 1.0;
        let max_cmp = new_r.max(new_g.max(new_b));
        if max_cmp > 255.0 {
            color_scale = 255.0 / max_cmp;
        }

        Color {
            r: (new_r * color_scale) as u8,
            g: (new_g * color_scale) as u8,
            b: (new_b * color_scale) as u8,
            a: 255,
        }
    }
}

