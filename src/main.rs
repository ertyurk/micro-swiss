use clap::Command;
use std::process;

mod module_registry;
mod tool_module;

// Include auto-generated modules
include!(concat!(env!("OUT_DIR"), "/modules.rs"));

use module_registry::get_module_registry;

fn main() {
    let registry = get_module_registry();

    let mut cmd = Command::new("my-shadow")
        .version("0.1.0")
        .about("A collection of utility tools for developers")
        .after_help("For more information about each command, use --help with the specific option.")
        .after_long_help(
            "EXAMPLES:
    # Base64 encoding/decoding
    my-shadow --encode \"Hello World\"
    my-shadow --decode \"SGVsbG8gV29ybGQ=\"

    # URL encoding/decoding
    my-shadow --url-encode \"hello world & test\"
    my-shadow --url-decode \"hello+world+%26+test\"

    # Password generation
    my-shadow --password 12        # Generate 12-char password
    my-shadow -p                   # Generate 16-char password (default)

    # Git branch name conversion
    my-shadow --generate-branch \"Fix: urgent bug with user data\"

    # Text flattening (remove newlines)
    my-shadow --flatten \"line1\\nline2\\nline3\"
    echo -e \"line1\\nline2\" | my-shadow --flatten

    # Run files by extension
    my-shadow --run script.py
    my-shadow --run main.go"
        );

    for module in registry.get_modules() {
        cmd = module.configure_args(cmd);
    }

    let matches = cmd.get_matches();

    let mut executed = false;
    for module in registry.get_modules() {
        if let Err(e) = module.execute(&matches) {
            eprintln!("Error executing module {}: {}", module.name(), e);
            process::exit(1);
        }

        // Check if any command was executed using auto-discovered command IDs
        let command_ids = module_registry::get_all_command_ids();
        for &cmd_id in &command_ids {
            if matches.contains_id(cmd_id) {
                executed = true;
                break;
            }
        }
    }

    if !executed {
        // eprintln!("Please specify a command. Use --help for usage information.");
        process::exit(1);
    }
}
