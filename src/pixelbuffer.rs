use crate::terminal;

pub struct PixelBuffer {
    pub width: u16,
    pub height: u16,
    pub pixels: Vec<char>,
}
// create new pixelpuffer and fill the pixels with a character
pub fn new(width: u16, height: u16) -> PixelBuffer {
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
   pub fn assign(&mut self, c: char, index: u16) {
       self.pixels[index as usize] = c;

   }
   pub fn display(&self) {
       // go to the top left corner
       terminal::goto(0, 0);
       // print pixelbuffer
       for y in 0..self.height {
           for x in 0..self.width {
               print!("{}", self.pixels[(y*self.width+x) as usize]);
           }
           println!("");
       }
   }
}
