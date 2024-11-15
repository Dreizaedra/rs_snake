use crate::block::Block;
use crate::rand::Rng;
use crate::draw::draw_block;
use piston_window::{Context, G2d};
use crate::settings::{FOOD_COLOR, SCREEN_HEIGHT, SCREEN_WIDTH};

#[derive(Debug)]
pub struct Food<Block> {
    pub block: Block,
}

impl Food<Block> {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            block: Block { x, y }
        }
    }

    pub fn spawn(&mut self) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..SCREEN_WIDTH) as i32;
        let y = rng.gen_range(0..SCREEN_HEIGHT) as i32;

        Self::new(x, y)
    }

    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        draw_block(FOOD_COLOR, self.block.x, self.block.y, ctx, g);
    }
}