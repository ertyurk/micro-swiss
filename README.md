# my-shadow

A collection of utility tools written in Rust with **automatic module discovery**. This CLI application provides various text processing and file execution utilities in a self-expanding modular architecture.

## 🚀 Installation & Setup

### Build Release Binary (Recommended)
```bash
# Build optimized release binary
cargo build --release

# The optimized binary will be at target/release/my-shadow
```

### Add Global Shell Alias
For easy access from anywhere, add to your shell configuration:

**For zsh (`~/.zshrc`):**
```bash
# Add my-shadow alias for global access
alias shadow='/path/to/my-shadow/target/release/my-shadow'

# Reload your shell config
source ~/.zshrc
```

**For bash (`~/.bashrc`):**
```bash
alias shadow='/path/to/my-shadow/target/release/my-shadow'
source ~/.bashrc
```

### Usage with Alias
```bash
# Now use 'ms' from anywhere  
ms -g "Feature Request Name"  # Generate branch name (auto-copied!)
ms -f "multi\nline text"      # Flatten text
ms -e "hello world"           # Base64 encode
ms -u "test@example.com"      # URL encode
ms -r script.py               # Run file
```

## 🛠️ Available Tools

### 🌿 Branch Name Generator (`-g, --generate-branch`)
Convert strings to git-friendly branch names with **automatic clipboard copy**
```bash
ms -g "Feature Request Name"
# Output: feature-request-name (copied to clipboard)
```

✨ **Auto-clipboard**: The generated branch name is automatically copied to your clipboard for instant use!

### 📝 Text Flattener (`-f, --flatten`)  
Remove newlines from text input
```bash
# From argument
shadow -f "Line 1\nLine 2\nLine 3"
# Output: Line 1Line 2Line 3

# From stdin
echo -e "Line 1\nLine 2" | shadow -f
# Output: Line 1Line 2
```

### 🔐 Base64 Encoder (`-e, --encode`)
Encode strings to base64 format
```bash
shadow -e "hello world"
# Output: aGVsbG8gd29ybGQ=
```

### 🌐 URL Encoder (`-u, --url-encode`)
URL encode strings for web use
```bash
shadow -u "hello@world.com?test=true"
# Output: hello%40world.com%3Ftest%3Dtrue
```

### 🚀 Smart File Runner (`-r, --run`)
Execute files with automatic interpreter detection
```bash
# Python (uses uv)
shadow -r script.py

# JavaScript (uses node)
shadow -r app.js

# TypeScript (uses deno)  
shadow -r main.ts

# Go (uses go run)
shadow -r main.go

# Mojo (uses mojo)
shadow -r app.mojo

# Pass arguments to the executed file
shadow -r script.py arg1 arg2 --flag
```

## 📋 Supported File Types

| Extension | Runtime | Command |
|-----------|---------|---------|
| `.py` | uv | `uv run` |
| `.js` | node | `node` |
| `.ts` | deno | `deno run --allow-all` |
| `.go` | go | `go run` |
| `.mojo`, `.🔥` | mojo | `mojo` |

## 🏗️ Auto-Discovery Architecture

**Zero-configuration module system!** The project uses **automatic build-time module discovery**:

- **Drop & Go**: Create a new module directory in `src/modules/` and it's automatically discovered
- **No Registration**: No manual registration in any files required  
- **Build-time Safety**: All modules verified at compile time
- **Clean Interface**: Each module implements the `ToolModule` trait

### Current Auto-Discovered Modules:
- `convert_to_branch/` - Branch name generation
- `flatten_text/` - Text flattening
- `run_file/` - Smart file execution  
- `base64_encode/` - Base64 encoding
- `url_encode/` - URL encoding

### Adding New Modules
1. Create directory: `src/modules/your_module/`
2. Create `mod.rs` with a struct implementing `ToolModule`
3. Build - your module is automatically discovered and registered!

```rust
// src/modules/your_module/mod.rs
use crate::tool_module::ToolModule;
use clap::{Arg, ArgMatches, Command};
use std::error::Error;

pub struct YourModule;

impl ToolModule for YourModule {
    fn name(&self) -> &'static str { "your-module" }
    fn description(&self) -> &'static str { "Your module description" }
    fn configure_args(&self, cmd: Command) -> Command {
        cmd.arg(Arg::new("your-flag").short('y').long("your-flag"))
    }
    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        // Your implementation
        Ok(())
    }
    fn handles_subcommand(&self, subcommand: &str) -> bool {
        subcommand == "your-flag"
    }
}
```

## ⚡ Performance Notes

- **Release builds** are significantly faster than debug builds
- **Shell aliases** provide instant access without path lookup
- **Static linking** means no runtime dependencies
- **Minimal startup time** with optimized Rust binary

## 🧪 Development

```bash
# Run tests for all modules
cargo test

# Test specific module
cargo test -p my-shadow --test base64_encode

# Run in development mode
cargo run -- --help
cargo run -- -g "test string"

# Build optimized binary
cargo build --release
```

## 📦 Dependencies

- `clap` - Command line argument parsing with derive features
- `colored` - Terminal color output for file runner
- `arboard` - Cross-platform clipboard access for auto-copy feature

## 🔧 Legacy Shell Scripts

The repository includes shell script versions for reference:
- `convert_to_branch_name.sh` - Branch name conversion
- `make_params.sh` - Text flattening  
- `runner_helper.sh` - File execution