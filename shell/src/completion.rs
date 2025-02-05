use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper};
use std::fs;
use std::path::Path;

pub struct ShellCompleter;

impl ShellCompleter {
    pub fn new() -> Self {
        ShellCompleter
    }

    fn get_completions(&self, partial: &str) -> Vec<String> {
        let path = Path::new(partial);
        let (dir, prefix) = if partial.ends_with('/') {
            (path, "")
        } else {
            (path.parent().unwrap_or_else(|| Path::new("")), 
             path.file_name().map_or("", |s| s.to_str().unwrap_or("")))
        };

        let mut completions = Vec::new();

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(name) = entry.file_name().to_str() {
                        if name.starts_with(prefix) {
                            let mut completion = name.to_string();
                            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                                completion.push('/');
                            }
                            completions.push(completion);
                        }
                    }
                }
            }
        }

        completions
    }
}

impl Completer for ShellCompleter {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> rustyline::Result<(usize, Vec<Pair>)> {
        let (start, word) = line[..pos]
            .rfind(char::is_whitespace)
            .map(|i| (i + 1, &line[i + 1..pos]))
            .unwrap_or((0, &line[..pos]));

        let completions = self.get_completions(word)
            .into_iter()
            .map(|s| Pair {
                display: s.clone(),
                replacement: s,
            })
            .collect();

        Ok((start, completions))
    }
}

impl Helper for ShellCompleter {}

impl Hinter for ShellCompleter {
    type Hint = String;
}

impl Highlighter for ShellCompleter {}

impl Validator for ShellCompleter {} 