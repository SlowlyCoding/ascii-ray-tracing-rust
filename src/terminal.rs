use std::io::stdout;
use std::time::Duration;
use crossterm::ExecutableCommand;
use crossterm::terminal::{size, enable_raw_mode, disable_raw_mode};
use crossterm::cursor::{MoveTo, Show, Hide};
use crossterm::event::{poll, read, Event, KeyCode};
use crate::scene;
use crate::vector;

// move cursor to a position
pub fn goto(x: u16, y: u16) {
    stdout().execute(MoveTo(x, y)).expect("moving cursor");
}

// get terminal size
pub fn get_size() -> (u16, u16) {
    let (width, height) = size().expect("getting terminal size");
    (width, height-2)

}

pub fn show_cursor(show: bool) {
    if show {
        stdout().execute(Show).expect("showing cursor");
    } else {
        stdout().execute(Hide).expect("hiding cursor");
    }
}

// raw mode enables input to be received without the need to press enter
pub fn raw_mode(enable: bool) {
    if enable {
        enable_raw_mode().expect("enabling raw mode");
    } else {
        disable_raw_mode().expect("disabling raw mode");
    }
}
 
pub fn handle_events(scene: &mut scene::Scene, frametime: f64) -> bool {
    raw_mode(true);
    if poll(Duration::from_millis(1)).expect("polling events") {
            let event = read().expect("reading events");

            // move forwards, backwards
            if event == Event::Key(KeyCode::Char('w').into()) {
                scene.camera.view_point = 
                    scene.camera.view_point + 
                    scene.camera.view_direction.scale(scene.options.camera_move_speed*frametime);
            } else if event == Event::Key(KeyCode::Char('s').into()) {
                scene.camera.view_point = 
                    scene.camera.view_point - 
                    scene.camera.view_direction.scale(scene.options.camera_move_speed*frametime);

            // move left, right
            } else if event == Event::Key(KeyCode::Char('a').into()) {
                scene.camera.view_point = 
                    scene.camera.view_point - 
                    scene.camera.view_left.scale(scene.options.camera_move_speed*frametime);
            } else if event == Event::Key(KeyCode::Char('d').into()) {
                scene.camera.view_point = 
                    scene.camera.view_point + 
                    scene.camera.view_left.scale(scene.options.camera_move_speed*frametime);

            // move up, down
            } else if event == Event::Key(KeyCode::Char('q').into()) {
                scene.camera.view_point = 
                    scene.camera.view_point - 
                    vector::Vec3f{x:0.0,y:0.0,z:1.0}.scale(scene.options.camera_move_speed*frametime);
            } else if event == Event::Key(KeyCode::Char('e').into()) {
                scene.camera.view_point = 
                    scene.camera.view_point + 
                    vector::Vec3f{x:0.0,y:0.0,z:1.0}.scale(scene.options.camera_move_speed*frametime);

            // tilt camera
            } else if event == Event::Key(KeyCode::Left.into()) {
                scene.camera.view_angle_z -= scene.options.camera_tilt_speed*frametime;
                scene.camera.view_angle_updated();
            } else if event == Event::Key(KeyCode::Right.into()) {
                scene.camera.view_angle_z += scene.options.camera_tilt_speed*frametime;
                scene.camera.view_angle_updated();
            } else if event == Event::Key(KeyCode::Down.into()) {
                scene.camera.view_angle_x -= scene.options.camera_tilt_speed*frametime;
                scene.camera.view_angle_updated();
            } else if event == Event::Key(KeyCode::Up.into()) {
                scene.camera.view_angle_x += scene.options.camera_tilt_speed*frametime;
                scene.camera.view_angle_updated();

            // change fov
            } else if event == Event::Key(KeyCode::Char('+').into()) {
                scene.camera.fov += scene.options.camera_move_speed*frametime;
            } else if event == Event::Key(KeyCode::Char('-').into()) {
                scene.camera.fov -= scene.options.camera_move_speed*frametime;

            // quit
            } else if event == Event::Key(KeyCode::Esc.into()) {
                return false;
            }
    }
    raw_mode(false);
    true
}
