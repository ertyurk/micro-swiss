use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_modules.rs");
    
    let modules_dir = Path::new("src/modules");
    let mut modules = Vec::new();
    
    if modules_dir.exists() && modules_dir.is_dir() {
        scan_modules_directory(modules_dir, &mut modules);
    }
    
    generate_module_code(&dest_path, &modules);
    
    // Tell Cargo to rerun this build script if modules directory changes
    println!("cargo:rerun-if-changed=src/modules");
}

fn scan_modules_directory(dir: &Path, modules: &mut Vec<ModuleInfo>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    let module_name = path.file_name().unwrap().to_str().unwrap();
                    
                    // Look for mod.rs in the module directory
                    let mod_file = path.join("mod.rs");
                    if mod_file.exists() {
                        if let Some(module_info) = extract_module_info(&mod_file, module_name) {
                            modules.push(module_info);
                        }
                    }
                }
            }
        }
    }
}

fn extract_module_info(mod_file: &Path, module_name: &str) -> Option<ModuleInfo> {
    let content = fs::read_to_string(mod_file).ok()?;
    
    // Look for struct that implements ToolModule
    // This is a simple pattern matching approach
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("pub struct ") && line.ends_with(";") {
            let struct_name = line
                .strip_prefix("pub struct ")?
                .strip_suffix(";")?;
            
            // Check if there's an impl ToolModule for this struct
            if content.contains(&format!("impl ToolModule for {}", struct_name)) {
                return Some(ModuleInfo {
                    module_name: module_name.to_string(),
                    struct_name: struct_name.to_string(),
                });
            }
        }
    }
    
    None
}

#[derive(Debug)]
struct ModuleInfo {
    module_name: String,
    struct_name: String,
}

fn generate_module_code(dest_path: &Path, modules: &[ModuleInfo]) {
    let mut file = fs::File::create(dest_path).unwrap();
    
    // Create a modules.rs file that includes all discovered modules
    let modules_dest = Path::new(&env::var("OUT_DIR").unwrap()).join("modules.rs");
    let mut modules_file = fs::File::create(&modules_dest).unwrap();
    
    // Generate module declarations for modules.rs
    writeln!(modules_file, "// Auto-generated module declarations").unwrap();
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    for module in modules {
        writeln!(modules_file, "#[path = \"{}/src/modules/{}/mod.rs\"]", cargo_manifest_dir, module.module_name).unwrap();
        writeln!(modules_file, "pub mod {};", module.module_name).unwrap();
    }
    
    // Generate imports for main file  
    writeln!(file, "// Auto-generated imports").unwrap();
    for module in modules {
        writeln!(
            file,
            "use crate::{}::{};",
            module.module_name, module.struct_name
        ).unwrap();
    }
    
    writeln!(file).unwrap();
    
    // Generate registration function
    writeln!(file, "// Auto-generated module registration").unwrap();
    writeln!(file, "pub fn register_modules() -> Vec<ToolModuleBox> {{").unwrap();
    writeln!(file, "    vec![").unwrap();
    
    for module in modules {
        writeln!(file, "        Box::new({}),", module.struct_name).unwrap();
    }
    
    writeln!(file, "    ]").unwrap();
    writeln!(file, "}}").unwrap();
    
    writeln!(file).unwrap();
    
    // Generate automatic command detection function
    writeln!(file, "// Auto-generated command detection").unwrap();
    writeln!(file, "pub fn get_all_command_ids() -> Vec<&'static str> {{").unwrap();
    writeln!(file, "    vec![").unwrap();
    
    // Extract command IDs from module files
    for module in modules {
        if let Some(command_ids) = extract_command_ids(&format!("src/modules/{}/mod.rs", module.module_name)) {
            for id in command_ids {
                writeln!(file, "        \"{}\",", id).unwrap();
            }
        }
    }
    
    writeln!(file, "    ]").unwrap();
    writeln!(file, "}}").unwrap();
}

fn extract_command_ids(file_path: &str) -> Option<Vec<String>> {
    let content = fs::read_to_string(file_path).ok()?;
    let mut command_ids = Vec::new();
    
    for line in content.lines() {
        let line = line.trim();
        if line.contains(".arg(") && line.contains("Arg::new(") {
            if let Some(start) = line.find("Arg::new(\"") {
                if let Some(end) = line[start + 10..].find("\"") {
                    let command_id = &line[start + 10..start + 10 + end];
                    command_ids.push(command_id.to_string());
                }
            }
        }
    }
    
    Some(command_ids)
}