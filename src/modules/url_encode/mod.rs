use crate::tool_module::ToolModule;
use clap::{Arg, ArgMatches, Command};
use std::error::Error;

pub struct UrlEncodeModule;

impl ToolModule for UrlEncodeModule {
    fn name(&self) -> &'static str {
        "url-encode"
    }

    fn configure_args(&self, cmd: Command) -> Command {
        cmd.arg(
            Arg::new("url-encode")
                .short('u')
                .long("url-encode")
                .value_name("STRING")
                .help("URL encode a string")
        )
        .arg(
            Arg::new("url-decode")
                .long("url-decode")
                .value_name("STRING")
                .help("URL decode a string")
        )
    }

    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        if let Some(text) = matches.get_one::<String>("url-encode") {
            let encoded = url_encode(text);
            println!("{}", encoded);
        } else if let Some(text) = matches.get_one::<String>("url-decode") {
            match url_decode(text) {
                Ok(decoded) => println!("{}", decoded),
                Err(e) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, e))),
            }
        }
        Ok(())
    }
}

pub fn url_encode(input: &str) -> String {
    input
        .bytes()
        .map(|b| {
            match b {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => (b as char).to_string(),
                b' ' => "+".to_string(),
                _ => format!("%{:02X}", b),
            }
        })
        .collect()
}

