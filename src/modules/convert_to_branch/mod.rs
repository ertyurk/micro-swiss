use crate::tool_module::ToolModule;
use arboard::Clipboard;
use clap::{Arg, ArgMatches, Command};
use std::error::Error;

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
                .help("Convert string to Git branch-friendly format (auto-copied to clipboard)")
                .long_help("Convert any string to a Git branch-friendly format by converting to lowercase, replacing non-alphanumeric characters with dashes, collapsing multiple dashes, and removing leading/trailing dashes. Perfect for creating branch names from issue titles or feature descriptions. Result is automatically copied to the clipboard."),
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
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
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
        assert_eq!(
            convert_to_branch_name("Feature  Name   Test"),
            "feature-name-test"
        );
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
        assert_eq!(
            convert_to_branch_name("Feature: Fix bug (urgent)!"),
            "feature-fix-bug-urgent"
        );
    }

    #[test]
    fn test_convert_slashes_and_semicolons() {
        assert_eq!(
            convert_to_branch_name("API/endpoints; database/queries"),
            "api-endpoints-database-queries"
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
        assert_eq!(convert_to_branch_name("---"), "");
    }

    #[test]
    fn test_convert_only_spaces() {
        assert_eq!(convert_to_branch_name("   "), "");
        assert_eq!(convert_to_branch_name("\t\n\r"), "");
    }

    #[test]
    fn test_convert_single_word() {
        assert_eq!(convert_to_branch_name("feature"), "feature");
        assert_eq!(convert_to_branch_name("FEATURE"), "feature");
    }

    #[test]
    fn test_convert_numbers() {
        assert_eq!(convert_to_branch_name("Feature 123"), "feature-123");
        assert_eq!(convert_to_branch_name("v1.2.3"), "v1-2-3");
    }

    #[test]
    fn test_convert_underscores() {
        assert_eq!(convert_to_branch_name("feature_name_test"), "feature-name-test");
        assert_eq!(convert_to_branch_name("__important__"), "important");
    }

    #[test]
    fn test_convert_mixed_separators() {
        assert_eq!(convert_to_branch_name("feature__name--test"), "feature-name-test");
        assert_eq!(convert_to_branch_name("a___b---c"), "a-b-c");
    }

    #[test]
    fn test_convert_unicode() {
        assert_eq!(convert_to_branch_name("café"), "café");
        assert_eq!(convert_to_branch_name("naïve approach"), "naïve-approach");
    }

    #[test]
    fn test_convert_very_long_string() {
        let long_input = "a".repeat(500) + " " + &"b".repeat(500);
        let result = convert_to_branch_name(&long_input);
        assert_eq!(result, format!("{}-{}", "a".repeat(500), "b".repeat(500)));
    }

    #[test]
    fn test_convert_extreme_punctuation() {
        assert_eq!(convert_to_branch_name("Fix: (urgent!!!) - handle & process data!!!"), "fix-urgent-handle-process-data");
    }

    #[test]
    fn test_convert_quotes_and_brackets() {
        assert_eq!(convert_to_branch_name("\"Feature\" [urgent] {todo}"), "feature-urgent-todo");
        assert_eq!(convert_to_branch_name("'single' and \"double\" quotes"), "single-and-double-quotes");
    }
}
