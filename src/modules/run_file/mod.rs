use crate::error::{MsError, MsResult};
use crate::tool_module::ToolModule;
use clap::{Arg, ArgMatches, Command};
use colored::*;
use std::time::Instant;

pub struct RunFileModule;

impl ToolModule for RunFileModule {
    fn name(&self) -> &'static str {
        "run"
    }

    fn command(&self) -> Command {
        Command::new("run")
            .about("Run file based on extension")
            .long_about(
                "Execute a file using the appropriate interpreter based on its extension.\n\
                 Supports: .py (uv run), .js (node), .ts (deno), .go (go run), .mojo/🔥 (mojo)",
            )
            .arg(
                Arg::new("file")
                    .value_name("FILE")
                    .required(true)
                    .help("File to execute"),
            )
            .arg(
                Arg::new("args")
                    .help("Additional arguments")
                    .num_args(0..)
                    .last(true),
            )
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        let file = matches.get_one::<String>("file").unwrap();
        let args: Vec<String> = matches
            .get_many::<String>("args")
            .unwrap_or_default()
            .cloned()
            .collect();
        let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        FileRunner::run(file, &arg_refs)
    }
}

pub struct FileRunner;

impl FileRunner {
    pub fn run(file: &str, args: &[&str]) -> MsResult<()> {
        let start = Instant::now();
        let extension = file.rsplit('.').next().unwrap_or("");

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
                println!(
                    "{}",
                    "TypeScript triggered. Running with Deno.".blue().bold()
                );
                ("deno", vec!["run", "--allow-all"])
            }
            "mojo" | "🔥" => {
                println!("{}", "Mojo triggered 🔥".red().bold());
                ("mojo", vec![])
            }
            _ => {
                return Err(MsError::InvalidInput(format!(
                    "Unknown file type: {}",
                    extension
                )));
            }
        };

        let mut cmd_args: Vec<&str> = interpreter_args;
        cmd_args.push(file);
        cmd_args.extend(args);

        let status = std::process::Command::new(command)
            .args(&cmd_args)
            .status()
            .map_err(|e| MsError::Other(format!("Failed to execute command: {}", e)))?;

        let duration = start.elapsed();
        println!(
            "{}",
            format!("Task duration: {}ms", duration.as_millis()).color("orange")
        );

        if !status.success() {
            return Err(MsError::Other(format!(
                "Command exited with code {}",
                status.code().unwrap_or(1)
            )));
        }

        Ok(())
    }

    pub fn get_supported_extensions() -> Vec<&'static str> {
        vec!["go", "py", "js", "ts", "mojo", "🔥"]
    }

    pub fn is_supported_file(file: &str) -> bool {
        let extension = file.rsplit('.').next().unwrap_or("");
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
    fn test_is_supported_file_edge_cases() {
        assert!(!FileRunner::is_supported_file(""));
        assert!(!FileRunner::is_supported_file("."));
        assert!(!FileRunner::is_supported_file(".."));
    }

    #[test]
    fn test_is_supported_file_case_sensitivity() {
        assert!(!FileRunner::is_supported_file("test.PY"));
        assert!(!FileRunner::is_supported_file("test.JS"));
    }

    #[test]
    fn test_is_supported_file_unicode_emoji() {
        assert!(FileRunner::is_supported_file("test.🔥"));
    }
}
