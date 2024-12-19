#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

const BUILTINS: [&str; 3] = ["exit", "echo", "type"];

fn echo(arguments: &[&str]) {
    println!("{0}", arguments.join(" "));
}

fn type_check(arguments: &[&str]) {
    if arguments.len() != 1 {
        println!("type: should have only one argument")
    } else {
        if BUILTINS.contains(&arguments[0]) {
            println!("{} is a shell builtin", arguments[0]);
        } else {
            println!("{}: not found", arguments [0]);
        }
    }
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
                "type" => type_check(&arguments[1..]),
                &_ => println!("{0}: command not found", input.trim()),
            }
        }
    }
}
