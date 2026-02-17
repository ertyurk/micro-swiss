use crate::error::{MsError, MsResult};
use crate::tool_module::ToolModule;
use crate::util::stdin;
use clap::{Arg, ArgMatches, Command};

pub struct FlattenTextModule;

impl ToolModule for FlattenTextModule {
    fn name(&self) -> &'static str {
        "flatten"
    }

    fn command(&self) -> Command {
        Command::new("flatten")
            .about("Remove newlines from text")
            .arg(Arg::new("text").value_name("TEXT").help("Text to flatten"))
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        let text = stdin::get_text_input(matches.get_one::<String>("text"))
            .ok_or_else(|| MsError::InvalidInput("No text provided".into()))?;
        println!("{}", flatten_text(&text));
        Ok(())
    }
}

#[must_use]
pub fn flatten_text(input: &str) -> String {
    input.replace('\n', "")
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
        assert_eq!(
            flatten_text("Line 1\nLine 2\nLine 3"),
            "Line 1Line 2Line 3"
        );
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
    }

    #[test]
    fn test_flatten_tabs_preserved() {
        assert_eq!(
            flatten_text("Hello\t\tWorld   Test"),
            "Hello\t\tWorld   Test"
        );
    }

    #[test]
    fn test_flatten_unicode() {
        assert_eq!(flatten_text("🔥\nfire\n🌊\nwater"), "🔥fire🌊water");
    }

    #[test]
    fn test_flatten_long_text() {
        let long_text = "a".repeat(1000) + "\n" + &"b".repeat(1000);
        let expected = "a".repeat(1000) + &"b".repeat(1000);
        assert_eq!(flatten_text(&long_text), expected);
    }
}
