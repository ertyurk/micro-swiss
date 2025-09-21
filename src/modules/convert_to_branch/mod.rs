use crate::tool_module::ToolModule;
use clap::{Arg, ArgMatches, Command};
use std::error::Error;

pub struct ConvertToBranchModule;

impl ToolModule for ConvertToBranchModule {
    fn name(&self) -> &'static str {
        "convert-to-branch"
    }

    fn configure_args(&self, cmd: Command) -> Command {
        cmd.arg(
            Arg::new("generate-branch")
                .short('g')
                .long("generate-branch")
                .value_name("STRING")
                .help("Convert string to branch-friendly format")
        )
    }

    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        if let Some(text) = matches.get_one::<String>("generate-branch") {
            let branch_name = convert_to_branch_name(text);
            println!("{}", branch_name);
        }
        Ok(())
    }
}

pub fn convert_to_branch_name(input: &str) -> String {
    input
        .to_lowercase()
        .chars()
        .map(|c| if c.is_whitespace() { '-' } else { c })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_basic() {
        assert_eq!(convert_to_branch_name("Hello World"), "hello-world");
    }

    #[test]
    fn test_convert_multiple_spaces() {
        assert_eq!(convert_to_branch_name("Feature  Name   Test"), "feature--name---test");
    }

    #[test]
    fn test_convert_mixed_case() {
        assert_eq!(convert_to_branch_name("CamelCase Test"), "camelcase-test");
    }
}