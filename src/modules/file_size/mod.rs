use crate::tool_module::ToolModule;
use arboard::Clipboard;
use clap::{Arg, ArgMatches, Command};
use std::error::Error;
use std::fs;
use std::path::Path;

pub struct FileSizeModule;

impl ToolModule for FileSizeModule {
    fn name(&self) -> &'static str {
        "file-size"
    }

    fn configure_args(&self, cmd: Command) -> Command {
        cmd.arg(
            Arg::new("file-size")
                .long("file-size")
                .value_name("PATH_OR_BYTES")
                .help("Get human-readable file size or convert bytes")
                .long_help("Calculate human-readable file size from file path or convert raw bytes to human-readable format. Result is automatically copied to clipboard.")
        )
    }

    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        if let Some(input) = matches.get_one::<String>("file-size") {
            let result = if Path::new(input).exists() {
                let metadata = fs::metadata(input)?;
                let size = metadata.len();
                format!("{} ({})", format_bytes(size), input)
            } else if let Ok(bytes) = input.parse::<u64>() {
                format_bytes(bytes)
            } else {
                return Err("Input must be a valid file path or number of bytes".into());
            };
            
            match Clipboard::new() {
                Ok(mut clipboard) => {
                    if let Err(e) = clipboard.set_text(&result) {
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

fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
    const THRESHOLD: f64 = 1024.0;
    
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    let bytes_f = bytes as f64;
    let i = (bytes_f.log10() / THRESHOLD.log10()).floor() as usize;
    let i = i.min(UNITS.len() - 1);
    
    if i == 0 {
        format!("{} B", bytes)
    } else {
        let size = bytes_f / THRESHOLD.powi(i as i32);
        format!("{:.1} {}", size, UNITS[i])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(1), "1 B");
        assert_eq!(format_bytes(1023), "1023 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1536), "1.5 KB");
        assert_eq!(format_bytes(1048576), "1.0 MB");
        assert_eq!(format_bytes(1073741824), "1.0 GB");
    }

    #[test]
    fn test_large_files() {
        assert_eq!(format_bytes(1_099_511_627_776), "1.0 TB");
        assert_eq!(format_bytes(1_125_899_906_842_624), "1.0 PB");
    }

    #[test]
    fn test_precise_formatting() {
        assert_eq!(format_bytes(1_536_000), "1.5 MB");
        assert_eq!(format_bytes(2_048_000), "2.0 MB");
    }
}