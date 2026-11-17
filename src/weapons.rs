use crate::{common::Geometry, wasm4::SCREEN_SIZE};

pub struct Bullet {
    pub g: Geometry,
    pub speed: f32,
    pub vx: f32,
    pub vy: f32,
    pub life: i32,
    pub damage: i32,
}

#[allow(dead_code, unused)]
impl Bullet {
    pub fn new(start: Geometry, target: Geometry) -> Self {
        let (start_x, start_y) = start.center();
        let (target_x, target_y) = target.center();
        let dx: f32 = (target_x - start_x) as f32;
        let dy: f32 = (target_y - start_y) as f32;
        let rad = dy.atan2(dx);
        let speed = 2.;
        Self {
            g: Geometry {
                x: start_x,
                y: start_y,
                w: 1,
                h: 1,
            },
            speed: speed,
            vx: speed * rad.cos(),
            vy: speed * rad.sin(),
            life: 1,
            damage: 1,
        }
    }

    pub fn shoot(&mut self){
        self.g.x += self.vx;
        self.g.y += self.vy;
    }

    pub fn disappear(&self) -> bool {
        self.g.x < 0. || SCREEN_SIZE < self.g.x as u32 || self.g.y < 0. || SCREEN_SIZE < self.g.y as u32 || self.life <= 0
    }
}
