use crate::error::{MsError, MsResult};
use crate::tool_module::ToolModule;
use crate::util::clipboard;
use clap::{Arg, ArgMatches, Command};
use std::fs;
use std::path::Path;

pub struct FileSizeModule;

impl ToolModule for FileSizeModule {
    fn name(&self) -> &'static str {
        "filesize"
    }

    fn command(&self) -> Command {
        Command::new("filesize")
            .about("Get human-readable file size or convert bytes")
            .arg(
                Arg::new("input")
                    .required(true)
                    .help("File path or byte count"),
            )
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        let input = matches.get_one::<String>("input").unwrap();
        let result = if Path::new(input).exists() {
            let metadata = fs::metadata(input)?;
            format!("{} ({})", format_bytes(metadata.len()), input)
        } else if let Ok(bytes) = input.parse::<u64>() {
            format_bytes(bytes)
        } else {
            return Err(MsError::InvalidInput(
                "Input must be a valid file path or number of bytes".into(),
            ));
        };

        clipboard::copy_and_print(&result);
        Ok(())
    }
}

#[must_use]
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
}
