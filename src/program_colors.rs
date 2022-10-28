
use std::{sync::Arc};
use termcolor::{
    ColorSpec, Color, StandardStream, WriteColor
};

use crate::structs::HandledProgramArguments;


pub fn set_output_color_if_necessary(handled_program_arguments : Arc<HandledProgramArguments>, program_output : &mut StandardStream){
    // Checking if foreground color was passed to program by command line
    match *handled_program_arguments.foreground_color_pointer {
        Some(the_color) => {
            // If yes, then set output color
            program_output.set_color(ColorSpec::new().set_fg(Some(the_color))).unwrap();
        },
        None => {
            // If not, there's no problem, a default color is already setted
        }
    }

    // Checking if foreground color was passed to program by command line
    match *handled_program_arguments.background_color_pointer{
        Some(the_color) => {
            // If yes, then set output color
            program_output.set_color(ColorSpec::new().set_bg(Some(the_color))).unwrap();
        },
        None => {
            // If not, there's no problem, a default color is already setted
        }
    }
}

pub fn check_if_color_exists(selected_color : String) -> Option<Color>{
    // Switch sequence to check if passed color exists in termcolor library
    match selected_color.as_str() {
        "Black" => {
            return Some(Color::Black);
        },
        "Blue" => { 
            return Some(Color::Blue);
        },
        "Cyan" => {
            return Some(Color::Cyan);
        },
        "Green" => {
            return Some(Color::Green);
        },
        "Magenta" => {
            return Some(Color::Magenta);
        },
        "Red" => {
            return Some(Color::Red);
        },
        "White" => {
            return Some(Color::White);
        },
        "Yellow" => {
            return Some(Color::Yellow);
        },
        &_ => {
            // If not, return Option<> with None
            return None;
        }
    }
}