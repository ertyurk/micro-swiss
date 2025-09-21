use crate::tool_module::ToolModule;
use arboard::Clipboard;
use clap::{Arg, ArgMatches, Command};
use serde_json::Value;
use std::error::Error;

pub struct JsonFormatModule;

impl ToolModule for JsonFormatModule {
    fn name(&self) -> &'static str {
        "json-format"
    }

    fn configure_args(&self, cmd: Command) -> Command {
        cmd.arg(
            Arg::new("json-pretty")
                .long("json-pretty")
                .value_name("JSON")
                .help("Pretty print JSON with indentation")
                .long_help("Format JSON with proper indentation and spacing. Result is automatically copied to clipboard.")
        )
        .arg(
            Arg::new("json-minify")
                .long("json-minify")
                .value_name("JSON")
                .help("Minify JSON by removing whitespace")
                .long_help("Compact JSON by removing all unnecessary whitespace. Result is automatically copied to clipboard.")
        )
    }

    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        if let Some(json_str) = matches.get_one::<String>("json-pretty") {
            let formatted = format_json_pretty(json_str)?;
            copy_to_clipboard_and_print(&formatted);
        } else if let Some(json_str) = matches.get_one::<String>("json-minify") {
            let minified = format_json_minify(json_str)?;
            copy_to_clipboard_and_print(&minified);
        }
        Ok(())
    }
}

fn format_json_pretty(json_str: &str) -> Result<String, Box<dyn Error>> {
    let value: Value = serde_json::from_str(json_str)?;
    let pretty = serde_json::to_string_pretty(&value)?;
    Ok(pretty)
}

fn format_json_minify(json_str: &str) -> Result<String, Box<dyn Error>> {
    let value: Value = serde_json::from_str(json_str)?;
    let minified = serde_json::to_string(&value)?;
    Ok(minified)
}

fn copy_to_clipboard_and_print(text: &str) {
    match Clipboard::new() {
        Ok(mut clipboard) => {
            if let Err(e) = clipboard.set_text(text) {
                eprintln!("Warning: Failed to copy to clipboard: {}", e);
                println!("{}", text);
            } else {
                println!("{}\n(copied to clipboard)", text);
            }
        }
        Err(e) => {
            eprintln!("Warning: Failed to access clipboard: {}", e);
            println!("{}", text);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_json_pretty() {
        let json = r#"{"name":"John","age":30,"city":"New York"}"#;
        let result = format_json_pretty(json).unwrap();
        assert!(result.contains("  \"name\": \"John\""));
        assert!(result.contains("  \"age\": 30"));
        assert!(result.contains("  \"city\": \"New York\""));
    }

    #[test]
    fn test_format_json_minify() {
        let json = r#"{
  "name": "John",
  "age": 30,
  "city": "New York"
}"#;
        let result = format_json_minify(json).unwrap();
        assert!(result.contains("\"name\":\"John\""));
        assert!(result.contains("\"age\":30"));
        assert!(result.contains("\"city\":\"New York\""));
        assert!(!result.contains(": "));  // No space after colon
        assert!(!result.contains("\n"));  // No newlines
    }

    #[test]
    fn test_format_json_array() {
        let json = r#"[{"id":1,"name":"Alice"},{"id":2,"name":"Bob"}]"#;
        let pretty = format_json_pretty(json).unwrap();
        assert!(pretty.contains("[\n  {\n    \"id\": 1"));
        
        let minified = format_json_minify(&pretty).unwrap();
        assert_eq!(minified, json);
    }

    #[test]
    fn test_format_json_nested() {
        let json = r#"{"user":{"profile":{"name":"Test","settings":{"theme":"dark"}}}}"#;
        let pretty = format_json_pretty(json).unwrap();
        assert!(pretty.contains("    \"profile\": {"));
        assert!(pretty.contains("      \"settings\": {"));
        assert!(pretty.contains("        \"theme\": \"dark\""));
    }

    #[test]
    fn test_invalid_json() {
        let invalid_json = r#"{"name": "John", "age":}"#;
        assert!(format_json_pretty(invalid_json).is_err());
        assert!(format_json_minify(invalid_json).is_err());
    }

    #[test]
    fn test_empty_objects_and_arrays() {
        assert_eq!(format_json_pretty("{}").unwrap(), "{}");
        assert_eq!(format_json_pretty("[]").unwrap(), "[]");
        assert_eq!(format_json_minify("{}").unwrap(), "{}");
        assert_eq!(format_json_minify("[]").unwrap(), "[]");
    }

    #[test]
    fn test_json_with_special_characters() {
        let json = r#"{"message":"Hello\nWorld","emoji":"🚀","quote":"He said \"Hi\""}"#;
        let pretty = format_json_pretty(json).unwrap();
        let minified = format_json_minify(&pretty).unwrap();
        
        // Check that all original keys and values are preserved
        assert!(minified.contains("\"message\":\"Hello\\nWorld\""));
        assert!(minified.contains("\"emoji\":\"🚀\""));
        assert!(minified.contains("\"quote\":\"He said \\\"Hi\\\"\""));
    }
}