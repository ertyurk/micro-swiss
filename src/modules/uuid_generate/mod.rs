use crate::error::{MsError, MsResult};
use crate::tool_module::ToolModule;
use crate::util::clipboard;
use clap::{Arg, ArgMatches, Command};
use uuid::Uuid;

pub struct UuidGenerateModule;

impl ToolModule for UuidGenerateModule {
    fn name(&self) -> &'static str {
        "uuid"
    }

    fn command(&self) -> Command {
        Command::new("uuid")
            .about("Generate UUID (v4 or v7)")
            .arg(
                Arg::new("version")
                    .value_name("VERSION")
                    .help("UUID version: v4 (random) or v7 (timestamp)")
                    .default_value("v4"),
            )
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        let version = matches
            .get_one::<String>("version")
            .map(|s| s.as_str())
            .unwrap_or("v4");

        let uuid = match version {
            "v4" => Uuid::new_v4().to_string(),
            "v7" => Uuid::now_v7().to_string(),
            _ => {
                return Err(MsError::InvalidInput(
                    "Unsupported UUID version. Use v4 or v7".into(),
                ))
            }
        };

        clipboard::copy_and_print(&uuid);
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
    }

    #[test]
    fn test_uuid_uniqueness() {
        let uuid1 = Uuid::new_v4().to_string();
        let uuid2 = Uuid::new_v4().to_string();
        assert_ne!(uuid1, uuid2);
    }
}
