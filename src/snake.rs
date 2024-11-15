use piston_window::{Context, G2d};

use crate::draw::draw_block;
use crate::direction::Direction;
use crate::block::Block;
use crate::settings::{HEAD_COLOR, BODY_COLOR};

/**
 * The Snake struct represents the snake in the game, including its current direction and body
 */
#[derive(Debug)]
pub struct Snake {
    direction: Direction,
    body: Vec<Block>,
}

impl Snake {
    /**
     * Create a new Snake instance with the specified starting position
     * 
     * Parameters:
     * - x: x-coordinate of the snake's starting position
     * - y: y-coordinate of the snake's starting position
     * 
     * Returns:
     * A new Snake instance
     */
    pub fn new(x: i32, y: i32) -> Self {
        let mut body: Vec<Block> = Vec::new();
        // Initialize the body with 3 blocks (snake head + two body parts)
        body.push(Block { x: x + 2, y });
        body.push(Block { x: x + 1, y });
        body.push(Block { x, y });

        Self {
            direction: Direction::Right,
            body
        }
    }

    /**
     * Draw the snake on the screen
     * 
     * Parameters:
     * - ctx: Context for drawing
     * - g: Graphics context
     */
    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        // Draw the head of the snake (first block in the body)
        draw_block(HEAD_COLOR, self.body.first().unwrap().x, self.body.first().unwrap().y, ctx, g);
        
        // Draw the rest of the body (starting from the second block)
        for block in &self.body[1..] {
            draw_block(BODY_COLOR, block.x, block.y, ctx, g);
        }
    }

    /**
     * Get the current position of the snake's head
     * 
     * Returns:
     * - (i32, i32): Tuple containing the x and y coordinates of the snake's head
     */
    pub fn head_position(&self) -> (i32, i32) {
        // Retrieve the coordinates of the first block (the head of the snake)
        let head_block = self.body.first().unwrap();
        (head_block.x, head_block.y)
    }

    /**
     * Move the snake in the specified direction (or continue in the current direction)
     * 
     * Parameters:
     * - dir: Optional direction to move the snake
     */
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        // If a direction is specified, update the snake's direction unless it's the opposite
        match dir {
            Some(d) => {
                if self.direction.opposite() != d {
                    self.direction = d;
                }
            },
            None => {} // If no direction is provided, keep moving in the current direction
        }

        // Get the current head position (last block of the body)
        let (last_x, last_y): (i32, i32) = self.head_position();

        // Create a new block for the head based on the direction of movement
        let new_block = match self.direction {
            Direction::Up => Block { x: last_x, y: last_y - 1 },
            Direction::Down => Block { x: last_x, y: last_y + 1 },
            Direction::Left => Block { x: last_x - 1, y: last_y },
            Direction::Right => Block { x: last_x + 1, y: last_y }
        };

        // Add the new block to the front of the body (snake head)
        self.body.insert(0, new_block);

        // Remove the last block of the body (tail) to keep the snake's length constant
        self.body.pop();
    }
}
