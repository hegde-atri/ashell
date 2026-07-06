use std::io::{self, Write, stdin};

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

        // Evaluate input
        let (cmd, args) = parse_input(input.clone());
        match cmd.as_str() {
          "exit" => Command::handle_exit(),
          _ => Command::handle_not_found(cmd),
        }
    }
}


/// Parses input into command, its arg + args
fn parse_input(input: String) -> (String, Vec<String>){
    // let cmd: String = input.trim().split(" ").next().expect("Failed to parse input").to_string();
    let input_as_vec: Vec<String> = input.trim().split(" ").map(|e| e.to_string()).collect();
    let cmd: String = input_as_vec[0].to_string();
    let args: Vec<String> = input_as_vec[1..].to_vec();

    return (cmd, args)
}

enum Command {
  Exit,
  NotFound
}

impl Command {
  pub fn handle_exit() {
    std::process::exit(0);
  }
  pub fn handle_not_found(cmd: String){
    println!("{}: command not found", cmd);
  }
}