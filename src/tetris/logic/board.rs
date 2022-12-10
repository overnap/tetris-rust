use super::*;

pub struct Board {
    data: Vec<Vec<BlockType>>,
    height: usize,
    width: usize
}

impl Board {
    pub fn new(height: usize, width: usize) -> Self {
        Self{ data: vec![vec![BlockType::Empty; width]; height+4], height: height+4, width }
    }

    pub fn get_block(& self, y: i32, x: i32) -> BlockType {
        if y >= self.height as i32 {
            return BlockType::Empty;
        }

        if x < 0 || y < 0 || x >= self.width as i32 {
            return BlockType::Outside;
        }

        self.data[y as usize][x as usize]
    }

    pub fn set_block(&mut self, y: i32, x: i32, block: BlockType) {
        if y >= 0 && y < self.height as i32 && x >= 0 && x < self.width as i32 {
            self.data[y as usize][x as usize] = block;
        }
    }

    pub fn is_line_full(& self, y: i32) -> bool {
        for x in 0..self.width as i32 {
            if self.get_block(y, x) == BlockType::Empty {
                return false;
            }
        }

        true
    }

    pub fn trim(&mut self) -> u32 {
        let mut trimmed_count = 0;
        let mut trimmed_board = vec![vec![BlockType::Empty; self.width]; self.height];
        let mut top = 0;

        for y in 0..self.height {
            if self.is_line_full(y as i32) {
                trimmed_count += 1;
            } else {
                trimmed_board[top] = self.data[y].clone();
                top += 1;
            }
        }

        self.data = trimmed_board;
        trimmed_count
    }
}