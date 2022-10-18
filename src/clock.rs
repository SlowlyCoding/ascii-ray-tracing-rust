use std::time::{Duration, Instant};
use std::thread;

pub struct Clock {
    pub fps_limit: u64,
    pub t_start: Instant,
    pub t_render: Instant,
    pub t_display: Instant,
    pub t_frame: Instant,
    // times in seconds
    pub rendertime: f64,
    pub displaytime: f64,
    pub frametime: f64,
}
pub fn new(fps_limit: u64) -> Clock{
    Clock {
        fps_limit,
        t_start: Instant::now(),
        t_render: Instant::now(),
        t_display: Instant::now(), 
        t_frame: Instant::now(),
        rendertime: 0.0,
        displaytime: 0.0,
        frametime: 0.0,
    }
}

impl Clock {
    pub fn start(&mut self) {
        self.t_start = Instant::now();
    }
    pub fn finished_render(&mut self) {
        self.t_render = Instant::now();
        self.rendertime = (self.t_render - self.t_start).as_micros() as f64 / 1000000.;
    }
    pub fn finished_display(&mut self) {
        self.t_display = Instant::now();
        self.displaytime = (self.t_display - self.t_render).as_micros() as f64 / 1000000.;
    }
    pub fn finished_frame(&mut self) {
        self.t_frame = Instant::now();
        self.frametime = (self.t_frame - self.t_start).as_micros() as f64 / 1000000.;

        // limiting fps
        if self.fps_limit != 0 {
            sleep( (1000.0/self.fps_limit as f64 - self.frametime*1000.0) as u64 );
            self.t_frame = Instant::now();
            self.frametime = (self.t_frame - self.t_start).as_micros() as f64 / 1000000.;
        }
    }
    pub fn display_stats(&self) {
        println!("  fps: {:.0} | render: {:.2}ms | display: {:.2}ms | frame : {:.2}ms       ", 
                 1./self.frametime, 
                 self.rendertime*1000.0,
                 self.displaytime*1000.0,
                 self.frametime*1000.0);
    }

}
pub fn sleep(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}
