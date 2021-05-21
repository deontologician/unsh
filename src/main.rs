use anyhow::{Context, Result};

use import_parser::Path;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::{collections::HashMap, io, process::Command};
mod import_parser;
fn main() {
    println!("unsh v1.0");
    for (varname, _val) in std::env::vars() {
        std::env::remove_var(varname);
    }
    let mut unsh = Unsh::new();
    loop {
        if let Err(exit_msg) = unsh.readline_loop() {
            eprintln!("{}", exit_msg);
            break;
        }
    }
}

#[derive(Debug)]
enum Execution<'a> {
    Subproc(Command),
    Pwd,
    Cd,
    Use(Path<'a>),
}

struct Unsh {
    rl: Editor<()>,
    commands: HashMap<String, String>,
}

impl Unsh {
    fn new() -> Self {
        let working_dir = std::env::temp_dir().join(format!("unsh-{}", std::process::id()));
        std::fs::create_dir(&working_dir).expect("Expected to be create the directory");
        std::env::set_current_dir(&working_dir).expect("Couldn't set current working directory");
        std::fs::remove_dir(&working_dir).expect("Couldnt remove directory");
        Self {
            rl: Editor::<()>::new(),
            commands: HashMap::new(),
        }
    }

    fn readline_loop(&mut self) -> Result<()> {
        match self.rl.readline("¥ ") {
            Ok(line) => {
                self.rl.add_history_entry(line.as_str());
                match self.parse_command(&line) {
                    Ok(cmd) => match self.execute_line(cmd) {
                        Err(e) => eprintln!("{}", anyhow::format_err!(e)),
                        _ => (),
                    },
                    Err(e) => {
                        eprintln!("{}", anyhow::format_err!(e));
                    }
                };
                Ok(())
            }
            Err(ReadlineError::Interrupted) => anyhow::bail!("CTRL-C".to_string()),
            Err(ReadlineError::Eof) => anyhow::bail!("CTRL-D".to_string()),
            Err(err) => Err(err).context("Readline failed"),
        }
    }
    fn execute_line(&mut self, cmd: Execution) -> Result<()> {
        match cmd {
            Execution::Pwd => {
                eprintln!("You're nowhere")
            }
            Execution::Cd => {
                eprintln!("You can't move around, you have no location.");
            }
            Execution::Subproc(mut r) => {
                let child = r.spawn();
                match child {
                    Ok(mut c) => match c.wait() {
                        Ok(es) => println!("{}", es),
                        Err(e) => anyhow::bail!("Command failed. {}", e),
                    },
                    Err(e) => match e.kind() {
                        io::ErrorKind::NotFound => {
                            anyhow::bail!(
                                "Couldn't find the program. Did you import it with `use`?"
                            )
                        }
                        _ => {
                            anyhow::bail!("Spawn failed");
                        }
                    },
                }
            }
            Execution::Use(path) => {
                println!("Use {:#?}", path);
            }
        }
        Ok(())
    }

    /// Parse a command. Failures are failures to parse the input line
    fn parse_command<'a>(&mut self, inp: &'a str) -> Result<Execution<'a>> {
        // If it starts with a colon, assume it's a shell commmand and parse it that way
        if inp.len() > 0 && inp.starts_with(':') {
            return Ok(Execution::Use(import_parser::import(inp)?))
        }
        // Otherwise, just do normal shell splitting
        let split_input =
            shlex::split(inp).ok_or_else(|| anyhow::anyhow!("Couldn't split input"))?;
        if split_input.is_empty() {
            return Err(anyhow::anyhow!("😶"));
        }
        match split_input[0].as_str() {
            "pwd" => Ok(Execution::Pwd),
            "cd" => Ok(Execution::Cd),
            program_name => {
                let args = &split_input[1..];
                let mut cmd = Command::new(program_name);
                cmd.args(args);
                cmd.env_clear();
                Ok(Execution::Subproc(cmd))
            }
        }
    }
}
