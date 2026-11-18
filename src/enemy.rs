use oorandom::Rand32;
use std::f32::consts::PI;

use crate::common::Geometry;


pub struct Enemy {
    pub g: Geometry,
    pub life: i32,
    pub speed: f32,
    pub point: u32,
}


#[allow(dead_code, unused)]
impl Enemy {
    pub fn new(seed: u64) -> Self {
        let mut rng = Rand32::new(seed);
        let d = rng.rand_range(0..360) as f32;
        let size = rng.rand_range(5..15) as u32;
        let rad = d/180. * PI;
        Self {
            g: Geometry {
                x: 120. * rad.cos() + 80.,
                y: 120. * rad.sin() + 80.,
                w: size,
                h: size,
            },
            life: size as i32 / 5,
            speed: 1.,
            point: size * 20,
        }
    }

    pub fn follow(&mut self, target: Geometry) {
        let (center_x, center_y) = self.g.center();
        let (target_center_x, target_center_y) = target.center();

        if target_center_x < center_x {
            self.g.x -= self.speed;
        } else if target_center_x > center_x {
            self.g.x += self.speed;
        }
        if target_center_y < center_y {
            self.g.y -= self.speed;
        } else if target_center_y > center_y {
            self.g.y += self.speed;
        }
    }
}