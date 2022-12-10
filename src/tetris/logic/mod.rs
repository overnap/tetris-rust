mod bag;
mod board;
mod block_type;
mod config;
mod piece;
mod piece_type;
mod input;
mod input_type;
mod level;
mod tspin_type;

use bag::*;
use board::*;
use level::*;
use tspin_type::*;
pub use block_type::*;
pub use config::*;
pub use piece::*;
pub use piece_type::*;
pub use input::*;
pub use input_type::*;

pub struct Logic {
    pub bag: Bag,
    pub board: Board,
    pub config: Config,
    pub current_piece: Option<Piece>,
    pub held_piece: Option<PieceType>,
    pub input: Input,
    
    level_index: usize,

    hold_lock: bool,
    move_direction: i32,
    move_last: f32,
    drop_last: f32,
    land_last: f32,
    gravity_acc: f32
}

impl Logic {
    pub fn new(config: Config) -> Self {
        Self{
            bag: Bag::new(None),
            board: Board::new(config.board_height, config.board_width),
            config,
            current_piece: None,
            held_piece: None,
            input: Input::new(),
            level_index: 0,
            hold_lock: false,
            move_direction: 0,
            move_last: 0.0,
            drop_last: 0.0,
            land_last: 0.0,
            gravity_acc: 0.0
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Get the piece from the bag
        if let None = &self.current_piece {
            self.current_piece = Some(Piece::new(self.bag.pop(), self.config.board_height, self.config.board_width));
        }

        // Hold
        if self.input.is_pressed(InputType::Hold) && !self.hold_lock {
            if let Some(piece) = &mut self.current_piece {
                let previous_held = self.held_piece;
                self.held_piece = Some(piece.get_type());
                self.hold_lock = true;

                self.land_last = 0.0;
                self.move_last = 0.0;
                self.gravity_acc = 0.0;
                
                if let Some(held) = previous_held {
                    self.current_piece = Some(Piece::new(held, self.config.board_height, self.config.board_width));
                } else {
                    self.current_piece = Some(Piece::new(self.bag.pop(), self.config.board_height, self.config.board_width));
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

                if piece.shift(&self.board, 0, direction_pressed) {
                    self.land_last = 0.0;
                    self.gravity_acc = 0.0;
                }
            }
            
            if self.move_direction != 0 && (direction_released == self.move_direction ||
                !(self.input.is_held(InputType::MoveLeft) || self.input.is_held(InputType::MoveRight))) {
                self.move_direction = 0;
                self.move_last = 0.0;
            }
            
            if self.move_direction != 0 {
                self.move_last += dt;

                while self.move_last >= self.config.das + self.config.arr {
                    self.move_last -= self.config.arr;

                    if piece.shift(&self.board, 0, self.move_direction) {
                        self.land_last = 0.0;
                        self.gravity_acc = 0.0;
                    }
                }
            }

            // Rotate
            if self.input.is_pressed(InputType::RotateCW) {
                if piece.rotate(&self.board, true) {
                    self.land_last = 0.0;
                    self.gravity_acc = 0.0;
                }
            }

            if self.input.is_pressed(InputType::RotateCCW) {
                if piece.rotate(&self.board, false) {
                    self.land_last = 0.0;
                    self.gravity_acc = 0.0;
                }
            }

            if self.input.is_pressed(InputType::Flip) {
                if piece.flip(&self.board) {
                    self.land_last = 0.0;
                    self.gravity_acc = 0.0;
                }
            }

            // Soft drop
            if self.input.is_pressed(InputType::SoftDrop) {
                self.drop_last = 0.0;
                
                if piece.shift(&self.board, -1, 0) {
                    self.land_last = 0.0;
                    self.gravity_acc = 0.0;
                }
            }

            if self.input.is_held(InputType::SoftDrop) {
                self.drop_last += dt;
                self.gravity_acc = 0.0;

                while self.drop_last >= self.config.sdf {
                    self.drop_last -= self.config.sdf;

                    if piece.shift(&self.board, -1, 0) {
                        self.land_last = 0.0;
                    }
                }
            }

            // Hard drop
            if self.input.is_pressed(InputType::HardDrop) {
                while piece.shift(&self.board, -1, 0) {};
                piece.place(&mut self.board);

                self.board.trim();
                self.current_piece = None;
                self.hold_lock = false;
                self.land_last = 0.0;
                self.move_last = 0.0;
                self.gravity_acc = 0.0;

                return;
            }

            // Lock
            if !piece.shift(&self.board, -1, 0) {
                self.land_last += dt;
        
                if self.land_last >= self.config.levels[self.level_index].lock_delay {
                    piece.place(&mut self.board);

                    self.board.trim();
                    self.current_piece = None;
                    self.hold_lock = false;
                    self.land_last = 0.0;
                    self.move_last = 0.0;
                    self.gravity_acc = 0.0;

                    return;
                }
            } else {
                piece.shift(&self.board, 1, 0);
            }

            // Gravity
            self.gravity_acc += dt * self.config.levels[self.level_index].gravity * 60.0;

            while self.gravity_acc >= 1.0 {
                self.gravity_acc -= 1.0;
                
                if piece.shift(&self.board, -1, 0) {
                    self.land_last = 0.0;
                }
            }
        }
    }

    pub fn land_rate(& self) -> f32 {
        self.land_last / self.config.levels[self.level_index].lock_delay
    }

    pub fn is_hold_locked(& self) -> bool {
        self.hold_lock  
    }
}