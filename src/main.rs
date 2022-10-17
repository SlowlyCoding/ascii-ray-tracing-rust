use std::{thread, time};
mod terminal;
mod options;
mod vector;
mod ray;
mod pixelbuffer;
mod camera; 
mod object;
mod scene;

fn main() {
    //let width = 80;
    //let height = 35;
    let (width, height) = terminal::get_size();
    let mut pixelbuffer = pixelbuffer::new( width, height );
    let mut scene = scene::Scene {
        camera: camera::Camera {
                    view_point:     vector::Vec3f { x:0.0, y:0.0, z:0.0 },
                    view_direction: vector::Vec3f { x:0.0, y:1.0, z:0.0 },
                    view_up:        vector::Vec3f { x:0.0, y:0.0, z:1.0 },
                    fov: 75.0,
                    },   
        objects: vec![
            Box::new(object::sphere::Sphere {
                center: vector::Vec3f { x:-10.0, y:0.0, z:0.0 },
                radius: 4.0,
                reflective: false,
            }),
            Box::new(object::sphere::Sphere {
                center: vector::Vec3f { x:10.0, y:0.0, z:0.0 },
                radius: 4.0,
                reflective: false,
            }),
            Box::new(object::cube::Cube {
                bottom_left_front: vector::Vec3f { x:-5.0, y:-5.0, z:-5.0 },
                top_right_back: vector::Vec3f { x:5.0, y:5.0, z:5.0 },
                reflective: false,
            }),
        ],
        light: vector::Vec3f { x:20.0, y:-25.0, z:30.0 },
        options: options::Options {
            max_ray_bounces: 5,
            shadows_enabled: true,
            grayscale: String::from(" .:-=+*#%@").chars().collect::<Vec<char>>(),
        },
    };
    terminal::show_cursor(false);
    let mut angle: f64 = 0.0;
    while angle < 8.*3.14 {
        scene.camera.view_point.x = angle.cos()*30.;
        scene.camera.view_point.y = angle.sin()*30.;
        scene.camera.view_direction = (vector::Vec3f{x:0.,y:0.,z:0.}-scene.camera.view_point).normalize();

        scene.render(&mut pixelbuffer);
        pixelbuffer.display();
        thread::sleep(time::Duration::from_millis(15));
        angle += 0.03;
    }
    terminal::show_cursor(true);
}
