use super::Object;
use crate::vector;
use crate::ray;
use crate::scene;

pub struct Triangle {
    pub p1: vector::Vec3f,
    pub p2: vector::Vec3f,
    pub p3: vector::Vec3f,
    pub reflective: bool,
}

impl Object for Triangle {
    fn intersection(&self, ray: &ray::Ray, ii: &mut scene::IntersectionInformation) -> bool {
        let plane_normal = vector::cross(&(self.p2-self.p1), &(self.p3-self.p1));
        let d = vector::dot(&self.p1, &plane_normal);
        // the same dotproduct gets calculated 3 times :(
        if vector::dot(&ray.direction, &plane_normal) ==  0.0 {return false;}
        let t = (d - vector::dot(&ray.direction, &plane_normal)) / vector::dot(&ray.direction, &plane_normal);
        let ii_point = ray.point(t);
        if vector::dot(&vector::cross(&(self.p2-self.p1),&(ii_point-self.p1)), &plane_normal) >= 0.0 && 
            vector::dot(&vector::cross(&(self.p3-self.p2),&(ii_point-self.p2)), &plane_normal) >= 0.0 && 
                vector::dot(&vector::cross(&(self.p1-self.p3),&(ii_point-self.p3)), &plane_normal) >= 0.0 { 
                    ii.t = t;
                    ii.point = ii_point;
                    ii.normal = plane_normal.normalize();
                    ii.reflective_surface = self.reflective;
                    return true;
                }
        false
    }
}
