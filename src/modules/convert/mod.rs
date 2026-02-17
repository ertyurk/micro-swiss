use crate::error::{MsError, MsResult};
use crate::tool_module::ToolModule;
use crate::util::stdin;
use clap::{Arg, ArgMatches, Command};

pub struct ConvertModule;

impl ToolModule for ConvertModule {
    fn name(&self) -> &'static str {
        "convert"
    }

    fn command(&self) -> Command {
        Command::new("convert")
            .about("Convert between JSON, YAML, and TOML")
            .arg(
                Arg::new("format")
                    .required(true)
                    .help("Target format: json, yaml, or toml"),
            )
            .arg(
                Arg::new("input")
                    .value_name("INPUT")
                    .help("Input data (or pipe via stdin)"),
            )
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        let target = matches.get_one::<String>("format").unwrap();
        let input = stdin::get_text_input(matches.get_one::<String>("input"))
            .ok_or_else(|| MsError::InvalidInput("No input provided".into()))?;

        let value = detect_and_parse(&input)?;
        let output = serialize_to(target, &value)?;
        println!("{}", output);
        Ok(())
    }
}

fn detect_and_parse(input: &str) -> MsResult<serde_json::Value> {
    let trimmed = input.trim();

    // Try JSON first
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(trimmed) {
        return Ok(v);
    }

    // Try TOML
    if let Ok(v) = toml::from_str::<toml::Value>(trimmed) {
        let json_str = serde_json::to_string(&v)?;
        return Ok(serde_json::from_str(&json_str)?);
    }

    // Try YAML
    if let Ok(v) = serde_yml::from_str::<serde_yml::Value>(trimmed) {
        let json_str = serde_json::to_string(&v)?;
        return Ok(serde_json::from_str(&json_str)?);
    }

    Err(MsError::InvalidInput(
        "Could not detect input format. Supported: JSON, YAML, TOML".into(),
    ))
}

fn serialize_to(format: &str, value: &serde_json::Value) -> MsResult<String> {
    match format.to_lowercase().as_str() {
        "json" => Ok(serde_json::to_string_pretty(value)?),
        "yaml" | "yml" => {
            serde_yml::to_string(value).map_err(|e| MsError::Other(e.to_string()))
        }
        "toml" => {
            let toml_val: toml::Value = serde_json::from_value(value.clone())
                .map_err(|e| MsError::Other(e.to_string()))?;
            toml::to_string_pretty(&toml_val)
                .map_err(|e| MsError::Other(e.to_string()))
        }
        _ => Err(MsError::InvalidInput(format!(
            "Unsupported target format: {}. Use json, yaml, or toml",
            format
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_to_yaml() {
        let json = r#"{"name": "test", "value": 42}"#;
        let value = detect_and_parse(json).unwrap();
        let yaml = serialize_to("yaml", &value).unwrap();
        assert!(yaml.contains("name: test"));
        assert!(yaml.contains("value: 42"));
    }

    #[test]
    fn test_yaml_to_json() {
        let yaml = "name: test\nvalue: 42";
        let value = detect_and_parse(yaml).unwrap();
        let json = serialize_to("json", &value).unwrap();
        assert!(json.contains("\"name\": \"test\""));
    }

    #[test]
    fn test_invalid_input() {
        // This string can't be parsed as any format meaningfully
        let result = detect_and_parse("}{not valid at all{}{");
        assert!(result.is_err());
    }

    #[test]
    fn test_unsupported_format() {
        let json = r#"{"a": 1}"#;
        let value = detect_and_parse(json).unwrap();
        assert!(serialize_to("xml", &value).is_err());
    }
}
