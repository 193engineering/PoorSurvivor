pub enum TimerState{
    Yet,
    Notice
}

pub struct Timer {
    pub tag: String,
    pub started: bool,
    pub period: u32,
    current: u32,
}

#[allow(dead_code, unused)]
impl Timer {
    pub fn new(tag: String, period: u32) -> Self {
        Self { 
            tag: tag,
            period: period,
            current: period,
            started: false,
        }
    }

    pub fn start(&mut self, reset: bool){
        if reset {
            self.current = self.period;
        }
        self.started = true;
    }

    pub fn stop(&mut self){
        self.started = false;
    }

    pub fn tick(&mut self) -> TimerState{
        if self.started && self.current > 0 {
            self.current -= 1;
            return TimerState::Yet;
        }
        else if 0 == self.current {
            self.current = self.period;
            return TimerState::Notice;
        }
        TimerState::Yet
    }
}


#[derive(Copy, Clone)]
pub struct Geometry {
    pub x: f32,
    pub y: f32,
    pub w: u32,
    pub h: u32,
}

#[allow(dead_code, unused)]
impl Geometry{
    pub fn new() -> Self {
        Self { x: 0., y: 0., w: 0, h: 0 }
    }

    pub fn center(&self) -> (f32, f32) {
        let x = self.x + (self.w/2) as f32 - 1.;
        let y = self.y + (self.h/2) as f32 - 1.;
        (x,y)
    }
}


pub fn hit(a: Geometry, b: Geometry) -> bool {
    a.x < b.x + b.w as f32 &&
    a.x + a.w as f32 > b.x &&
    a.y < b.y + b.h as f32 &&
    a.y + a.h as f32 > b.y
}