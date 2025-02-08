use pong_rust::{get_screen, set_screen, Screen};
use std::process;

mod game;
mod menu;

fn main() {
    set_screen(Screen::Menu);

    while get_screen() != Screen::Close {
        match get_screen() {
            Screen::Menu => menu::menu(),
            Screen::Game => game::game(),
            _ => process::exit(1),
        }
    }
}
