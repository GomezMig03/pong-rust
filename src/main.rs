use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;

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
        position: Vector2::new(
            SCREEN_WIDTH - (SCREEN_WIDTH / 64.0) - 32.0,
            SCREEN_HEIGHT - (SCREEN_HEIGHT / 6.0) - 32.0,
        ),
        size: Vector2::new(SCREEN_WIDTH / 64.0, SCREEN_HEIGHT / 6.0),
        speed: SCREEN_HEIGHT / 120.0,
    };

    fn reset_ball(ball: &mut Ball, current_speed: &mut f32, rl: &mut RaylibHandle) {
        let mut yspeed = rl.get_random_value::<i32>(-4..4) as f32;
        if yspeed == 0.0 {
            yspeed = 1.0
        }

        ball.position = Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0);
        ball.speed = Vector2::new(
            (rl.get_random_value::<i32>(3..5) as f32) * *current_speed,
            yspeed,
        );
    }

    fn ball_hit(ball: &mut Ball, player: &Player) {
        if ball.position.y > player.position.y + (player.size.y / 2.0) {
            if ball.speed.y < 0.0 {
                ball.speed.y *= -1.04;
            } else {
                ball.speed.y *= 1.04;
            }
        } else {
            if ball.speed.y > 0.0 {
                ball.speed.y *= -1.04
            } else {
                ball.speed.y *= 1.04;
            }
        }

        ball.speed.x *= -1.04;
    }

    let mut current_speed: f32;

    let mut points1: u32 = 0;
    let mut points2: u32 = 0;

    let mut hit: u32 = 0;

    while !rl.window_should_close() {
        // UPDATE
        if rl.is_key_down(KEY_W) && player1.position.y > UP_LIMIT {
            player1.position.y -= player1.speed;
        }
        if rl.is_key_down(KEY_S) && player1.position.y < DOWN_LIMIT {
            player1.position.y += player1.speed;
        }

        if rl.is_key_down(KEY_UP) && player2.position.y > UP_LIMIT {
            player2.position.y -= player2.speed;
        }
        if rl.is_key_down(KEY_DOWN) && player2.position.y < DOWN_LIMIT {
            player2.position.y += player2.speed;
        }

        ball.position += ball.speed;

        // Player 2 makes a goal
        if ball.position.x <= ball.radius {
            current_speed = 1.0;
            reset_ball(&mut ball, &mut current_speed, &mut rl);
            points2 += 1; // puntos jugador 2
        }

        // Player 1 makes a goal
        if ball.position.x >= SCREEN_WIDTH - ball.radius {
            current_speed = -1.0;
            reset_ball(&mut ball, &mut current_speed, &mut rl);
            points1 += 1; // puntos jugador 1
        }

        if ball.position.x <= player1.position.x + (player1.size.x + ball.radius)
            && (ball.position.y <= player1.position.y + player1.size.y + 4.0
                && ball.position.y >= player1.position.y)
        {
            if hit == 0 {
                ball_hit(&mut ball, &player1);
            }
            hit += 1;
        }

        if ball.position.x >= player2.position.x - ball.radius
            && (ball.position.y <= player2.position.y + player2.size.y + 4.0
                && ball.position.y >= player2.position.y)
        {
            if hit == 0 {
                ball_hit(&mut ball, &player2);
            }
            hit += 1;
        }

        if ball.position.y >= SCREEN_HEIGHT - ball.radius || ball.position.y <= ball.radius {
            ball.speed.y *= -1.0;
        }

        if hit != 0 {
            hit += 1;
        }
        if hit >= 18 {
            hit = 0;
        }

        //println!("Player position Y: {}", player1.position.y);

        // DRAW
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        d.draw_text(
            &format!("Player 1: {points1}"),
            SCREEN_WIDTH as i32 / 16,
            12,
            24,
            Color::WHITE,
        );
        d.draw_text(
            &format!("Player 2: {points2}"),
            SCREEN_WIDTH as i32 - 256,
            12,
            24,
            Color::WHITE,
        );

        d.draw_rectangle_v(player1.position, player1.size, Color::WHITE);
        d.draw_rectangle_v(player2.position, player2.size, Color::WHITE);
        d.draw_circle_v(ball.position, ball.radius, Color::WHITE);
    }
}
