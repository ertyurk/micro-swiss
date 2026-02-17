use crate::error::{MsError, MsResult};
use crate::tool_module::ToolModule;
use clap::{Arg, ArgAction, ArgMatches, Command};
use colored::Colorize;
use std::fs;
use std::path::Path;

pub struct TreeModule;

impl ToolModule for TreeModule {
    fn name(&self) -> &'static str {
        "tree"
    }

    fn command(&self) -> Command {
        Command::new("tree")
            .about("Display directory tree (gitignore-aware)")
            .arg(
                Arg::new("path")
                    .value_name("PATH")
                    .help("Root directory (default: .)")
                    .default_value("."),
            )
            .arg(
                Arg::new("files")
                    .short('f')
                    .long("files")
                    .help("Show files too (default: directories only)")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("depth")
                    .short('L')
                    .long("depth")
                    .help("Max display depth")
                    .value_parser(clap::value_parser!(usize)),
            )
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        let path = matches
            .get_one::<String>("path")
            .map(|s| s.as_str())
            .unwrap_or(".");
        let show_files = matches.get_flag("files");
        let max_depth = matches
            .get_one::<usize>("depth")
            .copied()
            .unwrap_or(if show_files { 2 } else { usize::MAX });

        let root = Path::new(path);
        if !root.is_dir() {
            return Err(MsError::InvalidInput(format!(
                "'{}' is not a directory",
                path
            )));
        }

        let root_patterns = load_gitignore(root);

        println!("{}", path.blue().bold());

        let mut state = TreeState {
            max_depth,
            show_files,
            dir_count: 0,
            file_count: 0,
        };
        print_tree(root, "", 0, &root_patterns, &mut state)?;

        if show_files {
            println!("\n{} directories, {} files", state.dir_count, state.file_count);
        } else {
            println!("\n{} directories", state.dir_count);
        }

        Ok(())
    }
}

fn load_gitignore(dir: &Path) -> Vec<String> {
    let mut patterns = vec![".git".to_string()];
    read_gitignore_into(dir, &mut patterns);
    patterns
}

fn read_gitignore_into(dir: &Path, patterns: &mut Vec<String>) {
    let gitignore_path = dir.join(".gitignore");
    if let Ok(content) = fs::read_to_string(gitignore_path) {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with('!') {
                continue;
            }
            let pattern = line.trim_end_matches('/');
            if !patterns.contains(&pattern.to_string()) {
                patterns.push(pattern.to_string());
            }
        }
    }
}

fn should_ignore(name: &str, patterns: &[String]) -> bool {
    for pattern in patterns {
        if let Some(suffix) = pattern.strip_prefix('*') {
            if name.ends_with(suffix) {
                return true;
            }
        } else if name == pattern {
            return true;
        }
    }
    false
}

struct TreeState {
    max_depth: usize,
    show_files: bool,
    dir_count: usize,
    file_count: usize,
}

fn print_tree(
    dir: &Path,
    prefix: &str,
    depth: usize,
    parent_patterns: &[String],
    state: &mut TreeState,
) -> MsResult<()> {
    if depth >= state.max_depth {
        return Ok(());
    }

    // Inherit parent patterns and add any local .gitignore
    let mut patterns = parent_patterns.to_vec();
    read_gitignore_into(dir, &mut patterns);

    let mut entries: Vec<_> = fs::read_dir(dir)
        .map_err(MsError::Io)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            if should_ignore(&name, &patterns) {
                return false;
            }
            if !state.show_files {
                return e.path().is_dir();
            }
            true
        })
        .collect();

    entries.sort_by(|a, b| {
        let a_dir = a.path().is_dir();
        let b_dir = b.path().is_dir();
        match (a_dir, b_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.file_name().cmp(&b.file_name()),
        }
    });

    let count = entries.len();
    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == count - 1;
        let connector = if is_last { "└── " } else { "├── " };
        let child_prefix = if is_last { "    " } else { "│   " };

        let name = entry.file_name().to_string_lossy().to_string();
        let is_dir = entry.path().is_dir();

        if is_dir {
            state.dir_count += 1;
            println!("{}{}{}", prefix, connector, name.blue().bold());
            print_tree(
                &entry.path(),
                &format!("{}{}", prefix, child_prefix),
                depth + 1,
                &patterns,
                state,
            )?;
        } else {
            state.file_count += 1;
            println!("{}{}{}", prefix, connector, name);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_should_ignore_exact() {
        let patterns = vec![".git".to_string(), "node_modules".to_string()];
        assert!(should_ignore(".git", &patterns));
        assert!(should_ignore("node_modules", &patterns));
        assert!(!should_ignore("src", &patterns));
    }

    #[test]
    fn test_should_ignore_wildcard() {
        let patterns = vec!["*.swp".to_string(), "*.log".to_string()];
        assert!(should_ignore("file.swp", &patterns));
        assert!(should_ignore("app.log", &patterns));
        assert!(!should_ignore("main.rs", &patterns));
    }

    #[test]
    fn test_load_gitignore() {
        let dir = std::env::temp_dir().join("ms_test_gitignore");
        let _ = fs::create_dir_all(&dir);
        fs::write(
            dir.join(".gitignore"),
            "# comment\ntarget/\nnode_modules\n*.log\n\n!keep.log\n",
        )
        .unwrap();

        let patterns = load_gitignore(&dir);
        assert!(patterns.contains(&".git".to_string()));
        assert!(patterns.contains(&"target".to_string()));
        assert!(patterns.contains(&"node_modules".to_string()));
        assert!(patterns.contains(&"*.log".to_string()));
        assert!(!patterns.contains(&"!keep.log".to_string()));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_load_gitignore_no_file() {
        let dir = std::env::temp_dir().join("ms_test_no_gitignore");
        let _ = fs::create_dir_all(&dir);
        let _ = fs::remove_file(dir.join(".gitignore"));

        let patterns = load_gitignore(&dir);
        assert_eq!(patterns, vec![".git".to_string()]);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_nested_gitignore() {
        let root = std::env::temp_dir().join("ms_test_nested_gi");
        let sub = root.join("sub");
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&sub).unwrap();

        fs::write(root.join(".gitignore"), "*.log\n").unwrap();
        fs::write(sub.join(".gitignore"), "*.tmp\n").unwrap();

        let root_patterns = load_gitignore(&root);
        assert!(should_ignore("app.log", &root_patterns));
        assert!(!should_ignore("data.tmp", &root_patterns));

        let mut child_patterns = root_patterns.clone();
        read_gitignore_into(&sub, &mut child_patterns);
        assert!(should_ignore("app.log", &child_patterns));
        assert!(should_ignore("data.tmp", &child_patterns));

        let _ = fs::remove_dir_all(&root);
    }
}
