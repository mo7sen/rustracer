use cgmath::{ Vector3, Point3 };
use cgmath::prelude::*;

use std::mem::swap;

use crate::tracer::material;
use crate::tracer::types;
use crate::tracer::traceable;

pub struct Cube {
    pub vmin: Vector3<f32>,
    pub vmax: Vector3<f32>,
    pub material: material::Material,
}

impl Cube {
    pub fn new(vmin: Vector3<f32>, vmax: Vector3<f32>, material: material::Material) -> Self {
        Self {
            vmin,
            vmax,
            material,
        }
    }

    pub fn normal_at(&self, point: Vector3<f32>) -> Vector3<f32> {
        let center = (self.vmax + self.vmin) / 2.0;
        let half_extents = (self.vmax - self.vmin) / 2.0;
        let local_point = point - center;

        let mut normal = Vector3::<f32>::new(1.0, 0.0, 0.0);

        let mut min = f32::INFINITY;

        let mut diff = (local_point.x.abs() - half_extents.x.abs()).abs();
        if diff < min {
            min = diff;
            normal = Vector3::unit_x() * local_point.x.signum();
        }

        diff = (local_point.y.abs() - half_extents.y.abs()).abs();
        if diff < min {
            min = diff;
            normal = Vector3::unit_y() * local_point.y.signum();
        }

        diff = (local_point.z.abs() - half_extents.z.abs()).abs();
        if diff < min {
            // min = diff;
            normal = Vector3::unit_z() * local_point.z.signum();
        }

        return normal.normalize();
    }
}

impl traceable::Traceable for Cube {
    fn ray_intersect(&self, ray: &types::Ray) -> std::option::Option<types::RayHit> {
        let o = ray.origin;
        let d = ray.direction;

        let t0: [f32;3] = (self.vmin - o).div_element_wise(d).into();
        let t1: [f32;3] = (self.vmax - o).div_element_wise(d).into();

        let mut tx_min = t0[0];
        let mut tx_max = t1[0];
        let mut ty_min = t0[1];
        let mut ty_max = t1[1];
        let mut tz_min = t0[2];
        let mut tz_max = t1[2];

        if tx_min > tx_max { swap(&mut tx_min, &mut tx_max); }
        if ty_min > ty_max { swap(&mut ty_min, &mut ty_max); }
        if tz_min > tz_max { swap(&mut tz_min, &mut tz_max); }

        let mut tmin = tx_min;
        let mut tmax = tx_max;

        if tmin > ty_max || tmax < ty_min { return None; }

        if ty_min > tmin { tmin = ty_min; }
        if ty_max < tmax { tmax = ty_max; }

        if tmin > tz_max || tmax < tz_min { return None; }

        if tz_min > tmin { tmin = tz_min; }
        if tz_max < tmax { tmax = tz_max; }

        let mut rayhit = types::RayHit::default();
        if tmin > 0.0 {
            rayhit.distance = tmin;
            rayhit.hit = o + (d * tmin);
        } else if tmax > 0.0 {
            rayhit.distance = tmax;
            rayhit.hit = o + (d * tmax);
        } else {
            return None;
        }
        rayhit.normal = self.normal_at(rayhit.hit);
        rayhit.material = self.material;

        return Some(rayhit);
    }
}

