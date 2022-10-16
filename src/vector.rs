use std::{ops, fmt};

#[derive(Copy, Clone)]
pub struct Vec3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// implementing Display trait
impl fmt::Display for Vec3f {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} | {} | {} )", self.x, self.y, self.z)
    }
}

// implementing Add and Subtract trait
impl ops::Add for Vec3f {
    type Output = Vec3f;
    fn add(self, other: Vec3f) -> Vec3f {
        Vec3f {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl ops::Sub for Vec3f {
    type Output = Vec3f;
    fn sub(self, other: Vec3f) -> Vec3f {
        Vec3f {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// length(), normalize() and scale()
impl Vec3f {
    pub fn length(&self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }
    pub fn normalize(&self) -> Vec3f {
        let l = self.length();
        Vec3f {
            x: self.x/l,
            y: self.y/l,
            z: self.z/l,
        }
    }
    pub fn scale(&self, s: f64) -> Vec3f {
        Vec3f {
            x: self.x*s,
            y: self.y*s,
            z: self.z*s,
        }
    }
}

// dot product: dot(v1, v2)
pub fn dot(v1: &Vec3f, v2: &Vec3f) -> f64 {
    v1.x*v2.x + v1.y*v2.y + v1.z*v2.z
}
// cross product: cross(v1, v2)
pub fn cross(v1: &Vec3f, v2: &Vec3f) -> Vec3f {
    Vec3f {
        x: v1.y*v2.z - v1.z*v2.y,
        y: v1.z*v2.x - v1.x*v2.z,
        z: v1.x*v2.y - v1.y*v2.x,
    }
}
