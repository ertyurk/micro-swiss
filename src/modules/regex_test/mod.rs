use crate::tool_module::ToolModule;
use clap::{Arg, ArgMatches, Command};
use regex::Regex;
use std::error::Error;

pub struct RegexTestModule;

impl ToolModule for RegexTestModule {
    fn name(&self) -> &'static str {
        "regex-test"
    }

    fn configure_args(&self, cmd: Command) -> Command {
        cmd.arg(
            Arg::new("regex-test")
                .long("regex-test")
                .value_names(["PATTERN", "TEXT"])
                .num_args(2)
                .help("Test regex pattern against text")
                .long_help("Test a regular expression pattern against the given text and show matches with positions.")
        )
    }

    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        if let Some(values) = matches.get_many::<String>("regex-test") {
            let values: Vec<&String> = values.collect();
            if values.len() == 2 {
                let pattern = values[0];
                let text = values[1];
                test_regex(pattern, text)?;
            }
        }
        Ok(())
    }
}

fn test_regex(pattern: &str, text: &str) -> Result<(), Box<dyn Error>> {
    let regex = Regex::new(pattern)?;
    let matches: Vec<_> = regex.find_iter(text).collect();
    
    if matches.is_empty() {
        println!("No matches found");
    } else {
        println!("Found {} match(es):", matches.len());
        for (i, m) in matches.iter().enumerate() {
            println!("  Match {}: '{}' at position {}-{}", 
                i + 1, m.as_str(), m.start(), m.end());
        }
        
        // Show captures if any
        if let Some(caps) = regex.captures(text) {
            if caps.len() > 1 {
                println!("\nCapture groups:");
                for i in 1..caps.len() {
                    if let Some(group) = caps.get(i) {
                        println!("  Group {}: '{}'", i, group.as_str());
                    }
                }
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_match() {
        let result = test_regex("hello", "hello world");
        assert!(result.is_ok());
    }

    #[test]
    fn test_no_match() {
        let result = test_regex("xyz", "hello world");
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_regex() {
        let result = test_regex("[", "hello");
        assert!(result.is_err());
    }

    #[test]
    fn test_regex_with_groups() {
        let result = test_regex(r"(\w+)@(\w+\.\w+)", "test@example.com");
        assert!(result.is_ok());
    }
}