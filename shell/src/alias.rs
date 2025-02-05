use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

pub struct AliasManager {
    aliases: HashMap<String, String>,
    file_path: PathBuf,
}

impl AliasManager {
    pub fn new() -> Self {
        let file_path = dirs::home_dir()
            .unwrap_or_default()
            .join(".myshell_aliases");
        
        AliasManager {
            aliases: HashMap::new(),
            file_path,
        }
    }

    pub fn add(&mut self, name: &str, command: &str) {
        self.aliases.insert(name.to_string(), command.to_string());
    }

    pub fn remove(&mut self, name: &str) {
        self.aliases.remove(name);
    }

    pub fn expand(&self, input: &str) -> String {
        let mut result = input.to_string();
        
        // Try to match aliases, starting with the longest ones first
        let mut aliases: Vec<(&String, &String)> = self.aliases.iter().collect();
        aliases.sort_by(|a, b| b.0.len().cmp(&a.0.len())); // Sort by length descending
        
        for (alias, expansion) in aliases {
            if result.starts_with(alias) {
                // Replace only at the start of the string or after whitespace
                result = result.replacen(alias, expansion, 1);
                break; // Only replace the first occurrence
            }
        }
        
        result
    }

    pub fn list(&self) -> Vec<(String, String)> {
        self.aliases
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    pub fn get(&self, name: &str) -> Option<String> {
        self.aliases.get(name).cloned()
    }

    pub fn load(&mut self) -> io::Result<()> {
        if let Ok(file) = File::open(&self.file_path) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    if let Some((name, command)) = line.split_once('=') {
                        self.aliases.insert(name.to_string(), command.to_string());
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

        for (name, command) in &self.aliases {
            writeln!(file, "{}={}", name, command)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alias_expansion() {
        let mut manager = AliasManager::new();
        manager.add("ll", "ls -l");
        manager.add("gst", "git status");
        
        assert_eq!(manager.expand("ll"), "ls -l");
        assert_eq!(manager.expand("ll -a"), "ls -l -a");
        assert_eq!(manager.expand("gst"), "git status");
        assert_eq!(manager.expand("echo hello"), "echo hello");
    }

    #[test]
    fn test_alias_management() {
        let mut manager = AliasManager::new();
        manager.add("ll", "ls -l");
        
        assert_eq!(manager.get("ll"), Some("ls -l".to_string()));
        manager.remove("ll");
        assert_eq!(manager.get("ll"), None);
    }
} 