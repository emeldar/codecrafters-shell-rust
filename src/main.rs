#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let arguments: Vec<_> = input.trim().split(" ").collect();

        if arguments.len() > 0 {
            // Cover builtins
            match arguments[0] {
                "exit" => process::exit(0),
                &_ => (),
            }
        }

        println!("{0}: command not found", input.trim());
    }
}
