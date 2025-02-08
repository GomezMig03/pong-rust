use std::sync::OnceLock;

pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;
pub const UP_LIMIT: f32 = 8.0;
pub const DOWN_LIMIT: f32 = SCREEN_HEIGHT - 128.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Screen {
    Menu,
    Game,
    Close,
}

pub static CURRENT_SCREEN: OnceLock<Screen> = OnceLock::new();

pub fn get_screen() -> Screen {
    *CURRENT_SCREEN.get().unwrap()
}

pub fn set_screen(screen: Screen) {
    let _ = CURRENT_SCREEN.set(screen);
}
