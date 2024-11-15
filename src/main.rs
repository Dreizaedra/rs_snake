mod draw;
mod direction;
mod block;
mod snake;
mod food;
mod settings;
extern crate rand;
extern crate piston_window;

use draw::draw_rectangle;
use piston_window::{PistonWindow, PressEvent, WindowSettings, clear, Button, Key, RenderEvent, UpdateEvent};
use snake::Snake;
use direction::Direction;
use settings::{BACKGROUND_COLOR, MOVING_PERIOD, SCREEN_HEIGHT, SCREEN_WIDTH, STARTING_X, STARTING_Y, WALL_COLOR};

fn main() {
    // Create a Piston window with specified dimensions and settings
    let mut window: PistonWindow = WindowSettings::new("Snake", [SCREEN_WIDTH, SCREEN_HEIGHT])
        .exit_on_esc(true) // Exit the game when the escape key is pressed
        .build()
        .expect("Failed to create PistonWindow");

    // Initialize the snake at the starting position
    let mut snake: Snake = Snake::new(STARTING_X, STARTING_Y);
    
    // Variable to accumulate the elapsed time for movement updates
    let mut time_acc: f64 = 0.0;

    // Main game loop
    while let Some(ev) = window.next() {
        // Handle key press events to change snake's direction
        if let Some(Button::Keyboard(key)) = ev.press_args() {
            let dir: Option<Direction> = match key {
                Key::Up => Some(Direction::Up),
                Key::Down => Some(Direction::Down),
                Key::Left => Some(Direction::Left),
                Key::Right => Some(Direction::Right),
                _ => None, // Ignore other keys
            };

            // Update snake's direction based on the key pressed
            snake.move_forward(dir);
        }

        // Handle update events (movement logic, game state updates)
        if let Some(update_args) = ev.update_args() {
            time_acc += update_args.dt; // Increment the time accumulator by the delta time (dt)
            // If enough time has passed, move the snake
            if time_acc > MOVING_PERIOD {
                snake.move_forward(None); // Move the snake in the current direction
                time_acc = 0.0; // Reset the time accumulator
            }
        }

        // Render the game state (drawing the game window and snake)
        if let Some(_) = ev.render_args() {
            window.draw_2d(&ev, |ctx, g, _| {
                clear(BACKGROUND_COLOR, g); // Clear the screen with the background color
                draw_rectangle(WALL_COLOR, 0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, &ctx, g); // Draw the game boundary (walls)
                snake.draw(&ctx, g); // Draw the snake on the screen
            });
        }
    }
}
