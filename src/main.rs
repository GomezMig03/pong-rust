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

struct Ball {
    position: Vector2,
    speed: Vector2,
    radius: f32,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Pong")
        .vsync()
        .build();
    
    let mut ball = Ball {
        position: Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
        speed: Vector2::new(4.5, 2.5),
        radius: 20.0,
    };

    let mut player1 = Player {
        position: Vector2::new(32.0, 32.0),
        size: Vector2::new(SCREEN_WIDTH / 64.0, SCREEN_HEIGHT / 6.0),
        speed: SCREEN_HEIGHT / 120.0,
    };

    let mut player2 = Player {
        position: Vector2::new(SCREEN_WIDTH - (SCREEN_WIDTH / 64.0) - 32.0, SCREEN_HEIGHT - (SCREEN_HEIGHT / 6.0) - 32.0),
        size: Vector2::new(SCREEN_WIDTH / 64.0, SCREEN_HEIGHT / 6.0),
        speed: SCREEN_HEIGHT / 120.0,
    };


    fn resetBall(ball: &mut Ball, currentSpeed: &mut f32, rl: &mut RaylibHandle) {
        let mut yspeed = rl.get_random_value::<i32>(-4..4) as f32;
        if yspeed == 0.0 { yspeed = 1.0 }

        ball.position = Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0);
        ball.speed = Vector2::new((rl.get_random_value::<i32>(3..5) as f32) * *currentSpeed, yspeed);
    }

    let mut currentSpeed: f32 = 1.0;

    let mut points1: u32 = 0;
    let mut points2: u32 = 0;

    while !rl.window_should_close() {
        // UPDATE
        if (rl.is_key_down(KEY_UP) || rl.is_key_down(KEY_W)) && player1.position.y > UP_LIMIT { player1.position.y -= player1.speed; }
        if (rl.is_key_down(KEY_DOWN) || rl.is_key_down(KEY_S)) && player1.position.y < DOWN_LIMIT { player1.position.y += player1.speed; }  

        if rl.is_key_down(KEY_O) && player2.position.y > UP_LIMIT { player2.position.y -= player2.speed; }
        if rl.is_key_down(KEY_L) && player2.position.y < DOWN_LIMIT { player2.position.y += player2.speed; } 

        ball.position += ball.speed;

        if ball.position.x <= ball.radius {
            currentSpeed = 1.0;
            resetBall(&mut ball, &mut currentSpeed, &mut rl);
            points2 += 1; // puntos jugador 2
        }

        if ball.position.x >= SCREEN_WIDTH - ball.radius {
            currentSpeed = -1.0;
            resetBall(&mut ball, &mut currentSpeed, &mut rl);
            points1 += 1; // puntos jugador 1
        }

        if ball.position.x <= player1.position.x + (player1.size.x + ball.radius)
        && (ball.position.y <= player1.position.y + player1.size.y + 4.0 
        && ball.position.y >= player1.position.y)  { 
            ball.speed.x *= -1.05;
            ball.speed.y *= 1.05;
        }

        if ball.position.x >= player2.position.x - ball.radius
        && (ball.position.y <= player2.position.y + player2.size.y + 4.0 
        && ball.position.y >= player2.position.y){
            ball.speed.x *= -1.05;
            ball.speed.y *= 1.05;
        }


        if ball.position.y >= SCREEN_HEIGHT - ball.radius
        || ball.position.y <= ball.radius {
            ball.speed.y *= -1.0;
        }

        //println!("Player position Y: {}", player1.position.y);

        // DRAW
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        d.draw_text(&format!("Player 1: {points1}"), SCREEN_WIDTH as i32 / 16, 12, 24, Color::WHITE);
        d.draw_text(&format!("Player 2: {points2}"), SCREEN_WIDTH as i32 - 256, 12, 24, Color::WHITE);

        d.draw_rectangle_v(player1.position, player1.size, Color::WHITE);
        d.draw_rectangle_v(player2.position, player2.size, Color::WHITE);
        d.draw_circle_v(ball.position, ball.radius, Color::WHITE);
    }
}
