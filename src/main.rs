use pong_rust::{get_screen, set_screen, Screen};
use std::process;

mod game;
mod menu;

fn main() {
    while get_screen() != Screen::Close {
        println!("{}", get_screen());
        match get_screen() {
            Screen::Menu => {
                set_screen(Screen::Close);
                menu::menu();
            }
            Screen::Game => {
                set_screen(Screen::Close);
                game::game();
            }
            _ => process::exit(1),
        }
    }
}
