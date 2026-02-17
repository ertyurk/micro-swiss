use crate::error::{MsError, MsResult};
use crate::tool_module::ToolModule;
use clap::{ArgMatches, Command};
use std::fs;
use std::path::PathBuf;

pub struct CleanModule;

impl ToolModule for CleanModule {
    fn name(&self) -> &'static str {
        "clean"
    }

    fn command(&self) -> Command {
        Command::new("clean")
            .about("Clean temporary files")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(Command::new("swaps").about("Remove Neovim swap files"))
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        match matches.subcommand() {
            Some(("swaps", _)) => clean_swaps(),
            _ => unreachable!(),
        }
    }
}

fn swap_dir() -> MsResult<PathBuf> {
    let home = std::env::var("HOME")
        .map_err(|_| MsError::InvalidInput("Could not determine home directory".into()))?;
    Ok(PathBuf::from(home).join(".local/state/nvim/swap"))
}

fn clean_swaps() -> MsResult<()> {
    let dir = swap_dir()?;

    if !dir.exists() {
        println!("No swap directory found at {}", dir.display());
        return Ok(());
    }

    let mut count = 0;
    for entry in fs::read_dir(&dir).map_err(MsError::Io)? {
        let entry = entry.map_err(MsError::Io)?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("swp") {
            fs::remove_file(&path).map_err(MsError::Io)?;
            count += 1;
        }
    }

    if count > 0 {
        println!("Removed {} swap file(s)", count);
    } else {
        println!("No swap files found");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_dir_path() {
        let dir = swap_dir().unwrap();
        assert!(dir.ends_with(".local/state/nvim/swap"));
    }
}
