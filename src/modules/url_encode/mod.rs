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
    }

    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        if let Some(text) = matches.get_one::<String>("url-encode") {
            let encoded = url_encode(text);
            println!("{}", encoded);
        }
        Ok(())
    }
}

pub fn url_encode(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
                ' ' => "+".to_string(),
                _ => format!("%{:02X}", c as u8),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_encode() {
        assert_eq!(url_encode("hello world"), "hello+world");
        assert_eq!(url_encode("test@example.com"), "test%40example.com");
    }
}