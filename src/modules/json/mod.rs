use crate::error::{MsError, MsResult};
use crate::tool_module::ToolModule;
use crate::util::{clipboard, stdin};
use clap::{Arg, ArgMatches, Command};
use serde_json::Value;

pub struct JsonModule;

impl ToolModule for JsonModule {
    fn name(&self) -> &'static str {
        "json"
    }

    fn command(&self) -> Command {
        Command::new("json")
            .about("Pretty print or minify JSON")
            .subcommand_required(true)
            .subcommand(
                Command::new("pretty")
                    .about("Pretty print JSON")
                    .arg(Arg::new("text").value_name("JSON").help("JSON to format")),
            )
            .subcommand(
                Command::new("minify")
                    .about("Minify JSON")
                    .arg(Arg::new("text").value_name("JSON").help("JSON to minify")),
            )
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        match matches.subcommand() {
            Some(("pretty", sub_m)) => {
                let text = stdin::get_text_input(sub_m.get_one::<String>("text"))
                    .ok_or_else(|| {
                        MsError::InvalidInput("No JSON provided".into())
                    })?;
                let formatted = format_json_pretty(&text)?;
                clipboard::copy_and_print_block(&formatted);
            }
            Some(("minify", sub_m)) => {
                let text = stdin::get_text_input(sub_m.get_one::<String>("text"))
                    .ok_or_else(|| {
                        MsError::InvalidInput("No JSON provided".into())
                    })?;
                let minified = format_json_minify(&text)?;
                clipboard::copy_and_print(&minified);
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}

fn format_json_pretty(json_str: &str) -> MsResult<String> {
    let value: Value = serde_json::from_str(json_str)?;
    Ok(serde_json::to_string_pretty(&value)?)
}

fn format_json_minify(json_str: &str) -> MsResult<String> {
    let value: Value = serde_json::from_str(json_str)?;
    Ok(serde_json::to_string(&value)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_json_pretty() {
        let json = r#"{"name":"John","age":30}"#;
        let result = format_json_pretty(json).unwrap();
        assert!(result.contains("  \"name\": \"John\""));
        assert!(result.contains("  \"age\": 30"));
    }

    #[test]
    fn test_format_json_minify() {
        let json = "{\n  \"name\": \"John\",\n  \"age\": 30\n}";
        let result = format_json_minify(json).unwrap();
        assert!(result.contains("\"name\":\"John\""));
        assert!(!result.contains("\n"));
    }

    #[test]
    fn test_invalid_json() {
        assert!(format_json_pretty("{invalid}").is_err());
        assert!(format_json_minify("{invalid}").is_err());
    }

    #[test]
    fn test_empty_objects_and_arrays() {
        assert_eq!(format_json_pretty("{}").unwrap(), "{}");
        assert_eq!(format_json_pretty("[]").unwrap(), "[]");
        assert_eq!(format_json_minify("{}").unwrap(), "{}");
        assert_eq!(format_json_minify("[]").unwrap(), "[]");
    }

    #[test]
    fn test_json_roundtrip() {
        let json = r#"{"name":"John","age":30}"#;
        let pretty = format_json_pretty(json).unwrap();
        let minified = format_json_minify(&pretty).unwrap();
        assert!(minified.contains("\"name\":\"John\""));
    }
}
