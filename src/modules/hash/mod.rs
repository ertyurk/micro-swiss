use crate::tool_module::ToolModule;
use arboard::Clipboard;
use clap::{Arg, ArgMatches, Command};
use md5;
use sha2::{Sha256, Digest};
use std::error::Error;

pub struct HashModule;

impl ToolModule for HashModule {
    fn name(&self) -> &'static str {
        "hash"
    }

    fn configure_args(&self, cmd: Command) -> Command {
        cmd.arg(
            Arg::new("hash")
                .long("hash")
                .value_names(["TEXT", "ALGORITHM"])
                .num_args(1..=2)
                .help("Generate hash for text (MD5/SHA256)")
                .long_help("Generate MD5 or SHA256 hash for the given text. Default algorithm is SHA256. Result is automatically copied to clipboard.")
        )
    }

    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        if let Some(values) = matches.get_many::<String>("hash") {
            let values: Vec<&String> = values.collect();
            let text = values[0];
            let algorithm = values.get(1).map(|s| s.as_str()).unwrap_or("sha256");
            
            let hash = match algorithm.to_lowercase().as_str() {
                "md5" => generate_md5(text),
                "sha256" => generate_sha256(text),
                _ => return Err("Unsupported algorithm. Use 'md5' or 'sha256'".into()),
            };
            
            // Copy to clipboard
            match Clipboard::new() {
                Ok(mut clipboard) => {
                    if let Err(e) = clipboard.set_text(&hash) {
                        eprintln!("Warning: Failed to copy to clipboard: {}", e);
                        println!("{}", hash);
                    } else {
                        println!("{} (copied to clipboard)", hash);
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Failed to access clipboard: {}", e);
                    println!("{}", hash);
                }
            }
        }
        Ok(())
    }
}

fn generate_md5(text: &str) -> String {
    let digest = md5::compute(text.as_bytes());
    format!("{:x}", digest)
}

fn generate_sha256(text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_md5() {
        assert_eq!(generate_md5("hello"), "5d41402abc4b2a76b9719d911017c592");
        assert_eq!(generate_md5(""), "d41d8cd98f00b204e9800998ecf8427e");
        assert_eq!(generate_md5("The quick brown fox jumps over the lazy dog"), "9e107d9d372bb6826bd81d3542a419d6");
    }

    #[test]
    fn test_generate_sha256() {
        assert_eq!(generate_sha256("hello"), "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
        assert_eq!(generate_sha256(""), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        assert_eq!(generate_sha256("abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    }

    #[test]
    fn test_hash_unicode() {
        let unicode_text = "🦀 Rust";
        assert_eq!(generate_md5(unicode_text).len(), 32); // MD5 always 32 chars
        assert_eq!(generate_sha256(unicode_text).len(), 64); // SHA256 always 64 chars
    }

    #[test]
    fn test_hash_large_text() {
        let large_text = "a".repeat(1000);
        assert_eq!(generate_md5(&large_text).len(), 32); // MD5 always 32 chars
        assert_eq!(generate_sha256(&large_text).len(), 64); // SHA256 always 64 chars
    }
}