pub struct Level {
    pub gravity: f32,
    pub lock_delay: f32,
    pub lines: u32
}

impl Default for Level {
    fn default() -> Self {
        Self { gravity: 1.0 / 60.0, lock_delay: 500.0 / 1000.0, lines: 0 }
    }
}