use crate::vector; 
use crate::ray; 
use crate::camera;
use crate::sphere;
use crate::options;
use crate::pixelbuffer;

pub struct IntersectionInformation {
    pub point: vector::Vec3f,
    pub normal: vector::Vec3f,
}

pub struct Scene {
    pub camera: camera::Camera,
    pub objects: Vec<sphere::Sphere>,
    pub light: vector::Vec3f,
    pub options: options::Options,
}

impl Scene {
    pub fn render(&mut self, pixelbuffer: &mut pixelbuffer::PixelBuffer) {
        // calculate different camera vectors
        self.camera.calculate_vectors(&pixelbuffer);
        // loop through each pixel
        for y in 0..pixelbuffer.height {
            for x in 0..pixelbuffer.width {
                // create a ray for that pixel
                let pixel = self.camera.pixel0 + self.camera.pixel_step_x.scale(x as f64) + self.camera.pixel_step_y.scale(y as f64);
                let ray = ray::new(self.camera.view_point, pixel.normalize());
                // trace the ray through the scene and assign the outputted char to the
                // corresponding index
                pixelbuffer.assign(self.trace_ray(&ray), y*pixelbuffer.width+x);
            }
        }

    }
    pub fn trace_ray(&self, ray: &ray::Ray) -> char {
        let mut pixel = ' ';
        let mut ii = IntersectionInformation {
            point: vector::Vec3f { x:0.0, y:0.0, z:0.0 },
            normal: vector::Vec3f { x:0.0, y:0.0, z:0.0 },
        };
        if self.objects[0].intersection(&ray, &mut ii) {
            let l = (self.light - ii.point).normalize();
            let diffuse = f64::max(0.0, vector::dot(&ii.normal, &l));
            pixel = self.options.grayscale[(diffuse*self.options.grayscale.len() as f64) as usize]
        }
        pixel
    }
}
