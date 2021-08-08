use cgmath::Vector3;
use cgmath::dot;
use cgmath::prelude::*;

use crate::tracer::material;
use crate::tracer::types;
use crate::tracer::traceable;

pub struct Cube {
    pub vMin: Vector3<f32>,
    pub vMax: Vector3<f32>,
    pub material: material::Material,
}

impl Cube {
    pub fn new(vMin: Vector3<f32>, vMax: Vector3<f32>, material: material::Material) -> Self {
        Self {
            vMin,
            vMax,
            material,
        }
    }
}

impl traceable::Traceable for Cube {
    fn ray_intersect(&self, ray: &types::Ray) -> std::option::Option<types::RayHit> {
        // TODO
        return None;
    }
}

