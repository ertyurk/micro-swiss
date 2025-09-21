use crate::tool_module::ToolModule;
use arboard::Clipboard;
use clap::{Arg, ArgMatches, Command};
use md5;
use sha2::{Sha256, Digest};
use std::error::Error;
use std::fs;
use std::io::Read;

pub struct ChecksumModule;

impl ToolModule for ChecksumModule {
    fn name(&self) -> &'static str {
        "checksum"
    }

    fn configure_args(&self, cmd: Command) -> Command {
        cmd.arg(
            Arg::new("checksum")
                .long("checksum")
                .value_names(["FILE", "ALGORITHM"])
                .num_args(1..=2)
                .help("Generate file checksum (MD5/SHA256)")
                .long_help("Generate MD5 or SHA256 checksum for a file. Default algorithm is SHA256. Result is automatically copied to clipboard.")
        )
    }

    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        if let Some(values) = matches.get_many::<String>("checksum") {
            let values: Vec<&String> = values.collect();
            let file_path = values[0];
            let algorithm = values.get(1).map(|s| s.as_str()).unwrap_or("sha256");
            
            let checksum = calculate_checksum(file_path, algorithm)?;
            let result = format!("{}: {}", algorithm.to_uppercase(), checksum);
            
            match Clipboard::new() {
                Ok(mut clipboard) => {
                    if let Err(e) = clipboard.set_text(&checksum) {
                        eprintln!("Warning: Failed to copy to clipboard: {}", e);
                        println!("{}", result);
                    } else {
                        println!("{} (copied to clipboard)", result);
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Failed to access clipboard: {}", e);
                    println!("{}", result);
                }
            }
        }
        Ok(())
    }
}

fn calculate_checksum(file_path: &str, algorithm: &str) -> Result<String, Box<dyn Error>> {
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
        _ => Err("Unsupported algorithm. Use 'md5' or 'sha256'".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::fs::File;

    #[test]
    fn test_calculate_checksum() {
        // Create a temporary file
        let temp_file = "/tmp/test_checksum.txt";
        let mut file = File::create(temp_file).unwrap();
        writeln!(file, "Hello, World!").unwrap();
        
        let md5_result = calculate_checksum(temp_file, "md5");
        assert!(md5_result.is_ok());
        assert_eq!(md5_result.unwrap().len(), 32);
        
        let sha256_result = calculate_checksum(temp_file, "sha256");
        assert!(sha256_result.is_ok());
        assert_eq!(sha256_result.unwrap().len(), 64);
        
        // Clean up
        let _ = fs::remove_file(temp_file);
    }

    #[test]
    fn test_unsupported_algorithm() {
        let temp_file = "/tmp/test_unsupported.txt";
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