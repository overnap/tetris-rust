mod logic;

use raylib::prelude::*;
use logic::*;

const BLOCK_PIXELS: i32 = 28; // Pixels for the length of one side of blocks
const DISPLAY_HEIGHT: i32 = 20;
const DISPLAY_WIDTH: i32 = 10;
const BOARD_BIAS_Y: i32 = (810 - DISPLAY_HEIGHT * BLOCK_PIXELS) / 2;
const BOARD_BIAS_X: i32 = (1440 - DISPLAY_WIDTH * BLOCK_PIXELS) / 2;

pub struct Tetris {
    logic: Logic,
    sprite: Option<Texture2D>
}

impl Tetris {
    pub fn new() -> Self {
        Self{ logic: Logic::new(), sprite: None }
    }

    pub fn init(&mut self, rl: &mut RaylibHandle, thread: & RaylibThread) {
        self.sprite = rl.load_texture(&thread, "static/blocks.png").ok();
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        self.logic.input.set(InputType::MoveLeft, rl.is_key_down(KeyboardKey::KEY_LEFT));
        self.logic.input.set(InputType::MoveRight, rl.is_key_down(KeyboardKey::KEY_RIGHT));
        self.logic.input.set(InputType::SoftDrop, rl.is_key_down(KeyboardKey::KEY_DOWN));
        self.logic.input.set(InputType::HardDrop, rl.is_key_down(KeyboardKey::KEY_SPACE));
        self.logic.input.set(InputType::RotateCCW, rl.is_key_down(KeyboardKey::KEY_Z));
        self.logic.input.set(InputType::RotateCW, rl.is_key_down(KeyboardKey::KEY_UP) ||
                                                            rl.is_key_down(KeyboardKey::KEY_X));
        self.logic.input.set(InputType::Flip, rl.is_key_down(KeyboardKey::KEY_A));
        self.logic.input.set(InputType::Hold, rl.is_key_down(KeyboardKey::KEY_C));

        self.logic.update(rl.get_frame_time());

        // for Debug
        let mouse_left = rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON);
        let mouse_right = rl.is_mouse_button_down(MouseButton::MOUSE_RIGHT_BUTTON);

        if mouse_left || mouse_right {
            let mouse_position = rl.get_mouse_position();
            let grid_x = (mouse_position.x as i32 - BOARD_BIAS_X) / BLOCK_PIXELS;
            let grid_y = (BOARD_BIAS_Y + BLOCK_PIXELS * 20 - mouse_position.y as i32) / BLOCK_PIXELS;
            
            if mouse_left {
                self.logic.board.set_block(grid_y, grid_x, BlockType::Green);
            } else if mouse_right {
                self.logic.board.set_block(grid_y, grid_x, BlockType::Empty);
            }
        }
    }

    pub fn draw(& self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::BLACK);
        d.draw_rectangle_lines_ex(rrect(BOARD_BIAS_X-8, BOARD_BIAS_Y-8, BLOCK_PIXELS*10+16, BLOCK_PIXELS*20+16),
                                8, Color::GRAY);

        // Draw blocks in the board
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let block = self.logic.board.get_block(y, x);
                self.draw_block(d, y, x, block, 255);
            }
        }

        // Draw the piece (current block)
        if let Some(piece) = &self.logic.current_piece {
            for y in 0..4 {
                for x in 0..4 {
                    let block = piece.get_block(y, x);
                    self.draw_block(d, piece.y + y, piece.x + x, block, 255);
                }
            }
        }

        // Draw the ghost of the piece
        if let Some(piece) = &self.logic.current_piece {
            let mut ghost = piece.clone();

            while ghost.shift(&self.logic.board, -1, 0) {};
            for y in 0..4 {
                for x in 0..4 {
                    let block = ghost.get_block(y, x);
                    self.draw_block(d, ghost.y + y, ghost.x + x, block, 96);
                }
            }
        }

    }

    fn draw_block(& self, d: &mut RaylibDrawHandle, y: i32, x: i32, block: BlockType, alpha: u8) {
        if block != BlockType::Empty && block != BlockType::Outside {
            if let Some(texture) = &self.sprite {
                let color = Color::new(255, 255, 255, alpha);
                let index = block as i32 - 1;

                d.draw_texture_pro(texture, rrect(16*index, 0, 16, 16),
                                rrect(x*BLOCK_PIXELS + BOARD_BIAS_X, (19-y)*BLOCK_PIXELS + BOARD_BIAS_Y, BLOCK_PIXELS, BLOCK_PIXELS),
                                rvec2(0, 0),
                                0.0,
                                color);
            } else {
                let mut color = block.get_color();
                color.a = alpha;
                
                d.draw_rectangle(x*BLOCK_PIXELS + BOARD_BIAS_X, (19-y)*BLOCK_PIXELS + BOARD_BIAS_Y,
                            BLOCK_PIXELS, BLOCK_PIXELS, color);
            }
        }
    }
}