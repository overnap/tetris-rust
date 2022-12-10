mod logic;

use raylib::prelude::*;
use logic::*;

const BLOCK_PIXELS: i32 = 28; // Pixels for the length of one side of blocks
const DISPLAY_HEIGHT: i32 = 22;
const DISPLAY_WIDTH: i32 = 10;
const BOARD_BIAS_Y: i32 = (810 - DISPLAY_HEIGHT * BLOCK_PIXELS) / 2;
const BOARD_BIAS_X: i32 = (1440 - DISPLAY_WIDTH * BLOCK_PIXELS) / 2;

pub struct Tetris {
    logic: Logic,
    background: Option<Texture2D>,
    sprite: Option<Texture2D>
}

impl Tetris {
    pub fn new() -> Self {
        Self{ logic: Logic::new(), background: None, sprite: None }
    }

    pub fn init(&mut self, rl: &mut RaylibHandle, thread: & RaylibThread) {
        self.background = rl.load_texture(&thread, "static/background.png").ok();
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
        if let Some(img) = &self.background {
            d.draw_texture(img, 0, 0, Color::WHITE);
        }

        // Draw backgrounds
        d.draw_rectangle(BOARD_BIAS_X, BOARD_BIAS_Y, BLOCK_PIXELS*10, BLOCK_PIXELS*20, Color::new(0, 0, 0, 210));
        d.draw_rectangle_lines_ex(rrect(BOARD_BIAS_X-8, BOARD_BIAS_Y-8, BLOCK_PIXELS*10+16, BLOCK_PIXELS*20+16),
                                8, Color::GRAY);
        
        d.draw_rectangle(BOARD_BIAS_X-BLOCK_PIXELS*6, BOARD_BIAS_Y, BLOCK_PIXELS*5, BLOCK_PIXELS*4, Color::new(0, 0, 0, 210));
        d.draw_rectangle_lines_ex(rrect(BOARD_BIAS_X-BLOCK_PIXELS*6-8, BOARD_BIAS_Y-8, BLOCK_PIXELS*5+16, BLOCK_PIXELS*4+16),
                                    8, Color::GRAY);

        d.draw_rectangle(BOARD_BIAS_X+BLOCK_PIXELS*11, BOARD_BIAS_Y, BLOCK_PIXELS*5, BLOCK_PIXELS*10, Color::new(0, 0, 0, 210));
        d.draw_rectangle_lines_ex(rrect(BOARD_BIAS_X+BLOCK_PIXELS*11-8, BOARD_BIAS_Y-8, BLOCK_PIXELS*5+16, BLOCK_PIXELS*10+16),
                                    8, Color::GRAY);

        // Draw blocks in the board
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let block = self.logic.board.get_block(y, x);
                self.draw_block(d, BOARD_BIAS_Y+(19-y)*BLOCK_PIXELS, BOARD_BIAS_X+x*BLOCK_PIXELS, block, 255);
            }
        }

        // Draw the piece (current block)
        if let Some(piece) = &self.logic.current_piece {
            self.draw_piece(d, piece, 255);
        }

        // Draw the ghost of the piece
        if let Some(piece) = &self.logic.current_piece {
            let mut ghost = piece.clone();

            while ghost.shift(&self.logic.board, -1, 0) {};
            self.draw_piece(d, &ghost, 96);
        }

        // Draw the held piece
        if let Some(held) = self.logic.held_piece {
            self.draw_piece_plain(d, held, BOARD_BIAS_Y, BOARD_BIAS_X-5*BLOCK_PIXELS, 255);
        }

        // Draw next pieces
        for next in 0..3 {
            let seen = self.logic.bag.get_uncertain(next);

            if let Some(piece_type) = seen {
                self.draw_piece_plain(d, piece_type, BOARD_BIAS_Y+next as i32*BLOCK_PIXELS*3, BOARD_BIAS_X+BLOCK_PIXELS*12, 255);
            }
        }
    }

    fn draw_block(& self, d: &mut RaylibDrawHandle, y: i32, x: i32, block: BlockType, alpha: u8) {
        if block != BlockType::Empty && block != BlockType::Outside {
            if let Some(texture) = &self.sprite {
                let color = Color::new(255, 255, 255, alpha);
                let index = block as i32 - 1;

                d.draw_texture_pro(texture, rrect(16*index, 0, 16, 16),
                                rrect(x, y, BLOCK_PIXELS, BLOCK_PIXELS),
                                rvec2(0, 0),
                                0.0,
                                color);
            } else {
                let mut color = block.get_color();
                color.a = alpha;

                d.draw_rectangle(x, y, BLOCK_PIXELS, BLOCK_PIXELS, color);
            }
        }
    }

    fn draw_piece(& self, d: &mut RaylibDrawHandle, piece: & Piece, alpha: u8) {
        let size = piece.get_size();

        for y in 0..size.0 as i32 {
            for x in 0..size.1 as i32 {
                let block = piece.get_block(y, x);
                self.draw_block(d, (19-(piece.y+y))*BLOCK_PIXELS+BOARD_BIAS_Y, (piece.x+x)*BLOCK_PIXELS+BOARD_BIAS_X, block, alpha);
            }
        }
    }

    fn draw_piece_plain(& self, d: &mut RaylibDrawHandle, piece_type: PieceType, y: i32, x: i32, alpha: u8) {
        let piece = Piece::new(piece_type);
        let bias = (
            match piece_type {
                PieceType::O => -BLOCK_PIXELS,
                PieceType::I => BLOCK_PIXELS/2,
                _ => 0
            },
            match piece_type {
                PieceType::O | PieceType::I => -BLOCK_PIXELS/2,
                _ => 0
            }
        );

        for i in 0..4 {
            for j in 0..4 {
                let block = piece.get_block(i, j);

                if block != BlockType::Empty {
                    self.draw_block(d, y + (3-i)*BLOCK_PIXELS + bias.0,
                                    x + j*BLOCK_PIXELS + bias.1, block, alpha);
                }
            }
        }
    }
}