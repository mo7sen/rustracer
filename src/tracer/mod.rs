mod material;
pub mod traceable;

pub mod types {
    pub use super::material::Material as Material;
    pub use super::material::Color as Color;
    use super::traceable::Traceable;

    use cgmath::dot;
    use cgmath::prelude::*;
    use cgmath::Vector3;

    pub trait Illumine {
        fn get_direction(&self, at: Vector3<f32>) -> Vector3<f32>;
        fn get_position(&self) -> Vector3<f32>;
        fn get_intensity(&self) -> f32;
    }

    pub struct RayHit {
        pub distance: f32,
        pub hit: Vector3<f32>,
        pub normal: Vector3<f32>,
        pub material: Material,
    }

    impl Default for RayHit {
        fn default() -> Self {
            Self {
                distance: f32::MAX,
                hit: Vector3::zero(),
                normal: Vector3::zero(),
                material: Material::default(),
            }
        }
    }

    pub struct Light {
        position: Vector3<f32>,
        intensity: f32,
    }

    impl Light {
        pub fn new(position: Vector3<f32>, intensity: f32) -> Self {
            Self {
                position,
                intensity,
            }
        }
    }

    impl Illumine for Light {
        fn get_direction(&self, at: Vector3<f32>) -> Vector3<f32> {
            (self.position - at).normalize()
        }
        fn get_intensity(&self) -> f32 {
            self.intensity
        }
        fn get_position(&self) -> Vector3<f32> {
            self.position
        }
    }

    pub struct Scene {
        objects: Vec<Box<dyn super::traceable::Traceable>>,
        lights: Vec<Box<dyn Illumine>>,
    }

    unsafe impl Sync for Scene {}

    impl Scene {
        pub fn new() -> Self {
            Self {
                objects: Vec::new(),
                lights: Vec::new(),
            }
        }

        pub fn add_object(&mut self, object: Box<dyn super::traceable::Traceable>) -> &mut Self {
            self.objects.push(object);
            self
        }

        pub fn add_light(&mut self, light: Box<dyn Illumine>) -> &mut Self {
            self.lights.push(light);
            self
        }
    }

    impl Traceable for Scene {
        fn ray_intersect(&self, ray: &Ray) -> std::option::Option<RayHit> {
            let mut hit_res = RayHit::default();
            hit_res.distance = f32::MAX;
            let mut found = false;
            self.objects.iter().for_each(|object| {
                if let Some(hit_data) = object.ray_intersect(ray) {
                    if hit_data.distance < hit_res.distance {
                        hit_res = hit_data;
                        found = true;
                    }
                }
            });
            if found {
                Some(hit_res)
            } else {
                None
            }
        }
    }

    pub struct Ray {
        pub origin: Vector3<f32>,
        pub direction: Vector3<f32>,
    }

    impl Ray {
        pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Result<Self, String> {
            Ok(Self { origin, direction })
        }
    }

    pub struct PixelData {
        width: usize,
        height: usize,
        pub pixels: Vec<u8>,
    }

