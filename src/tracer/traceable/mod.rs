pub mod shapes;

use crate::tracer::types;

pub trait Traceable {
    fn ray_intersect(&self, ray: &types::Ray) -> Option<types::RayHit>;
}

