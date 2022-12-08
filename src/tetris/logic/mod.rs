mod board;
mod block_type;
mod piece;
mod piece_type;
mod input;
mod input_type;

use board::*;
use piece::*;
pub use block_type::*;
pub use piece_type::*;
pub use input::*;
pub use input_type::*;

pub struct Logic {
    pub board: Board,
    pub current_piece: Option<Piece>,
    pub input: Input,

    das: f32,
    arr: f32,

    move_direction: i32,
    move_last: f32
}

impl Logic {
    pub fn new() -> Self {
        Self{
            board: Board::new(),
            current_piece: Some(Piece::new(PieceType::L)),
            input: Input::new(),
            das: 15.0 / 60.0,
            arr: 2.0 / 60.0,
            move_direction: 0,
            move_last: 0.0
        }
    }

    pub fn update(&mut self, dt: f32) {
        if let Some(piece) = &mut self.current_piece {
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
        }
    }
}