    impl PixelData {
        pub fn new(width: usize, height: usize) -> Self {
            Self {
                width,
                height,
                pixels: vec![0; width * height * 4],
            }
        }
        pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
            let pixel_idx: usize = (x + (y * self.width)) * 4;
            self.pixels[pixel_idx + 0] = color.b;
            self.pixels[pixel_idx + 1] = color.g;
            self.pixels[pixel_idx + 2] = color.r;
            self.pixels[pixel_idx + 3] = color.a;
        }
    }

    pub struct Surface {
        pub width: usize,
        pub height: usize,
        pub pixels: Arc<Mutex<PixelData>>,
    }

    unsafe impl Sync for Surface {}
    impl Surface {
        pub fn new(width: usize, height: usize) -> Self {
            Self {
                width,
                height,
                pixels: Arc::new(Mutex::new(PixelData::new(width, height))),
            }
        }
    }

    use std::sync::{Arc, Mutex};

    pub struct Camera {
        fov: f32,
        pub origin: Vector3<f32>,
        render_target: Option<Arc<Surface>>,
    }

    impl Camera {
        pub fn new(origin: Vector3<f32>, fov: f32) -> Self {
            Self {
                fov,
                origin,
                render_target: None,
            }
        }

        pub fn set_surface(&mut self, surface: Arc<Surface>) {
            self.render_target = Some(surface);
        }

        pub fn render_scene(&self, scene: &Scene) -> Result<(), String> {
            if let Some(render_target) = &self.render_target {
                let width = render_target.width;
                let height = render_target.height;
                let mut pool = scoped_threadpool::Pool::new(256);
                pool.scoped(|scoped| {
                    (0..width).for_each(|x| {
                        (0..height).for_each(|y| {
                            scoped.execute(move || {
                                let mut ray_dir = Vector3::zero();

                                ray_dir.x = (2_f32 * ((x as f32) + 0.5_f32) / (width as f32)
                                    - 1_f32)
                                    * (self.fov.to_radians() / 2.0).tan()
                                    * (width as f32)
                                    / (height as f32);

                                ray_dir.y = -(2_f32 * ((y as f32) + 0.5) / (height as f32) - 1_f32)
                                    * (self.fov.to_radians() / 2_f32).tan();

                                ray_dir.z = -1_f32;

                                let ray: Ray = Ray::new(self.origin, ray_dir.normalize()).unwrap();

                                let depth = 4;
                                let pixel_color = self.cast_ray(&ray, scene, depth);
                                render_target
                                    .pixels
                                    .lock()
                                    .unwrap()
                                    .set_pixel(x, y, pixel_color)
                            });
                        });
                    });
                });
                return Ok(());
            } else {
                return Err(String::from("No Render Target specified"));
            }
        }

        pub fn cast_ray(&self, ray: &Ray, scene: &crate::tracer::types::Scene, depth: u32) -> Color {
            if depth > 0 {
                if let Some(hit_data) = scene.ray_intersect(ray) {
                    let view_v3 = ray.direction;
                    let mut diffuse_light_intensity = 0.0;
                    let mut specular_light_intensity = 0.0;
                    let view_reflect_v3 = super::ops::reflect(view_v3, hit_data.normal);
                    let reflect_orig: Vector3<f32> =
                        if dot(view_reflect_v3, hit_data.normal) < 0_f32 {
                            hit_data.hit - hit_data.normal * 1e-3
                        } else {
                            hit_data.hit + hit_data.normal * 1e-3
                        };

                    let refract_dir = super::ops::refract(
                        view_v3,
                        hit_data.normal,
                        hit_data.material.refractive_index,
                    )
                    .normalize();
                    let refract_orig = if refract_dir.dot(hit_data.normal) < 0_f32 {
                        hit_data.hit - hit_data.normal * 1e-3
                    } else {
                        hit_data.hit + hit_data.normal * 1e-3
                    };
                    let refract_color = self.cast_ray(
                        &Ray::new(refract_orig, refract_dir).unwrap(),
                        scene,
                        depth - 1,
                    );

                    let reflect_color = self.cast_ray(
                        &Ray::new(reflect_orig, view_reflect_v3).unwrap(),
                        scene,
                        depth - 1,
                    );
                    for light in scene.lights.iter() {
                        let light_dir = light.get_direction(hit_data.hit);
                        let light_dot_norm = light_dir.dot(hit_data.normal);

                        let light_distance = (light.get_position() - hit_data.hit).magnitude();

                        // Shadows
                        let shadow_test_orig = if light_dot_norm < 0_f32 {
                            hit_data.hit - hit_data.normal * 1e-3
                        } else {
                            hit_data.hit + hit_data.normal * 1e-3
                        };
                        let shadow_ray = Ray::new(shadow_test_orig, light_dir).unwrap();
                        if let Some(shadow_hit) = scene.ray_intersect(&shadow_ray) {
                            if shadow_hit.distance < light_distance {
                                continue;
                            }
                        }
                        // End of: Shadows

                        let light_reflect_v3 = super::ops::reflect(light_dir, hit_data.normal);

                        diffuse_light_intensity +=
                            light.get_intensity() * (0.0_f32).max(light_dot_norm);
                        specular_light_intensity += light.get_intensity()
                            * light_reflect_v3
                                .dot(view_v3)
                                .max(0.0_f32)
                                .powf(hit_data.material.specular_exp)
                    }
                    // diffuse part
                    let mut final_color = hit_data.material.base_color
                        * diffuse_light_intensity
                        * hit_data.material.diffuse_reflection;

                    // specular part
                    final_color = final_color
                        + (Color::RGB(255, 255, 255)
                            * specular_light_intensity
                            * hit_data.material.specular_reflection);

                    // reflective part
                    final_color = final_color + (reflect_color * hit_data.material.reflectiveness);

                    // refractive part
                    final_color = final_color + (refract_color * hit_data.material.refractiveness);

                    return final_color;
                }
            }
            // Color::default()
            Color::RGB(155, 200, 100)
        }
    }
}

pub mod ops {
    use cgmath::InnerSpace;
    use cgmath::Vector3;

    pub fn reflect(incident: Vector3<f32>, normal: Vector3<f32>) -> Vector3<f32> {
        incident - normal * 2.0 * incident.dot(normal)
    }

    pub fn refract(
        incident: Vector3<f32>,
        normal: Vector3<f32>,
        refractive_index: f32,
    ) -> Vector3<f32> {
        let mut cos_theta_1 = -incident.dot(normal).max(-1.0).min(1.0); // Assuming both vectors are normalized
        let mut n1: f32 = 1.0; // Default 'n' in vacuum
        let mut n2: f32 = refractive_index;
        let mut n = normal;

        if cos_theta_1 < 0.0 {
            std::mem::swap(&mut n1, &mut n2);
            n = n * -1.0;
            cos_theta_1 = cos_theta_1 * -1.0;
        }

        let r: f32 = n1 / n2;
        let cos_theta_2_sq: f32 = 1.0 - r.powi(2) * (1.0 - cos_theta_1.powi(2));

        if cos_theta_2_sq < 0.0 {
            Vector3::unit_x()
        } else {
            incident * r + n * (r * cos_theta_1 - cos_theta_2_sq.sqrt())
        }
    }
}
