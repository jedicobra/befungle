use rand::Rng;
use rand::distributions::{Distribution, Standard};

pub const GRID_MAX_X: usize = 100;
pub const GRID_MAX_Y: usize = 100;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=3){
            0 => Direction::UP,
            1 => Direction::DOWN,
            2 => Direction::LEFT,
            3 => Direction::RIGHT,
            _ => Direction::RIGHT,
        }
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub enum Instruction {
    TOGGLE_STRINGMODE,

    PUSH_INTEGER,

    POP_OUT_AS_CHAR,
    POP_OUT_AS_INTEGER,

    GREATER_THAN,
    NEGATE,

    DUPLICATE,
    SWAP,

    ADD,
    SUBTRACT,
    MULTIPLY,
    INTEGER_DIVIDE,
    MODULO,

    PC_DIRECTION_LEFT,
    PC_DIRECTION_RIGHT,
    PC_DIRECTION_DOWN,
    PC_DIRECTION_UP,
    PC_DIRECTION_RANDOM,

    HORIZONTAL_IF,
    VERTICAL_IF,

    END_PROGRAM,
    NOP,
}

impl Instruction{
    pub fn from_u8(byte_val: u8) -> Instruction{
        match byte_val {
            34 => Instruction::TOGGLE_STRINGMODE,
            44 => Instruction::POP_OUT_AS_CHAR,
            46 => Instruction::POP_OUT_AS_INTEGER,

            60 => Instruction::PC_DIRECTION_LEFT,
            62 => Instruction::PC_DIRECTION_RIGHT,
            94 => Instruction::PC_DIRECTION_UP,
            118 => Instruction::PC_DIRECTION_DOWN,
            63 => Instruction::PC_DIRECTION_RANDOM,

            95 => Instruction::HORIZONTAL_IF,
            124 => Instruction::VERTICAL_IF,

            58 => Instruction::DUPLICATE,

            92 => Instruction::SWAP,

            33 => Instruction::NEGATE,
            96 => Instruction::GREATER_THAN,

            37 => Instruction::MODULO,
            42 => Instruction::MULTIPLY,
            43 => Instruction::ADD,
            45 => Instruction::SUBTRACT,
            47 => Instruction::INTEGER_DIVIDE,
            
            64 => Instruction::END_PROGRAM,

            48 ..=57 => Instruction::PUSH_INTEGER,


            _  => Instruction::NOP,
        }
    }
}



pub struct InstructionPointer {
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
    pub stringmode_enabled: bool,
    pub stack: Vec<u8>
}

impl InstructionPointer {
    pub fn new() -> InstructionPointer{
        InstructionPointer {x: 0, y: 0, direction: Direction::RIGHT, stringmode_enabled: false, stack: Vec::new()}
    }

    pub fn move_by_1(&mut self) {
        // increment instruction pointer with wraparound
        match self.direction {
            Direction::LEFT => self.x = (self.x - 1) % GRID_MAX_X,
            Direction::RIGHT => self.x = (self.x + 1) % GRID_MAX_X,

            Direction::DOWN => self.y = (self.y + 1) % GRID_MAX_Y,
            Direction::UP => self.y = (self.y - 1) % GRID_MAX_Y,

            _ => {
                println!("Got a bad direction: {:?}", self.direction);
            },
        }
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