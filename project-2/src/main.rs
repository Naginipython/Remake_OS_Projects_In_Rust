use std::{
    process, //https://doc.rust-lang.org/std/process/index.html
    io::{self, Write},
};
use simple_shell::SimpleShell;

mod simple_shell;

fn main() {
    loop {
        // Gets command
        // TODO: get this to work after command executes
        print!("tsh> ");
        let mut input: String = String::new();
        let _ = io::stdout().flush(); // allows to print then stdin
        io::stdin().read_line(&mut input).expect("Error: Expected input");
        
        if &input != "\n" {
            let shell = SimpleShell::new(input);
            if shell.is_quit() {
                println!("Quitting...");
                process::exit(0);
            } else {
                shell.exec_command();
            }
        }
    }
}
