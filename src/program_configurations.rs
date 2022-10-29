//use termcolor::{StandardStream, ColorChoice};
//use std::borrow::BorrowMut;
//use std::collections::binary_heap::DrainSorted;
use std::sync::{Arc, Mutex, MutexGuard};
use std::{thread};
//use std::process::{Command, exit};
use std::io::{stdin, Stdin};

pub fn set_thread_for_program_exit(program_alive_flag : Arc<Mutex<bool>>){
    let program_stdin : Stdin = stdin();
    let mut exit_trigger : String = String::new();
    let program_alive_flag_clone : Arc<Mutex<bool>> = program_alive_flag.clone();

    thread::spawn(move || {
        let mut program_alive_mutex_guard : MutexGuard<bool>;
        program_stdin.read_line(&mut exit_trigger).unwrap();
        program_alive_mutex_guard = program_alive_flag_clone.lock().unwrap();
        *program_alive_mutex_guard = false;
    
        drop(program_alive_mutex_guard);
    });
}