pub const GRID_MAX_X: u64 = 100;
pub const GRID_MAX_Y: u64 = 100;

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub struct InstructionPointer {
    pub x: u64,
    pub y: u64,
    pub direction: Direction,
    pub stack: Vec<u8>
}

impl InstructionPointer {
    pub fn new() -> InstructionPointer{
        InstructionPointer {x: 0, y: 0, direction: Direction::RIGHT, stack: Vec::new()}
    }
}

/*
pub struct Stack {
    pub size: usize,
    pub data_array: [u64; 65_536],
}

impl Stack {
    pub fn new() -> Self {
        Stack {size: 0, data_array: [0; 65_536]}
    }

    pub fn pop(&mut self) -> u64 {
        let return_value: u64 = self.data_array[self.size];

        self.data_array[self.size] = 0;
        self.size -= 1;

        return return_value
    }

    pub fn push(&mut self, new_value: u64){
        self.size += 1;
        self.data_array[self.size] = new_value;
    }
}

*/