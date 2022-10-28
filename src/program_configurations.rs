use signal_hook::{iterator::Signals, consts::{SIGINT, SIGABRT, SIGQUIT}};
use termcolor::{StandardStream, ColorChoice};
use std::{thread};
use std::process::{Command, exit};
use crossterm::{
    cursor::Show, ExecutableCommand
};

use crate::helpers::reset_terminal_color;

pub fn set_signals_handling(){
    let mut signal_handling = Signals::new(&[SIGINT, SIGABRT, SIGQUIT]).unwrap();
    let mut program_stdout : StandardStream = StandardStream::stdout(ColorChoice::Always);

    thread::spawn(move || {
        for _ in signal_handling.forever(){
            Command::new("clear").status().unwrap();
            program_stdout.execute(Show).unwrap();
            reset_terminal_color();
            exit(0);
        }
    });
}