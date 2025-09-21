use crate::tool_module::ToolModule;
use clap::{Arg, ArgMatches, Command};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

pub struct UrlParseModule;

impl ToolModule for UrlParseModule {
    fn name(&self) -> &'static str {
        "url-parse"
    }

    fn configure_args(&self, cmd: Command) -> Command {
        cmd.arg(
            Arg::new("parse-url")
                .long("parse-url")
                .value_name("URL")
                .help("Parse URL and output structured JSON with components")
                .long_help("Parse a URL into its components (protocol, domain, path, query parameters) and output as prettified JSON. Query parameters are parsed into key-value pairs for easy access.")
        )
    }

    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        if let Some(url) = matches.get_one::<String>("parse-url") {
            match parse_url(url) {
                Ok(parsed) => {
                    let json = serde_json::to_string_pretty(&parsed)?;
                    println!("{}", json);
                }
                Err(e) => return Err(e.into()),
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ParsedUrl {
    pub protocol: String,
    pub domain: String,
    pub path: String,
    pub query: HashMap<String, Value>,
}

pub fn parse_url(url: &str) -> Result<ParsedUrl, String> {
    // Basic URL validation
    if url.is_empty() {
        return Err("URL cannot be empty".to_string());
    }

    // Special case: query-only input (starts with ?)
    if url.starts_with('?') {
        let query_string = &url[1..]; // Remove the leading '?'
        let query = parse_query_parameters(query_string);
        
        return Ok(ParsedUrl {
            protocol: "".to_string(),
            domain: "".to_string(),
            path: "".to_string(),
            query,
        });
    }

    // Parse protocol - be smart about domains vs paths
    let (protocol, rest) = if let Some(pos) = url.find("://") {
        let protocol = url[..pos].to_string();
        let rest = &url[pos + 3..];
        (protocol, rest)
    } else {
        // No protocol specified - check if this looks like a domain or a path
        if is_likely_domain(url) {
            // Looks like a domain, default to https
            ("https".to_string(), url)
        } else {
            // Looks like a relative path, treat as path-only
            return parse_path_only(url);
        }
    };

    // Parse domain and path+query
    let (domain, path_and_query) = if let Some(pos) = rest.find('/') {
        let domain = rest[..pos].to_string();
        let path_and_query = &rest[pos..];
        (domain, path_and_query)
    } else {
        // URL with domain only, no path
        (rest.to_string(), "/")
    };

    // Parse path and query
    let (path, query_string) = if let Some(pos) = path_and_query.find('?') {
        let path = path_and_query[..pos].to_string();
        let query_string = &path_and_query[pos + 1..];
        (path, query_string)
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
    // Consider it a domain if:
    // 1. Contains a dot (like api.example.com, localhost.local)
    // 2. Contains a colon for port (like localhost:3000)
    // 3. Does NOT start with a path separator
    
    if input.starts_with('/') {
        return false; // Definitely a path
    }
    
    // Check if it contains domain indicators BEFORE any path part
    if let Some(slash_pos) = input.find('/') {
        let before_slash = &input[..slash_pos];
        // Only treat as domain if the part before slash has . or :
        before_slash.contains('.') || before_slash.contains(':')
    } else {
        // No slash - only treat as domain if it has . or :
        input.contains('.') || input.contains(':')
    }
}

fn parse_path_only(input: &str) -> Result<ParsedUrl, String> {
    // Parse as path + query, no protocol or domain
    let (path, query_string) = if let Some(pos) = input.find('?') {
        let path = input[..pos].to_string();
        let query_string = &input[pos + 1..];
        (path, query_string)
    } else {
        (input.to_string(), "")
    };
    
    // Ensure path starts with / if it doesn't already
    let normalized_path = if path.is_empty() {
        "/".to_string()
    } else if path.starts_with('/') {
        path
    } else {
        format!("/{}", path)
    };
    
    let query = parse_query_parameters(query_string);
    
    Ok(ParsedUrl {
        protocol: "".to_string(),
        domain: "".to_string(),
        path: normalized_path,
        query,
    })
}

fn parse_query_parameters(query_string: &str) -> HashMap<String, Value> {
    let mut query = HashMap::new();
    if !query_string.is_empty() {
        for param in query_string.split('&') {
            if let Some(eq_pos) = param.find('=') {
                let key = &param[..eq_pos];
                let value = &param[eq_pos + 1..];

                // URL decode the key and value
                let decoded_key = url_decode_simple(key);
                let decoded_value = url_decode_simple(value);

                // Try to parse value as different types
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
                // Parameter without value
                let decoded_key = url_decode_simple(param);
                query.insert(decoded_key, Value::String("".to_string()));
            }
        }
    }
    query
}

// Simple URL decoding function to avoid circular dependencies
fn url_decode_simple(input: &str) -> String {
    let mut result = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '+' => result.push(b' '),
            '%' => {
                if let (Some(hex1), Some(hex2)) = (chars.next(), chars.next()) {
                    if let Ok(byte) = u8::from_str_radix(&format!("{}{}", hex1, hex2), 16) {
                        result.push(byte);
                    } else {
                        // Invalid hex, keep original characters
                        result.extend_from_slice(format!("%{}{}", hex1, hex2).as_bytes());
                    }
                } else {
                    // Incomplete percent encoding, keep as-is
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
    fn test_parse_simple_url() {
        let url = "https://example.com/path";
        let result = parse_url(url).unwrap();

        assert_eq!(result.protocol, "https");
        assert_eq!(result.domain, "example.com");
        assert_eq!(result.path, "/path");
        assert!(result.query.is_empty());
    }

    #[test]
    fn test_parse_url_with_query() {
        let url = "https://example.com/api?name=test&limit=100";
        let result = parse_url(url).unwrap();

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
    fn test_parse_complex_url() {
        let url = "https://medusa-beta.omwnow.com/vendor/category?fields=name%2Cdescription%2Cis_active&q=karak&limit=100";
        let result = parse_url(url).unwrap();

        assert_eq!(result.protocol, "https");
        assert_eq!(result.domain, "medusa-beta.omwnow.com");
        assert_eq!(result.path, "/vendor/category");
        assert_eq!(
            result.query.get("fields"),
            Some(&Value::String("name,description,is_active".to_string()))
        );
        assert_eq!(
            result.query.get("q"),
            Some(&Value::String("karak".to_string()))
        );
        assert_eq!(
            result.query.get("limit"),
            Some(&Value::Number(serde_json::Number::from(100)))
        );
    }

    #[test]
    fn test_parse_url_domain_only() {
        let url = "https://example.com";
        let result = parse_url(url).unwrap();

        assert_eq!(result.protocol, "https");
        assert_eq!(result.domain, "example.com");
        assert_eq!(result.path, "/");
        assert!(result.query.is_empty());
    }

    #[test]
    fn test_parse_url_with_port() {
        let url = "http://localhost:3000/api?debug=true";
        let result = parse_url(url).unwrap();

        assert_eq!(result.protocol, "http");
        assert_eq!(result.domain, "localhost:3000");
        assert_eq!(result.path, "/api");
        assert_eq!(result.query.get("debug"), Some(&Value::Bool(true)));
    }

    #[test]
    fn test_parse_url_with_boolean_params() {
        let url = "https://api.example.com/data?active=true&disabled=false&count=42";
        let result = parse_url(url).unwrap();

        assert_eq!(result.query.get("active"), Some(&Value::Bool(true)));
        assert_eq!(result.query.get("disabled"), Some(&Value::Bool(false)));
        assert_eq!(
            result.query.get("count"),
            Some(&Value::Number(serde_json::Number::from(42)))
        );
    }

    #[test]
    fn test_parse_url_with_empty_params() {
        let url = "https://example.com/search?q=&category=books";
        let result = parse_url(url).unwrap();

        assert_eq!(result.query.get("q"), Some(&Value::String("".to_string())));
        assert_eq!(
            result.query.get("category"),
            Some(&Value::String("books".to_string()))
        );
    }

    #[test]
    fn test_parse_url_with_no_value_params() {
        let url = "https://example.com/api?debug&verbose";
        let result = parse_url(url).unwrap();

        assert_eq!(
            result.query.get("debug"),
            Some(&Value::String("".to_string()))
        );
        assert_eq!(
            result.query.get("verbose"),
            Some(&Value::String("".to_string()))
        );
    }

    #[test]
    fn test_parse_invalid_urls() {
        assert!(parse_url("").is_err());
        // Note: Most strings are now valid as domains with default https protocol
        // Only truly empty strings are invalid
    }

    #[test]
    fn test_parse_url_with_fragment() {
        // Note: fragments (#) are typically handled client-side, but let's test
        let url = "https://example.com/page?param=value#section";
        let result = parse_url(url).unwrap();

        assert_eq!(result.protocol, "https");
        assert_eq!(result.domain, "example.com");
        assert_eq!(result.path, "/page");
        // Fragment should be treated as part of query for simplicity
    }

    #[test]
    fn test_query_type_inference() {
        let url = "https://api.com/test?string=hello&number=123&float=45.67&bool_true=true&bool_false=false&empty=";
        let result = parse_url(url).unwrap();

        assert_eq!(
            result.query.get("string"),
            Some(&Value::String("hello".to_string()))
        );
        assert_eq!(
            result.query.get("number"),
            Some(&Value::Number(serde_json::Number::from(123)))
        );
        assert!(matches!(result.query.get("float"), Some(Value::Number(_))));
        assert_eq!(result.query.get("bool_true"), Some(&Value::Bool(true)));
        assert_eq!(result.query.get("bool_false"), Some(&Value::Bool(false)));
        assert_eq!(
            result.query.get("empty"),
            Some(&Value::String("".to_string()))
        );
    }

    #[test]
    fn test_parse_url_without_protocol() {
        let url = "example.com/api?param=value";
        let result = parse_url(url).unwrap();

        assert_eq!(result.protocol, "https");
        assert_eq!(result.domain, "example.com");
        assert_eq!(result.path, "/api");
        assert_eq!(result.query.get("param"), Some(&Value::String("value".to_string())));
    }

    #[test]
    fn test_parse_url_domain_only_without_protocol() {
        let url = "api.example.com";
        let result = parse_url(url).unwrap();

        assert_eq!(result.protocol, "https");
        assert_eq!(result.domain, "api.example.com");
        assert_eq!(result.path, "/");
        assert!(result.query.is_empty());
    }

    #[test]
    fn test_parse_query_only() {
        let query = "?fields=name%2Cdescription&limit=100&active=true";
        let result = parse_url(query).unwrap();

        assert_eq!(result.protocol, "");
        assert_eq!(result.domain, "");
        assert_eq!(result.path, "");
        assert_eq!(result.query.get("fields"), Some(&Value::String("name,description".to_string())));
        assert_eq!(result.query.get("limit"), Some(&Value::Number(serde_json::Number::from(100))));
        assert_eq!(result.query.get("active"), Some(&Value::Bool(true)));
    }

    #[test]
    fn test_parse_query_only_complex() {
        let query = "?q=karak&sort=name&page=1&debug&empty=";
        let result = parse_url(query).unwrap();

        assert_eq!(result.protocol, "");
        assert_eq!(result.domain, "");
        assert_eq!(result.path, "");
        assert_eq!(result.query.get("q"), Some(&Value::String("karak".to_string())));
        assert_eq!(result.query.get("sort"), Some(&Value::String("name".to_string())));
        assert_eq!(result.query.get("page"), Some(&Value::Number(serde_json::Number::from(1))));
        assert_eq!(result.query.get("debug"), Some(&Value::String("".to_string())));
        assert_eq!(result.query.get("empty"), Some(&Value::String("".to_string())));
    }

    #[test]
    fn test_parse_query_only_empty() {
        let query = "?";
        let result = parse_url(query).unwrap();

        assert_eq!(result.protocol, "");
        assert_eq!(result.domain, "");
        assert_eq!(result.path, "");
        assert!(result.query.is_empty());
    }

    #[test]
    fn test_parse_relative_path_with_query() {
        let url = "vendor/category?fields=name%2Cdescription&limit=100";
        let result = parse_url(url).unwrap();

        assert_eq!(result.protocol, "");
        assert_eq!(result.domain, "");
        assert_eq!(result.path, "/vendor/category");
        assert_eq!(result.query.get("fields"), Some(&Value::String("name,description".to_string())));
        assert_eq!(result.query.get("limit"), Some(&Value::Number(serde_json::Number::from(100))));
    }

    #[test]
    fn test_parse_absolute_path() {
        let url = "/api/users?active=true";
        let result = parse_url(url).unwrap();

        assert_eq!(result.protocol, "");
        assert_eq!(result.domain, "");
        assert_eq!(result.path, "/api/users");
        assert_eq!(result.query.get("active"), Some(&Value::Bool(true)));
    }

    #[test]
    fn test_domain_vs_path_detection() {
        // These should be detected as domains (get https protocol)
        assert_eq!(parse_url("api.example.com").unwrap().protocol, "https");
        assert_eq!(parse_url("localhost:3000").unwrap().protocol, "https");
        assert_eq!(parse_url("sub.domain.com/path").unwrap().protocol, "https");

        // These should be detected as paths (no protocol)
        assert_eq!(parse_url("vendor/category").unwrap().protocol, "");
        assert_eq!(parse_url("/api/data").unwrap().protocol, "");
        assert_eq!(parse_url("search?q=test").unwrap().protocol, "");
    }

    #[test]
    fn test_path_normalization() {
        // Relative paths should get leading slash
        assert_eq!(parse_url("api/users").unwrap().path, "/api/users");
        assert_eq!(parse_url("vendor/category").unwrap().path, "/vendor/category");
        
        // Absolute paths should stay as-is
        assert_eq!(parse_url("/api/users").unwrap().path, "/api/users");
        assert_eq!(parse_url("/vendor/category").unwrap().path, "/vendor/category");
    }
}
