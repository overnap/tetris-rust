mod tetris;

use tetris::Tetris;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1440, 810)
        .title("Hello, World")
        .build();

    let mut tetris = Tetris::new();
    tetris.init();

    while !rl.window_should_close() {
        tetris.update(&mut rl);
        tetris.draw(&mut rl.begin_drawing(&thread));
    }
}