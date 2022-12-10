use super::*;

const HEIGHT: usize = 28;
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

    pub fn set_block(&mut self, y: i32, x: i32, block: BlockType) {
        if y >= 0 && y < HEIGHT as i32 && x >= 0 && x < WIDTH as i32 {
            self.data[y as usize][x as usize] = block;
        }
    }

    pub fn is_line_full(& self, y: i32) -> bool {
        for x in 0..WIDTH as i32 {
            if self.get_block(y, x) == BlockType::Empty {
                return false;
            }
        }

        true
    }

    pub fn trim(&mut self) -> u32 {
        let mut trimmed_count = 0;
        let mut trimmed_board = [[BlockType::Empty; WIDTH]; HEIGHT];
        let mut top = 0;

        for y in 0..HEIGHT {
            if self.is_line_full(y as i32) {
                trimmed_count += 1;
            } else {
                trimmed_board[top] = self.data[y];
                top += 1;
            }
        }

        self.data = trimmed_board;
        trimmed_count
    }
}