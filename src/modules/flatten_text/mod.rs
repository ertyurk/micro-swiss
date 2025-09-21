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
                .long("flatten")
                .value_name("TEXT")
                .help("Remove newlines from text (or read from stdin)")
                .long_help("Remove all newline characters from text, useful for converting multi-line text to single line format. If no text is provided as an argument, reads from stdin. Preserves all other whitespace characters (spaces, tabs).")
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

    #[test]
    fn test_flatten_only_newlines() {
        assert_eq!(flatten_text("\n\n\n"), "");
        assert_eq!(flatten_text("\n"), "");
    }

    #[test]
    fn test_flatten_mixed_line_endings() {
        assert_eq!(
            flatten_text("line1\nline2\r\nline3\rline4"),
            "line1line2\rline3\rline4"
        );
    }

    #[test]
    fn test_flatten_leading_trailing_newlines() {
        assert_eq!(flatten_text("\nHello World\n"), "Hello World");
        assert_eq!(flatten_text("\n\n\nContent\n\n\n"), "Content");
    }

    #[test]
    fn test_flatten_consecutive_newlines() {
        assert_eq!(flatten_text("Line1\n\n\nLine2"), "Line1Line2");
        assert_eq!(flatten_text("A\n\n\n\n\nB"), "AB");
    }

    #[test]
    fn test_flatten_tabs_and_spaces_preserved() {
        assert_eq!(
            flatten_text("Hello\t\tWorld   Test"),
            "Hello\t\tWorld   Test"
        );
        assert_eq!(flatten_text("  Indented\n  Text  "), "  Indented  Text  ");
    }

    #[test]
    fn test_flatten_unicode_with_newlines() {
        assert_eq!(flatten_text("🔥\nfire\n🌊\nwater"), "🔥fire🌊water");
        assert_eq!(flatten_text("café\nnaïve"), "cafénaïve");
    }

    #[test]
    fn test_flatten_very_long_text() {
        let long_text = "a".repeat(1000) + "\n" + &"b".repeat(1000);
        let expected = "a".repeat(1000) + &"b".repeat(1000);
        assert_eq!(flatten_text(&long_text), expected);
    }

    #[test]
    fn test_flatten_single_char() {
        assert_eq!(flatten_text("a"), "a");
        assert_eq!(flatten_text("🔥"), "🔥");
    }

    #[test]
    fn test_flatten_special_characters() {
        assert_eq!(flatten_text("!@#$%^&*()\n{}[]|\\"), "!@#$%^&*(){}[]|\\");
    }
}
