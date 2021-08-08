use cgmath::Vector3;
use cgmath::dot;
use cgmath::prelude::*;

use crate::tracer::material;
use crate::tracer::types;
use crate::tracer::traceable;

pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub material: material::Material,
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32, material: material::Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn set_radius(&mut self, r: f32) -> Result<(), String> {
        self.radius = r;
        Ok(())
    }

    pub fn set_center(&mut self, c: Vector3<f32>) -> Result<(), String> {
        self.center = c;
        Ok(())
    }

    pub fn set_material(&mut self, material: material::Material) -> Result<(), String> {
        self.material = material;
        Ok(())
    }
}
impl traceable::Traceable for Sphere {
    fn ray_intersect(&self, ray: &types::Ray) -> Option<types::RayHit> {
        let orig2center_v3 = self.center - ray.origin;
        let orig2pc_f: f32 = dot(orig2center_v3, ray.direction);
        let center2pc_fsq: f32 = orig2center_v3.magnitude2() - orig2pc_f.powi(2);
        if center2pc_fsq > self.radius.powi(2) {
            return None;
        }

        let i02pc_f: f32 = (self.radius.powi(2) - center2pc_fsq).sqrt();
        let mut t0: f32 = orig2pc_f - i02pc_f;
        if t0 < 0_f32 {
            t0 = orig2pc_f + i02pc_f;
            if t0 < 0_f32 {
                return None;
            }
        }

        let mut rayhit = types::RayHit::default();
        rayhit.distance = t0;
        rayhit.hit = ray.origin + (ray.direction * t0);
        rayhit.normal = (rayhit.hit - self.center).normalize();
        rayhit.material = self.material;

        return Some(rayhit);
    }
}

