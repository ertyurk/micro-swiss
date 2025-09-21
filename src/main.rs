use clap::Command;
use std::process;

mod tool_module;
mod module_registry;

// Include auto-generated modules  
include!(concat!(env!("OUT_DIR"), "/modules.rs"));

use module_registry::get_module_registry;

fn main() {
    let registry = get_module_registry();
    
    let mut cmd = Command::new("my-shadow")
        .version("0.1.0")
        .about("A collection of utility tools");

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
        eprintln!("Please specify a command. Use --help for usage information.");
        process::exit(1);
    }
}