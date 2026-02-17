use crate::error::MsResult;
use clap::{ArgMatches, Command};

pub trait ToolModule {
    fn name(&self) -> &'static str;
    fn command(&self) -> Command;
    fn execute(&self, matches: &ArgMatches) -> MsResult<()>;
}

pub type ToolModuleBox = Box<dyn ToolModule>;
