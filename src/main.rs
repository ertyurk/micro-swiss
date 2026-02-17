use clap::Command;
use clap_complete::{generate, Shell};
use std::process;

mod error;
mod module_registry;
mod tool_module;
mod util;

// Include auto-generated modules
include!(concat!(env!("OUT_DIR"), "/modules.rs"));

use module_registry::get_module_registry;

fn main() {
    let registry = get_module_registry();

    let mut cmd = Command::new("ms")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A collection of utility tools for developers")
        .subcommand_required(true)
        .arg_required_else_help(true);

    for module in registry.get_modules() {
        cmd = cmd.subcommand(module.command());
    }

    // Hidden completions subcommand
    cmd = cmd.subcommand(
        Command::new("completions")
            .about("Generate shell completions")
            .hide(true)
            .arg(
                clap::Arg::new("shell")
                    .required(true)
                    .value_parser(clap::value_parser!(Shell)),
            ),
    );

    // Hidden manpage subcommand
    cmd = cmd.subcommand(
        Command::new("manpage")
            .about("Generate man page")
            .hide(true),
    );

    let matches = cmd.clone().get_matches();

    match matches.subcommand() {
        Some(("completions", sub_m)) => {
            let shell = sub_m.get_one::<Shell>("shell").unwrap();
            generate(*shell, &mut cmd, "ms", &mut std::io::stdout());
        }
        Some(("manpage", _)) => {
            let man = clap_mangen::Man::new(cmd);
            let mut buf = Vec::new();
            if let Err(e) = man.render(&mut buf) {
                eprintln!("Error generating man page: {}", e);
                process::exit(1);
            }
            if let Err(e) = std::io::Write::write_all(&mut std::io::stdout(), &buf) {
                eprintln!("Error writing man page: {}", e);
                process::exit(1);
            }
        }
        Some((name, sub_m)) => {
            for module in registry.get_modules() {
                if module.name() == name {
                    if let Err(e) = module.execute(sub_m) {
                        eprintln!("Error: {}", e);
                        process::exit(1);
                    }
                    return;
                }
            }
            eprintln!("Unknown command: {}", name);
            process::exit(1);
        }
        None => {
            process::exit(1);
        }
    }
}
