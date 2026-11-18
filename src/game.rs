use std::cmp::min;

use crate::sprites::MAGICIAN_IMAGE;
use crate::wasm4::*;
use crate::common::*;
use crate::player::Player;
use crate::enemy::Enemy;
use crate::weapons::*;


const SPONE_LEVEL: u32 = 20;
const SPONE_PERIODS: [u32; 6] = [120, 110, 90, 70, 50, 30];

static mut PREVIOUS_GAMEPAD: u8 = 0;
static mut FRAME: u64 = 0;


//------------------------------------------------------------------------
enum Scene {
    Title,
    Play,
    Gameover,
}

pub struct Game{
    scene: Scene,
    score: u32,
    player: Player,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    clock: u32,
    timers: Vec<Timer>,
    attack_timer: Timer,
    best_score: u32,
}


impl Game{
    pub fn new() -> Self {
        let player = Player::new();
        let timers = vec![
            Timer::new("1sec".to_string(), 60),
            Timer::new("spone".to_string(), 120),
            Timer::new("enemy_move".to_string(), 5),
            Timer::new("invincible".to_string(), 60),
        ];
        let game = Self { 
            scene: Scene::Title,
            score: 0,
            player: player.clone(),
            enemies: vec![],
            bullets: vec![],
            clock: 0,
            timers: timers,
            attack_timer: Timer::new("bullet".to_string(), player.attack_rate),
            best_score: 0,
        };

        game
    }

    pub fn update(&mut self){
        self.parse_input();
        match self.scene {
            Scene::Title => {
                unsafe { *DRAW_COLORS = 3 }
                text("-------------", 30, 30);
                text("Poor Survivor", 30, 40);
                text("-------------", 30, 50);
                text(format!("BEST: {:5}", self.best_score), 35, 80);
                text("Press X to start!", 15, 120);
            }
            Scene::Play => {
                unsafe { *DRAW_COLORS = 3 }
                unsafe { FRAME = FRAME.wrapping_add(1) };
                self.timer_event();
                self.hits();

                text(format!("LIFE:{} LEVEL:{:02}  {:>03}", self.player.life, self.player.level+1, self.clock), 0, 2);
                text(format!("SCORE:{}", self.score), 0, 10);

                match self.player.invincible {
                    true => unsafe { *DRAW_COLORS = 0x3230 }
                    false => unsafe { *DRAW_COLORS = 0x2320 }
                }
                MAGICIAN_IMAGE.draw(self.player.g.x as i32, self.player.g.y as i32, 0);
                // oval(self.player.g.x as i32, self.player.g.y as i32, self.player.g.w, self.player.g.h);
                unsafe { *DRAW_COLORS = 4 }
                for e in &self.enemies {
                    oval(e.g.x as i32, e.g.y as i32, e.g.w, e.g.h);
                }
                for b in &self.bullets {
                    oval(b.g.x as i32, b.g.y as i32, b.g.w, b.g.h);
                }
            }
            Scene::Gameover => {
                unsafe { *DRAW_COLORS = 4 }
                text("GAME OVER", 40, 40);
                text(format!("SCORE: {:6}", self.score), 26, 70);
                text("[X] TITLE", 40, 110);
                text("[Z] RETRY", 40, 120);
            }
        }
    }

    pub fn hits(&mut self) {
        for e in self.enemies.iter_mut() {
            for b in self.bullets.iter_mut() {
                if hit(b.g, e.g){
                    e.life -= b.damage;
                    b.life -= 1;
                    if 0 == e.life {
                        self.score += e.point;
                        self.player.exp += e.point;
                        if self.player.levelup() {
                            self.attack_timer.period = self.player.attack_rate;
                        }
                    }
                }
            }

            if !self.player.invincible{
                if hit(self.player.g, e.g) {
                    self.player.life -= 1;
                    if 0 == self.player.life {
                        self.scene = Scene::Gameover;
                        if self.best_score < self.score {
                            self.best_score = self.score;
                        }
                        return;
                    }
                    self.player.invincible = true;
                    self.timers.iter_mut().filter(|t| t.tag.as_str() == "invincible").for_each(|t| t.start(true));
                    break;
                }
            }
        }
        self.enemies.retain(|e| e.life > 0);
        self.bullets.retain(|b| !b.disappear());
    }

