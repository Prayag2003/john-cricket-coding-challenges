use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

pub struct Environment {
    vars: HashMap<String, String>,
    file_path: PathBuf,
}

impl Environment {
    pub fn new() -> Self {
        let file_path = dirs::home_dir()
            .unwrap_or_default()
            .join(".myshell_env");
        
        Environment {
            vars: HashMap::new(),
            file_path,
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.vars.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.vars.get(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.vars.iter()
    }

    pub fn load(&mut self) -> io::Result<()> {
        if let Ok(file) = File::open(&self.file_path) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    if let Some((key, value)) = line.split_once('=') {
                        self.vars.insert(key.to_string(), value.to_string());
                    }
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

        for (key, value) in &self.vars {
            writeln!(file, "{}={}", key, value)?;
        }
        Ok(())
    }
} 