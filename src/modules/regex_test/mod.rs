use crate::error::MsResult;
use crate::tool_module::ToolModule;
use crate::util::stdin;
use clap::{Arg, ArgMatches, Command};
use regex::Regex;

pub struct RegexTestModule;

impl ToolModule for RegexTestModule {
    fn name(&self) -> &'static str {
        "regex"
    }

    fn command(&self) -> Command {
        Command::new("regex")
            .about("Test regex pattern against text")
            .arg(
                Arg::new("pattern")
                    .required(true)
                    .help("Regular expression pattern"),
            )
            .arg(Arg::new("text").value_name("TEXT").help("Text to test against"))
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        let pattern = matches.get_one::<String>("pattern").unwrap();
        let text = stdin::get_text_input(matches.get_one::<String>("text"))
            .unwrap_or_default();
        test_regex(pattern, &text)?;
        Ok(())
    }
}

fn test_regex(pattern: &str, text: &str) -> MsResult<()> {
    let regex = Regex::new(pattern)?;
    let matches: Vec<_> = regex.find_iter(text).collect();

    if matches.is_empty() {
        println!("No matches found");
    } else {
        println!("Found {} match(es):", matches.len());
        for (i, m) in matches.iter().enumerate() {
            println!(
                "  Match {}: '{}' at position {}-{}",
                i + 1,
                m.as_str(),
                m.start(),
                m.end()
            );
        }

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
        assert!(test_regex("hello", "hello world").is_ok());
    }

    #[test]
    fn test_no_match() {
        assert!(test_regex("xyz", "hello world").is_ok());
    }

    #[test]
    fn test_invalid_regex() {
        assert!(test_regex("[", "hello").is_err());
    }

    #[test]
    fn test_regex_with_groups() {
        assert!(test_regex(r"(\w+)@(\w+\.\w+)", "test@example.com").is_ok());
    }
}
