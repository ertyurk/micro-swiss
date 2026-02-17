use crate::error::MsResult;
use crate::tool_module::ToolModule;
use clap::{Arg, ArgMatches, Command};
use colored::*;
use std::fs;

pub struct EnvModule;

impl ToolModule for EnvModule {
    fn name(&self) -> &'static str {
        "env"
    }

    fn command(&self) -> Command {
        Command::new("env")
            .about("Display .env file with sensitive values masked")
            .arg(
                Arg::new("file")
                    .required(true)
                    .help("Path to .env file"),
            )
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        let file_path = matches.get_one::<String>("file").unwrap();
        let content = fs::read_to_string(file_path)?;

        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed.is_empty() || trimmed.starts_with('#') {
                println!("{}", line.dimmed());
                continue;
            }

            if let Some(eq_pos) = trimmed.find('=') {
                let key = &trimmed[..eq_pos];
                let value = &trimmed[eq_pos + 1..];

                if is_sensitive_key(key) {
                    let masked = mask_value(value);
                    println!(
                        "{}={}",
                        key.yellow(),
                        masked.red()
                    );
                } else {
                    println!(
                        "{}={}",
                        key.cyan(),
                        value.white()
                    );
                }
            } else {
                println!("{}", line);
            }
        }

        Ok(())
    }
}

fn is_sensitive_key(key: &str) -> bool {
    let key_upper = key.to_uppercase();
    let sensitive_patterns = [
        "SECRET", "PASSWORD", "PASSWD", "TOKEN", "KEY", "API", "AUTH",
        "CREDENTIAL", "PRIVATE",
    ];
    sensitive_patterns
        .iter()
        .any(|p| key_upper.contains(p))
}

fn mask_value(value: &str) -> String {
    let value = value.trim_matches('"').trim_matches('\'');
    if value.len() <= 4 {
        "*".repeat(value.len())
    } else {
        let visible = &value[..2];
        format!("{}{}",visible, "*".repeat(value.len() - 2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_sensitive_key() {
        assert!(is_sensitive_key("API_KEY"));
        assert!(is_sensitive_key("SECRET_TOKEN"));
        assert!(is_sensitive_key("DB_PASSWORD"));
        assert!(is_sensitive_key("AUTH_TOKEN"));
        assert!(!is_sensitive_key("DATABASE_URL"));
        assert!(!is_sensitive_key("PORT"));
        assert!(!is_sensitive_key("HOST"));
    }

    #[test]
    fn test_mask_value() {
        assert_eq!(mask_value("ab"), "**");
        assert_eq!(mask_value("abcdef"), "ab****");
        assert_eq!(mask_value("a"), "*");
    }

    #[test]
    fn test_mask_value_with_quotes() {
        assert_eq!(mask_value("\"secret123\""), "se*******");
    }
}
