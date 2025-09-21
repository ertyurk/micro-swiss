use crate::tool_module::ToolModule;
use arboard::Clipboard;
use clap::{Arg, ArgMatches, Command};
use rand::Rng;
use std::error::Error;

pub struct PasswordGenModule;

impl ToolModule for PasswordGenModule {
    fn name(&self) -> &'static str {
        "password-gen"
    }

    fn configure_args(&self, cmd: Command) -> Command {
        cmd.arg(
            Arg::new("password")
                .short('p')
                .long("password")
                .value_name("LENGTH")
                .help("Generate a cryptographically secure random password (auto-copied to clipboard)")
                .long_help("Generate a cryptographically secure random password using a carefully chosen character set that excludes confusing characters (0/O, 1/l/I). The password is automatically copied to the clipboard. Default length is 16 characters if not specified.")
                .num_args(0..=1)
                .default_missing_value("16"),
        )
    }

    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        if matches.contains_id("password") {
            let default_value = "16".to_string();
            let length_str = matches
                .get_one::<String>("password")
                .unwrap_or(&default_value);
            let length: usize = length_str.parse().unwrap_or(16);

            if length == 0 {
                return Err("Password length must be greater than 0".into());
            }

            if length > 1000 {
                return Err("Password length must be 1000 or less".into());
            }

            let password = generate_secure_password(length);

            // Copy to clipboard
            match Clipboard::new() {
                Ok(mut clipboard) => {
                    if let Err(e) = clipboard.set_text(&password) {
                        eprintln!("Warning: Failed to copy to clipboard: {}", e);
                        println!("{}", password);
                    } else {
                        println!("{} (copied to clipboard)", password);
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Failed to access clipboard: {}", e);
                    println!("{}", password);
                }
            }
        }
        Ok(())
    }
}

pub fn generate_secure_password(length: usize) -> String {
    // Use base64-like characters for maximum entropy and readability
    // Excludes potentially confusing characters like 0/O, 1/l/I
    const CHARSET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZabcdefghjkmnpqrstuvwxyz23456789+/=";

    let mut rng = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    password
}

pub fn generate_base64_password(length: usize) -> String {
    // Generate random bytes and encode as base64, then trim to desired length
    let byte_count = (length * 3 + 3) / 4; // Calculate bytes needed for base64 encoding
    let mut rng = rand::thread_rng();

    let random_bytes: Vec<u8> = (0..byte_count).map(|_| rng.gen()).collect();
    let base64_string = base64_encode_bytes(&random_bytes);

    // Trim to exact length and remove padding characters
    base64_string.chars().take(length).collect()
}

fn base64_encode_bytes(bytes: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();

    for chunk in bytes.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &b) in chunk.iter().enumerate() {
            buf[i] = b;
        }

        let b = ((buf[0] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[2] as u32);

        result.push(CHARS[((b >> 18) & 63) as usize] as char);
        result.push(CHARS[((b >> 12) & 63) as usize] as char);

        if chunk.len() > 1 {
            result.push(CHARS[((b >> 6) & 63) as usize] as char);
        }

        if chunk.len() > 2 {
            result.push(CHARS[(b & 63) as usize] as char);
        }
    }

    result
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
        // Should only contain valid charset characters
        const CHARSET: &str = "ABCDEFGHJKLMNPQRSTUVWXYZabcdefghjkmnpqrstuvwxyz23456789+/=";
        for c in password.chars() {
            assert!(CHARSET.contains(c), "Invalid character '{}' in password", c);
        }
    }

    #[test]
    fn test_generate_secure_password_randomness() {
        // Generate multiple passwords and ensure they're different
        let passwords: Vec<String> = (0..10).map(|_| generate_secure_password(20)).collect();

        // Check that all passwords are unique (very high probability with 20-char passwords)
        for i in 0..passwords.len() {
            for j in i + 1..passwords.len() {
                assert_ne!(passwords[i], passwords[j], "Generated duplicate passwords");
            }
        }
    }

    #[test]
    fn test_generate_base64_password_length() {
        assert_eq!(generate_base64_password(12).len(), 12);
        assert_eq!(generate_base64_password(24).len(), 24);
        assert_eq!(generate_base64_password(1).len(), 1);
    }

    #[test]
    fn test_generate_base64_password_charset() {
        let password = generate_base64_password(50);
        const BASE64_CHARS: &str =
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        for c in password.chars() {
            assert!(
                BASE64_CHARS.contains(c),
                "Invalid character '{}' in base64 password",
                c
            );
        }
    }

    #[test]
    fn test_generate_base64_password_randomness() {
        let passwords: Vec<String> = (0..10).map(|_| generate_base64_password(16)).collect();

        // Check uniqueness
        for i in 0..passwords.len() {
            for j in i + 1..passwords.len() {
                assert_ne!(
                    passwords[i], passwords[j],
                    "Generated duplicate base64 passwords"
                );
            }
        }
    }

    #[test]
    fn test_base64_encode_bytes() {
        assert_eq!(base64_encode_bytes(&[]), "");
        assert_eq!(base64_encode_bytes(&[0x4d]), "TQ");
        assert_eq!(base64_encode_bytes(&[0x4d, 0x61]), "TWE");
        assert_eq!(base64_encode_bytes(&[0x4d, 0x61, 0x6e]), "TWFu");
    }

    #[test]
    fn test_password_entropy() {
        // Test that generated passwords have good character distribution
        let password = generate_secure_password(1000);
        let unique_chars: std::collections::HashSet<char> = password.chars().collect();

        // Should use a good portion of the available character set
        assert!(
            unique_chars.len() > 30,
            "Password should use diverse character set"
        );
    }

    #[test]
    fn test_edge_cases() {
        // Test zero length (should be handled by validation in execute())
        assert_eq!(generate_secure_password(0).len(), 0);

        // Test very large length
        let long_password = generate_secure_password(500);
        assert_eq!(long_password.len(), 500);
    }
}
