use crossterm::cursor::MoveTo;
use crossterm::ExecutableCommand;
use std::process::Command;
use std::cell::RefCell;
use std::io::{stdout, Stdout};
use std::rc::Rc;
use std::sync::{Mutex, MutexGuard};
use std::{sync::Arc, thread, time::Duration};

use crate::structs::{HandledProgramArguments};
use crate::structs::TerminalSize;
use crate::helpers::{generate_random_number, print_matrix, self, allocate_empty_locations_flag, allocate_remaining_chars_of_positions, allocate_matrix};



fn swap_matrix_string_chars(
    matrix_string: &mut Vec<char>,
    first_position: usize,
    second_position: usize,
) {
    let swap_variable = matrix_string[first_position];
    matrix_string[first_position] = matrix_string[second_position];
    matrix_string[second_position] = swap_variable;
}

fn pull_matrix_string_down(
    matrix_string: &mut Vec<char>,
    current_string: usize,
    remaining_chars_of_columns: Arc<Mutex<Vec<i16>>>,
    empty_positions_flags: Arc<Mutex<Vec<bool>>>,
    available_matrix_characters : &Vec<char>
) {
    let mut the_last: i32 = matrix_string.len() as i32 - 1;
    let mut above_of_the_last: i32 = the_last - 1;
    let mut remaining_chars: MutexGuard<Vec<i16>>;
    let mut empty_flags: MutexGuard<Vec<bool>>;

    matrix_string[the_last as usize] = ' ';

    // Pulling down all strings in matrix
    while above_of_the_last >= 0 {
        // This function does the magic
        swap_matrix_string_chars(matrix_string, above_of_the_last as usize, the_last as usize);

        above_of_the_last -= 1;
        the_last -= 1;
    }

    empty_flags = empty_positions_flags.lock().unwrap();

    // Checking if exists remaining chars to be pulled to matrix string effect
    // checking if false because is comparing if is "empty_flags"
    if empty_flags[current_string] == false {
        // If entered here mean that exists something to be pulled to matrix effect, then...

        // In the top of current string a new random character is created
        matrix_string[0] =
            available_matrix_characters[generate_random_number(0, available_matrix_characters.len() as i32, false)];
        {
            // A character was pulled, then the counter of it is decreased
            remaining_chars = remaining_chars_of_columns.lock().unwrap();
            remaining_chars[current_string] -= 1;

            // If its counter reached zero, then that string won't produce more characters at the matrix's top
            // a flag declaring that that column will not produce is setted to true.
            // the entry_buffer_controller() will fill it in another time
            if remaining_chars[current_string] <= 0 {
                empty_flags[current_string] = true;
            }

            drop(remaining_chars);
        }
    }

    drop(empty_flags);
}

// This code segment will make the matrix effect of strings falling.
pub fn matrix_renderer(
    matrix: Arc<Mutex<Vec<Vec<char>>>>,
    remaining_chars_of_columns: Arc<Mutex<Vec<i16>>>,
    empty_positions_flags: Arc<Mutex<Vec<bool>>>,
    available_matrix_characters : Vec<char>,
    handled_program_arguments : Arc<HandledProgramArguments>
) {
    let mut matrix_locking: MutexGuard<Vec<Vec<char>>>;
    let mut current_string: usize;
    let mut matrix_size: usize;
    let mut i: usize;
    let mut program_output: Stdout = stdout();

    loop {
        // To avoid clear out all screen each time to draw the matrix
        // the program just put the cursor at the begin of terminal's buffer
        // to redraw everything again
        program_output.execute(MoveTo(0, 0)).unwrap();
        current_string = 0;
        i = 0;

        matrix_locking = matrix.lock().unwrap();
        matrix_size = matrix_locking.len();

        // Pull each matrix's string down with this loop
        while i < matrix_size {
            pull_matrix_string_down(
                &mut matrix_locking[i],
                current_string,
                remaining_chars_of_columns.clone(),
                empty_positions_flags.clone(),
                &available_matrix_characters
            );

            current_string += 1;
            i += 1;
        }

        print_matrix(matrix_locking.clone());

        drop(matrix_locking);

        thread::sleep(Duration::from_millis(*handled_program_arguments.matrix_redraw_cooldown));
    }
}

// This code segment will create new strings to fall in matrix
pub fn entry_buffer_controller(
    matrix : Arc<Mutex<Vec<Vec<char>>>>,
    empty_positions_flags: Arc<Mutex<Vec<bool>>>,
    remaining_chars_of_columns: Arc<Mutex<Vec<i16>>>,
    term_size : Rc<RefCell<TerminalSize>>,
    handled_program_arguments : Arc<HandledProgramArguments>
) {
    let mut new_term_size : TerminalSize;
    let mut matrix_guard : MutexGuard<Vec<Vec<char>>>;
    let mut empty_flags: MutexGuard<Vec<bool>>;
    let mut remaining_chars: MutexGuard<Vec<i16>>;
    let mut available_positions_to_add_string: Vec<usize> = Vec::new();
    let mut position_in_empty_positions_flags: usize = 0;
    let mut selected_position_to_new_string: usize;

    loop {
        matrix_guard = matrix.lock().unwrap();
        new_term_size = helpers::get_terminal_size().unwrap();
        empty_flags = empty_positions_flags.lock().unwrap();
        remaining_chars = remaining_chars_of_columns.lock().unwrap();

        // Check if terminal got its size changed
        {
            let mut term_size_borrowing = term_size.borrow_mut();
            if term_size_borrowing.width != new_term_size.width || term_size_borrowing.height != new_term_size.height{
                term_size_borrowing.width = new_term_size.width;
                term_size_borrowing.height = new_term_size.height;

                *empty_flags = allocate_empty_locations_flag(term_size_borrowing.width);
                *remaining_chars = allocate_remaining_chars_of_positions(term_size_borrowing.width);
                *matrix_guard = allocate_matrix(term_size_borrowing.width, term_size_borrowing.height);

                Command::new("clear").status().unwrap();
            }
        }

        for flag in empty_flags.iter() {
            if *flag == true {
                available_positions_to_add_string.push(position_in_empty_positions_flags);
            }

            position_in_empty_positions_flags += 1;
        }

        if !available_positions_to_add_string.is_empty() {
            selected_position_to_new_string = available_positions_to_add_string
                [generate_random_number(0, available_positions_to_add_string.len() as i32, false)];
            empty_flags[selected_position_to_new_string] = false;
            remaining_chars[selected_position_to_new_string] = generate_random_number(*handled_program_arguments.min_string_size as i32, *handled_program_arguments.max_string_size as i32, true) as i16;
        }

        drop(empty_flags);
        drop(remaining_chars);
        drop(matrix_guard);

        available_positions_to_add_string.clear();
        position_in_empty_positions_flags = 0;

        thread::sleep(Duration::from_millis(*handled_program_arguments.matrix_string_generator_cooldown));
    }
}
