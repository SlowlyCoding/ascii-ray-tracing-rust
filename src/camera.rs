use crate::vector;
use crate::pixelbuffer;

pub struct Camera {
    pub view_point: vector::Vec3f,
    pub view_direction: vector::Vec3f,
    pub view_up: vector::Vec3f,
    pub fov: f64,
    pub pixel0: vector::Vec3f,
    pub pixel_step_x: vector::Vec3f,
    pub pixel_step_y: vector::Vec3f,
}

pub fn new(
    view_point: vector::Vec3f,
    view_direction: vector::Vec3f,
    view_up: vector::Vec3f,
    fov: f64) -> Camera {
    Camera {
    view_point,
    view_direction,
    view_up,
    fov,
    pixel0: vector::Vec3f { x:0.0, y:0.0, z:0.0 },
    pixel_step_x: vector::Vec3f { x:0.0, y:0.0, z:0.0 },
    pixel_step_y: vector::Vec3f { x:0.0, y:0.0, z:0.0 },
    }
}

impl Camera {
    pub fn calculate_vectors(&mut self, pixelbuffer: &pixelbuffer::PixelBuffer) {
        let mut half_screen_x = vector::cross(&self.view_direction, &self.view_up);
        let mut half_screen_y = vector::cross(&self.view_direction, &half_screen_x).scale(2.0);
        half_screen_x = half_screen_x.scale(((self.fov/2.0)*3.1415/180.0).tan());
        half_screen_y = half_screen_y.scale((((self.fov*(pixelbuffer.height as f64/pixelbuffer.width as f64))/2.0)*3.1415/180.0).tan());
        self.pixel0 = self.view_direction - half_screen_x - half_screen_y;
        self.pixel_step_x = half_screen_x.scale(2.0 / pixelbuffer.width as f64);
        self.pixel_step_y = half_screen_y.scale(2.0 / pixelbuffer.height as f64);
    }
}
