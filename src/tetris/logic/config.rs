use super::*;

pub struct Config {
    pub das: f32,
    pub arr: f32,
    pub sdf: f32,

    pub board_height: usize,
    pub board_width: usize,

    pub levels: Vec<Level>
}

impl Default for Config {
    fn default() -> Self {
        Self{
            das: 10.0 / 60.0,
            arr: 1.25 / 60.0,
            sdf: 1.0 / 60.0,
            board_height: 20,
            board_width: 10,
            levels: vec![Level::default()]
        }
    }
}