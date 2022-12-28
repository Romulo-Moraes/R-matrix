
use std::{sync::Arc};
use termcolor::{
    ColorSpec, Color, StandardStream, WriteColor
};

use crate::structs::HandledProgramArguments;


pub fn set_output_color_if_necessary(handled_program_arguments : Arc<HandledProgramArguments>, program_output : &mut StandardStream){
    let mut foreground_color_was_set : bool = false;
    let mut background_color_was_set : bool = false;

    /* Black is the default value of these variables, the only reason is for the compiler be happy,
    if these variables will be used a new value will be assigned to them */
    let mut foreground_color : Color = Color::Black;
    let mut background_color : Color = Color::Black;

    // Checking if foreground color was passed to program by command line
    match *handled_program_arguments.foreground_color_pointer {
        Some(the_color) => {
            // If yes, then set output foreground color flag
            foreground_color_was_set = true;
            foreground_color = the_color;
        },
        None => {
            // If not, there's no problem, a default color is already set
        }
    }

    // Checking if foreground color was passed to program by command line
    match *handled_program_arguments.background_color_pointer{
        Some(the_color) => {
            // If yes, then set output background color flag
            background_color_was_set = true;
            background_color = the_color;
        },
        None => {
            // If not, there's no problem, a default color is already set
        }
    }

    if background_color_was_set == true && foreground_color_was_set == true{
        program_output.set_color(ColorSpec::new().set_fg(Some(foreground_color)).set_bg(Some(background_color))).unwrap();
    }
    else {
        if background_color_was_set == true{
            program_output.set_color(ColorSpec::new().set_bg(Some(background_color))).unwrap();
        }
        else{
            if foreground_color_was_set == true{
                program_output.set_color(ColorSpec::new().set_fg(Some(foreground_color))).unwrap();
            }
        }
    }
}

pub fn check_if_color_exists(selected_color : String) -> Option<(Color, bool)>{
    // Switch sequence to check if passed color exists in termcolor library
    match selected_color.as_str() {
        "Black" => {
            return Some((Color::Rgb(0, 0, 0), false));
        },
        "Gray" => {
            return Some((Color::Black, false));
        }
        "Blue" => { 
            return Some((Color::Blue, false));
        },
        "BrightBlue" => {
            return Some((Color::Rgb(29, 240, 211), false));
        }
        ,
        "Cyan" => {
            return Some((Color::Cyan, false));
        },
        "BrightCyan" => {
            return Some((Color::Rgb(9, 208, 239), false));
        }
        "Green" => {
            return Some((Color::Green, false));
        },
        "BrightGreen" => {
            return Some((Color::Rgb(102, 255, 0), false));
        },
        "Magenta" => {
            return Some((Color::Magenta, false));
        },
        "BrightMagenta" => {
            return Some((Color::Rgb(255, 0, 205), false));
        },
        "Red" => {
            return Some((Color::Red, false));
        },
        "BrightRed" => {
            return Some((Color::Rgb(255, 0, 0), false));
        },
        "White" => {
            return Some((Color::White, false));
        },
        "BrightWhite" => {
            return Some((Color::Rgb(255, 255, 255), false));
        },
        "Yellow" => {
            return Some((Color::Yellow, false));
        },
        "BrightYellow" => {
            return Some((Color::Rgb(255, 235, 42), false));
        },
        "Rainbow" => {
            return Some((Color::Black, true))
        },
        &_ => {
            // If not, return Option<> with None
            return None;
        }
    }
}