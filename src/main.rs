mod options;
mod vector;
mod ray;
mod pixelbuffer;
mod camera; 
mod object;
mod scene;
use crossterm::cursor::{Hide, Show};
use crossterm::ExecutableCommand;
use std::io::stdout;

fn main() {
    let width = 80;
    let height = 35;
    let mut pixelbuffer = pixelbuffer::new( width, height );
    let scene = scene::Scene {
        camera: camera::Camera {
                    view_point:     vector::Vec3f { x:0.0, y:0.0, z:0.0 },
                    view_direction: vector::Vec3f { x:0.0, y:1.0, z:0.0 },
                    view_up:        vector::Vec3f { x:0.0, y:0.0, z:1.0 },
                    fov: 75.0,
                    },   
        objects: vec![
            Box::new(object::sphere::Sphere {
                center: vector::Vec3f { x:5.0, y:20.0, z:0.0 },
                radius: 7.0,
                reflective: false,
            }),
            Box::new(object::sphere::Sphere {
                center: vector::Vec3f { x:-5.0, y:20.0, z:0.0 },
                radius: 7.0,
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
    stdout().execute(Hide).expect("");
    scene.render(&mut pixelbuffer);
    pixelbuffer.display();
    stdout().execute(Show).expect("");
}
