use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};
use crate::settings::BLOCK_SIZE;

/**
 * Converts a game coordinate to a GUI coordinate by scaling it by BLOCK_SIZE
 * parameters:
 * - game_coord: Game coordinate to be converted
 * 
 * returns:
 * - f64: GUI coordinate
 */
pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

/**
 * Draw a single block at the specified position (x, y) with a given color
 * parameters:
 * - color: Color of the block
 * - x: x-coordinate of the block
 * - y: y-coordinate of the block
 * - ctx: Context for drawing
 * - g: Graphics context
 */
pub fn draw_block(color: Color, x: i32, y: i32, ctx: &Context, g: &mut G2d) {
    // Convert the game coordinates to GUI coordinates
    let gui_x: f64 = to_coord(x);
    let gui_y: f64 = to_coord(y);

    // Draw a rectangle (a block) at the calculated position
    rectangle(
        color,         
        [
            gui_x,
            gui_y,
            BLOCK_SIZE, 
            BLOCK_SIZE,  
        ],
        ctx.transform,                  
        g             
    );
}

/**
 * Draw a rectangle with a specified color, position, and size (in terms of game blocks)
 * parameters:
 * - color: Color of the rectangle
 * - x: x-coordinate of the rectangle
 * - y: y-coordinate of the rectangle
 * - width: Width of the rectangle
 * - height: Height of the rectangle
 * - ctx: Context for drawing
 * - g: Graphics context
 */
pub fn draw_rectangle(color: Color, x: i32, y: i32, width: u32, height: u32, ctx: &Context, g: &mut G2d) {
    // Convert the game coordinates to GUI coordinates
    let x: f64 = to_coord(x);
    let y: f64 = to_coord(y);

    // Draw a rectangle (bigger than a block) with the specified width and height
    rectangle(
        color,             
        [
            x,             
            y,             
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        ctx.transform,                  
        g             
    );
}
