use std::sync::Arc;
use termcolor::Color;
use clap::{Parser};

const AVAILABLE_COLORS : &str = "Black\nGray\nBlue\nCyan\nGreen\nMagenta\nRed\nWhite\nYellow\nBrightBlue\nBrightCyan\nBrightGreen\nBrightMagenta\nBrightRed\nBrightWhite\nBrightYellow\n";
pub struct TerminalSize{
    pub width : u16,
    pub height : u16
}

pub struct HandledProgramArguments{
    pub max_string_size : Arc<i16>,
    pub min_string_size : Arc<i16>,
    pub foreground_color_pointer : Arc<Option<Color>>,
    pub background_color_pointer : Arc<Option<Color>>,
    pub matrix_redraw_cooldown : Arc<u64>,
    pub matrix_string_generator_cooldown : Arc<u64>
}

#[derive(Parser,Debug)]
#[command(author="Rômulo Moraes", version="0.1.0", about="Matrix effect", long_about="Matrix effect of strings falling through the terminal")]
pub struct ProgramArguments{
    #[arg(short, long, help=AVAILABLE_COLORS)]
    pub foreground : Option<String>,

    #[arg(short, long, help=AVAILABLE_COLORS)]
    pub background : Option<String>,

    #[arg(short='m', long)]
    pub min_string_size : Option<i16>,

    #[arg(short='M', long)]
    pub max_string_size : Option<i16>,

    #[arg(short='r', long, help="Default is 40")]
    pub matrix_redraw_cooldown : Option<u64>,

    #[arg(short='s', long, help="Default is 40")]
    pub matrix_string_generator_cooldown : Option<u64>
}
