use clap::{Arg, ArgMatches, Command};
use std::error::Error;

pub trait ToolModule {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn configure_args(&self, cmd: Command) -> Command;
    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>>;
    fn handles_subcommand(&self, subcommand: &str) -> bool;
}

pub type ToolModuleBox = Box<dyn ToolModule>;