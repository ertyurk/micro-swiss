# MicroSwiss

A comprehensive collection of developer utility tools written in Rust with **automatic module discovery** and **clipboard integration**. This CLI application provides 17 essential developer tools including text processing, file operations, cryptographic utilities, data conversion, and more in a self-expanding modular architecture.

## 🚀 Installation & Setup

### Build Release Binary (Recommended)

```bash
# Build optimized release binary
cargo build --release

# The optimized binary will be at target/release/micro-swiss
```

### Add Global Shell Alias

For easy access from anywhere, add to your shell configuration:

**For zsh (`~/.zshrc`):**

```bash
# Add MicroSwiss alias for global access
alias ms='/path/to/micro-swiss/target/release/micro-swiss'

# Reload your shell config
source ~/.zshrc
```

**For bash (`~/.bashrc`):**

```bash
alias ms='/path/to/micro-swiss/target/release/micro-swiss'
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

### 🔐 Cryptographic & Security Tools

#### Password Generator (`-p, --password`)
Generate cryptographically secure passwords with automatic clipboard copy
```bash
ms -p           # Generate 16-char password (default)
ms -p 24        # Generate 24-char password
```

#### Hash Generator (`--hash`)
Generate MD5 or SHA256 hashes for text input
```bash
ms --hash "text to hash"        # SHA256 (default)
ms --hash "text to hash" md5    # MD5 hash
```

#### File Checksum (`--checksum`)
Calculate file checksums for integrity verification
```bash
ms --checksum file.txt          # SHA256 checksum (default)
ms --checksum file.txt md5      # MD5 checksum
```

#### UUID Generator (`--uuid-generate`)
Generate UUIDs for unique identifiers
```bash
ms --uuid-generate      # Generate UUID v4 (random)
ms --uuid-generate v7   # Generate UUID v7 (timestamp-based)
```

### 🎨 Text & Data Processing

#### Case Converter (`--case-convert`)
Convert text between multiple case formats
```bash
ms --case-convert "hello world" camel    # helloWorld
ms --case-convert "hello world" pascal   # HelloWorld
ms --case-convert "hello world" snake    # hello_world
ms --case-convert "hello world" kebab    # hello-world
ms --case-convert "hello world" constant # HELLO_WORLD
ms --case-convert "hello world" title    # Hello World
ms --case-convert "hello world" upper    # HELLO WORLD
ms --case-convert "hello world" lower    # hello world
```

#### Base64 Encoder (`-e, --encode`)
Encode strings to base64 format
```bash
ms -e "hello world"
# Output: aGVsbG8gd29ybGQ=
```

#### URL Encoder (`-u, --url-encode`)
URL encode strings for web use
```bash
ms -u "hello@world.com?test=true"
# Output: hello%40world.com%3Ftest%3Dtrue
```

#### Text Flattener (`-f, --flatten`)
Remove newlines from text input
```bash
ms -f "Line 1\nLine 2\nLine 3"    # From argument
echo -e "Line 1\nLine 2" | ms -f  # From stdin
```

### 🌐 Web & Data Tools

#### JSON Formatter (`--json-pretty`, `--json-minify`)
Format and minify JSON data
```bash
ms --json-pretty '{"name":"test","value":123}'   # Pretty print
ms --json-minify '{ "name" : "test" }'           # Minify
```

#### URL Parser (`--parse-url`)
Parse URLs into structured JSON components
```bash
ms --parse-url "https://example.com/path?param=value"
# Extracts protocol, domain, path, query parameters
```

#### Color Converter (`--color-convert`)
Convert between hex, RGB, and HSL color formats
```bash
ms --color-convert "#ff0000"          # Convert to all formats
ms --color-convert "rgb(255,0,0)" hex # Convert to specific format
ms --color-convert "hsl(0,100%,50%)"  # Auto-detect input format
```

#### QR Code Generator (`--qr-generate`)
Generate QR codes as ASCII art in terminal
```bash
ms --qr-generate "https://example.com"
ms --qr-generate "Hello World"
```

### 📅 Date & Time Tools

#### Date Calculator (`--date-add`, `--date-sub`)
Perform date arithmetic operations
```bash
ms --date-add "25/12/2023" 7    # Add 7 days
ms --date-sub "01-01-2024" 30   # Subtract 30 days
# Supports formats: DDMMYYYY, DD/MM/YYYY, DD-MM-YYYY
```

### 🔧 Development Tools

#### Branch Name Generator (`-g, --generate-branch`)
Convert strings to git-friendly branch names with automatic clipboard copy
```bash
ms -g "Feature Request Name"
# Output: feature-request-name (copied to clipboard)
```

#### Smart File Runner (`-r, --run`)
Execute files with automatic interpreter detection
```bash
ms -r script.py     # Python (uses uv)
ms -r app.js        # JavaScript (uses node)
ms -r main.ts       # TypeScript (uses deno)
ms -r main.go       # Go (uses go run)
ms -r app.mojo      # Mojo (uses mojo)
ms -r script.py arg1 arg2 --flag  # Pass arguments
```

#### File Size Calculator (`--file-size`)
Get human-readable file sizes or convert byte values
```bash
ms --file-size /path/to/file    # File size in human format
ms --file-size 1048576          # Convert bytes to readable format
```

#### Regex Tester (`--regex-test`)
Test regular expressions against text
```bash
ms --regex-test "\d+" "abc123def456"
# Shows matches with positions and capture groups
```

## 📋 Supported File Types

| Extension      | Runtime | Command                |
| -------------- | ------- | ---------------------- |
| `.py`          | uv      | `uv run`               |
| `.js`          | node    | `node`                 |
| `.ts`          | deno    | `deno run --allow-all` |
| `.go`          | go      | `go run`               |
| `.mojo`, `.🔥` | mojo    | `mojo`                 |

## 🏗️ Auto-Discovery Architecture

**Zero-configuration module system!** The project uses **automatic build-time module discovery**:

- **Drop & Go**: Create a new module directory in `src/modules/` and it's automatically discovered
- **No Registration**: No manual registration in any files required
- **Build-time Safety**: All modules verified at compile time
- **Clean Interface**: Each module implements the `ToolModule` trait

### Current Auto-Discovered Modules (17 total):

**Cryptographic & Security:**
- `password_gen/` - Password generation
- `hash/` - Text hashing (MD5/SHA256)
- `checksum/` - File checksum calculation
- `uuid_generate/` - UUID generation

**Text & Data Processing:**
- `case_convert/` - Text case conversion
- `base64_encode/` - Base64 encoding
- `url_encode/` - URL encoding
- `flatten_text/` - Text flattening

**Web & Data Tools:**
- `json_format/` - JSON formatting/minification
- `url_parse/` - URL parsing
- `color_convert/` - Color format conversion
- `qr_generate/` - QR code generation

**Date & Time:**
- `date_calc/` - Date arithmetic

**Development Tools:**
- `convert_to_branch/` - Git branch name generation
- `run_file/` - Smart file execution
- `file_size/` - File size calculation
- `regex_test/` - Regular expression testing

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
    
    fn configure_args(&self, cmd: Command) -> Command {
        cmd.arg(Arg::new("your-flag").short('y').long("your-flag"))
    }
    
    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        // Your implementation
        Ok(())
    }
}
```

