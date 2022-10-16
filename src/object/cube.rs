use super::Object;
use crate::vector;
use crate::ray;
use crate::scene;

pub struct Cube {
    pub bottom_left_front: vector::Vec3f,
    pub top_right_back: vector::Vec3f,
    pub reflective: bool,
}

impl Object for Cube {
    fn intersection(&self, ray: &ray::Ray, ii: &mut scene::IntersectionInformation) -> bool {
        let mut t_min = ray.min_t;
        let mut t_max = ray.max_t;

        let x_plane_normal = vector::Vec3f { x:1.0, y:0.0, z:0.0 };
        let inverse_x = 1.0 / vector::dot(&ray.direction, &x_plane_normal);
        let t_x0 = (self.bottom_left_front.x - vector::dot(&ray.origin, &x_plane_normal)) * inverse_x;
        let t_x1 = (self.top_right_back.x - vector::dot(&ray.origin, &x_plane_normal)) * inverse_x;
        if vector::dot(&ray.direction, &x_plane_normal) != 0.0 {
            t_min = f64::max(t_min, f64::min(t_x0, t_x1));
            t_max = f64::min(t_max, f64::max(t_x0, t_x1));
        } 
        let y_plane_normal = vector::Vec3f { x:0.0, y:1.0, z:0.0 };
        let inverse_y = 1.0 / vector::dot(&ray.direction, &y_plane_normal);
        let t_y0 = (self.bottom_left_front.y - vector::dot(&ray.origin, &y_plane_normal)) * inverse_y;
        let t_y1 = (self.top_right_back.y - vector::dot(&ray.origin, &y_plane_normal)) * inverse_y;
        if vector::dot(&ray.direction, &y_plane_normal) != 0.0 {
            t_min = f64::max(t_min, f64::min(t_y0, t_y1));
            t_max = f64::min(t_max, f64::max(t_y0, t_y1));
        } 
        let z_plane_normal = vector::Vec3f { x:0.0, y:0.0, z:1.0 };
        let inverse_z = 1.0 / vector::dot(&ray.direction, &z_plane_normal);
        let t_z0 = (self.bottom_left_front.z - vector::dot(&ray.origin, &z_plane_normal)) * inverse_z;
        let t_z1 = (self.top_right_back.z - vector::dot(&ray.origin, &z_plane_normal)) * inverse_z;
        if vector::dot(&ray.direction, &z_plane_normal) != 0.0 {
            t_min = f64::max(t_min, f64::min(t_z0, t_z1));
            t_max = f64::min(t_max, f64::max(t_z0, t_z1));
        } 

        if t_min < t_max {
            ii.t = t_min;
            ii.point = ray.point(t_min);
            ii.reflective_surface = self.reflective;
            let cube_center = self.bottom_left_front + (self.top_right_back-self.bottom_left_front).scale(0.5);
            let cti = (ii.point - cube_center).normalize(); // center to intersection (cti)
            if cti.x.abs() >= cti.y.abs() {
                if cti.x.abs() >= cti.z.abs() {
                    ii.normal = vector::Vec3f { x:cti.x.round(), y:0.0, z:0.0 };
                } else {
                    ii.normal = vector::Vec3f { x:0.0, y:0.0, z:cti.z.round() };
                }
            } else {
                if cti.y.abs() >= cti.z.abs() {
                    ii.normal = vector::Vec3f { x:0.0, y:cti.y.round(), z:0.0 };
                } else {
                    ii.normal = vector::Vec3f { x:0.0, y:0.0, z:cti.z.round() };
                }
            }
            return true;
        }
        false
    }
}
