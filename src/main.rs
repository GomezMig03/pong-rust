use pong_rust::{get_screen, Screen, SCREEN_HEIGHT, SCREEN_WIDTH};
use std::process;

mod game;
mod menu;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Pong")
        .vsync()
        .build();

    while !rl.window_should_close() {
        match get_screen() {
            Screen::Menu => menu::menu(&mut rl, &thread),
            Screen::Game => game::game(&mut rl, &thread),
            _ => process::exit(1),
        }
    }
}
