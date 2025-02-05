use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct Command {
    pub command: String,
    pub args: Vec<String>,
    pub input_redirect: Option<String>,
    pub output_redirect: Option<String>,
    pub append_redirect: Option<String>,
    pub background: bool,
}

#[derive(Debug)]
pub struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error: {}", self.0)
    }
}

impl Error for ParseError {}

pub fn parse_input(input: &str) -> Result<Vec<Command>, ParseError> {
    let mut commands = Vec::new();
    let pipelines: Vec<&str> = input.split('|').map(str::trim).collect();

    for cmd_str in pipelines {
        let mut parts = shellwords::split(cmd_str)
            .map_err(|e| ParseError(e.to_string()))?;
        
        if parts.is_empty() {
            continue;
        }

        let mut cmd = Command {
            command: parts.remove(0),
            args: Vec::new(),
            input_redirect: None,
            output_redirect: None,
            append_redirect: None,
            background: false,
        };

        let mut i = 0;
        while i < parts.len() {
            match parts[i].as_str() {
                "<" => {
                    if i + 1 < parts.len() {
                        cmd.input_redirect = Some(parts[i + 1].clone());
                        i += 2;
                    } else {
                        return Err(ParseError("Missing input redirect file".into()));
                    }
                }
                ">" => {
                    if i + 1 < parts.len() {
                        cmd.output_redirect = Some(parts[i + 1].clone());
                        i += 2;
                    } else {
                        return Err(ParseError("Missing output redirect file".into()));
                    }
                }
                ">>" => {
                    if i + 1 < parts.len() {
                        cmd.append_redirect = Some(parts[i + 1].clone());
                        i += 2;
                    } else {
                        return Err(ParseError("Missing append redirect file".into()));
                    }
                }
                "&" => {
                    cmd.background = true;
                    i += 1;
                }
                _ => {
                    cmd.args.push(parts[i].clone());
                    i += 1;
                }
            }
        }

        commands.push(cmd);
    }

    Ok(commands)
} 