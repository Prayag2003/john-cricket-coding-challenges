use std::io::{self, Write};
use log::{info, error};
use rustyline::Editor;
use rustyline::error::ReadlineError;
use crate::{
    commands::execute_command,
    history::History,
    signals::setup_signal_handlers,
    completion::ShellCompleter,
    environment::Environment,
    alias::AliasManager,
    parser::parse_input
};

pub struct Shell {
    editor: Editor<ShellCompleter>,
    history: History,
    environment: Environment,
    alias_manager: AliasManager,
    running: bool,
}

impl Shell {
    pub fn new() -> io::Result<Self> {
        setup_signal_handlers();
        
        let mut editor = Editor::<ShellCompleter>::new();
        let completer = ShellCompleter::new();
        editor.set_helper(Some(completer));
        
        let mut shell = Shell {
            editor,
            history: History::new(),
            environment: Environment::new(),
            alias_manager: AliasManager::new(),
            running: true,
        };
        
        shell.environment.load()?;
        shell.alias_manager.load()?;
        shell.history.load()?;
        
        Ok(shell)
    }

    pub fn run(&mut self) -> io::Result<()> {
        loop {
            print!("> ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input.is_empty() {
                continue;
            }

            self.history.add(input.to_string());

            match input {
                "history" => {
                    self.history.show()?;
                    continue;
                }
                "exit" => break,
                _ => (),
            }

            match parse_input(input) {
                Ok(commands) => {
                    for cmd in commands {
                        if let Err(e) = execute_command(&cmd, &self.environment) {
                            eprintln!("Error executing command: {}", e);
                        }
                    }
                }
                Err(e) => eprintln!("Error parsing command: {}", e),
            }
        }

        // Save state before exit
        self.environment.save()?;
        self.alias_manager.save()?;
        self.history.save()?;

        Ok(())
    }
} 