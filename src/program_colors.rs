
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

pub fn check_if_color_exists(selected_color : String) -> Option<Color>{
    // Switch sequence to check if passed color exists in termcolor library
    match selected_color.as_str() {
        "Black" => {
            return Some(Color::Rgb(0, 0, 0));
        },
        "Gray" => {
            return Some(Color::Black);
        }
        "Blue" => { 
            return Some(Color::Blue);
        },
        "BrightBlue" => {
            return Some(Color::Rgb(29, 240, 211));
        }
        ,
        "Cyan" => {
            return Some(Color::Cyan);
        },
        "BrightCyan" => {
            return Some(Color::Rgb(9, 208, 239));
        }
        "Green" => {
            return Some(Color::Green);
        },
        "BrightGreen" => {
            return Some(Color::Rgb(102, 255, 0));
        },
        "Magenta" => {
            return Some(Color::Magenta);
        },
        "BrightMagenta" => {
            return Some(Color::Rgb(255, 0, 205));
        },
        "Red" => {
            return Some(Color::Red);
        },
        "BrightRed" => {
            return Some(Color::Rgb(255, 0, 0));
        },
        "White" => {
            return Some(Color::White);
        },
        "BrightWhite" => {
            return Some(Color::Rgb(255, 255, 255));
        },
        "Yellow" => {
            return Some(Color::Yellow);
        },
        "BrightYellow" => {
            return Some(Color::Rgb(255, 235, 42));
        },
        &_ => {
            // If not, return Option<> with None
            return None;
        }
    }
}