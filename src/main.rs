// Another modules
mod handle_arguments;
mod program_configurations;
mod helpers;
mod controllers;
mod program_colors;
mod structs;

// Libs
use std::cell::{RefCell};
use std::rc::Rc;
use std::thread;
use clap::{Parser};
use helpers::{reset_terminal_color, clear_terminal};
use structs::HandledProgramArguments;
use termcolor::{StandardStream, ColorChoice};
use std::sync::{Arc, Mutex};
use controllers::matrix_renderer;
use crossterm::{
    cursor::{Hide, Show}, ExecutableCommand
};

fn main(){
    // Common variables
    let available_matrix_chars: Vec<char> = vec![
        '0', 'a', 'b', 'c', '1', 'd', 'e', 'f', '2', 'g', 'h', 'i', '3','j', 'k', 'l', '4','m', 'n', 'o', '5', 'p', 'q', 'r',
        '6', 's', 't', 'u', '7', 'v', 'w', 'x', '8' ,'y', 'z', '9'
    ];
    let mut program_stdout = StandardStream::stdout(ColorChoice::Always);
    let matrix : Arc<Mutex<Vec<Vec<char>>>>;
    let term_size : Rc<RefCell<structs::TerminalSize>>;
    let remaining_chars_of_columns_clone  : Arc<Mutex<Vec<i16>>>;
    let empty_positions_flags : Arc<Mutex<Vec<bool>>>;
    let program_arguments : structs::ProgramArguments;
    let handled_program_arguments : Arc<HandledProgramArguments>;
    let handled_program_arguments_clone : Arc<HandledProgramArguments>;
    let program_alive_flag : Arc<Mutex<bool>>;
    let program_alive_flag_clone : Arc<Mutex<bool>>;
    let mut color_stdout : StandardStream;

    // Main variables
    let matrix_clone : Arc<Mutex<Vec<Vec<char>>>>;
    let remaining_chars_of_columns : Arc<Mutex<Vec<i16>>>;
    let empty_positions_flags_clone : Arc<Mutex<Vec<bool>>>;

    color_stdout = StandardStream::stdout(ColorChoice::Always);

    // Set the program to alive. This will be use to stop program when necessary
    program_alive_flag = Arc::new(Mutex::new(true));

    // Parsing arguments with clap lib
    program_arguments = structs::ProgramArguments::parse();

    // All necessary arguments are now inside of Arc<HandledProgramArguments> variable
    handled_program_arguments = handle_arguments::handle_cli_arguments(program_arguments);

    // Set color of matrix if necessary
    program_colors::set_output_color_if_necessary(handled_program_arguments.clone(), &mut program_stdout);
    
    // Hide cursor and set sinal handling to show cursor when necessary
    program_stdout.execute(Hide).unwrap();
    program_configurations::set_thread_for_program_exit(program_alive_flag.clone());

    term_size = Rc::new(RefCell::new(helpers::get_terminal_size().unwrap()));

    // Allocating matrix and empty positions flags
    {
        let term_size_borrowing = term_size.borrow_mut();
        matrix = Arc::new(Mutex::new(helpers::allocate_matrix(term_size_borrowing.width, term_size_borrowing.height)));
        empty_positions_flags = Arc::new(Mutex::new(helpers::allocate_empty_locations_flag(term_size_borrowing.width)));
        remaining_chars_of_columns = Arc::new(Mutex::new(helpers::allocate_remaining_chars_of_positions(term_size_borrowing.width)));
    }

    // Cloning Arc smart pointers to pass through matrix_renderer thread
    matrix_clone = matrix.clone();
    remaining_chars_of_columns_clone = remaining_chars_of_columns.clone();
    empty_positions_flags_clone = empty_positions_flags.clone();
    handled_program_arguments_clone = handled_program_arguments.clone(); 
    program_alive_flag_clone = program_alive_flag.clone();

    // Spawn matrix renderer in a new thread..
    // This code segment will make the matrix effect of strings falling.
    let matrix_renderer_join_handler = thread::spawn(move || matrix_renderer(matrix_clone, remaining_chars_of_columns_clone, empty_positions_flags_clone, available_matrix_chars, handled_program_arguments_clone, program_alive_flag_clone));

    // Starting entry buffer controller
    // This code segment will create new strings to fall in matrix
    controllers::entry_buffer_controller(matrix.clone() ,empty_positions_flags.clone(), remaining_chars_of_columns.clone(), term_size.clone(), handled_program_arguments.clone(), program_alive_flag.clone());

    matrix_renderer_join_handler.join().unwrap();

    reset_terminal_color();
    color_stdout.execute(Show).unwrap();
    clear_terminal();
}