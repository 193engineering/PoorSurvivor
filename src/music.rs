use crate::{start, wasm4::*};


pub struct Sound {
    start_freq: u32,
    end_freq: u32,
    attack: u32,
    decay: u32,
    sustain: u32,
    release: u32,
    volume: u32,
    mode: u32
}


pub const SE_SHOT: Sound = Sound {
    start_freq: 2000,
    end_freq: 600,
    attack: 0,
    decay: 0,
    sustain: 0,
    release: 10,
    volume: 40,
    mode: TONE_PULSE1,
};


pub const SE_DAMAGE: Sound = Sound {
    start_freq: 100,
    end_freq: 400,
    attack: 0,
    decay: 0,
    sustain: 5,
    release: 0,
    volume: 50,
    mode: TONE_NOISE,
};


pub fn play(s: Sound) {
    tone(
        s.start_freq | s.end_freq << 16,
        s.sustain | s.attack << 24 | s.decay << 16 | s.release << 8,
        s.volume,
        s.mode
    );
}