use crate::tool_module::ToolModuleBox;

// Include the auto-generated module registration code  
include!(concat!(env!("OUT_DIR"), "/generated_modules.rs"));

pub fn get_module_registry() -> ModuleRegistry {
    ModuleRegistry::new()
}

pub struct ModuleRegistry {
    modules: Vec<ToolModuleBox>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            modules: register_modules(),
        }
    }

    pub fn get_modules(&self) -> &[ToolModuleBox] {
        &self.modules
    }

}