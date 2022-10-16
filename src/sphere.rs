use crate::vector;
use crate::ray;
use crate::scene;

pub struct Sphere {
    pub center: vector::Vec3f,
    pub radius: f64,
}

impl Sphere {
    pub fn intersection(&self, ray: &ray::Ray, ii: &mut scene::IntersectionInformation) -> bool {
        let a = vector::dot(&ray.direction, &ray.direction);
        let b = 2.0 * vector::dot(&ray.direction, &(ray.origin-self.center));
        let c = vector::dot(&(ray.origin-self.center), &(ray.origin-self.center)) - self.radius*self.radius;
        let discriminant = b*b - 4.0*a*c;
        if discriminant > 0.0 {
            let t1 = (-b + discriminant.sqrt()) / 2.0*a; 
            let t2 = (-b - discriminant.sqrt()) / 2.0*a; 
            let t = f64::min(t1, t2);
            if t > ray.min_t && t < ray.max_t {
                ii.point = ray.point(t);
                ii.normal = (ray.point(t)-self.center).normalize();
                return true;
            }
        }
        false
    }
}
