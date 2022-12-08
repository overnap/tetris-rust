mod logic;

use raylib::prelude::*;
use logic::*;

const BLOCK_PIXELS: i32 = 28; // Pixels for the length of one side of blocks
const DISPLAY_HEIGHT: i32 = 20;
const DISPLAY_WIDTH: i32 = 10;
const BOARD_BIAS_Y: i32 = (810 - DISPLAY_HEIGHT * BLOCK_PIXELS) / 2;
const BOARD_BIAS_X: i32 = (1440 - DISPLAY_WIDTH * BLOCK_PIXELS) / 2;

pub struct Tetris {
    logic: Logic
}

impl Tetris {
    pub fn new() -> Self {
        Self{ logic: Logic::new() }
    }

    pub fn init(&mut self) {
        self.logic.board.set_block_debug(0, 0, BlockType::Green);
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        self.logic.input.set(InputType::MoveLeft, rl.is_key_down(KeyboardKey::KEY_LEFT));
        self.logic.input.set(InputType::MoveRight, rl.is_key_down(KeyboardKey::KEY_RIGHT));
        self.logic.input.set(InputType::SoftDrop, rl.is_key_down(KeyboardKey::KEY_DOWN));
        self.logic.input.set(InputType::HardDrop, rl.is_key_down(KeyboardKey::KEY_SPACE));
        self.logic.input.set(InputType::RotateCCW, rl.is_key_down(KeyboardKey::KEY_Z));
        self.logic.input.set(InputType::RotateCW, rl.is_key_down(KeyboardKey::KEY_UP) ||
                                                            rl.is_key_down(KeyboardKey::KEY_X));

        self.logic.update(rl.get_frame_time());
    }

    pub fn draw(& self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::BLACK);
        d.draw_rectangle(BOARD_BIAS_X, BOARD_BIAS_Y, BLOCK_PIXELS*10, BLOCK_PIXELS*20, Color::DARKGRAY);

        // Draw blocks in the board
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let block = self.logic.board.get_block(y, x);
                self.draw_block(d, y, x, block);
            }
        }

        // Draw the piece (current block)
        if let Some(piece) = &self.logic.current_piece {
            for y in 0..4 {
                for x in 0..4 {
                    let block = piece.get_block(y, x);
                    self.draw_block(d, piece.y + y, piece.x + x, block);
                }
            }
        }

    }

    fn draw_block(& self, d: &mut RaylibDrawHandle, y: i32, x: i32, block: BlockType) {
        if block != BlockType::Empty && block != BlockType::Outside {
            d.draw_rectangle(x*BLOCK_PIXELS + BOARD_BIAS_X, (19-y)*BLOCK_PIXELS + BOARD_BIAS_Y,
                             BLOCK_PIXELS, BLOCK_PIXELS, block.get_color());
        }
    }
}