## ✨ Key Features

- **🔒 Secure**: Cryptographically secure password generation and hashing
- **📋 Clipboard Integration**: Most commands automatically copy results to clipboard
- **🎨 Rich Text Processing**: Multiple case formats, encoding/decoding, formatting
- **🌐 Web Development**: URL parsing, color conversion, JSON formatting
- **📅 Date Utilities**: Date arithmetic with multiple format support
- **🧰 Developer Tools**: File execution, regex testing, branch naming
- **⚡ Performance**: Optimized Rust binary with minimal startup time
- **🔧 Modular**: Self-expanding architecture with automatic module discovery

## 🧪 Development

```bash
# Run tests for all modules
cargo test

# Run in development mode
cargo run -- --help
cargo run -- -g "test string"
cargo run -- --password 12
cargo run -- --hash "test text"

# Build optimized binary
cargo build --release

# Install locally for testing
cargo install --path .
```

## 📦 Dependencies

Key dependencies used by MicroSwiss:

- **clap** - Command-line argument parsing
- **arboard** - Clipboard integration
- **colored** - Terminal color output
- **chrono** - Date and time handling
- **serde/serde_json** - JSON serialization
- **regex** - Regular expression support
- **uuid** - UUID generation
- **md5/sha2** - Cryptographic hashing
- **qrcode** - QR code generation
- **rand** - Cryptographically secure random numbers
