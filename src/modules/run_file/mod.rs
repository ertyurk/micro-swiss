use crate::tool_module::ToolModule;
use clap::{Arg, ArgMatches, Command};
use colored::*;
use std::error::Error;
use std::process;
use std::time::Instant;

pub struct RunFileModule;

impl ToolModule for RunFileModule {
    fn name(&self) -> &'static str {
        "run-file"
    }

    fn description(&self) -> &'static str {
        "Run file based on extension"
    }

    fn configure_args(&self, cmd: Command) -> Command {
        cmd.arg(
            Arg::new("run")
                .short('r')
                .long("run")
                .value_name("FILE")
                .help("Run file based on extension")
        )
        .arg(
            Arg::new("args")
                .help("Additional arguments for run command")
                .num_args(0..)
                .last(true)
        )
    }

    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        if let Some(file) = matches.get_one::<String>("run") {
            let args: Vec<String> = matches.get_many::<String>("args").unwrap_or_default().cloned().collect();
            let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
            FileRunner::run(file, &arg_refs);
        }
        Ok(())
    }

    fn handles_subcommand(&self, subcommand: &str) -> bool {
        subcommand == "run"
    }
}

pub struct FileRunner;

impl FileRunner {
    pub fn run(file: &str, args: &[&str]) {
        let start = Instant::now();
        
        let extension = file.split('.').last().unwrap_or("");
        
        let (command, interpreter_args) = match extension {
            "go" => {
                println!("{}", "Golang triggered".blue().bold());
                ("go", vec!["run"])
            }
            "py" => {
                println!("{}", "uv for python triggered".green().bold());
                ("uv", vec!["run"])
            }
            "js" => {
                println!("{}", "Node interpreter triggered".yellow().bold());
                ("node", vec![])
            }
            "ts" => {
                println!("{}", "TypeScript triggered. Running with Deno.".blue().bold());
                ("deno", vec!["run", "--allow-all"])
            }
            "mojo" | "🔥" => {
                println!("{}", "Mojo triggered 🔥".red().bold());
                ("mojo", vec![])
            }
            _ => {
                eprintln!("Unknown file type: {}", extension);
                process::exit(1);
            }
        };
        
        let mut cmd_args = interpreter_args;
        cmd_args.push(file);
        cmd_args.extend(args);
        
        let status = process::Command::new(command)
            .args(&cmd_args)
            .status()
            .expect("Failed to execute command");
        
        let duration = start.elapsed();
        println!("{}", format!("Task duration: {}ms", duration.as_millis()).color("orange"));
        
        if !status.success() {
            process::exit(status.code().unwrap_or(1));
        }
    }

    pub fn get_supported_extensions() -> Vec<&'static str> {
        vec!["go", "py", "js", "ts", "mojo", "🔥"]
    }

    pub fn is_supported_file(file: &str) -> bool {
        let extension = file.split('.').last().unwrap_or("");
        Self::get_supported_extensions().contains(&extension)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_extensions() {
        assert!(FileRunner::is_supported_file("test.py"));
        assert!(FileRunner::is_supported_file("test.js"));
        assert!(FileRunner::is_supported_file("test.go"));
        assert!(FileRunner::is_supported_file("test.ts"));
        assert!(FileRunner::is_supported_file("test.mojo"));
        assert!(!FileRunner::is_supported_file("test.txt"));
        assert!(!FileRunner::is_supported_file("test"));
    }

    #[test]
    fn test_get_supported_extensions() {
        let extensions = FileRunner::get_supported_extensions();
        assert!(extensions.contains(&"py"));
        assert!(extensions.contains(&"js"));
        assert!(extensions.contains(&"go"));
        assert!(extensions.len() >= 5);
    }
}