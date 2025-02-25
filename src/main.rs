#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::PathBuf;
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

    let mut command = process::Command::new(arguments[0]);

    for i in 1..arguments.len() {
        command.arg(arguments[i]);
    }

    let output = command.output().expect("Failed to execute process");

    print!("{}", String::from_utf8(output.stdout).unwrap());
    print!("{}", String::from_utf8(output.stderr).unwrap());
}

fn pwd(cwd: &PathBuf) {
    let current_dir = cwd.display();

    println!("{current_dir}");
}

fn cd(cwd: &mut PathBuf, arguments: &[&str]) {
    let path_change = arguments[0];

    let new_path = if path_change.starts_with('/') {
        PathBuf::from(path_change)
    } else if path_change.starts_with("~") {
        PathBuf::from(
            env::var("HOME")
                .map(|home| path_change.replacen('~', &home, 1))
                .unwrap_or_else(|_| path_change.to_string())
        )
    } else {
        cwd.join(path_change)
    };

    if new_path.exists() && new_path.is_dir() {
        *cwd = new_path.canonicalize().unwrap();
    } else {
        println!("cd: {path_change}: No such file or directory");
    }
}

fn parse_args(input: &str) -> Vec<String> {
    let mut args = vec![];
    let mut single_quote = false;
    let mut current_arg = String::new();

    for c in input.chars() {
        match c {
            '\'' => single_quote = !single_quote,
            ' ' => {
                if single_quote {
                    current_arg.push(c);
                } else {
                    if !current_arg.is_empty() {
                        args.push(current_arg);
                    }

                    current_arg = String::new();
                }
            }
            _ => {
                current_arg.push(c);
            }
        }
    }

    args.push(current_arg);

    return args;
}

fn main() {
    let mut cwd = env::current_dir().unwrap();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let arguments = parse_args(&input.trim());
        let arguments: Vec<&str> = arguments.iter().map(|s| &**s).collect();

        if arguments.len() > 0 {
            // Cover builtins
            match arguments[0] {
                "exit" => process::exit(0),
                "echo" => echo(&arguments[1..]),
                "type" => type_check(&arguments[1..]),
                "pwd" => pwd(&cwd),
                "cd" => cd(&mut cwd, &arguments[1..]),
                &_ => try_run(&arguments),
            }
        }
    }
}
