use raylib::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum BlockType {
    Outside,
    Empty,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Cyan,
    Purple
}

impl BlockType {
    pub fn get_color(& self) -> Color {
        match self {
            Self::Red => Color::RED,
            Self::Orange => Color::ORANGE,
            Self::Yellow => Color::YELLOW,
            Self::Green => Color::GREEN,
            Self::Blue => Color::BLUE,
            Self::Cyan => Color::SKYBLUE,
            Self::Purple => Color::PINK,
            _ => Color::BLANK
        }
    }
}