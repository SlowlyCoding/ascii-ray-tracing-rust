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
        camera: camera::new (
                    vector::Vec3f { x:30.0, y:-30.0, z:10.0 },
                    0.0, -0.7,
                    75.0,
                    ),   
        objects: vec![
            Box::new(object::triangle::Triangle {
                p1: vector::Vec3f { x:-20.0, y:-20.0, z:0.0 },
                p2: vector::Vec3f { x:20.0, y:-20.0, z:0.0 },
                p3: vector::Vec3f { x:20.0, y:20.0, z:0.0 },
                reflective: false,
            }),
            Box::new(object::triangle::Triangle {
                p1: vector::Vec3f { x:-20.0, y:-20.0, z:0.0 },
                p2: vector::Vec3f { x:20.0, y:20.0, z:0.0 },
                p3: vector::Vec3f { x:-20.0, y:20.0, z:0.0 },
                reflective: false,
            }),
            Box::new(object::triangle::Triangle {
                p1: vector::Vec3f { x:-20.0, y:20.0, z:2.0 },
                p2: vector::Vec3f { x:20.0, y:20.0, z:2.0 },
                p3: vector::Vec3f { x:20.0, y:20.0, z:10.0 },
                reflective: true,
            }),
            Box::new(object::triangle::Triangle {
                p1: vector::Vec3f { x:-20.0, y:20.0, z:2.0 },
                p2: vector::Vec3f { x:-20.0, y:20.0, z:10.0 },
                p3: vector::Vec3f { x:20.0, y:20.0, z:10.0 },
                reflective: true,
            }),
            Box::new(object::cube::Cube {
                bottom_left_front: vector::Vec3f { x:-3.0, y:-3.0, z:0.0 },
                top_right_back: vector::Vec3f { x:3.0, y:3.0, z:6.0 },
                reflective: false,
            }),
            Box::new(object::sphere::Sphere {
                center: vector::Vec3f { x:-8.0, y:15.0, z:2.0 },
                radius: 2.0,
                reflective: false,
            }),
            Box::new(object::sphere::Sphere {
                center: vector::Vec3f { x:13.0, y:10.0, z:2.0 },
                radius: 2.0,
                reflective: false,
            }),
            Box::new(object::sphere::Sphere {
                center: vector::Vec3f { x:-10.0, y:-14.0, z:2.0 },
                radius: 2.0,
                reflective: false,
            }),
        ],
        light: vector::Vec3f { x:rand::thread_rng().gen_range(-20.0..20.0), y:rand::thread_rng().gen_range(-20.0..20.0), z:20.0 },
        options: options::Options {
            camera_move_speed: 35.0,
            camera_tilt_speed: 5.0,
            fps_limit: 60, // 0 to disable fps_limit
            shadows_enabled: true,
            grayscale: String::from(" .:-=+*#%@").chars().collect::<Vec<char>>(),
        },
    };
    let mut clock = clock::new(scene.options.fps_limit);

    terminal::show_cursor(false);
    terminal::raw_mode(true);
    let mut run = true;
    while run {
        clock.start();
        scene.render(&mut pixelbuffer);
        clock.finished_render();
        pixelbuffer.display();
        clock.finished_display();
        run = terminal::handle_events(&mut scene, clock.frametime);
        clock.finished_frame();
        clock.display_stats();
    }
    terminal::raw_mode(false);
    terminal::show_cursor(true);
}
