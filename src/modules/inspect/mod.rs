use crate::error::{MsError, MsResult};
use crate::tool_module::ToolModule;
use crate::util::stdin;
use clap::{Arg, ArgMatches, Command};

pub struct InspectModule;

impl ToolModule for InspectModule {
    fn name(&self) -> &'static str {
        "inspect"
    }

    fn command(&self) -> Command {
        Command::new("inspect")
            .about("Inspect text: byte length, char count, lines, words")
            .arg(
                Arg::new("text")
                    .value_name("TEXT")
                    .help("Text to inspect"),
            )
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        let text = stdin::get_text_input(matches.get_one::<String>("text"))
            .ok_or_else(|| MsError::InvalidInput("No text provided".into()))?;

        let bytes = text.len();
        let chars = text.chars().count();
        let lines = if text.is_empty() {
            0
        } else {
            text.lines().count()
        };
        let words = text.split_whitespace().count();

        println!("Bytes:      {}", bytes);
        println!("Characters: {}", chars);
        println!("Lines:      {}", lines);
        println!("Words:      {}", words);

        if bytes != chars {
            println!("\nCodepoints:");
            for (i, c) in text.chars().take(50).enumerate() {
                if !c.is_ascii() {
                    println!(
                        "  [{}] U+{:04X} {}",
                        i,
                        c as u32,
                        c
                    );
                }
            }
            if text.chars().count() > 50 {
                println!("  ... (truncated)");
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_inspect() {
        let text = "hello world";
        assert_eq!(text.len(), 11);
        assert_eq!(text.chars().count(), 11);
        assert_eq!(text.lines().count(), 1);
        assert_eq!(text.split_whitespace().count(), 2);
    }

    #[test]
    fn test_unicode_inspect() {
        let text = "café 🔥";
        assert!(text.len() > text.chars().count());
    }

    #[test]
    fn test_multiline() {
        let text = "line1\nline2\nline3";
        assert_eq!(text.lines().count(), 3);
    }
}
