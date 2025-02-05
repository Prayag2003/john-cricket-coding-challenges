use std::process::{Command as ProcessCommand, Stdio};
use std::fs::{File, OpenOptions};
use std::io;
use crate::builtins;
use crate::parser::Command;
use crate::environment::Environment;

pub fn execute_command(cmd: &Command, env: &Environment) -> io::Result<()> {
    if let Some(result) = builtins::handle_builtin(cmd, env) {
        return result;
    }

    let mut process = ProcessCommand::new(&cmd.command);
    process.args(&cmd.args);
    
    if let Some(input_file) = &cmd.input_redirect {
        let file = File::open(input_file)?;
        process.stdin(Stdio::from(file));
    }

    if let Some(output_file) = &cmd.output_redirect {
        let file = File::create(output_file)?;
        process.stdout(Stdio::from(file));
    } else if let Some(append_file) = &cmd.append_redirect {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(append_file)?;
        process.stdout(Stdio::from(file));
    }

    // set environment variables
    for (key, value) in env.iter() {
        process.env(key, value);
    }

    let mut child = process.spawn()?;

    if !cmd.background {
        let status = child.wait()?;
        if !status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Command '{}' failed with exit code: {}", cmd.command, status)
            ));
        }
    } else {
        println!("[{}] {}", child.id(), cmd.command);
    }

    Ok(())
} 