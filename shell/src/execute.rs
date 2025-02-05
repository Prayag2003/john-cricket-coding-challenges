use std::process::Command; // to run system cmds from rust

pub fn run_command(command: &str) {
    println!("Received command: {}", command);

    let mut parts = command.split_whitespace();
    if let Some(cmd) = parts.next() {
        println!("Command: {}", cmd);
        let args: Vec<&str> = parts.collect();
        println!("Arguments: {:?}", args);

        let output = Command::new(cmd).args(&args).output();
        match output {
            Ok(out) => {
                println!("Command executed successfully.");
                println!("{}", String::from_utf8_lossy(&out.stdout));
            }
            Err(err) => {
                eprintln!("Error executing command: {}", err);
            }
        }
    } else {
        eprintln!("No command provided.");
    }
}
