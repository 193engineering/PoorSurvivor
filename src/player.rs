use crate::{common::Geometry, wasm4::*};

const LEVEL:[u32; 10] = [0, 300, 600, 1000, 1500, 2600, 3500, 5000, 10000, 20000];

#[allow(unused)]
#[derive(Clone)]
pub struct Player {
    pub g: Geometry,
    pub life: i32,
    pub invincible: bool,
    pub attack_rate: u32,
    pub exp: u32,
    pub level: u32,
}

impl Player {
    pub fn new() -> Self {
        let width: i32 = 13;
        let height: i32 = 13;
        Self { 
            g: Geometry { 
                x: (SCREEN_SIZE as i32 / 2 - width/2) as f32,
                y: (SCREEN_SIZE as i32 / 2 - height/2) as f32,
                w: width as u32,
                h: height as u32,
            },
            life: 3,
            invincible: false,
            attack_rate: 180,
            exp: 0,
            level: 0,
        }
    }

    pub fn levelup(&mut self) -> bool{
        let current = self.level;
        for (i, &border) in LEVEL.iter().enumerate() {
            if self.exp >= border {
                self.level = i as u32;
            }
        }
        if current < self.level {
            self.attack_rate = 180 - (self.level * 18);
            return true;
        }
        false
    }
}