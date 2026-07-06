use std::{io::{self, Write, stdin}, str::FromStr};

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
        match Command::from_str(cmd.as_str()) {
          Ok(Command::Echo) => Command::handle_echo(args),
          Ok(Command::Exit) => Command::handle_exit(),
          Ok(Command::Type) => Command::handle_type(args),
          _ => Command::handle_not_found(cmd),
        }
    }
}


/// Parses input into command, its arg + args
fn parse_input(input: String) -> (String, Vec<String>){
    let input_as_vec: Vec<String> = input.trim().split(" ").map(|e| e.to_string()).collect();
    let cmd: String = input_as_vec[0].to_string();
    let args: Vec<String> = input_as_vec[1..].to_vec();

    return (cmd, args)
}

#[derive(Debug, EnumString, strum_macros::Display)]
enum Command {
  #[strum(serialize = "exit")]
  Exit,
  #[strum(serialize = "echo")]
  Echo,
  #[strum(serialize = "type")]
  Type,
  #[strum(disabled)]
  NotFound,
}

impl Command {
  pub fn handle_exit() {
    std::process::exit(0);
  }
  
  pub fn handle_not_found(cmd: String){
    println!("{}: command not found", cmd);
  }
  
  pub fn handle_echo(args: Vec<String>){
    println!("{}", args.join(" "));
  }
  
  pub fn handle_type(args: Vec<String>){
    let cmd = args.join(" ");
    match Command::from_str(cmd.as_str()) {
      Ok(cmd) => println!("{} is a shell builtin", cmd),
      Err(_) => println!("{}: not found", cmd),
    }
  }
}