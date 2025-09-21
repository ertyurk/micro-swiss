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

    fn configure_args(&self, cmd: Command) -> Command {
        cmd.arg(
            Arg::new("run")
                .short('r')
                .long("run")
                .value_name("FILE")
                .help("Run file based on extension")
                .long_help("Execute a file using the appropriate interpreter based on its extension. Supports: .py (uv run), .js (node), .ts (deno), .go (go run), .mojo/🔥 (mojo). Shows execution time and handles exit codes properly.")
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

    #[test]
    fn test_is_supported_file_edge_cases() {
        assert!(!FileRunner::is_supported_file(""));
        assert!(!FileRunner::is_supported_file("."));
        assert!(!FileRunner::is_supported_file(".."));
        assert!(!FileRunner::is_supported_file("file."));
        assert!(!FileRunner::is_supported_file(".hidden"));
    }

    #[test]
    fn test_is_supported_file_case_sensitivity() {
        assert!(!FileRunner::is_supported_file("test.PY"));
        assert!(!FileRunner::is_supported_file("test.JS"));
        assert!(!FileRunner::is_supported_file("test.GO"));
        assert!(!FileRunner::is_supported_file("test.TS"));
    }

    #[test]
    fn test_is_supported_file_multiple_extensions() {
        assert!(!FileRunner::is_supported_file("test.tar.gz"));
        assert!(FileRunner::is_supported_file("test.backup.py"));
        assert!(FileRunner::is_supported_file("config.json.js"));
    }

    #[test]
    fn test_is_supported_file_special_characters() {
        assert!(FileRunner::is_supported_file("test-file.py"));
        assert!(FileRunner::is_supported_file("test_file.js"));
        assert!(FileRunner::is_supported_file("test file.go"));
        assert!(FileRunner::is_supported_file("test@file.ts"));
    }

    #[test]
    fn test_is_supported_file_unicode_emoji() {
        assert!(FileRunner::is_supported_file("test.🔥"));
        assert!(!FileRunner::is_supported_file("🔥.txt"));
    }

    #[test]
    fn test_is_supported_file_path_with_directories() {
        assert!(FileRunner::is_supported_file("/path/to/file.py"));
        assert!(FileRunner::is_supported_file("../relative/path/file.js"));
        assert!(FileRunner::is_supported_file("./file.go"));
    }

    #[test]
    fn test_is_supported_file_long_filename() {
        let long_name = "a".repeat(255);
        assert!(FileRunner::is_supported_file(&format!("{}.py", long_name)));
        assert!(!FileRunner::is_supported_file(&format!("{}.unknown", long_name)));
    }

    #[test]
    fn test_get_supported_extensions_immutability() {
        let extensions1 = FileRunner::get_supported_extensions();
        let extensions2 = FileRunner::get_supported_extensions();
        assert_eq!(extensions1.len(), extensions2.len());
        for ext in extensions1 {
            assert!(extensions2.contains(&ext));
        }
    }

    #[test]
    fn test_is_supported_file_no_extension() {
        assert!(!FileRunner::is_supported_file("makefile"));
        assert!(!FileRunner::is_supported_file("dockerfile"));
        assert!(!FileRunner::is_supported_file("readme"));
    }
}