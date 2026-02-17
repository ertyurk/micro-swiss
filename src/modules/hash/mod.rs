use crate::error::{MsError, MsResult};
use crate::tool_module::ToolModule;
use crate::util::{clipboard, stdin};
use clap::{Arg, ArgMatches, Command};
use md5;
use sha2::{Digest, Sha256};

pub struct HashModule;

impl ToolModule for HashModule {
    fn name(&self) -> &'static str {
        "hash"
    }

    fn command(&self) -> Command {
        Command::new("hash")
            .about("Generate hash for text (MD5/SHA256)")
            .arg(
                Arg::new("text")
                    .value_name("TEXT")
                    .help("Text to hash"),
            )
            .arg(
                Arg::new("algo")
                    .value_name("ALGO")
                    .help("Algorithm: md5 or sha256 (default: sha256)"),
            )
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        let text = stdin::get_text_input(matches.get_one::<String>("text"))
            .ok_or_else(|| MsError::InvalidInput("No text provided".into()))?;

        let algorithm = matches
            .get_one::<String>("algo")
            .map(|s| s.as_str())
            .unwrap_or("sha256");

        let hash = match algorithm.to_lowercase().as_str() {
            "md5" => generate_md5(&text),
            "sha256" => generate_sha256(&text),
            _ => {
                return Err(MsError::InvalidInput(
                    "Unsupported algorithm. Use 'md5' or 'sha256'".into(),
                ))
            }
        };

        clipboard::copy_and_print(&hash);
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
        assert_eq!(
            generate_md5("hello"),
            "5d41402abc4b2a76b9719d911017c592"
        );
        assert_eq!(
            generate_md5(""),
            "d41d8cd98f00b204e9800998ecf8427e"
        );
    }

    #[test]
    fn test_generate_sha256() {
        assert_eq!(
            generate_sha256("hello"),
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
        assert_eq!(
            generate_sha256(""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_hash_unicode() {
        assert_eq!(generate_md5("🦀 Rust").len(), 32);
        assert_eq!(generate_sha256("🦀 Rust").len(), 64);
    }
}
