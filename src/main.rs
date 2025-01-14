use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;

struct Player {
    position: Vector2,
    size: Vector2,
    speed: f32,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Pong")
        .vsync()
        .build();

    let mut player1 = Player {
        position: Vector2::new(32.0, 32.0),
        size: Vector2::new(SCREEN_WIDTH / 64.0, SCREEN_HEIGHT / 6.0),
        speed: 3.5,
    };

    while !rl.window_should_close() {
        // UPDATE
        if rl.is_key_down(KEY_UP) || rl.is_key_down(KEY_W) { player1.position.y -= player1.speed; }
        if rl.is_key_down(KEY_DOWN) || rl.is_key_down(KEY_S) { player1.position.y += player1.speed; }

        // DRAW
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_rectangle_v(player1.position, player1.size, Color::WHITE);
    }
}
