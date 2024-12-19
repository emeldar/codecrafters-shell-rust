#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn echo(arguments: &[&str]) {
    println!("{0}", arguments.join(" "));
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let arguments: Vec<_> = input.trim().split(" ").filter(|s| !s.is_empty()).collect();

        if arguments.len() > 0 {
            // Cover builtins
            match arguments[0] {
                "exit" => process::exit(0),
                "echo" => echo(&arguments[1..]),
                &_ => println!("{0}: command not found", input.trim()),
            }
        }
    }
}
