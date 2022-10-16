use crate::vector;
use crate::pixelbuffer;

pub struct Camera {
    pub view_point: vector::Vec3f,
    pub view_direction: vector::Vec3f,
    pub view_up: vector::Vec3f,
    pub fov: f64,
}

impl Camera {
    // pixel0: top left corrner of camera view frame
    // pixel_step: vector that goes from one pixel to the next one in x direction or y direction
    pub fn calculate_vectors(&self, pixelbuffer: &pixelbuffer::PixelBuffer) -> (vector::Vec3f, vector::Vec3f, vector::Vec3f) {
        let mut half_screen_x = vector::cross(&self.view_direction, &self.view_up);
        let mut half_screen_y = vector::cross(&self.view_direction, &half_screen_x).scale(2.0);
        half_screen_x = half_screen_x.scale(((self.fov/2.0)*3.1415/180.0).tan());
        half_screen_y = half_screen_y.scale((((self.fov*(pixelbuffer.height as f64/pixelbuffer.width as f64))/2.0)*3.1415/180.0).tan());
        let pixel0 = self.view_direction - half_screen_x - half_screen_y;
        let pixel_step_x = half_screen_x.scale(2.0 / pixelbuffer.width as f64);
        let pixel_step_y = half_screen_y.scale(2.0 / pixelbuffer.height as f64);
        (pixel0, pixel_step_x, pixel_step_y)
    }
}
