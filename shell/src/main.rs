use std::io::{self, Write};

fn main() {
    loop {
        print!("ccsh> ");
        io::stdout().flush().unwrap(); 

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();
        if input == "exit" {
            break;
        }
        
        println!("You entered: {}", input);
    }
}
