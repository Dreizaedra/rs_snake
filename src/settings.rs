use piston_window::types::Color;

pub const SCREEN_WIDTH: u32 = 640;
pub const SCREEN_HEIGHT: u32 = 480;
pub const BACKGROUND_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
pub const WALL_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
pub const MOVING_PERIOD: f64 = 0.1;

pub const BLOCK_SIZE: f64 = 25.0;
pub const STARTING_X: i32 = 10;
pub const STARTING_Y: i32 = 10;
pub const HEAD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
pub const BODY_COLOR: Color = [0.00, 0.80, 0.00, 1.0];
pub const FOOD_COLOR: Color = [1.0, 1.0, 0.0, 1.0];