use crate::tool_module::ToolModule;
use clap::{Arg, ArgMatches, Command};
use std::error::Error;
use std::io::{self, Read};

pub struct FlattenTextModule;

impl ToolModule for FlattenTextModule {
    fn name(&self) -> &'static str {
        "flatten-text"
    }

    fn configure_args(&self, cmd: Command) -> Command {
        cmd.arg(
            Arg::new("flatten")
                .short('f')
                .long("flatten")
                .value_name("TEXT")
                .help("Remove newlines from text (or read from stdin)")
                .num_args(0..=1)
        )
    }

    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        if matches.contains_id("flatten") {
            if let Some(text) = matches.get_one::<String>("flatten") {
                println!("{}", flatten_text(text));
            } else {
                match flatten_from_stdin() {
                    Ok(result) => println!("{}", result),
                    Err(e) => return Err(Box::new(e)),
                }
            }
        }
        Ok(())
    }
}

pub fn flatten_text(input: &str) -> String {
    input.replace('\n', "")
}

pub fn flatten_from_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(flatten_text(&buffer))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten_simple() {
        assert_eq!(flatten_text("Hello\nWorld"), "HelloWorld");
    }

    #[test]
    fn test_flatten_multiple_lines() {
        assert_eq!(flatten_text("Line 1\nLine 2\nLine 3"), "Line 1Line 2Line 3");
    }

    #[test]
    fn test_flatten_no_newlines() {
        assert_eq!(flatten_text("NoNewlines"), "NoNewlines");
    }

    #[test]
    fn test_flatten_empty() {
        assert_eq!(flatten_text(""), "");
    }
}