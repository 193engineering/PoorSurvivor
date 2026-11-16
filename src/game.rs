use std::{f64::consts::PI, vec};
use oorandom::Rand32;
use crate::wasm4::*;

static mut PREVIOUS_GAMEPAD: u8 = 0;
static mut FRAME: u64 = 0;


//------------------------------------------------------------------------
enum TimerState{
    Yet,
    Notice
}

struct Timer {
    period: u32,
    current: u32,
    started: bool,
}

impl Timer {
    pub fn new(period: u32) -> Self {
        Self { 
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


//------------------------------------------------------------------------
#[derive(Clone)]
struct Player {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    life: i32,
    invincible: bool,
    attack_rate: u32,
}

impl Player {
    pub fn new() -> Self {
        let width: i32 = 7;
        let height: i32 = 15;
        Self { 
            x: SCREEN_SIZE as i32 / 2 - width/2,
            y: SCREEN_SIZE as i32 / 2 - height/2,
            w: width as u32,
            h: height as u32,
            life: 3,
            invincible: false,
            attack_rate: 120,
        }
    }
}


//------------------------------------------------------------------------
struct Enemy {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    life: i32,
    speed: i32,
}

impl Enemy {
    pub fn new() -> Self {
        let mut rng = unsafe { Rand32::new(FRAME) };
        let d = rng.rand_range(0..360) as f64;
        let rad = d/180. * PI;
        Self {
            x: (80. * rad.cos()) as i32 + 80,
            y: (80. * rad.sin()) as i32 + 80,
            w: 5,
            h: 5,
            life: 1,
            speed: 1,
        }
    }

    pub fn follow(&mut self, target_x: i32, target_y: i32) {
        if target_x < self.x {
            self.x -= self.speed;
        } else if target_x > self.x {
            self.x += self.speed;
        }
        if target_y < self.y {
            self.y -= self.speed;
        } else if target_y > self.y {
            self.y += self.speed;
        }
    }
}


//------------------------------------------------------------------------
struct Bullet {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    speed: u32,
    direction: f64,
    life: i32,
    damage: i32,
}

impl Bullet {
    pub fn new(start_x: i32, start_y: i32, target_x: i32, target_y: i32) -> Self {
        let dx: f64 = (target_x - start_x) as f64;
        let dy: f64 = (target_y - start_y) as f64;
        let rad = dy.atan2(dx);
        Self {
            x: start_x,
            y: start_y,
            w: 1,
            h: 1,
            speed: 2,
            direction: rad,
            life: 1,
            damage: 1,
        }
    }

    pub fn shoot(&mut self){
        self.x += (self.speed as f64 * self.direction.cos()) as i32;
        self.y += (self.speed as f64 * self.direction.sin()) as i32;
    }

    pub fn is_outed(&self) -> bool {
        self.x < 0 || SCREEN_SIZE < self.x as u32 || self.y < 0 || SCREEN_SIZE < self.y as u32 || self.life <= 0
    }
}


//------------------------------------------------------------------------
pub struct Game{
    score: u32,
    player: Player,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    clock: u64,
    timer_1sec: Timer,
    timer_spone: Timer,
    timer_enemy_move: Timer,
    timer_invincible: Timer,
    timer_bullet: Timer,
}

impl Game{
    pub fn new() -> Self {
        let player = Player::new();
        let mut game = Self { 
            score: 0,
            player: player.clone(),
            enemies: vec![],
            bullets: vec![],
            clock: 0,
            timer_1sec: Timer::new(60),
            timer_spone: Timer::new(120),
            timer_enemy_move: Timer::new(5),
            timer_invincible: Timer::new(60),
            timer_bullet: Timer::new(player.attack_rate),
        };
        game.timer_1sec.start(false);
        game.timer_spone.start(false);
        game.timer_enemy_move.start(false);
        game.timer_bullet.start(false);

        game
    }

    pub fn update(&mut self){
        if 0 == self.player.life {
            text("GAME OVER", 50, 60);
            text(format!("SCORE: {:8}", self.score), 20, 80);
            text("Press Z to retry...", 10, 120);
            unsafe {
                if *GAMEPAD1 & BUTTON_2 != 0 {
                    self.reset();
                }
            }
        }
        else{
            unsafe { FRAME = FRAME.wrapping_add(1) };
            self.timer_event();
            self.parse_input();
            self.hits();

            text(format!("TIME:{}, LIFE:{}", self.clock, self.player.life), 0, 2);
            text(format!("SCORE:{}", self.score), 0, 10);
            oval(self.player.x, self.player.y, self.player.w, self.player.h);
            for e in &self.enemies {
                oval(e.x, e.y, e.w, e.h);
            }
            for b in &self.bullets {
                oval(b.x, b.y, b.w, b.h);
            }
        }
    }

    fn hit(x1: i32, y1: i32, w1: u32, h1: u32, x2: i32, y2: i32, w2: u32, h2: u32) -> bool {
        x1 < x2 + w2 as i32 &&
        x1 + w1 as i32 > x2 &&
        y1 < y2 + h2 as i32 &&
        y1 + h1 as i32 > y2
    }

    pub fn hits(&mut self) {
        for mut e in self.enemies.iter_mut() {
            for b in self.bullets.iter_mut() {
                if Game::hit(b.x, b.y, b.w, b.h, e.x, e.y, e.w, e.h){
                    e.life -= b.damage;
                    b.life -= 1;
                    if 0 == e.life {
                        self.score += 100;
                    }
                }
            }

            if !self.player.invincible{
                if Game::hit(self.player.x, self.player.y, self.player.w, self.player.h, e.x, e.y, e.w, e.h) {
                    self.player.life -= 1;
                    self.player.invincible = true;
                    self.timer_invincible.start(true);
                    break;
                }
            }
        }
        self.enemies.retain(|e| e.life > 0);
        self.bullets.retain(|b| !b.is_outed());
    }

    pub fn timer_event(&mut self){
        match self.timer_1sec.tick() {
            TimerState::Yet => (),
            TimerState::Notice => self.clock += 1,
        };

        match self.timer_spone.tick() {
            TimerState::Yet => (),
            TimerState::Notice => {
                self.enemies.push(Enemy::new());
            },
        }

        match self.timer_enemy_move.tick() {
            TimerState::Yet => (),
            TimerState::Notice => {
                for e in self.enemies.iter_mut() {
                    e.follow(self.player.x + (self.player.w/2-1) as i32, self.player.y + (self.player.h/2-1) as i32);
                }
            },
        }

        match self.timer_invincible.tick() {
            TimerState::Yet => (),
            TimerState::Notice => {
                self.player.invincible = false;
                self.timer_invincible.stop();
            }
        }

        match self.timer_bullet.tick() {
            TimerState::Yet => (),
            TimerState::Notice => {
                let mut target_x = 0;
                let mut target_y = 0;
                let mut nearest_len = 0x7FFFFFFF;
                for e in &self.enemies {
                    let dx = e.x - self.player.x;
                    let dy = e.y - self.player.y;
                    let temp_len = dx.pow(2) + dy.pow(2);
                    if nearest_len > temp_len {
                        nearest_len = temp_len;
                        target_x = e.x;
                        target_y = e.y;
                    }
                }
                self.bullets.push(Bullet::new(
                    self.player.x + (self.player.w as i32/2), 
                    self.player.y + (self.player.h as i32/2),
                    target_x+1,
                    target_y+1
                ));
            }
        }

        for mut b in self.bullets.iter_mut() {
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

        if gamepad & BUTTON_RIGHT != 0 {
            self.player.x += 1;
        }
        if gamepad & BUTTON_LEFT != 0 {
            self.player.x -= 1;
        }
        if gamepad & BUTTON_DOWN != 0 {
            self.player.y += 1;
        }
        if gamepad & BUTTON_UP != 0 {
            self.player.y -= 1;
        }
    }

    fn reset(&mut self){
        self.score = 0;
        self.player = Player::new();
        self.enemies.clear();
        self.bullets.clear();
        self.clock = 0;
        self.timer_1sec.start(true);
        self.timer_bullet.start(true);
        self.timer_spone.start(true);
        self.timer_enemy_move.start(true);
        self.timer_invincible.stop();
    }
}