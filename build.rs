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

    println!("cargo:rerun-if-changed=src/modules");
}

fn scan_modules_directory(dir: &Path, modules: &mut Vec<ModuleInfo>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let module_name = path.file_name().unwrap().to_str().unwrap();

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

fn extract_module_info(mod_file: &Path, module_name: &str) -> Option<ModuleInfo> {
    let content = fs::read_to_string(mod_file).ok()?;

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("pub struct ") && line.ends_with(';') {
            let struct_name = line.strip_prefix("pub struct ")?.strip_suffix(';')?;

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

fn feature_gate(module_name: &str) -> Option<&'static str> {
    match module_name {
        "db_connect" => Some("db"),
        "http" => Some("http"),
        _ => None,
    }
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

    writeln!(modules_file, "// Auto-generated module declarations").unwrap();
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    for module in modules {
        let gate = feature_gate(&module.module_name);
        if let Some(feature) = gate {
            writeln!(modules_file, "#[cfg(feature = \"{}\")]", feature).unwrap();
        }
        writeln!(
            modules_file,
            "#[path = \"{}/src/modules/{}/mod.rs\"]",
            cargo_manifest_dir, module.module_name
        )
        .unwrap();
        writeln!(modules_file, "pub mod {};", module.module_name).unwrap();
    }

    // Generate imports
    writeln!(file, "// Auto-generated imports").unwrap();
    for module in modules {
        let gate = feature_gate(&module.module_name);
        if let Some(feature) = gate {
            writeln!(file, "#[cfg(feature = \"{}\")]", feature).unwrap();
        }
        writeln!(
            file,
            "use crate::{}::{};",
            module.module_name, module.struct_name
        )
        .unwrap();
    }

    writeln!(file).unwrap();

    // Generate registration function
    writeln!(file, "#[allow(clippy::vec_init_then_push)]").unwrap();
    writeln!(file, "// Auto-generated module registration").unwrap();
    writeln!(file, "pub fn register_modules() -> Vec<ToolModuleBox> {{").unwrap();
    writeln!(
        file,
        "    let mut modules: Vec<ToolModuleBox> = Vec::new();"
    )
    .unwrap();

    for module in modules {
        let gate = feature_gate(&module.module_name);
        if let Some(feature) = gate {
            writeln!(file, "    #[cfg(feature = \"{}\")]", feature).unwrap();
        }
        writeln!(file, "    modules.push(Box::new({}));", module.struct_name).unwrap();
    }

    writeln!(file, "    modules").unwrap();
    writeln!(file, "}}").unwrap();
}
