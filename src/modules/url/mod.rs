use crate::error::{MsError, MsResult};
use crate::tool_module::ToolModule;
use crate::util::stdin;
use clap::{Arg, ArgMatches, Command};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub struct UrlModule;

impl ToolModule for UrlModule {
    fn name(&self) -> &'static str {
        "url"
    }

    fn command(&self) -> Command {
        Command::new("url")
            .about("URL encode, decode, or parse")
            .subcommand_required(true)
            .subcommand(
                Command::new("encode")
                    .about("URL encode a string")
                    .arg(Arg::new("text").value_name("TEXT").help("Text to encode")),
            )
            .subcommand(
                Command::new("decode")
                    .about("URL decode a string")
                    .arg(Arg::new("text").value_name("TEXT").help("Text to decode")),
            )
            .subcommand(
                Command::new("parse")
                    .about("Parse URL into structured JSON")
                    .arg(Arg::new("text").value_name("URL").help("URL to parse")),
            )
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        match matches.subcommand() {
            Some(("encode", sub_m)) => {
                let text = stdin::get_text_input(sub_m.get_one::<String>("text"))
                    .ok_or_else(|| MsError::InvalidInput("No text provided".into()))?;
                println!("{}", url_encode(&text));
            }
            Some(("decode", sub_m)) => {
                let text = stdin::get_text_input(sub_m.get_one::<String>("text"))
                    .ok_or_else(|| MsError::InvalidInput("No text provided".into()))?;
                let decoded = url_decode(&text)?;
                println!("{}", decoded);
            }
            Some(("parse", sub_m)) => {
                let text = stdin::get_text_input(sub_m.get_one::<String>("text"))
                    .ok_or_else(|| MsError::InvalidInput("No URL provided".into()))?;
                let parsed = parse_url(&text)?;
                let json = serde_json::to_string_pretty(&parsed)?;
                println!("{}", json);
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}

#[must_use]
pub fn url_encode(input: &str) -> String {
    input
        .bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (b as char).to_string()
            }
            b' ' => "+".to_string(),
            _ => format!("%{:02X}", b),
        })
        .collect()
}

pub fn url_decode(input: &str) -> MsResult<String> {
    let mut result = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '+' => result.push(b' '),
            '%' => {
                let hex1 = chars.next().ok_or_else(|| {
                    MsError::InvalidInput(
                        "Incomplete percent encoding: missing first hex digit".into(),
                    )
                })?;
                let hex2 = chars.next().ok_or_else(|| {
                    MsError::InvalidInput(
                        "Incomplete percent encoding: missing second hex digit".into(),
                    )
                })?;
                let hex_str = format!("{}{}", hex1, hex2);
                let byte = u8::from_str_radix(&hex_str, 16).map_err(|_| {
                    MsError::InvalidInput(format!(
                        "Invalid hex digits in percent encoding: {}",
                        hex_str
                    ))
                })?;
                result.push(byte);
            }
            _ => {
                let mut buffer = [0; 4];
                let bytes = c.encode_utf8(&mut buffer).as_bytes();
                result.extend_from_slice(bytes);
            }
        }
    }

    String::from_utf8(result)
        .map_err(|e| MsError::InvalidInput(format!("Invalid UTF-8 sequence: {}", e)))
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ParsedUrl {
    pub protocol: String,
    pub domain: String,
    pub path: String,
    pub query: HashMap<String, Value>,
}

