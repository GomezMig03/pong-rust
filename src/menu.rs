use pong_rust::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub fn menu() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Pong in rust")
        .vsync()
        .build();
}
