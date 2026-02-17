use crate::error::{MsError, MsResult};
use crate::tool_module::ToolModule;
use crate::util::clipboard;
use clap::{Arg, ArgMatches, Command};
use rand::Rng;

pub struct PasswordGenModule;

impl ToolModule for PasswordGenModule {
    fn name(&self) -> &'static str {
        "password"
    }

    fn command(&self) -> Command {
        Command::new("password")
            .about("Generate a cryptographically secure random password")
            .long_about(
                "Generate a cryptographically secure random password using a carefully \
                 chosen character set that excludes confusing characters (0/O, 1/l/I). \
                 The password is automatically copied to the clipboard.",
            )
            .arg(
                Arg::new("length")
                    .value_name("LENGTH")
                    .help("Password length (default: 16, max: 1000)")
                    .default_value("16"),
            )
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        let length_str = matches.get_one::<String>("length").unwrap();
        let length: usize = length_str
            .parse()
            .map_err(|_| MsError::InvalidInput("Length must be a number".into()))?;

        if length == 0 {
            return Err(MsError::InvalidInput(
                "Password length must be greater than 0".into(),
            ));
        }
        if length > 1000 {
            return Err(MsError::InvalidInput(
                "Password length must be 1000 or less".into(),
            ));
        }

        let password = generate_secure_password(length);
        clipboard::copy_and_print(&password);
        Ok(())
    }
}

#[must_use]
pub fn generate_secure_password(length: usize) -> String {
    const CHARSET: &[u8] =
        b"ABCDEFGHJKLMNPQRSTUVWXYZabcdefghjkmnpqrstuvwxyz23456789+/=";

    let mut rng = rand::rng();
    (0..length)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_secure_password_length() {
        assert_eq!(generate_secure_password(8).len(), 8);
        assert_eq!(generate_secure_password(16).len(), 16);
        assert_eq!(generate_secure_password(32).len(), 32);
        assert_eq!(generate_secure_password(1).len(), 1);
    }

    #[test]
    fn test_generate_secure_password_charset() {
        let password = generate_secure_password(100);
        const CHARSET: &str =
            "ABCDEFGHJKLMNPQRSTUVWXYZabcdefghjkmnpqrstuvwxyz23456789+/=";
        for c in password.chars() {
            assert!(
                CHARSET.contains(c),
                "Invalid character '{}' in password",
                c
            );
        }
    }

    #[test]
    fn test_generate_secure_password_randomness() {
        let passwords: Vec<String> =
            (0..10).map(|_| generate_secure_password(20)).collect();
        for i in 0..passwords.len() {
            for j in i + 1..passwords.len() {
                assert_ne!(passwords[i], passwords[j]);
            }
        }
    }

    #[test]
    fn test_password_entropy() {
        let password = generate_secure_password(1000);
        let unique_chars: std::collections::HashSet<char> = password.chars().collect();
        assert!(unique_chars.len() > 30);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(generate_secure_password(0).len(), 0);
        let long_password = generate_secure_password(500);
        assert_eq!(long_password.len(), 500);
    }
}
