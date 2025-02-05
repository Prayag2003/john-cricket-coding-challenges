use std::env;
use std::io;
use std::path::Path;
use crate::parser::Command;
use crate::environment::Environment;

pub fn handle_builtin(cmd: &Command, env: &Environment) -> Option<io::Result<()>> {
    match cmd.command.as_str() {
        "cd" => Some(change_directory(&cmd.args)),
        "pwd" => Some(print_working_directory()),
        "history" => Some(show_history()),
        "alias" => Some(handle_alias(cmd)),
        "export" => Some(handle_export(cmd, env)),
        _ => None,
    }
}

fn change_directory(args: &[String]) -> io::Result<()> {
    let new_dir = args.get(0).map(|s| s.as_str()).unwrap_or("~");
    let new_dir = if new_dir == "~" {
        dirs::home_dir().ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "Home directory not found")
        })?
    } else {
        Path::new(new_dir).to_path_buf()
    };

    env::set_current_dir(&new_dir).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to change directory: {}", e)
        )
    })
}

fn print_working_directory() -> io::Result<()> {
    let current_dir = env::current_dir()?;
    println!("{}", current_dir.display());
    Ok(())
}

fn show_history() -> io::Result<()> {
    Ok(())
}

fn handle_alias(cmd: &Command) -> io::Result<()> {
    Ok(())
}

fn handle_export(cmd: &Command, _env: &Environment) -> io::Result<()> {
    Ok(())
}
