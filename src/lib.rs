use core::fmt;
use std::sync::Mutex;

pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;
pub const UP_LIMIT: f32 = 8.0;
pub const DOWN_LIMIT: f32 = SCREEN_HEIGHT - 128.0;
pub const TARGET_FPS: u32 = 60;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Screen {
    Menu,
    Game,
}

static CURRENT_SCREEN: Mutex<Screen> = Mutex::new(Screen::Menu);

pub fn get_screen() -> Screen {
    let screen = CURRENT_SCREEN.lock().unwrap();
    screen.clone()
}

pub fn set_screen(screen: Screen) {
    let mut sc = CURRENT_SCREEN.lock().unwrap();
    *sc = screen;
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Screen::Game => write!(f, "Game"),
            Screen::Menu => write!(f, "Menu"),
        }
    }
}
