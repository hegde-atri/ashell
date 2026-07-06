use std::io::{self, stdin, Write};

fn main() {
  // REPL Loop
  loop {
    // Print prompt
    print!("$ ");
    io::stdout().flush().unwrap();

    // Handle input from user
    let mut input = String::new();
    match stdin().read_line(&mut input) {
      Ok(_) => {},
      Err(err) => {
        println!("Failed to read input: {err}");
      }
    }

    // Evaluate input
    println!("{}: command not found", input.trim());
  }
}
