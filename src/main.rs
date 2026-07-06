use is_executable::is_executable;
use std::{
    io::{self, Write, stdin},
    path::PathBuf,
    str::FromStr,
};
use strum_macros::EnumString;

fn main() {
    // REPL Loop
    loop {
        // Print prompt
        print!("$ ");
        io::stdout().flush().unwrap();

        // Handle input from user
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(err) => {
                println!("Failed to read input: {err}");
            }
        }

        // Parse input
        let (cmd, args) = parse_input(input.clone());
        // Eval input
        
        // If we hit enter without nothing in it, it should skip.
        if !cmd.is_empty() {
            match Command::from_str(cmd.as_str()) {
                Ok(Command::Echo) => Command::handle_echo(args),
                Ok(Command::Exit) => Command::handle_exit(),
                Ok(Command::Type) => Command::handle_type(args),
                Ok(Command::Pwd) => Command::handle_pwd(),
                _ => Command::exec(cmd, args),
            }
        }
    }
}

/// Parses input into command, its arg + args
fn parse_input(input: String) -> (String, Vec<String>) {
    let input_as_vec: Vec<String> = input.trim().split(" ").map(|e| e.to_string()).collect();
    let cmd: String = input_as_vec[0].to_string();
    let args: Vec<String> = input_as_vec[1..].to_vec();

    return (cmd, args);
}

#[derive(Debug, EnumString, strum_macros::Display)]
enum Command {
    #[strum(serialize = "exit")]
    Exit,
    #[strum(serialize = "echo")]
    Echo,
    #[strum(serialize = "type")]
    Type,
    #[strum(serialize = "pwd")]
    Pwd,
    // #[strum(disabled)]
    // NotFound,
}

impl Command {
    pub fn handle_exit() {
        std::process::exit(0);
    }

    pub fn handle_echo(args: Vec<String>) {
        println!("{}", args.join(" "));
    }

    pub fn handle_pwd() {
        println!("{}", std::env::current_dir().expect("Could not get current directory").display());
    }

    pub fn handle_cd(args: Vec<String>) {
        // Handle absolute dirs
        // Handle relative dirs
        // Handle ~
        let mut pwd = std::env::current_dir().expect("Could not get current directory");
        pwd.push(args[0].clone());
    }
    
    pub fn handle_type(args: Vec<String>) {
        let cmd = args.join(" ");
        match Command::from_str(cmd.as_str()) {
            Ok(cmd) => println!("{} is a shell builtin", cmd),
            Err(_) => {
                // If not builtin, try to see if it exists on path
                match Self::find_executable(&cmd) {
                    Some(path) => println!("{} is {}", cmd, path.display()),
                    None => println!("{cmd}: not found"),
                }
            }
        }
    }

    pub fn exec(cmd: String, args: Vec<String>) {
        match Self::find_executable(&cmd) {
            Some(_) => {
                let raw_output = std::process::Command::new(cmd)
                    .args(args)
                    .output()
                    .expect("failed to execute process");
                
                let string_output = String::from_utf8(raw_output.stdout).expect("Failed to parse output");
                for line in string_output.trim().split("\n") {
                    println!("{line}");
                }
            }
            None => println!("{cmd}: not found"),
        }
    }

    fn find_executable(cmd: &str) -> Option<PathBuf> {
        let path_env = std::env::var("PATH").expect("Could not find $PATH");
        for mut path in std::env::split_paths(&path_env) {
            path.push(cmd);
            if path.exists() && is_executable(&path) {
                return Some(path);
            }
        }
        return None;
    }

}
