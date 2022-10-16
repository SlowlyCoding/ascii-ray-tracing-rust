mod options;
mod vector;
mod ray;
mod pixelbuffer;
mod camera; 
mod sphere;
mod scene;

fn main() {
    let width = 50;
    let height = 25;
    let mut pixelbuffer = pixelbuffer::new( width, height );
    let mut scene = scene::Scene {
        camera: camera::new (
                    vector::Vec3f { x:0.0, y:0.0, z:0.0 },
                    vector::Vec3f { x:0.0, y:1.0, z:0.0 },
                    vector::Vec3f { x:0.0, y:0.0, z:1.0 },
                    80.0,
                    ),   
        objects: vec![
            sphere::Sphere {
                center: vector::Vec3f { x:0.0, y:20.0, z:0.0 },
                radius: 10.0,
            }
        ],
        light: vector::Vec3f { x:20.0, y:-5.0, z:30.0 },
        options: options::Options {
            max_ray_bounces: 5,
            shadows_enabled: true,
            grayscale: String::from(" .:-=+*#%@").chars().collect::<Vec<char>>(),
        },
    };
    scene.render(&mut pixelbuffer);
    pixelbuffer.display();
}
