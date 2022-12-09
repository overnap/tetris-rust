use super:: *;

pub struct Piece {
    piece_type: PieceType,
    size: (i32, i32),
    blocks: [[BlockType; 4]; 4],
    kicks: [[(i32, i32); 5]; 8],
    rotate_state: i32,
    pub y: i32,
    pub x: i32
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
            _ => [
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
                [BlockType::Empty, BlockType::Empty, BlockType::Empty, BlockType::Empty],
            ]
        };

        let size = match piece_type {
            PieceType::Z => (3, 3),
            PieceType::S => (3, 3),
            PieceType::L => (3, 3),
            PieceType::J => (3, 3),
            PieceType::I => (4, 4),
            PieceType::O => (4, 2),
            PieceType::T => (3, 3),
            _ => (0, 0),
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
            y: 21 - size.1,
            rotate_state: 0,
        }
    }

    pub fn get_block(&self, y: i32, x: i32) -> BlockType {
        self.blocks[y as usize][x as usize]
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

    pub fn rotate(&mut self, board: &Board, clockwise: bool) -> bool {
        false
    }

    fn test(& self, board: &Board) -> bool {
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
    }
}
