use std::vec;
use rand::{self, Rng};
use termcolor::{StandardStream, ColorChoice, WriteColor};
use terminal_size::{Width, Height, terminal_size};
use crate::{structs::TerminalSize};


pub fn allocate_matrix(matrix_columns : u16, matrix_lines : u16) -> Vec<Vec<char>>{
    return vec![vec![' '; matrix_lines as usize]; matrix_columns as usize];
}

pub fn allocate_empty_locations_flag(matrix_width : u16) -> Vec<bool>{
    return vec![true; matrix_width as usize];
}

pub fn allocate_remaining_chars_of_positions(matrix_width : u16) -> Vec<i16>{
    return vec![0; matrix_width as usize];
}

pub fn print_matrix(matrix: Vec<Vec<char>>) {
    let matrix_width: usize = matrix.len();
    let matrix_height: usize = matrix[0].len();
    let mut matrix_line: String = String::new();
    let mut i: usize = 0;
    let mut j: usize = 0;

    // These loops sequences will fetch each line of matrix and
    // print without break line

    // Running in y 
    while j < matrix_height {
        // Running in x 
        while i < matrix_width {
            matrix_line.push(matrix[i][j]);
            i += 1;
        }

        // Print and clear the string for next loop
        print!("{}", matrix_line);
        matrix_line.clear();

        j += 1;
        i = 0;
    }
}

pub fn generate_random_number(min_value : i32, max_value : i32, can_plus_one : bool) -> usize{
    let mut random_number_generator = rand::thread_rng();

    // canPlusOne was created for the utility of generate the mininum and the maximum characters count
    // in remaining chars generator, and the else statement is useful for use the random number generator
    // to select something into a vector avoiding select something out of borders.
    if can_plus_one == true{
        return random_number_generator.gen_range(min_value..max_value + 1) as usize;
    }
    else{
        return random_number_generator.gen_range(min_value..max_value) as usize;
    }
}

pub fn get_terminal_size() -> Result<TerminalSize, i8>{
    let sizes = terminal_size();

    match sizes{
        Some((Width(w), Height(h))) => {
            Ok(TerminalSize { width: w, height: h })
        },
        None => {
            Err(-1)
        }
    }
}

pub fn reset_terminal_color(){
    let mut program_stdout : StandardStream = StandardStream::stdout(ColorChoice::Always);

    program_stdout.reset().unwrap();
}