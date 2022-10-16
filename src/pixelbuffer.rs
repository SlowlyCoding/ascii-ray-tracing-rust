use crossterm::cursor::MoveTo;
use crossterm::ExecutableCommand;
use std::io::stdout;

pub struct PixelBuffer {
    pub width: i32,
    pub height: i32,
    pub pixels: Vec<char>,
}
// create new pixelpuffer and fill the pixels with a character
pub fn new(width: i32, height: i32) -> PixelBuffer {
    let mut pixelbuffer = PixelBuffer {
        width,
        height,
        pixels: Vec::new(),
    };
    pixelbuffer.fill('.');
    return pixelbuffer;
}

// fill(), assign() and display()
impl PixelBuffer {
   pub fn fill(&mut self, c: char) {
       for _ in 0..self.width*self.height {
               self.pixels.push(c);
       }
   }
   pub fn assign(&mut self, c: char, index: i32) {
       self.pixels[index as usize] = c;

   }
   pub fn display(&self) {
       // go to the top left of the terminal
       stdout().execute(MoveTo(0, 0)).expect("failed to move cursor");
       // print pixelbuffer
       for y in 0..self.height {
           for x in 0..self.width {
               print!("{}", self.pixels[(y*self.width+x) as usize]);
           }
           println!("");
       }
   }
}
