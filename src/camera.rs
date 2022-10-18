use crate::vector;
use crate::pixelbuffer;

pub struct Camera {
    pub view_point: vector::Vec3f,
    pub view_angle_x: f64,
    pub view_angle_z: f64,
    pub view_direction: vector::Vec3f,
    pub view_up: vector::Vec3f,
    pub view_left: vector::Vec3f,
    pub fov: f64,
}

pub fn new(
    view_point: vector::Vec3f,
    view_angle_x: f64,
    view_angle_z: f64,
    fov: f64) -> Camera {
    Camera {
        view_point,
        view_angle_x,
        view_angle_z,
        view_direction: vector::Vec3f { x:view_angle_z.sin(), y:view_angle_z.cos(), z:view_angle_x.sin() },
        view_up: vector::Vec3f { x:view_angle_z.sin(), y:view_angle_z.cos(), z:view_angle_x.cos() },
        view_left: vector::Vec3f { x:(view_angle_z+(3.14/2.0)).sin(), y:(view_angle_z+(3.14/2.0)).cos(), z:view_angle_x.sin() },
        fov,
    }
}

impl Camera {
    pub fn view_angle_updated(&mut self) {
        self.view_direction.x = self.view_angle_z.sin();
        self.view_direction.y = self.view_angle_z.cos();
        self.view_direction.z = self.view_angle_x.sin();
        self.view_up.x = self.view_angle_z.sin();
        self.view_up.y = self.view_angle_z.cos();
        self.view_up.z = self.view_angle_x.cos();
        self.view_left.x = (self.view_angle_z+(3.14/2.0)).sin();
        self.view_left.y = (self.view_angle_z+(3.14/2.0)).cos();
        self.view_left.z = self.view_angle_x.sin();
    }
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
