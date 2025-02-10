use core::fmt;
use std::sync::Mutex;

pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;
/// The limit of the player's upwards movement
pub const UP_LIMIT: f32 = SCREEN_HEIGHT / 160.0;
/// The limit of the player's downwards movement
pub const DOWN_LIMIT: f32 = SCREEN_HEIGHT - (SCREEN_HEIGHT / 5.6);
pub const TARGET_FPS: u32 = 60;

/// Posible screens/interfaces
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Screen {
    Menu,
    Game,
}

static CURRENT_SCREEN: Mutex<Screen> = Mutex::new(Screen::Menu);

/// Returns what's the current screen/interface.
/// The previous window has to end (normally with a Return) for the current one to show up.
pub fn get_screen() -> Screen {
    let screen = CURRENT_SCREEN.lock().unwrap();
    screen.clone()
}

/// Sets the screen/interface.
/// Ending the previous screen (e.g. with a return) is needed for the next one to show up.
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
