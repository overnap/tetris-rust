use super::*;

const HEIGHT: usize = 40;
const WIDTH: usize = 10;

pub struct Board {
    data: [[BlockType; WIDTH]; HEIGHT]
}

impl Board {
    pub fn new() -> Self {
        Self{ data: [[BlockType::Empty; WIDTH]; HEIGHT] }
    }

    pub fn get_block(& self, y: i32, x: i32) -> BlockType {
        if y >= HEIGHT as i32 {
            return BlockType::Empty;
        }

        if x < 0 || y < 0 || x >= WIDTH as i32 {
            return BlockType::Outside;
        }

        self.data[y as usize][x as usize]
    }

    pub fn set_block_debug(&mut self, y: i32, x: i32, block: BlockType) {
        self.data[y as usize][x as usize] = block;
    }
}