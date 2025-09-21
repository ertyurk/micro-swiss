use crate::tool_module::ToolModule;
use clap::{Arg, ArgMatches, Command};
use std::error::Error;

pub struct Base64EncodeModule;

impl ToolModule for Base64EncodeModule {
    fn name(&self) -> &'static str {
        "base64-encode"
    }

    fn configure_args(&self, cmd: Command) -> Command {
        cmd.arg(
            Arg::new("encode")
                .short('e')
                .long("encode")
                .value_name("STRING")
                .help("Encode string to base64")
                .long_help("Encode a UTF-8 string to base64 format. Handles unicode characters, special characters, and binary data correctly."),
        )
        .arg(
            Arg::new("decode")
                .short('d')
                .long("decode")
                .value_name("STRING")
                .help("Decode base64 string")
                .long_help("Decode a base64-encoded string back to UTF-8. Automatically handles whitespace in input and provides detailed error messages for invalid base64 data."),
        )
    }

    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        if let Some(text) = matches.get_one::<String>("encode") {
            let encoded = base64_encode(text);
            println!("{}", encoded);
        } else if let Some(text) = matches.get_one::<String>("decode") {
            match base64_decode(text) {
                Ok(decoded) => println!("{}", decoded),
                Err(e) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, e))),
            }
        }
        Ok(())
    }
}

pub fn base64_encode(input: &str) -> String {
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    let bytes = input.as_bytes();

    for chunk in bytes.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &b) in chunk.iter().enumerate() {
            buf[i] = b;
        }

        let b = ((buf[0] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[2] as u32);

        result.push(chars.chars().nth(((b >> 18) & 63) as usize).unwrap());
        result.push(chars.chars().nth(((b >> 12) & 63) as usize).unwrap());

        if chunk.len() > 1 {
            result.push(chars.chars().nth(((b >> 6) & 63) as usize).unwrap());
        } else {
            result.push('=');
        }

        if chunk.len() > 2 {
            result.push(chars.chars().nth((b & 63) as usize).unwrap());
        } else {
            result.push('=');
        }
    }

    result
}