pub fn url_decode(input: &str) -> Result<String, String> {
    let mut result = Vec::new();
    let mut chars = input.chars().peekable();
    
    while let Some(c) = chars.next() {
        match c {
            '+' => result.push(b' '),
            '%' => {
                // Get the next two characters for hex decoding
                let hex1 = chars.next().ok_or("Incomplete percent encoding: missing first hex digit")?;
                let hex2 = chars.next().ok_or("Incomplete percent encoding: missing second hex digit")?;
                
                // Parse hex digits
                let hex_str = format!("{}{}", hex1, hex2);
                match u8::from_str_radix(&hex_str, 16) {
                    Ok(byte) => result.push(byte),
                    Err(_) => return Err(format!("Invalid hex digits in percent encoding: {}", hex_str)),
                }
            }
            _ => {
                // Convert char to UTF-8 bytes and add them
                let mut buffer = [0; 4];
                let bytes = c.encode_utf8(&mut buffer).as_bytes();
                result.extend_from_slice(bytes);
            }
        }
    }
    
    String::from_utf8(result).map_err(|e| format!("Invalid UTF-8 sequence: {}", e))
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
        assert_eq!(url_encode("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_.~"), "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_.~");
    }

    #[test]
    fn test_url_encode_spaces() {
        assert_eq!(url_encode("   "), "+++");
        assert_eq!(url_encode(" a b c "), "+a+b+c+");
    }

    #[test]
    fn test_url_encode_special_characters() {
        assert_eq!(url_encode("!@#$%^&*()"), "%21%40%23%24%25%5E%26%2A%28%29");
        assert_eq!(url_encode("+=?&"), "%2B%3D%3F%26");
    }

    #[test]
    fn test_url_encode_unicode() {
        let result_fire = url_encode("🔥");
        assert!(result_fire.contains('%'));
        assert_eq!(url_encode("café"), "caf%C3%A9");
        assert_eq!(url_encode("naïve"), "na%C3%AFve");
    }

    #[test]
    fn test_url_encode_mixed_content() {
        assert_eq!(url_encode("user@domain.com?param=value&other=test"), "user%40domain.com%3Fparam%3Dvalue%26other%3Dtest");
    }

    #[test]
    fn test_url_encode_newlines_and_tabs() {
        assert_eq!(url_encode("\n\r\t"), "%0A%0D%09");
        assert_eq!(url_encode("line1\nline2"), "line1%0Aline2");
    }

    #[test]
    fn test_url_encode_quotes() {
        assert_eq!(url_encode("\"'"), "%22%27");
        assert_eq!(url_encode("say \"hello\""), "say+%22hello%22");
    }

    #[test]
    fn test_url_encode_brackets_and_braces() {
        assert_eq!(url_encode("{}[]()<>"), "%7B%7D%5B%5D%28%29%3C%3E");
    }

    #[test]
    fn test_url_encode_control_characters() {
        assert_eq!(url_encode("\x00\x01\x1F"), "%00%01%1F");
    }

    #[test]
    fn test_url_encode_high_ascii() {
        let high_ascii = String::from_utf8_lossy(&[0x80, 0xFF]).to_string();
        let result = url_encode(&high_ascii);
        assert!(result.contains('%'));
    }

    #[test]
    fn test_url_encode_long_string() {
        let long_input = "a".repeat(1000);
        let result = url_encode(&long_input);
        assert_eq!(result, long_input);
    }

    #[test]
    fn test_url_encode_path_like() {
        assert_eq!(url_encode("/path/to/file"), "%2Fpath%2Fto%2Ffile");
        assert_eq!(url_encode("../relative/path"), "..%2Frelative%2Fpath");
    }

    #[test]
    fn test_url_decode_basic() {
        assert_eq!(url_decode("hello+world").unwrap(), "hello world");
        assert_eq!(url_decode("test%40example.com").unwrap(), "test@example.com");
    }

    #[test]
    fn test_url_decode_empty_string() {
        assert_eq!(url_decode("").unwrap(), "");
    }

    #[test]
    fn test_url_decode_spaces() {
        assert_eq!(url_decode("+++").unwrap(), "   ");
        assert_eq!(url_decode("+a+b+c+").unwrap(), " a b c ");
        assert_eq!(url_decode("hello+world").unwrap(), "hello world");
    }

    #[test]
    fn test_url_decode_special_characters() {
        assert_eq!(url_decode("%21%40%23%24%25%5E%26%2A%28%29").unwrap(), "!@#$%^&*()");
        assert_eq!(url_decode("%2B%3D%3F%26").unwrap(), "+=?&");
    }

    #[test]
    fn test_url_decode_unicode() {
        assert_eq!(url_decode("caf%C3%A9").unwrap(), "café");
        assert_eq!(url_decode("na%C3%AFve").unwrap(), "naïve");
    }

    #[test]
    fn test_url_decode_mixed_content() {
        assert_eq!(url_decode("user%40domain.com%3Fparam%3Dvalue%26other%3Dtest").unwrap(), "user@domain.com?param=value&other=test");
    }

    #[test]
    fn test_url_decode_newlines_and_tabs() {
        assert_eq!(url_decode("%0A%0D%09").unwrap(), "\n\r\t");
        assert_eq!(url_decode("line1%0Aline2").unwrap(), "line1\nline2");
    }

    #[test]
    fn test_url_decode_quotes() {
        assert_eq!(url_decode("%22%27").unwrap(), "\"'");
        assert_eq!(url_decode("say+%22hello%22").unwrap(), "say \"hello\"");
    }

    #[test]
    fn test_url_decode_brackets_and_braces() {
        assert_eq!(url_decode("%7B%7D%5B%5D%28%29%3C%3E").unwrap(), "{}[]()<>");
    }

    #[test]
    fn test_url_decode_control_characters() {
        assert_eq!(url_decode("%00%01%1F").unwrap(), "\x00\x01\x1F");
    }

    #[test]
    fn test_url_decode_path_like() {
        assert_eq!(url_decode("%2Fpath%2Fto%2Ffile").unwrap(), "/path/to/file");
        assert_eq!(url_decode("..%2Frelative%2Fpath").unwrap(), "../relative/path");
    }

    #[test]
    fn test_url_decode_unreserved_chars() {
        let input = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_.~";
        assert_eq!(url_decode(input).unwrap(), input);
    }

    #[test]
    fn test_url_decode_invalid_percent_encoding() {
        assert!(url_decode("%").is_err());
        assert!(url_decode("%1").is_err());
        assert!(url_decode("%GG").is_err());
        assert!(url_decode("%1G").is_err());
    }

    #[test]
    fn test_url_decode_mixed_valid_invalid() {
        assert_eq!(url_decode("hello%20world").unwrap(), "hello world");
        assert!(url_decode("hello%GGworld").is_err());
    }

    #[test]
    fn test_url_roundtrip() {
        let original = "Hello World! @#$%^&*()";
        let encoded = url_encode(original);
        let decoded = url_decode(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_url_roundtrip_edge_cases() {
        let test_cases = vec![
            "",
            "hello world",
            "!@#$%^&*()",
            "café",
            "/path/to/file",
            "user@domain.com?param=value&other=test",
        ];
        for case in test_cases {
            let encoded = url_encode(case);
            let decoded = url_decode(&encoded).unwrap();
            assert_eq!(case, decoded);
        }
    }
}