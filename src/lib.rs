#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
mod game;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use game::Game;

static mut PREVIOUS_GAMEPAD: u8 = 0;

static GAME: Lazy<Mutex<Game>> = Lazy::new(|| Mutex::new(Game::new()));

#[no_mangle]
fn start() {
    // unsafe {
    //     *PALETTE = [
    //         0xfff6d3,
    //         0xf9a875,
    //         0xeb6b6f,
    //         0x7c3f58,
    //     ];
    // }
}

#[no_mangle]
fn update(){
    GAME.lock().expect("game_state").update();
}