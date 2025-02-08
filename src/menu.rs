use pong_rust::{set_screen, Screen, SCREEN_HEIGHT, SCREEN_WIDTH};
use raylib::ffi::Rectangle;
use raylib::prelude::*;

pub fn menu() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Pong in rust")
        .vsync()
        .build();

    let button = Rectangle {
        x: (SCREEN_WIDTH / 2.0) - (SCREEN_WIDTH / 8.0),
        y: SCREEN_HEIGHT / 2.0,
        width: SCREEN_WIDTH / 4.0,
        height: SCREEN_HEIGHT / 12.0,
    };

    while !rl.window_should_close() {
        // UPDATE

        // DRAW
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        d.draw_text(
            "Pong Rust",
            (SCREEN_WIDTH as i32 / 2) - (48 * 2 + 12),
            24,
            48,
            Color::WHITE,
        );

        d.draw_rectangle_rounded(button, 5.0, 8, Color::WHITE);

        d.draw_text(
            "Local Multiplayer",
            button.x as i32 + button.x as i32 / 16,
            button.y as i32 + button.y as i32 / 20,
            button.height as i32 / 2,
            Color::BLACK,
        );
    }
    set_screen(Screen::Close);
}
