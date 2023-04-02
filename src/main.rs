mod util;
use std::fs;

const PROGRAM_PATH: &str = "program.txt";

fn main() {
    let mut instruction_pointer = util::InstructionPointer::new();
    
    let mut program_grid = Vec<Vec<u8>>;
    load_program(PROGRAM_PATH, program_grid);

    
    
    println!("{:?}", program_grid);

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
            _ => program_array[row][column] = *byte;  column += 1,
        }
       
    }
}
