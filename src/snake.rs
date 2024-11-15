use piston_window::types::Color;
use piston_window::{Context, G2d};

use crate::draw::draw_block;
use crate::direction::Direction;

const HEAD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BODY_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: Vec<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Self {
        let mut body: Vec<Block> = Vec::new();
        body.push(Block { x: x + 2, y });
        body.push(Block { x: x + 1, y });
        body.push(Block { x, y });

        Self {
            direction: Direction::Right,
            body
        }
    }

    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        draw_block(HEAD_COLOR, self.body.first().unwrap().x, self.body.first().unwrap().y, ctx, g);
        
        for block in &self.body[1..] {
            draw_block(BODY_COLOR, block.x, block.y, ctx, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.first().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => {
                if self.direction.opposite() != d {
                    self.direction = d;
                }
            },
            None => {}
        }

        let (last_x, last_y): (i32, i32) = self.head_position();

        let new_block = match self.direction {
            Direction::Up => Block { x: last_x, y: last_y - 1 },
            Direction::Down => Block { x: last_x, y: last_y + 1 },
            Direction::Left => Block { x: last_x - 1, y: last_y },
            Direction::Right => Block { x: last_x + 1, y: last_y }
        };

        self.body.insert(0, new_block);
        self.body.pop();
    }
}