pub fn parse_url(url: &str) -> MsResult<ParsedUrl> {
    if url.is_empty() {
        return Err(MsError::InvalidInput("URL cannot be empty".into()));
    }

    if let Some(query_string) = url.strip_prefix('?') {
        let query = parse_query_parameters(query_string);
        return Ok(ParsedUrl {
            protocol: String::new(),
            domain: String::new(),
            path: String::new(),
            query,
        });
    }

    let (protocol, rest) = if let Some(pos) = url.find("://") {
        let protocol = url[..pos].to_string();
        let rest = &url[pos + 3..];
        (protocol, rest)
    } else if is_likely_domain(url) {
        ("https".to_string(), url)
    } else {
        return parse_path_only(url);
    };

    let (domain, path_and_query) = if let Some(pos) = rest.find('/') {
        (rest[..pos].to_string(), &rest[pos..])
    } else if let Some(pos) = rest.find('?') {
        (rest[..pos].to_string(), &rest[pos..])
    } else {
        (rest.to_string(), "/")
    };

    let (path, query_string) = if let Some(pos) = path_and_query.find('?') {
        let p = &path_and_query[..pos];
        let path = if p.is_empty() { "/".to_string() } else { p.to_string() };
        (path, &path_and_query[pos + 1..])
    } else {
        (path_and_query.to_string(), "")
    };

    let query = parse_query_parameters(query_string);
    Ok(ParsedUrl {
        protocol,
        domain,
        path,
        query,
    })
}

fn is_likely_domain(input: &str) -> bool {
    if input.starts_with('/') {
        return false;
    }
    if let Some(slash_pos) = input.find('/') {
        let before_slash = &input[..slash_pos];
        before_slash.contains('.') || before_slash.contains(':')
    } else {
        input.contains('.') || input.contains(':')
    }
}

fn parse_path_only(input: &str) -> MsResult<ParsedUrl> {
    let (path, query_string) = if let Some(pos) = input.find('?') {
        (input[..pos].to_string(), &input[pos + 1..])
    } else {
        (input.to_string(), "")
    };

    let normalized_path = if path.is_empty() {
        "/".to_string()
    } else if path.starts_with('/') {
        path
    } else {
        format!("/{}", path)
    };

    let query = parse_query_parameters(query_string);
    Ok(ParsedUrl {
        protocol: String::new(),
        domain: String::new(),
        path: normalized_path,
        query,
    })
}

fn parse_query_parameters(query_string: &str) -> HashMap<String, Value> {
    let mut query = HashMap::new();
    if query_string.is_empty() {
        return query;
    }
    for param in query_string.split('&') {
        if let Some(eq_pos) = param.find('=') {
            let key = &param[..eq_pos];
            let value = &param[eq_pos + 1..];
            let decoded_key = url_decode_simple(key);
            let decoded_value = url_decode_simple(value);

            let json_value = if decoded_value.is_empty() {
                Value::String(decoded_value)
            } else if let Ok(num) = decoded_value.parse::<i64>() {
                Value::Number(serde_json::Number::from(num))
            } else if let Ok(float) = decoded_value.parse::<f64>() {
                Value::Number(
                    serde_json::Number::from_f64(float)
                        .unwrap_or_else(|| serde_json::Number::from(0)),
                )
            } else if decoded_value.to_lowercase() == "true" {
                Value::Bool(true)
            } else if decoded_value.to_lowercase() == "false" {
                Value::Bool(false)
            } else {
                Value::String(decoded_value)
            };
            query.insert(decoded_key, json_value);
        } else {
            let decoded_key = url_decode_simple(param);
            query.insert(decoded_key, Value::String(String::new()));
        }
    }
    query
}

