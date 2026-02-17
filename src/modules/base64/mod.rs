use crate::error::{MsError, MsResult};
use crate::tool_module::ToolModule;
use crate::util::stdin;
use clap::{Arg, ArgMatches, Command};

pub struct Base64Module;

impl ToolModule for Base64Module {
    fn name(&self) -> &'static str {
        "base64"
    }

    fn command(&self) -> Command {
        Command::new("base64")
            .about("Base64 encode or decode text")
            .subcommand_required(true)
            .subcommand(
                Command::new("encode")
                    .about("Encode text to base64")
                    .arg(Arg::new("text").value_name("TEXT").help("Text to encode")),
            )
            .subcommand(
                Command::new("decode")
                    .about("Decode base64 text")
                    .arg(Arg::new("text").value_name("TEXT").help("Base64 to decode")),
            )
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        match matches.subcommand() {
            Some(("encode", sub_m)) => {
                let text = stdin::get_text_input(sub_m.get_one::<String>("text"))
                    .ok_or_else(|| {
                        MsError::InvalidInput("No text provided".into())
                    })?;
                let encoded = base64_encode(&text);
                println!("{}", encoded);
            }
            Some(("decode", sub_m)) => {
                let text = stdin::get_text_input(sub_m.get_one::<String>("text"))
                    .ok_or_else(|| {
                        MsError::InvalidInput("No text provided".into())
                    })?;
                let decoded = base64_decode(&text)?;
                println!("{}", decoded);
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}

#[must_use]
pub fn base64_encode(input: &str) -> String {
    const CHARS: &[u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    let bytes = input.as_bytes();

    for chunk in bytes.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &b) in chunk.iter().enumerate() {
            buf[i] = b;
        }

        let b =
            ((buf[0] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[2] as u32);

        result.push(CHARS[((b >> 18) & 63) as usize] as char);
        result.push(CHARS[((b >> 12) & 63) as usize] as char);

        if chunk.len() > 1 {
            result.push(CHARS[((b >> 6) & 63) as usize] as char);
        } else {
            result.push('=');
        }

        if chunk.len() > 2 {
            result.push(CHARS[(b & 63) as usize] as char);
        } else {
            result.push('=');
        }
    }

    result
}

pub fn base64_decode(input: &str) -> MsResult<String> {
    const CHARS: &str =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    if input.is_empty() {
        return Ok(String::new());
    }

    let input: String = input.chars().filter(|c| !c.is_whitespace()).collect();

    for c in input.chars() {
        if c != '=' && !CHARS.contains(c) {
            return Err(MsError::InvalidInput(format!(
                "Invalid character '{}' in base64 string",
                c
            )));
        }
    }

    let padding_count = input.chars().rev().take_while(|&c| c == '=').count();
    if padding_count > 2 {
        return Err(MsError::InvalidInput(
            "Too many padding characters".into(),
        ));
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
            if let Some(pos) = CHARS.find(c) {
                buf[i] = pos as u8;
                valid_chars += 1;
            }
        }

        if valid_chars == 0 {
            break;
        }

        let b = ((buf[0] as u32) << 18)
            | ((buf[1] as u32) << 12)
            | ((buf[2] as u32) << 6)
            | (buf[3] as u32);

        result.push(((b >> 16) & 0xFF) as u8);
        if valid_chars > 2 {
            result.push(((b >> 8) & 0xFF) as u8);
        }
        if valid_chars > 3 {
            result.push((b & 0xFF) as u8);
        }
    }

    String::from_utf8(result).map_err(|e| {
        MsError::InvalidInput(format!("Invalid UTF-8 sequence: {}", e))
    })
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
    fn test_base64_decode_basic() {
        assert_eq!(base64_decode("aGVsbG8=").unwrap(), "hello");
        assert_eq!(base64_decode("d29ybGQ=").unwrap(), "world");
    }

    #[test]
    fn test_base64_decode_empty_string() {
        assert_eq!(base64_decode("").unwrap(), "");
    }

    #[test]
    fn test_base64_decode_unicode() {
        assert_eq!(base64_decode("8J+UpQ==").unwrap(), "🔥");
        assert_eq!(base64_decode("Y2Fmw6k=").unwrap(), "café");
    }

    #[test]
    fn test_base64_decode_with_whitespace_input() {
        assert_eq!(base64_decode(" aGVs bG8= ").unwrap(), "hello");
    }

    #[test]
    fn test_base64_decode_invalid_characters() {
        assert!(base64_decode("aGVs@G8=").is_err());
    }

    #[test]
    fn test_base64_decode_invalid_padding() {
        assert!(base64_decode("aGVsbG8===").is_err());
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
