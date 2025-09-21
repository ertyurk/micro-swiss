use crate::tool_module::ToolModule;
use arboard::Clipboard;
use clap::{Arg, ArgMatches, Command};
use std::error::Error;
use uuid::Uuid;

pub struct UuidGenerateModule;

impl ToolModule for UuidGenerateModule {
    fn name(&self) -> &'static str {
        "uuid-generate"
    }

    fn configure_args(&self, cmd: Command) -> Command {
        cmd.arg(
            Arg::new("uuid-generate")
                .long("uuid-generate")
                .value_name("VERSION")
                .help("Generate UUID (v4 by default)")
                .long_help("Generate UUID. Versions: v4 (random), v7 (timestamp). Result is automatically copied to clipboard.")
                .num_args(0..=1)
                .default_missing_value("v4")
        )
    }

    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        if matches.contains_id("uuid-generate") {
            let version = matches.get_one::<String>("uuid-generate")
                .map(|s| s.as_str()).unwrap_or("v4");
            
            let uuid = match version {
                "v4" => Uuid::new_v4().to_string(),
                "v7" => Uuid::now_v7().to_string(),
                _ => return Err("Unsupported UUID version. Use v4 or v7".into()),
            };
            
            match Clipboard::new() {
                Ok(mut clipboard) => {
                    if let Err(e) = clipboard.set_text(&uuid) {
                        eprintln!("Warning: Failed to copy to clipboard: {}", e);
                        println!("{}", uuid);
                    } else {
                        println!("{} (copied to clipboard)", uuid);
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Failed to access clipboard: {}", e);
                    println!("{}", uuid);
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_v4_format() {
        let uuid = Uuid::new_v4().to_string();
        assert_eq!(uuid.len(), 36);
        assert_eq!(uuid.chars().filter(|&c| c == '-').count(), 4);
    }

    #[test]
    fn test_uuid_v7_format() {
        let uuid = Uuid::now_v7().to_string();
        assert_eq!(uuid.len(), 36);
        assert_eq!(uuid.chars().filter(|&c| c == '-').count(), 4);
    }

    #[test]
    fn test_uuid_uniqueness() {
        let uuid1 = Uuid::new_v4().to_string();
        let uuid2 = Uuid::new_v4().to_string();
        assert_ne!(uuid1, uuid2);
    }
}