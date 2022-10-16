use crate::vector;

pub struct Ray {
    pub origin: vector::Vec3f,
    pub direction: vector::Vec3f,
    pub min_t: f64,
    pub max_t: f64,
}

pub fn new(origin: vector::Vec3f, direction: vector::Vec3f) -> Ray {
    Ray {
        origin,
        direction,
        min_t: 0.0,
        max_t: 9999.9,
    }
}

impl Ray {
    pub fn point(&self, t: f64) -> vector::Vec3f{
        self.origin + self.direction.scale(t)
    }
}
