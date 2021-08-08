use cgmath::Vector3;
use cgmath::dot;
use cgmath::prelude::*;

use crate::tracer::material;
use crate::tracer::types;
use crate::tracer::traceable;

pub struct Disk {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub radius: f32,
    pub material: material::Material
}

impl Disk {
    pub fn new(position: Vector3<f32>, normal: Vector3<f32>, radius: f32, material: material::Material) -> Self {
        Self {
            position,
            normal: normal.normalize(),
            radius,
            material
        }
    }
}

impl traceable::Traceable for Disk {
    fn ray_intersect(&self, ray: &types::Ray) -> std::option::Option<types::RayHit> {
        let raydotnorm = dot(self.normal, ray.direction * -1.0);
        if raydotnorm < 1e-6 {
            return None;
        }

        let t = ((ray.origin - self.position).dot(self.normal)) / raydotnorm;
        if t < 0.0 {
            return None;
        }

        let mut rayhit = types::RayHit::default();
        rayhit.distance = t;
        rayhit.hit = ray.origin + (ray.direction * t);
        if (rayhit.hit - self.position).magnitude() > self.radius {
            return None;
        }
        rayhit.normal = self.normal;
        rayhit.material = self.material;
        return Some(rayhit);
    }
}

