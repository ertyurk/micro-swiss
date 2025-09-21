use crate::tool_module::ToolModule;
use clap::{Arg, ArgMatches, Command};
use std::error::Error;

pub struct Base64EncodeModule;

impl ToolModule for Base64EncodeModule {
    fn name(&self) -> &'static str {
        "base64-encode"
    }

    fn description(&self) -> &'static str {
        "Encode string to base64"
    }

    fn configure_args(&self, cmd: Command) -> Command {
        cmd.arg(
            Arg::new("encode")
                .short('e')
                .long("encode")
                .value_name("STRING")
                .help("Encode string to base64"),
        )
    }

    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        if let Some(text) = matches.get_one::<String>("encode") {
            let encoded = base64_encode(text);
            println!("{}", encoded);
        }
        Ok(())
    }

    fn handles_subcommand(&self, subcommand: &str) -> bool {
        subcommand == "encode"
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        assert_eq!(base64_encode("hello"), "aGVsbG8=");
        assert_eq!(base64_encode("world"), "d29ybGQ=");
    }
}
