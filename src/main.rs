use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 720.0;
const UP_LIMIT: f32 = 8.0;
const DOWN_LIMIT: f32 = SCREEN_HEIGHT - 128.0;

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
        speed: SCREEN_HEIGHT / 150.0,
    };

    let mut player2 = Player {
        position: Vector2::new(SCREEN_WIDTH - (SCREEN_WIDTH / 64.0) - 32.0, SCREEN_HEIGHT - (SCREEN_HEIGHT / 6.0) - 32.0),
        size: Vector2::new(SCREEN_WIDTH / 64.0, SCREEN_HEIGHT / 6.0),
        speed: SCREEN_HEIGHT / 150.0,
    };

    while !rl.window_should_close() {
        // UPDATE
        if (rl.is_key_down(KEY_UP) || rl.is_key_down(KEY_W)) && player1.position.y > UP_LIMIT { player1.position.y -= player1.speed; }
        if (rl.is_key_down(KEY_DOWN) || rl.is_key_down(KEY_S)) && player1.position.y < DOWN_LIMIT { player1.position.y += player1.speed; }  

        if rl.is_key_down(KEY_O) && player2.position.y > UP_LIMIT { player2.position.y -= player2.speed; }
        if rl.is_key_down(KEY_L) && player2.position.y < DOWN_LIMIT { player2.position.y += player2.speed; }  

        //println!("Player position Y: {}", player1.position.y);

        // DRAW
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_rectangle_v(player1.position, player1.size, Color::WHITE);
        d.draw_rectangle_v(player2.position, player2.size, Color::WHITE);
    }
}