fn url_decode_simple(input: &str) -> String {
    let mut result = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '+' => result.push(b' '),
            '%' => {
                if let (Some(hex1), Some(hex2)) = (chars.next(), chars.next()) {
                    if let Ok(byte) =
                        u8::from_str_radix(&format!("{}{}", hex1, hex2), 16)
                    {
                        result.push(byte);
                    } else {
                        result
                            .extend_from_slice(format!("%{}{}", hex1, hex2).as_bytes());
                    }
                } else {
                    result.push(b'%');
                }
            }
            _ => {
                let mut buffer = [0; 4];
                let bytes = c.encode_utf8(&mut buffer).as_bytes();
                result.extend_from_slice(bytes);
            }
        }
    }

    String::from_utf8(result).unwrap_or_else(|_| input.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_encode() {
        assert_eq!(url_encode("hello world"), "hello+world");
        assert_eq!(url_encode("test@example.com"), "test%40example.com");
    }

    #[test]
    fn test_url_encode_empty_string() {
        assert_eq!(url_encode(""), "");
    }

    #[test]
    fn test_url_encode_unreserved_chars() {
        assert_eq!(
            url_encode(
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_.~"
            ),
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_.~"
        );
    }

    #[test]
    fn test_url_encode_special_characters() {
        assert_eq!(
            url_encode("!@#$%^&*()"),
            "%21%40%23%24%25%5E%26%2A%28%29"
        );
    }

    #[test]
    fn test_url_encode_unicode() {
        assert_eq!(url_encode("café"), "caf%C3%A9");
    }

    #[test]
    fn test_url_decode_basic() {
        assert_eq!(url_decode("hello+world").unwrap(), "hello world");
        assert_eq!(
            url_decode("test%40example.com").unwrap(),
            "test@example.com"
        );
    }

    #[test]
    fn test_url_decode_empty_string() {
        assert_eq!(url_decode("").unwrap(), "");
    }

    #[test]
    fn test_url_decode_unicode() {
        assert_eq!(url_decode("caf%C3%A9").unwrap(), "café");
    }

    #[test]
    fn test_url_decode_invalid_percent_encoding() {
        assert!(url_decode("%").is_err());
        assert!(url_decode("%1").is_err());
        assert!(url_decode("%GG").is_err());
    }

    #[test]
    fn test_url_roundtrip() {
        let original = "Hello World! @#$%^&*()";
        let encoded = url_encode(original);
        let decoded = url_decode(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_parse_simple_url() {
        let result = parse_url("https://example.com/path").unwrap();
        assert_eq!(result.protocol, "https");
        assert_eq!(result.domain, "example.com");
        assert_eq!(result.path, "/path");
        assert!(result.query.is_empty());
    }

    #[test]
    fn test_parse_url_with_query() {
        let result =
            parse_url("https://example.com/api?name=test&limit=100").unwrap();
        assert_eq!(result.protocol, "https");
        assert_eq!(result.domain, "example.com");
        assert_eq!(result.path, "/api");
        assert_eq!(
            result.query.get("name"),
            Some(&Value::String("test".to_string()))
        );
        assert_eq!(
            result.query.get("limit"),
            Some(&Value::Number(serde_json::Number::from(100)))
        );
    }

    #[test]
    fn test_parse_url_domain_only() {
        let result = parse_url("https://example.com").unwrap();
        assert_eq!(result.path, "/");
        assert!(result.query.is_empty());
    }

    #[test]
    fn test_parse_url_with_port() {
        let result =
            parse_url("http://localhost:3000/api?debug=true").unwrap();
        assert_eq!(result.domain, "localhost:3000");
        assert_eq!(result.query.get("debug"), Some(&Value::Bool(true)));
    }

    #[test]
    fn test_parse_url_without_protocol() {
        let result = parse_url("example.com/api?param=value").unwrap();
        assert_eq!(result.protocol, "https");
        assert_eq!(result.domain, "example.com");
    }

    #[test]
    fn test_parse_query_only() {
        let result = parse_url("?fields=name%2Cdescription&limit=100").unwrap();
        assert_eq!(result.protocol, "");
        assert_eq!(result.domain, "");
        assert_eq!(result.path, "");
    }

    #[test]
    fn test_parse_relative_path_with_query() {
        let result = parse_url("vendor/category?limit=100").unwrap();
        assert_eq!(result.protocol, "");
        assert_eq!(result.path, "/vendor/category");
    }

    #[test]
    fn test_parse_invalid_urls() {
        assert!(parse_url("").is_err());
    }

    #[test]
    fn test_domain_vs_path_detection() {
        assert_eq!(parse_url("api.example.com").unwrap().protocol, "https");
        assert_eq!(parse_url("localhost:3000").unwrap().protocol, "https");
        assert_eq!(parse_url("vendor/category").unwrap().protocol, "");
        assert_eq!(parse_url("/api/data").unwrap().protocol, "");
    }
}
