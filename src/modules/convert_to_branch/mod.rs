use crate::tool_module::ToolModule;
use clap::{Arg, ArgMatches, Command};
use std::error::Error;
use arboard::Clipboard;

pub struct ConvertToBranchModule;

impl ToolModule for ConvertToBranchModule {
    fn name(&self) -> &'static str {
        "convert-to-branch"
    }

    fn configure_args(&self, cmd: Command) -> Command {
        cmd.arg(
            Arg::new("generate-branch")
                .short('g')
                .long("generate-branch")
                .value_name("STRING")
                .help("Convert string to branch-friendly format")
        )
    }

    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        if let Some(text) = matches.get_one::<String>("generate-branch") {
            let branch_name = convert_to_branch_name(text);
            
            // Copy to clipboard
            match Clipboard::new() {
                Ok(mut clipboard) => {
                    if let Err(e) = clipboard.set_text(&branch_name) {
                        eprintln!("Warning: Failed to copy to clipboard: {}", e);
                    } else {
                        println!("{} (copied to clipboard)", branch_name);
                        return Ok(());
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Failed to access clipboard: {}", e);
                }
            }
            
            // Fallback: just print if clipboard fails
            println!("{}", branch_name);
        }
        Ok(())
    }
}

pub fn convert_to_branch_name(input: &str) -> String {
    let result: String = input
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else {
                '-'
            }
        })
        .collect();
    
    // Replace multiple consecutive dashes with single dash
    let mut collapsed = String::new();
    let mut prev_was_dash = false;
    
    for c in result.chars() {
        if c == '-' {
            if !prev_was_dash {
                collapsed.push(c);
                prev_was_dash = true;
            }
        } else {
            collapsed.push(c);
            prev_was_dash = false;
        }
    }
    
    // Remove leading/trailing dashes
    collapsed.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_basic() {
        assert_eq!(convert_to_branch_name("Hello World"), "hello-world");
    }

    #[test]
    fn test_convert_multiple_spaces() {
        assert_eq!(convert_to_branch_name("Feature  Name   Test"), "feature-name-test");
    }

    #[test]
    fn test_convert_mixed_case() {
        assert_eq!(convert_to_branch_name("CamelCase Test"), "camelcase-test");
    }

    #[test]
    fn test_convert_special_characters() {
        assert_eq!(convert_to_branch_name("Product-level modifier limits are ignored; app uses Modifier Group min/max instead of Product override"), 
                   "product-level-modifier-limits-are-ignored-app-uses-modifier-group-min-max-instead-of-product-override");
    }

    #[test]
    fn test_convert_punctuation() {
        assert_eq!(convert_to_branch_name("Feature: Fix bug (urgent)!"), "feature-fix-bug-urgent");
    }

    #[test]
    fn test_convert_slashes_and_semicolons() {
        assert_eq!(convert_to_branch_name("API/endpoints; database/queries"), "api-endpoints-database-queries");
    }

    #[test]
    fn test_leading_trailing_dashes() {
        assert_eq!(convert_to_branch_name("!!! Important Feature !!!"), "important-feature");
    }
}