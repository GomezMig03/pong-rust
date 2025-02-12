use pong_rust::{set_screen, Screen, SCREEN_HEIGHT, SCREEN_WIDTH};
//use raylib::consts::GuiControl::*;
//use raylib::consts::GuiDefaultProperty::*;
use raylib::ffi::Rectangle;
use raylib::prelude::*;
//use raylib::rstr;

pub fn menu(rl: &mut RaylibHandle, thread: &RaylibThread) {
    let buttonx: f32 = (SCREEN_WIDTH / 2.0) - (SCREEN_WIDTH / 8.0);
    let buttony: f32 = SCREEN_HEIGHT / 2.0;
    let buttonw: f32 = SCREEN_WIDTH / 4.0;
    let buttonh: f32 = SCREEN_HEIGHT / 12.0;

    //let mut font_size: i32 = 24;

    let mut local_button = Rectangle {
        x: buttonx,
        y: buttony,
        width: buttonw,
        height: buttonh,
    };

    /// Gives a button the given coordinates and size
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

    /// Detects if the cursor (currentx and currenty) is inside given coordinates (button values)
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
            //font_size = 26;
        } else {
            reset_button(&mut local_button, &buttonx, &buttony, &buttonh, &buttonw);
            //font_size = 24;
        }

        // DRAW
        let mut d = rl.begin_drawing(&thread);

        //d.gui_set_style(DEFAULT, TEXT_SIZE as i32, font_size);
        //if d.gui_button(local_button, Some(rstr!("Local Multiplayer"))) {}

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
