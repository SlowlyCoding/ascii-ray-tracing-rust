use std::io::stdout;
use crossterm::terminal::size;
use crossterm::cursor::{MoveTo, Show, Hide};
use crossterm::ExecutableCommand;

pub fn goto(x: u16, y: u16) {
    stdout().execute(MoveTo(x, y)).expect("failed to move cursor");
}

pub fn get_size() -> (u16, u16) {
    let (width, height) = size().expect("error getting terminal size");
    (width, height-2)

}

pub fn show_cursor(show: bool) {
    if show {
        stdout().execute(Show).expect("failed to show cursor");
    } else {
        stdout().execute(Hide).expect("failed to show cursor");
    }
}
