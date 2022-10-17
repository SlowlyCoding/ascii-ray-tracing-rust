mod terminal;
mod options;
mod vector;
mod ray;
mod clock;
mod pixelbuffer;
mod camera; 
mod object;
mod scene;
use rand::Rng;

fn main() {
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
            // Box::new(object::cube::Cube {
            //     bottom_left_front: vector::Vec3f { x:-5.0, y:-5.0, z:-5.0 },
            //     top_right_back: vector::Vec3f { x:5.0, y:5.0, z:5.0 },
            //     reflective: false,
            // }),
        ],
        light: vector::Vec3f { x:0.0, y:0.0, z:0.0 },
        options: options::Options {
            max_ray_bounces: 5,
            shadows_enabled: true,
            grayscale: String::from(" .:-=+*#%@").chars().collect::<Vec<char>>(),
        },
    };
    let mut rng = rand::thread_rng();
    for _ in 0..=30 {
        scene.objects.push(
            Box::new(object::sphere::Sphere {
                center: vector::Vec3f { 
                    x:(rng.gen_range(-20.0..20.0)), 
                    y:(rng.gen_range(-20.0..20.0)), 
                    z:(rng.gen_range(-20.0..20.0)), 
                },
                radius: 2.5,
                reflective: false,
            }),
            );
    }

    let mut clock = clock::new(60);
    terminal::show_cursor(false);
    let mut angle: f64 = 0.0;
    while angle < 8.*3.14 {
        clock.start();
        scene.camera.view_point.x = angle.cos()*30.;
        scene.camera.view_point.y = angle.sin()*30.;
        scene.camera.view_direction = (vector::Vec3f{x:0.,y:0.,z:0.}-scene.camera.view_point).normalize();
        angle += 1.5 * clock.frametime;

        scene.render(&mut pixelbuffer);
        clock.finished_render();
        pixelbuffer.display();
        clock.finished_display();
        clock.finished_frame();
        clock.show_stats();
    }
    terminal::show_cursor(true);
}
