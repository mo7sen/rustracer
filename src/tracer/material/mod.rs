mod color;
pub use color::Color;

#[derive(Clone, Copy)]
pub struct Material {
    pub base_color: Color,
    pub diffuse_reflection: f32,
    pub specular_reflection: f32,
    pub specular_exp: f32,
    pub reflectiveness: f32,
    pub refractiveness: f32,
    pub refractive_index: f32,
}

impl Default for Material {
    fn default() -> Material {
        Material {
            base_color: Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            },
            diffuse_reflection: 1_f32,
            specular_reflection: 0_f32,
            specular_exp: 0_f32,
            reflectiveness: 0_f32,
            refractiveness: 0_f32,
            refractive_index: 1_f32,
        }
    }
}

impl Material {
    pub fn new(
        color: Color,
        diffuse_reflection: f32,
        specular_reflection: f32,
        specular_exp: f32,
        reflectiveness: f32,
        refractiveness: f32,
        refractive_index: f32,
    ) -> Self {
        Self {
            base_color: color,
            diffuse_reflection,
            specular_reflection,
            specular_exp,
            reflectiveness,
            refractiveness,
            refractive_index,
        }
    }
}
