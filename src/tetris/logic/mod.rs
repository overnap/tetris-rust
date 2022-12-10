mod bag;
mod board;
mod block_type;
mod piece;
mod piece_type;
mod input;
mod input_type;
mod tspin_type;

use bag::*;
use board::*;
use tspin_type::*;
pub use block_type::*;
pub use piece::*;
pub use piece_type::*;
pub use input::*;
pub use input_type::*;

pub struct Logic {
    pub bag: Bag,
    pub board: Board,
    pub current_piece: Option<Piece>,
    pub held_piece: Option<PieceType>,
    pub input: Input,

    das: f32,
    arr: f32,
    sdf: f32,

    move_direction: i32,
    move_last: f32,
    drop_last: f32
}

impl Logic {
    pub fn new() -> Self {
        Self{
            bag: Bag::new(None),
            board: Board::new(),
            current_piece: None,
            held_piece: None,
            input: Input::new(),
            das: 15.0 / 60.0,
            arr: 2.0 / 60.0,
            sdf: 2.0 / 60.0,
            move_direction: 0,
            move_last: 0.0,
            drop_last: 0.0
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Get the piece from the bag
        if let None = &self.current_piece {
            self.current_piece = Some(Piece::new(self.bag.pop()));
        }

        // Hold
        if self.input.is_pressed(InputType::Hold) {
            if let Some(piece) = &mut self.current_piece {
                let previous_held = self.held_piece;
                self.held_piece = Some(piece.get_type());
                
                if let Some(held) = previous_held {
                    self.current_piece = Some(Piece::new(held));
                } else {
                    self.current_piece = Some(Piece::new(self.bag.pop()));
                }
            }
        }

        if let Some(piece) = &mut self.current_piece {
            // Move horizontally
            let direction_pressed = (self.input.is_pressed(InputType::MoveRight) as i32)
                                       - (self.input.is_pressed(InputType::MoveLeft) as i32);
            let direction_released = (self.input.is_released(InputType::MoveRight) as i32)
                                        - (self.input.is_released(InputType::MoveLeft) as i32);
            
            if direction_pressed != 0 {
                self.move_direction = direction_pressed;
                self.move_last = 0.0;
                piece.shift(&self.board, 0, direction_pressed);
            }
            
            if self.move_direction != 0 && (direction_released == self.move_direction ||
                !(self.input.is_held(InputType::MoveLeft) || self.input.is_held(InputType::MoveRight))) {
                self.move_direction = 0;
                self.move_last = 0.0;
            }
            
            if self.move_direction != 0 {
                self.move_last += dt;

                while self.move_last >= self.das + self.arr {
                    self.move_last -= self.arr;
                    piece.shift(&self.board, 0, self.move_direction);
                }
            }

            // Rotate
            if self.input.is_pressed(InputType::RotateCW) {
                piece.rotate(&self.board, true);
            }

            if self.input.is_pressed(InputType::RotateCCW) {
                piece.rotate(&self.board, false);
            }

            if self.input.is_pressed(InputType::Flip) {
                piece.flip(&self.board);
            }

            // Soft drop
            if self.input.is_pressed(InputType::SoftDrop) {
                self.drop_last = 0.0;
                piece.shift(&self.board, -1, 0);
            }

            if self.input.is_held(InputType::SoftDrop) {
                self.drop_last += dt;

                while self.drop_last >= self.sdf {
                    self.drop_last -= self.sdf;
                    piece.shift(&self.board, -1, 0);
                }
            }

            // Hard drop
            if self.input.is_pressed(InputType::HardDrop) {
                while piece.shift(&self.board, -1, 0) {};
                piece.place(&mut self.board);
                self.board.trim();
                self.current_piece = None;
            }
        }
    }
}