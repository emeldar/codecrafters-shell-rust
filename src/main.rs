#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;
use std::env;
use std::fs;

const BUILTINS: [&str; 4] = ["exit", "echo", "type", "pwd"];

fn find_path_if_exists(file_to_find: &str) -> String {
    let paths: Vec<std::path::PathBuf> = env::split_paths(&env::var_os("PATH").expect("Couldn't retrieve PATH")).collect();

    for path in paths {
        if !path.exists() {
            continue;
        }

        let files: fs::ReadDir = fs::read_dir(path.clone()).expect("Failed to read path");
        for file in files {
            let dir_entry = file.expect("Failed to get directory entry");
            let file_name = dir_entry.file_name().into_string().expect("Failed to convert OsString to String");
            if file_name == file_to_find {
                return dir_entry.path().to_str().expect("AHHHHH").to_string();
            }
        }
    }

    return "".to_string();
}

fn echo(arguments: &[&str]) {
    println!("{0}", arguments.join(" "));
}

fn type_check(arguments: &[&str]) {
    if arguments.len() != 1 {
        println!("type: should have only one argument")
    } else {
        if BUILTINS.contains(&arguments[0]) {
            println!("{} is a shell builtin", arguments[0]);
            return;
        }

        let found_file = find_path_if_exists(arguments[0]);

        if found_file != "".to_string() {
            println!("{} is {}", arguments[0], found_file);
            return;
        }

        println!("{}: not found", arguments [0]);
    }
}

fn try_run(arguments: &[&str]) {
    let found_file = find_path_if_exists(arguments[0]);

    if found_file == "".to_string() {
        println!("{0}: command not found", arguments[0]);
        return;
    }

    let mut command = process::Command::new(found_file);

    for i in 1..arguments.len() {
        command.arg(arguments[i]);
    }

    let output = command.output().expect("Failed to execute process");

    print!("{}", String::from_utf8(output.stdout).unwrap());
    print!("{}", String::from_utf8(output.stderr).unwrap());
}

fn pwd() {
    let current_dir = env::current_dir().unwrap();
    let current_dir = current_dir.display();

    println!("{current_dir}");
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
                "pwd" => pwd(),
                &_ => try_run(&arguments),
            }
        }
    }
}
