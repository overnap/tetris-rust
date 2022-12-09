use super:: *;

#[derive(Clone, Copy)]
pub struct Piece {
    piece_type: PieceType,
    size: (usize, usize),
    blocks: [[BlockType; 4]; 4],
    kicks: [[(i32, i32); 5]; 8],
    rotate_state: usize,

    pub y: i32,
    pub x: i32,
    pub tspin_state: TspinType
}

impl Piece {
    pub fn new(piece_type: PieceType) -> Self {
        let blocks = match piece_type {
            PieceType::Z => [
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Empty, BlockType::Red, BlockType::Red, BlockType::Empty],
                [BlockType::Red, BlockType::Red, BlockType::Empty, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
            ],
            PieceType::S => [
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Green, BlockType::Green, BlockType::Empty, BlockType::Empty],
                [BlockType::Empty, BlockType::Green, BlockType::Green, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
            ],
            PieceType::L => [
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Orange, BlockType::Orange, BlockType::Orange, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Orange, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
            ],
            PieceType::J => [
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Blue, BlockType::Blue, BlockType::Blue, BlockType::Empty],
                [BlockType::Blue, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
            ],
            PieceType::I => [
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Cyan, BlockType::Cyan, BlockType::Cyan, BlockType::Cyan],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
            ],
            PieceType::O => [
                [BlockType::Empty, BlockType::Yellow, BlockType::Yellow, BlockType::Empty],
                [BlockType::Empty, BlockType::Yellow, BlockType::Yellow, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
            ],
            PieceType::T => [
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Purple, BlockType::Purple, BlockType::Purple, BlockType::Empty],
                [BlockType::Empty, BlockType::Purple, BlockType::Empty, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
            ],
            // _ => [
            //     [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
            //     [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
            //     [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
            //     [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
            // ]
        };

        let size = match piece_type {
            PieceType::Z => (3, 3),
            PieceType::S => (3, 3),
            PieceType::L => (3, 3),
            PieceType::J => (3, 3),
            PieceType::I => (4, 4),
            PieceType::O => (3, 4),
            PieceType::T => (3, 3),
            // _ => (0, 0),
        };

        let kicks = match piece_type {
            PieceType::Z | PieceType::S | PieceType::L | PieceType::J | PieceType::T => [
                [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
            ],
            PieceType::I => [
                [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
                [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
                [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
                [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
                [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
                [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
                [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
                [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
            ],
            _ => [[(0, 0); 5]; 8],
        };

        Self {
            piece_type,
            blocks,
            size,
            kicks,
            x: 3,
            y: 21 - size.0 as i32,
            rotate_state: 0,
            tspin_state: TspinType::None
        }
    }

    pub fn get_block(& self, y: i32, x: i32) -> BlockType {
        self.blocks[y as usize][x as usize]
    }

    pub fn get_type(& self) -> PieceType {
        self.piece_type
    }

    pub fn get_size(& self) -> (usize, usize) {
        self.size
    }

    pub fn place(&mut self, board: &mut Board) {
        for y in 0..4 {
            for x in 0..4 {
                let block = self.get_block(y, x);
                if block != BlockType::Empty {
                    board.set_block(self.y + y, self.x + x, block);
                }
            }
        }
    }

    pub fn shift(&mut self, board: &Board, y: i32, x: i32) -> bool {
        self.y += y;
        self.x += x;

        if !self.test(board) {
            self.y -= y;
            self.x -= x;

            false
        } else {
            true
        }
    }

    pub fn rotate(&mut self, board: & Board, clockwise: bool) -> bool {
        if self.piece_type == PieceType::O {
            return true;
        }

        let previous_y = self.y;
        let previous_x = self.x;
        let blocks_original = self.blocks.clone();

        for y in 0..self.size.0 {
            for x in 0..self.size.1 {
                if clockwise {
                    self.blocks[self.size.0 - 1 - x][y] = blocks_original[y][x];
                } else {
                    self.blocks[x][self.size.1 - 1 - y] = blocks_original[y][x];
                }
            }
        }

        let next_rotate_state = (self.rotate_state + if clockwise { 1 } else { 3 }) % 4;
        let kick_table = self.kicks[if clockwise { self.rotate_state * 2 } else { next_rotate_state * 2 + 1 } as usize];

        for i in 0..5 {
            self.y = previous_y + kick_table[i].1;
            self.x = previous_x + kick_table[i].0;

            if self.test(board) {
                self.rotate_state = next_rotate_state;
                self.tspin_update(board);

                return true;
            }
        }

        self.y = previous_y;
        self.x = previous_x;
        self.blocks = blocks_original;
        
        false
    }

    pub fn flip(&mut self, board: & Board) -> bool {
        let blocks_origin = self.blocks;

        for y in 0..self.size.0 {
            for x in 0..self.size.1 {
                self.blocks[self.size.0 - 1 - y][self.size.1 - 1 - x] = blocks_origin[y][x];
            }
        }

        if self.test(board) {
            true
        } else {
            self.blocks = blocks_origin;

            false
        }
    }

    fn test(& self, board: & Board) -> bool {
        for y in 0..4 {
            for x in 0..4 {
                if self.get_block(y, x) != BlockType::Empty &&
                   board.get_block(self.y + y, self.x + x) != BlockType::Empty {
                    return false;
                }
            }
        }

        true
    }

    fn tspin_update(&mut self, board: &Board) {
        self.tspin_state = TspinType::None;

        if self.piece_type == PieceType::T {
            let corners = [
                (board.get_block(self.y+2, self.x) != BlockType::Empty) as i32,
                (board.get_block(self.y+2, self.x+2) != BlockType::Empty) as i32,
                (board.get_block(self.y, self.x+2) != BlockType::Empty) as i32,
                (board.get_block(self.y, self.x) != BlockType::Empty) as i32
            ];

            if corners.iter().sum::<i32>() >= 3 {
                let fronts = (self.rotate_state, (self.rotate_state+1) % 4);

                self.tspin_state = match corners[fronts.0] + corners[fronts.1] {
                    2 => TspinType::Normal,
                    _ => TspinType::Mini
                }
            }
        }
    }
}
