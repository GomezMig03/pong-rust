use pong_rust::{set_screen, Screen, DOWN_LIMIT, SCREEN_HEIGHT, SCREEN_WIDTH, UP_LIMIT};
use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;

struct Player {
    position: Vector2,
    size: Vector2,
    speed: f32,
    movingup: bool,
    movingdown: bool,
}

struct Ball {
    position: Vector2,
    speed: Vector2,
    radius: f32,
}

pub fn game(rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut ball = Ball {
        position: Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
        speed: Vector2::new(4.5, 2.5),
        radius: SCREEN_WIDTH / 64.0,
    };

    let mut player1 = Player {
        position: Vector2::new(32.0, 32.0),
        size: Vector2::new(SCREEN_WIDTH / 64.0, SCREEN_HEIGHT / 6.0),
        speed: SCREEN_HEIGHT / 120.0,
        movingup: false,
        movingdown: false,
    };

    let mut player2 = Player {
        position: Vector2::new(
            SCREEN_WIDTH - (SCREEN_WIDTH / 64.0) - 32.0,
            SCREEN_HEIGHT - (SCREEN_HEIGHT / 6.0) - 32.0,
        ),
        size: Vector2::new(SCREEN_WIDTH / 64.0, SCREEN_HEIGHT / 6.0),
        speed: SCREEN_HEIGHT / 120.0,
        movingup: false,
        movingdown: false,
    };

    fn reset_ball(ball: &mut Ball, current_speed: &mut f32, rl: &RaylibHandle) {
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
        if player.movingup {
            if ball.speed.y > 0.0 {
                ball.speed.y *= -1.04;
            } else {
                ball.speed.y *= 1.04;
            }
        } else if player.movingdown {
            if ball.speed.y < 0.0 {
                ball.speed.y *= -1.04
            } else {
                ball.speed.y *= 1.04;
            }
        } else {
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
        }
        ball.speed.x *= -1.04;
    }

    let mut current_speed: f32;

    let mut points1: u32 = 0;
    let mut points2: u32 = 0;

    let mut hit: u32 = 0;

    while !rl.window_should_close() {
        player1.movingup = false;
        player1.movingdown = false;
        player2.movingup = false;
        player2.movingdown = false;

        // UPDATE
        if rl.is_key_down(KEY_W) && player1.position.y > UP_LIMIT {
            player1.movingup = true;
            player1.speed *= -1.0;
            player1.position.y += player1.speed;
            player1.speed *= -1.0;
        }
        if rl.is_key_down(KEY_S) && player1.position.y < DOWN_LIMIT {
            player1.movingdown = true;
            player1.position.y += player1.speed;
        }

        if rl.is_key_down(KEY_UP) && player2.position.y > UP_LIMIT {
            player2.movingup = true;
            player2.speed *= -1.0;
            player2.position.y += player2.speed;
            player2.speed *= -1.0;
        }
        if rl.is_key_down(KEY_DOWN) && player2.position.y < DOWN_LIMIT {
            player2.movingdown = true;
            player2.position.y += player2.speed;
        }

        // Esc button go back
        if rl.is_key_down(KEY_ESCAPE) {
            set_screen(Screen::Menu);
            return;
        }

        ball.position += ball.speed;

        // Player 2 makes a goal
        if ball.position.x <= ball.radius {
            current_speed = 1.0;
            reset_ball(&mut ball, &mut current_speed, &rl);
            points2 += 1; // puntos jugador 2
        }

        // Player 1 makes a goal
        if ball.position.x >= SCREEN_WIDTH - ball.radius {
            current_speed = -1.0;
            reset_ball(&mut ball, &mut current_speed, &rl);
            points1 += 1; // puntos jugador 1
        }

        // Player 1 hits the ball
        if ball.position.x <= player1.position.x + (player1.size.x + ball.radius)
            && ball.position.x >= player1.position.x + (ball.radius / 2.0)
            && (ball.position.y <= player1.position.y + player1.size.y + 4.0
                && ball.position.y >= player1.position.y)
        {
            if hit == 0 {
                ball_hit(&mut ball, &player1);
            }
            hit += 1;
        }

        // Player 2 hits the ball
        if ball.position.x >= player2.position.x - ball.radius
            && ball.position.x <= player2.position.x + (ball.radius / 2.0)
            && (ball.position.y <= player2.position.y + player2.size.y + 4.0
                && ball.position.y >= player2.position.y)
        {
            if hit == 0 {
                ball_hit(&mut ball, &player2);
            }
            hit += 1;
        }

        // Ball hits the floor or the ceiling
        if ball.position.y >= SCREEN_HEIGHT - ball.radius || ball.position.y <= ball.radius {
            ball.speed.y *= -1.0;
        }

        // To avoid multiple hits if the same player
        if hit != 0 {
            hit += 1;
        }
        if hit >= 18 {
            hit = 0;
        }

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
