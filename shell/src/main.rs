mod history;
mod shell;
mod commands;
mod builtins;
mod environment;
mod alias;
mod parser;
mod signals;
mod logger;
mod completion;

use std::io;
use shell::Shell;
use std::error::Error;

fn main() -> io::Result<()> {
    // Initialize logger
    if let Err(e) = logger::init_logger() {
        eprintln!("Failed to initialize logger: {}", e);
    }

    // Create and run the shell
    let mut shell = Shell::new()?;
    shell.run()?;

    Ok(())
}
