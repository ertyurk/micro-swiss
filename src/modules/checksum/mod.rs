use crate::error::{MsError, MsResult};
use crate::tool_module::ToolModule;
use crate::util::clipboard;
use clap::{Arg, ArgMatches, Command};
use md5;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;

pub struct ChecksumModule;

impl ToolModule for ChecksumModule {
    fn name(&self) -> &'static str {
        "checksum"
    }

    fn command(&self) -> Command {
        Command::new("checksum")
            .about("Generate file checksum (MD5/SHA256)")
            .arg(
                Arg::new("file")
                    .required(true)
                    .help("File path to checksum"),
            )
            .arg(
                Arg::new("algo")
                    .value_name("ALGO")
                    .help("Algorithm: md5 or sha256 (default: sha256)"),
            )
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        let file_path = matches.get_one::<String>("file").unwrap();
        let algorithm = matches
            .get_one::<String>("algo")
            .map(|s| s.as_str())
            .unwrap_or("sha256");

        let checksum = calculate_checksum(file_path, algorithm)?;
        let result = format!("{}: {}", algorithm.to_uppercase(), checksum);
        clipboard::copy_and_print(&result);
        Ok(())
    }
}

fn calculate_checksum(file_path: &str, algorithm: &str) -> MsResult<String> {
    let mut file = fs::File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    match algorithm.to_lowercase().as_str() {
        "md5" => {
            let digest = md5::compute(&buffer);
            Ok(format!("{:x}", digest))
        }
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(&buffer);
            Ok(format!("{:x}", hasher.finalize()))
        }
        _ => Err(MsError::InvalidInput(
            "Unsupported algorithm. Use 'md5' or 'sha256'".into(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_calculate_checksum() {
        let temp_file = "/tmp/test_checksum_ms.txt";
        let mut file = File::create(temp_file).unwrap();
        writeln!(file, "Hello, World!").unwrap();

        let md5_result = calculate_checksum(temp_file, "md5");
        assert!(md5_result.is_ok());
        assert_eq!(md5_result.unwrap().len(), 32);

        let sha256_result = calculate_checksum(temp_file, "sha256");
        assert!(sha256_result.is_ok());
        assert_eq!(sha256_result.unwrap().len(), 64);

        let _ = fs::remove_file(temp_file);
    }

    #[test]
    fn test_unsupported_algorithm() {
        let temp_file = "/tmp/test_unsupported_ms.txt";
        let mut file = File::create(temp_file).unwrap();
        writeln!(file, "test").unwrap();

        let result = calculate_checksum(temp_file, "sha1");
        assert!(result.is_err());

        let _ = fs::remove_file(temp_file);
    }

    #[test]
    fn test_nonexistent_file() {
        let result = calculate_checksum("/nonexistent/file.txt", "md5");
        assert!(result.is_err());
    }
}
