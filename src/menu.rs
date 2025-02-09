use pong_rust::{set_screen, Screen, SCREEN_HEIGHT, SCREEN_WIDTH};
use raylib::ffi::Rectangle;
use raylib::prelude::*;

pub fn menu(rl: &mut RaylibHandle, thread: &RaylibThread) {
    let buttonx: f32 = (SCREEN_WIDTH / 2.0) - (SCREEN_WIDTH / 8.0);
    let buttony: f32 = SCREEN_HEIGHT / 2.0;
    let buttonw: f32 = SCREEN_WIDTH / 4.0;
    let buttonh: f32 = SCREEN_HEIGHT / 12.0;

    let mut local_button = Rectangle {
        x: buttonx,
        y: buttony,
        width: buttonw,
        height: buttonh,
    };

    fn reset_button(
        button: &mut Rectangle,
        buttonx: &f32,
        buttony: &f32,
        buttonh: &f32,
        buttonw: &f32,
    ) {
        button.x = *buttonx;
        button.y = *buttony;
        button.height = *buttonh;
        button.width = *buttonw;
    }

    fn is_touching(
        currentx: &i32,
        currenty: &i32,
        buttonx: &f32,
        buttony: &f32,
        buttonh: &f32,
        buttonw: &f32,
    ) -> bool {
        if *currentx >= *buttonx as i32 && *currentx <= (*buttonx as i32 + *buttonw as i32) {
            if *currenty >= *buttony as i32 && *currenty <= (*buttony as i32 + *buttonh as i32) {
                return true;
            }
        }
        false
    }

    while !rl.window_should_close() {
        let currentx = rl.get_mouse_x();
        let currenty = rl.get_mouse_y();

        // UPDATE
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            if is_touching(&currentx, &currenty, &buttonx, &buttony, &buttonh, &buttonw) {
                set_screen(Screen::Game);
                return;
            }
        }
        if is_touching(&currentx, &currenty, &buttonx, &buttony, &buttonh, &buttonw) {
            local_button.width = buttonw + 12.0;
            local_button.height = buttonh + 4.0;
            local_button.y = buttony - 2.0;
            local_button.x = buttonx - 4.0;
        } else {
            reset_button(&mut local_button, &buttonx, &buttony, &buttonh, &buttonw);
        }

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

        d.draw_rectangle_rounded(local_button, 5.0, 8, Color::WHITE);

        d.draw_text(
            "Local Multiplayer",
            buttonx as i32 + buttonx as i32 / 16,
            buttony as i32 + buttony as i32 / 20,
            buttonh as i32 / 2,
            Color::BLACK,
        );
    }
}
