use clap::{ArgMatches, Command};
use std::error::Error;

pub trait ToolModule {
    fn name(&self) -> &'static str;
    fn configure_args(&self, cmd: Command) -> Command;
    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>>;
}

pub type ToolModuleBox = Box<dyn ToolModule>;
