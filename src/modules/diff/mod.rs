use crate::error::MsResult;
use crate::tool_module::ToolModule;
use clap::{Arg, ArgMatches, Command};
use colored::*;
use similar::{ChangeTag, TextDiff};
use std::fs;

pub struct DiffModule;

impl ToolModule for DiffModule {
    fn name(&self) -> &'static str {
        "diff"
    }

    fn command(&self) -> Command {
        Command::new("diff")
            .about("Show colored diff between two files")
            .arg(
                Arg::new("file1")
                    .required(true)
                    .help("First file"),
            )
            .arg(
                Arg::new("file2")
                    .required(true)
                    .help("Second file"),
            )
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        let file1 = matches.get_one::<String>("file1").unwrap();
        let file2 = matches.get_one::<String>("file2").unwrap();

        let content1 = fs::read_to_string(file1)?;
        let content2 = fs::read_to_string(file2)?;

        let diff = TextDiff::from_lines(&content1, &content2);

        println!(
            "{}",
            format!("--- {}", file1).red()
        );
        println!(
            "{}",
            format!("+++ {}", file2).green()
        );

        for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
            if idx > 0 {
                println!("{}", "...".dimmed());
            }
            for op in group {
                for change in diff.iter_changes(op) {
                    let (sign, style) = match change.tag() {
                        ChangeTag::Delete => ("-", "red"),
                        ChangeTag::Insert => ("+", "green"),
                        ChangeTag::Equal => (" ", "white"),
                    };
                    let formatted = format!("{}{}", sign, change.value());
                    match style {
                        "red" => print!("{}", formatted.red()),
                        "green" => print!("{}", formatted.green()),
                        _ => print!("{}", formatted),
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_diff_identical_files() {
        let f1 = "/tmp/ms_diff_test1.txt";
        let f2 = "/tmp/ms_diff_test2.txt";
        let mut file1 = fs::File::create(f1).unwrap();
        let mut file2 = fs::File::create(f2).unwrap();
        writeln!(file1, "hello").unwrap();
        writeln!(file2, "hello").unwrap();

        let module = DiffModule;
        let cmd = module.command();
        let matches = cmd.get_matches_from(vec!["diff", f1, f2]);
        assert!(module.execute(&matches).is_ok());

        let _ = fs::remove_file(f1);
        let _ = fs::remove_file(f2);
    }

    #[test]
    fn test_diff_different_files() {
        let f1 = "/tmp/ms_diff_test3.txt";
        let f2 = "/tmp/ms_diff_test4.txt";
        let mut file1 = fs::File::create(f1).unwrap();
        let mut file2 = fs::File::create(f2).unwrap();
        writeln!(file1, "hello").unwrap();
        writeln!(file2, "world").unwrap();

        let module = DiffModule;
        let cmd = module.command();
        let matches = cmd.get_matches_from(vec!["diff", f1, f2]);
        assert!(module.execute(&matches).is_ok());

        let _ = fs::remove_file(f1);
        let _ = fs::remove_file(f2);
    }
}