    pub fn timer_event(&mut self){
        for timer in self.timers.iter_mut() {
            match timer.tag.as_str() {
                "1sec" => match timer.tick() {
                    TimerState::Yet => (),
                    TimerState::Notice => {
                        self.clock += 1;
                    }
                }

                "spone" => {
                    if 0 == self.clock % SPONE_LEVEL {
                        let i = min((self.clock/SPONE_LEVEL) as usize, SPONE_PERIODS.len()-1);
                        timer.period = SPONE_PERIODS[i];
                    }
                    match timer.tick() {
                        TimerState::Yet => (),
                        TimerState::Notice => {
                            self.enemies.push(Enemy::new(unsafe{FRAME}));
                        },
                    }
                }

                "enemy_move" => match timer.tick() {
                    TimerState::Yet => (),
                    TimerState::Notice => {
                        for e in self.enemies.iter_mut() {
                            e.follow(self.player.g);
                        }
                    },
                }

                "invincible" => match timer.tick() {
                    TimerState::Yet => (),
                    TimerState::Notice => {
                        self.player.invincible = false;
                        timer.stop();
                    }
                }
                _ => (),
            }
        }

         match self.attack_timer.tick() {
            TimerState::Yet => (),
            TimerState::Notice => {
                let mut target = Geometry::new();
                let mut nearest_len: f32 = 65535.;
                for e in &self.enemies {
                    let dx = e.g.x - self.player.g.x;
                    let dy = e.g.y - self.player.g.y;
                    let temp_len = dx.powf(2.) + dy.powf(2.);
                    if nearest_len > temp_len {
                        nearest_len = temp_len;
                        target = e.g;
                    }
                }
                self.bullets.push(Bullet::new(self.player.g, target));
            }
        }

        for b in self.bullets.iter_mut() {
            b.shoot();
        }
    }

    fn parse_input(&mut self){
        let (pressed_this_frame, gamepad, ..) = unsafe {
            let previous = PREVIOUS_GAMEPAD;
            let gamepad = *GAMEPAD1;
            let pressed_this_frame = gamepad & (gamepad ^ previous);
            PREVIOUS_GAMEPAD = gamepad;

            (pressed_this_frame, gamepad, previous)
        };

        match self.scene {
            Scene::Title => {
                if pressed_this_frame & BUTTON_1 != 0 {
                    self.reset();
                    self.scene = Scene::Play;
                }
            }
            Scene::Play => {
                if gamepad & BUTTON_RIGHT != 0 && self.player.g.x as u32 + self.player.g.w < SCREEN_SIZE {
                    self.player.g.x += 1.;
                }
                if gamepad & BUTTON_LEFT != 0 && self.player.g.x > 0. {
                    self.player.g.x -= 1.;
                }
                if gamepad & BUTTON_DOWN != 0 && self.player.g.y as u32 + self.player.g.h < SCREEN_SIZE {
                    self.player.g.y += 1.;
                }
                if gamepad & BUTTON_UP != 0 && self.player.g.y > 0. {
                    self.player.g.y -= 1.;
                }
            }
            Scene::Gameover => {
                if pressed_this_frame & BUTTON_1 != 0 {
                    self.scene = Scene::Title;
                }
                if pressed_this_frame & BUTTON_2 != 0 {
                    self.reset();
                    self.scene = Scene::Play;
                }
            }
        }
    }

    fn reset(&mut self){
        self.score = 0;
        self.player = Player::new();
        self.enemies.clear();
        self.bullets.clear();
        self.clock = 0;
        for timer in self.timers.iter_mut() {
            if "invincible" == timer.tag {
                timer.stop();
                continue;
            }
            timer.start(true);
        }
        self.attack_timer.start(true);
    }
}