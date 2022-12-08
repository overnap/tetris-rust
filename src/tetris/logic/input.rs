use super::*;

const INPUT_TYPES_SIZE: usize = InputType::SIZE as usize;

pub struct Input {
    previous: [bool; INPUT_TYPES_SIZE],
    current: [bool; INPUT_TYPES_SIZE]
}

impl Input {
    pub fn new() -> Self {
        Self{ previous: [false; INPUT_TYPES_SIZE], current: [false; INPUT_TYPES_SIZE] }
    }

    pub fn set(&mut self, input_type: InputType, is_pressed: bool) {
        let index = input_type as usize;
        self.previous[index] = self.current[index];
        self.current[index] = is_pressed;
    }

    pub fn is_held(& self, input_type: InputType) -> bool {
        let index = input_type as usize;
        self.current[index]
    }

    pub fn is_pressed(& self, input_type: InputType) -> bool {
        let index = input_type as usize;
        self.current[index] && !self.previous[index]
    }

    pub fn is_released(& self, input_type: InputType) -> bool {
        let index = input_type as usize;
        !self.current[index] && self.previous[index]
    }
}