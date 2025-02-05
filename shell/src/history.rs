use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

pub struct History {
    commands: Vec<String>,
    file_path: PathBuf,
}

impl History {
    pub fn new() -> Self {
        let file_path = dirs::home_dir()
            .unwrap_or_default()
            .join(".myshell_history");
        
        History {
            commands: Vec::new(),
            file_path,
        }
    }

    pub fn add(&mut self, command: String) {
        self.commands.push(command);
    }

    pub fn load(&mut self) -> io::Result<()> {
        if let Ok(file) = File::open(&self.file_path) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(command) = line {
                    self.commands.push(command);
                }
            }
        }
        Ok(())
    }

    pub fn save(&self) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)?;

        for command in &self.commands {
            writeln!(file, "{}", command)?;
        }
        Ok(())
    }

    pub fn show(&self) -> io::Result<()> {
        for (index, cmd) in self.commands.iter().enumerate() {
            println!("{}\t{}", index + 1, cmd);
        }
        Ok(())
    }
}
