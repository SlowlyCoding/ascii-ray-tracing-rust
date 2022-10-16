use crate::vector; 
use crate::ray; 
use crate::camera;
use crate::object;
use crate::options;
use crate::pixelbuffer;

#[derive(Copy, Clone)]
pub struct IntersectionInformation {
    pub t: f64,
    pub point: vector::Vec3f,
    pub normal: vector::Vec3f,
    pub reflective_surface: bool,
}

impl IntersectionInformation {
    pub fn new_empty() -> IntersectionInformation {
        IntersectionInformation {
            t: 0.0,
            point: vector::Vec3f { x:0.0, y:0.0, z:0.0 },
            normal: vector::Vec3f { x:0.0, y:0.0, z:0.0 },
            reflective_surface: false,
        }
    }
}


pub struct Scene {
    pub camera: camera::Camera,
    pub objects: Vec<Box<dyn object::Object>>,
    pub light: vector::Vec3f,
    pub options: options::Options,
}

impl Scene {
    pub fn render(&self, pixelbuffer: &mut pixelbuffer::PixelBuffer) {
        // calculate vectors used for the rays that shoot through each pixels
        let (pixel0, pixel_step_x, pixel_step_y) = self.camera.calculate_vectors(&pixelbuffer);
        // loop through each pixel
        for y in 0..pixelbuffer.height {
            for x in 0..pixelbuffer.width {
                // create a ray which shoots throgh that pixel
                let pixel = pixel0 + pixel_step_x.scale(x as f64) + pixel_step_y.scale(y as f64);
                let ray = ray::new(self.camera.view_point, pixel.normalize());
                // trace ray through the scene and assign the outputted char to the
                // corresponding index
                pixelbuffer.assign(self.trace_ray(&ray), y*pixelbuffer.width+x);
            }
        }

    }
    pub fn trace_ray(&self, ray: &ray::Ray) -> char {
        let mut pixel = ' '; // default background pixel
        let mut ii = IntersectionInformation::new_empty();
        if self.closest_intersection(&ray, &mut ii) {
            if ii.reflective_surface {
                let reflected_ray_direction = ray.direction - ii.normal.scale(vector::dot(&ray.direction, &ii.normal)*2.0);
                let reflected_ray = ray::new(
                    ii.point + reflected_ray_direction.scale(0.01),
                    reflected_ray_direction,
                );
                pixel = self.trace_ray(&reflected_ray);
            } else {
                let l = (self.light - ii.point).normalize();
                if self.options.shadows_enabled {
                    let light_ray = ray::new(ii.point+l.scale(0.01), l);
                    if self.closest_intersection(&light_ray, &mut ii) {
                        pixel = ' ';
                        return pixel;
                    }
                }
                let diffuse = f64::max(0.0, vector::dot(&ii.normal, &l));
                pixel = self.options.grayscale[(diffuse*self.options.grayscale.len() as f64) as usize]
            }

        }
        pixel
    }
    // returns the closest intersection of the whole scene
    pub fn closest_intersection(&self, ray: &ray::Ray, ii: &mut IntersectionInformation) -> bool {
        let mut any_intersection: bool = false;
        let mut temp_ii = IntersectionInformation::new_empty();
        let mut closest_t = ray.max_t;
        for i in 0..self.objects.len() {
            if self.objects[i].intersection(&ray, &mut temp_ii) {
                if temp_ii.t < closest_t {
                    closest_t = temp_ii.t;
                    *ii = temp_ii;
                }
                any_intersection = true;
            }
        }
        any_intersection
    }
}