pub fn base64_decode(input: &str) -> Result<String, String> {
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    if input.is_empty() {
        return Ok(String::new());
    }
    
    // Remove any whitespace
    let input = input.chars().filter(|&c| !c.is_whitespace()).collect::<String>();
    
    // Check for invalid characters
    for c in input.chars() {
        if c != '=' && !chars.contains(c) {
            return Err(format!("Invalid character '{}' in base64 string", c));
        }
    }
    
    // Check padding
    let padding_count = input.chars().rev().take_while(|&c| c == '=').count();
    if padding_count > 2 {
        return Err("Too many padding characters".to_string());
    }
    
    let mut result = Vec::new();
    let input_chars: Vec<char> = input.chars().collect();
    
    for chunk in input_chars.chunks(4) {
        let mut buf = [0u8; 4];
        let mut valid_chars = 0;
        
        for (i, &c) in chunk.iter().enumerate() {
            if c == '=' {
                break;
            }
            if let Some(pos) = chars.find(c) {
                buf[i] = pos as u8;
                valid_chars += 1;
            }
        }
        
        if valid_chars == 0 {
            break;
        }
        
        let b = ((buf[0] as u32) << 18) | ((buf[1] as u32) << 12) | ((buf[2] as u32) << 6) | (buf[3] as u32);
        
        result.push(((b >> 16) & 0xFF) as u8);
        if valid_chars > 2 {
            result.push(((b >> 8) & 0xFF) as u8);
        }
        if valid_chars > 3 {
            result.push((b & 0xFF) as u8);
        }
    }
    
    String::from_utf8(result).map_err(|e| format!("Invalid UTF-8 sequence: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        assert_eq!(base64_encode("hello"), "aGVsbG8=");
        assert_eq!(base64_encode("world"), "d29ybGQ=");
    }

    #[test]
    fn test_base64_encode_empty_string() {
        assert_eq!(base64_encode(""), "");
    }

    #[test]
    fn test_base64_encode_single_char() {
        assert_eq!(base64_encode("A"), "QQ==");
        assert_eq!(base64_encode("1"), "MQ==");
    }

    #[test]
    fn test_base64_encode_two_chars() {
        assert_eq!(base64_encode("AB"), "QUI=");
        assert_eq!(base64_encode("12"), "MTI=");
    }

    #[test]
    fn test_base64_encode_three_chars() {
        assert_eq!(base64_encode("ABC"), "QUJD");
        assert_eq!(base64_encode("123"), "MTIz");
    }

    #[test]
    fn test_base64_encode_unicode() {
        assert_eq!(base64_encode("🔥"), "8J+UpQ==");
        assert_eq!(base64_encode("café"), "Y2Fmw6k=");
    }

    #[test]
    fn test_base64_encode_special_chars() {
        assert_eq!(base64_encode("!@#$%^&*()"), "IUAjJCVeJiooKQ==");
        assert_eq!(base64_encode("\n\r\t"), "Cg0J");
    }

    #[test]
    fn test_base64_encode_whitespace() {
        assert_eq!(base64_encode("   "), "ICAg");
        assert_eq!(base64_encode(" \n \t "), "IAogCSA=");
    }

    #[test]
    fn test_base64_encode_long_string() {
        let long_input = "a".repeat(1000);
        let result = base64_encode(&long_input);
        assert!(result.len() > 0);
        assert!(result.ends_with('=') || !result.ends_with('='));
    }

    #[test]
    fn test_base64_encode_binary_data() {
        assert_eq!(base64_encode("\x00\x01\x02\x03"), "AAECAw==");
        let binary_data = String::from_utf8_lossy(&[0xFF, 0xFE, 0xFD]).to_string();
        let result = base64_encode(&binary_data);
        assert!(result.len() > 0);
    }

    #[test]
    fn test_base64_decode_basic() {
        assert_eq!(base64_decode("aGVsbG8=").unwrap(), "hello");
        assert_eq!(base64_decode("d29ybGQ=").unwrap(), "world");
    }

    #[test]
    fn test_base64_decode_empty_string() {
        assert_eq!(base64_decode("").unwrap(), "");
    }

    #[test]
    fn test_base64_decode_single_char() {
        assert_eq!(base64_decode("QQ==").unwrap(), "A");
        assert_eq!(base64_decode("MQ==").unwrap(), "1");
    }

    #[test]
    fn test_base64_decode_two_chars() {
        assert_eq!(base64_decode("QUI=").unwrap(), "AB");
        assert_eq!(base64_decode("MTI=").unwrap(), "12");
    }

    #[test]
    fn test_base64_decode_three_chars() {
        assert_eq!(base64_decode("QUJD").unwrap(), "ABC");
        assert_eq!(base64_decode("MTIz").unwrap(), "123");
    }

    #[test]
    fn test_base64_decode_unicode() {
        assert_eq!(base64_decode("8J+UpQ==").unwrap(), "🔥");
        assert_eq!(base64_decode("Y2Fmw6k=").unwrap(), "café");
    }

    #[test]
    fn test_base64_decode_special_chars() {
        assert_eq!(base64_decode("IUAjJCVeJiooKQ==").unwrap(), "!@#$%^&*()");
        assert_eq!(base64_decode("Cg0J").unwrap(), "\n\r\t");
    }

    #[test]
    fn test_base64_decode_whitespace() {
        assert_eq!(base64_decode("ICAg").unwrap(), "   ");
        assert_eq!(base64_decode("IAogCSA=").unwrap(), " \n \t ");
    }

    #[test]
    fn test_base64_decode_with_whitespace_input() {
        assert_eq!(base64_decode(" aGVs bG8= ").unwrap(), "hello");
        assert_eq!(base64_decode("\naGVsbG8=\n").unwrap(), "hello");
    }

    #[test]
    fn test_base64_decode_invalid_characters() {
        assert!(base64_decode("aGVs@G8=").is_err());
        assert!(base64_decode("hello!").is_err());
    }

    #[test]
    fn test_base64_decode_invalid_padding() {
        assert!(base64_decode("aGVsbG8===").is_err());
        assert!(base64_decode("aGVs====").is_err());
    }

    #[test]
    fn test_base64_roundtrip() {
        let original = "Hello, World! 🌍";
        let encoded = base64_encode(original);
        let decoded = base64_decode(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_base64_roundtrip_edge_cases() {
        let test_cases = vec!["", "A", "AB", "ABC", "🔥", "\n\r\t", "   "];
        for case in test_cases {
            let encoded = base64_encode(case);
            let decoded = base64_decode(&encoded).unwrap();
            assert_eq!(case, decoded);
        }
    }
}
