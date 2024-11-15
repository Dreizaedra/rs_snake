mod draw;
mod direction;
mod snake;
extern crate rand;
extern crate piston_window;

use draw::draw_rectangle;
use piston_window::{PistonWindow, PressEvent, WindowSettings, clear, Button, Key, RenderEvent, UpdateEvent};
use snake::Snake;
use direction::Direction;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;
const MOVING_PERIOD: f64 = 0.1;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake", [SCREEN_WIDTH, SCREEN_HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut snake = Snake::new(10, 10);
    let mut time_acc: f64 = 0.0;

    while let Some(ev) = window.next() {
        if let Some(Button::Keyboard(key)) = ev.press_args() {
            let dir: Option<Direction> = match key {
                Key::Up => Some(Direction::Up),
                Key::Down => Some(Direction::Down),
                Key::Left => Some(Direction::Left),
                Key::Right => Some(Direction::Right),
                _ => None,
            };

            snake.move_forward(dir);
        }

        if let Some(update_args) = ev.update_args() {
            time_acc += update_args.dt;
            if time_acc > MOVING_PERIOD {
                snake.move_forward(None);
                time_acc = 0.0;
            }
        }

        if let Some(_) = ev.render_args() {
            window.draw_2d(&ev, |ctx, g, _| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                draw_rectangle([0.5, 0.5, 0.5, 1.0], 0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, &ctx, g);
                snake.draw(&ctx, g);
            });
        }
    }
}
