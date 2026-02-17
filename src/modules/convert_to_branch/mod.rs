use crate::error::{MsError, MsResult};
use crate::tool_module::ToolModule;
use crate::util::{clipboard, stdin};
use clap::{Arg, ArgMatches, Command};

pub struct ConvertToBranchModule;

impl ToolModule for ConvertToBranchModule {
    fn name(&self) -> &'static str {
        "branch"
    }

    fn command(&self) -> Command {
        Command::new("branch")
            .about("Convert text to Git branch-friendly format")
            .arg(Arg::new("text").value_name("TEXT").help("Text to convert"))
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        let text = stdin::get_text_input(matches.get_one::<String>("text"))
            .ok_or_else(|| MsError::InvalidInput("No text provided".into()))?;
        let branch_name = convert_to_branch_name(&text);
        clipboard::copy_and_print(&branch_name);
        Ok(())
    }
}

#[must_use]
pub fn convert_to_branch_name(input: &str) -> String {
    let result: String = input
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect();

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
        assert_eq!(
            convert_to_branch_name("Feature  Name   Test"),
            "feature-name-test"
        );
    }

    #[test]
    fn test_convert_special_characters() {
        assert_eq!(
            convert_to_branch_name("Feature: Fix bug (urgent)!"),
            "feature-fix-bug-urgent"
        );
    }

    #[test]
    fn test_leading_trailing_dashes() {
        assert_eq!(
            convert_to_branch_name("!!! Important Feature !!!"),
            "important-feature"
        );
    }

    #[test]
    fn test_convert_empty_string() {
        assert_eq!(convert_to_branch_name(""), "");
    }

    #[test]
    fn test_convert_only_special_chars() {
        assert_eq!(convert_to_branch_name("!!!@@@###"), "");
    }

    #[test]
    fn test_convert_numbers() {
        assert_eq!(convert_to_branch_name("Feature 123"), "feature-123");
        assert_eq!(convert_to_branch_name("v1.2.3"), "v1-2-3");
    }

    #[test]
    fn test_convert_unicode() {
        assert_eq!(convert_to_branch_name("café"), "café");
    }
}
