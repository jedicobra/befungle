mod util;
use util::Instruction;
use util::Direction;
use std::{thread, time, fs};

const PROGRAM_PATH: &str = "program.txt";

fn main() {
    // 2d vec-based grid of instructions (as ASCII bytes per the docs)
    // accessed like program_grid[y][x]
    let mut program_grid: Vec<Vec<u8>> = vec![vec![0; util::GRID_MAX_Y]; util::GRID_MAX_X];
    load_program(PROGRAM_PATH, &mut program_grid);

    let mut instruction_pointer = util::InstructionPointer::new();

    loop {
        // this is the main execution loop (infinite unless broken)

        // load the next instruction from the current coordinates
        // its backwards cause i think that's just how vectors work
        let instruction_byte: u8 = program_grid[instruction_pointer.y][instruction_pointer.x];
        let next_instruction = Instruction::from_u8(instruction_byte);
        
        /* thread::sleep(time::Duration::from_millis(100));
        println!(
            "next byte: ({} = instruction {:?}) (stringmode: {})", instruction_byte as char, 
            next_instruction, instruction_pointer.stringmode_enabled
        ); */

        if instruction_pointer.stringmode_enabled  {
            if next_instruction != Instruction::TOGGLE_STRINGMODE{
                instruction_pointer.stack.push(instruction_byte);
                instruction_pointer.move_by_1();
                continue;
            }
        }

        
        match next_instruction {
            Instruction::PC_DIRECTION_LEFT => instruction_pointer.direction = Direction::LEFT,
            Instruction::PC_DIRECTION_RIGHT => instruction_pointer.direction = Direction::RIGHT,
            Instruction::PC_DIRECTION_DOWN => instruction_pointer.direction = Direction::DOWN,
            Instruction::PC_DIRECTION_UP => instruction_pointer.direction = Direction::UP,
            Instruction::PC_DIRECTION_RANDOM => { instruction_pointer.direction = rand::random::<Direction>(); },



            Instruction::ADD => {
                let a = instruction_pointer.stack.pop().unwrap_or_default();
                let b = instruction_pointer.stack.pop().unwrap_or_default();
                instruction_pointer.stack.push( a.wrapping_add(b) );
            },

            Instruction::SUBTRACT => {
                let a = instruction_pointer.stack.pop().unwrap_or_default();
                let b = instruction_pointer.stack.pop().unwrap_or_default();
                instruction_pointer.stack.push( a.wrapping_sub(b) );
            },

            Instruction::MULTIPLY => {
                let a = instruction_pointer.stack.pop().unwrap_or_default();
                let b = instruction_pointer.stack.pop().unwrap_or_default();
                instruction_pointer.stack.push(  a.wrapping_mul(b) );
            },

            Instruction::INTEGER_DIVIDE => {
                let a = instruction_pointer.stack.pop().unwrap_or_default();
                let b = instruction_pointer.stack.pop().unwrap_or_default();
                instruction_pointer.stack.push( a.wrapping_div(b) );
            },

            Instruction::MODULO => {
                let a = instruction_pointer.stack.pop().unwrap_or_default();
                let b = instruction_pointer.stack.pop().unwrap_or_default();
                instruction_pointer.stack.push( a.wrapping_rem(b) );
            },

            Instruction::NEGATE => {
                let top = instruction_pointer.stack.pop().unwrap_or_default();
                if top == 0 {
                    instruction_pointer.stack.push(1);}
                else {
                    instruction_pointer.stack.push(0);}
            },

            Instruction::GREATER_THAN => {
                let a = instruction_pointer.stack.pop().unwrap_or_default();
                let b = instruction_pointer.stack.pop().unwrap_or_default();
                if b > a{
                    instruction_pointer.stack.push(1);}
                else{
                    instruction_pointer.stack.push(0);}
            },

            Instruction::PUSH_INTEGER => {
                // instruction byte = ASCII value, subtract 48 to get the actual number 
                instruction_pointer.stack.push(instruction_byte - 48);
            }

            Instruction::POP_OUT_AS_CHAR => {
                let top = instruction_pointer.stack.pop();
                match top{
                    Some(val) => println!("{}", val as char),
                    None => (),
                }
            },

            Instruction::POP_OUT_AS_INTEGER => {
                let top = instruction_pointer.stack.pop();
                match top{
                    Some(val) => println!("{}", val),
                    None => (),
                }
            },

            Instruction::TOGGLE_STRINGMODE => {
                // (push each character's ASCII value all the way up to the next ")
                instruction_pointer.stringmode_enabled = !instruction_pointer.stringmode_enabled;
            }

            Instruction::END_PROGRAM => break,

            Instruction::NOP => (),

            _ => {
                println!(
                    "got a weird instruction: {:?} (stringmode is {})",
                    next_instruction,
                    instruction_pointer.stringmode_enabled
                );

                break;
            },
        }

        
        instruction_pointer.move_by_1(); 
    }
}

fn load_program(file_name: &str, program_grid: &mut Vec<Vec<u8>>){
    let contents = fs::read_to_string(file_name)
        .expect("Couldn't read the file");

    let mut row: usize = 0;
    let mut column: usize = 0;
    for byte in contents.as_bytes(){
        match byte {
            // carriage return
            13 => continue,

            // line feed (treat as actual newline)
            10 => {row += 1; column = 0},

            //other
            _ => {program_grid[row][column] = *byte;  column += 1},
        }
       
    }
}
