use std::{sync::Arc};
use std::process::exit;
use termcolor::{Color, StandardStream, ColorChoice, ColorSpec, WriteColor};
use crate::{helpers::{reset_terminal_color}, program_colors, structs::{ProgramArguments,HandledProgramArguments}};


pub fn handle_cli_arguments(program_arguments : ProgramArguments) -> Arc<HandledProgramArguments>{
    let foreground_color_pointer : Arc<Option<Color>>;
    let background_color_pointer : Arc<Option<Color>>;
    let mut max_string_size : Arc<i16> = Arc::new(12);
    let mut min_string_size : Arc<i16> = Arc::new(8);
    let mut matrix_redraw_cooldown : Arc<u64> = Arc::new(70);
    let mut matrix_string_generator_cooldown : Arc<u64> = Arc::new(150);
    let mut program_stdout : StandardStream = StandardStream::stdout(ColorChoice::Always);

    match program_arguments.matrix_redraw_cooldown {
        Some(cooldown_time) => {
            matrix_redraw_cooldown = Arc::new(cooldown_time);
        },
        None => {
        }
    }

    match program_arguments.matrix_string_generator_cooldown {
        Some(cooldown_time) => {
            matrix_string_generator_cooldown = Arc::new(cooldown_time);
        },
        None => {
        }
    }

    match program_arguments.foreground {
        Some(color_as_string) => {
            match program_colors::check_if_color_exists(color_as_string){
                Some(final_color) => {
                    foreground_color_pointer = Arc::new(Some(final_color));
                },
                None => {
                    program_stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
                    println!("Unknow foreground color detected. Try again.");
                    reset_terminal_color();
                    exit(1);
                }
            }
        },
        None => {
            foreground_color_pointer = Arc::new(None);
        }
    }

    match program_arguments.background {
        Some(color_as_string) => {
            match program_colors::check_if_color_exists(color_as_string){
                Some(final_color) => {
                    background_color_pointer = Arc::new(Some(final_color));
                },
                None => {
                    program_stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
                    println!("Unknow background color detected. Try again.");
                    reset_terminal_color();
                    exit(1);
                }
            }
        },
        None => {
            background_color_pointer = Arc::new(None);
        }
    }


    match program_arguments.max_string_size{
        Some(size) => {
            max_string_size = Arc::new(size);
        },
        None => {

        }
    }

    match program_arguments.min_string_size{
        Some(size) => {
            min_string_size = Arc::new(size);
        },
        None => {

        }
    }

    if min_string_size >= max_string_size{
        program_stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
        println!("The minimum size can't equal or greater than maximum string size. Min: {}, Max: {}", min_string_size, max_string_size);
        exit(1);
    }


    return Arc::new(HandledProgramArguments { max_string_size: max_string_size,
        min_string_size: min_string_size, foreground_color_pointer: foreground_color_pointer,
        background_color_pointer: background_color_pointer,
        matrix_redraw_cooldown : matrix_redraw_cooldown, 
        matrix_string_generator_cooldown : matrix_string_generator_cooldown});
}