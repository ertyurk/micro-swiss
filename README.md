# micro-swiss (`ms`)

A developer utility CLI written in Rust. 28 subcommands covering text processing, encoding, hashing, date math, file ops, networking, database connectivity, and more — with automatic module discovery and clipboard integration.

## Motivation

AI coding agents and developers repeatedly perform the same small operations — encoding, hashing, formatting, converting. `ms` puts them all behind one binary with zero startup cost.

## Installation

```bash
# Build optimized release binary
cargo build --release --all-features

# Binary is at target/release/ms
# Optionally install globally:
cargo install --path .
```

### Feature Flags

| Flag   | Includes                | Default          |
| ------ | ----------------------- | ---------------- |
| `db`   | PostgreSQL (`ms db`)    | yes (via `full`) |
| `http` | HTTP client (`ms http`) | yes (via `full`) |
| `full` | Both `db` + `http`      | yes              |

```bash
cargo build --release                    # everything
cargo build --release --no-default-features  # minimal, no db/http
cargo build --release --features db      # only db
```

## Commands

### Encoding & Decoding

```bash
ms base64 encode "hello"              # aGVsbG8=
ms base64 decode "aGVsbG8="           # hello
echo "hello" | ms base64 encode       # stdin works everywhere

ms url encode "hello world"           # hello+world
ms url decode "hello+world"           # hello world
ms url parse "https://example.com?a=1"  # structured JSON output
```

### Hashing & Checksums

```bash
ms hash "hello"                       # SHA256 (default), copied to clipboard
ms hash "hello" md5                   # MD5
echo "secret" | ms hash              # stdin

ms checksum file.txt                  # SHA256 file checksum
ms checksum file.txt md5              # MD5 file checksum
```

### Password & UUID

```bash
ms password                           # 16-char secure password, copied
ms password 32                        # custom length

ms uuid                               # UUID v4 (random), copied
ms uuid v7                            # UUID v7 (timestamp)
```

### Text Processing

```bash
ms case upper "hello world"           # HELLO WORLD
ms case camel "hello world"           # helloWorld
ms case snake "helloWorld"            # hello_world
ms case kebab "helloWorld"            # hello-world
ms case pascal "hello world"          # HelloWorld
ms case constant "hello world"        # HELLO_WORLD
ms case title "hello world"           # Hello World

ms flatten "line1\nline2"             # line1line2
echo -e "line1\nline2" | ms flatten   # stdin

ms inspect "hello 🌍"                # byte count, chars, lines, words, codepoints
```

### JSON

```bash
ms json pretty '{"a":1,"b":2}'       # pretty print, copied
ms json minify '{ "a": 1 }'          # minify, copied
echo '{"a":1}' | ms json pretty      # stdin
```

### Format Conversion

```bash
ms convert yaml '{"name":"test"}'     # JSON → YAML
ms convert json "name: test"          # YAML → JSON
ms convert toml '{"name":"test"}'     # JSON → TOML
```

### Date & Time

```bash
ms date add 01/01/2024 10            # 11/01/2024 (Thursday)
ms date sub 11/01/2024 10            # 01/01/2024 (Monday)
# Formats: DDMMYYYY, DD/MM/YYYY, DD-MM-YYYY

ms epoch                              # current epoch + UTC
ms epoch 1700000000                   # epoch → human date
ms epoch "2024-01-15"                 # date string → epoch
```

### Git & Dev Tools

```bash
ms branch "Fix: urgent bug"          # fix-urgent-bug, copied
echo "Feature Name" | ms branch      # stdin

ms run script.py                     # uv run
ms run app.js                        # node
ms run main.ts                       # deno run --allow-all
ms run main.go                       # go run
ms run script.py -- arg1 arg2        # pass arguments

ms filesize /path/to/file            # human-readable size, copied
ms filesize 1048576                  # 1.0 MB

ms regex '\d+' "abc123def456"        # match positions + capture groups
echo "long text" | ms regex '\w+'    # stdin for text
```

### Color

```bash
ms color "#ff0000"                   # all formats (hex, rgb, hsl)
ms color "rgb(255,0,0)" hex          # → #ff0000
ms color "hsl(0,100%,50%)" rgb       # → rgb(255,0,0)
```

### QR Code

```bash
ms qr "https://example.com"          # ASCII QR code in terminal
echo "hello" | ms qr                 # stdin
```

### JWT

```bash
ms jwt "eyJhbGci..."                 # decode header + payload, show expiration
```

### HTTP (feature: `http`)

```bash
ms http GET https://httpbin.org/get
ms http POST https://httpbin.org/post '{"key":"value"}'
ms http PUT https://api.example.com/resource '{"updated":true}'
ms http DELETE https://api.example.com/resource/1
```

### Database (feature: `db`)

```bash
ms db "postgres://user:pass@localhost:5432/mydb"
# Interactive SQL session with CSV output
# sql> SELECT * FROM users LIMIT 3;
# sql> exit
```

### File Diff

```bash
ms diff file1.txt file2.txt          # colored unified diff
```

### Networking

```bash
ms ip                                # local + public IP
ms ip local                          # local only
ms ip public                         # public only

ms port check 8080                   # OPEN or CLOSED
ms port listen 8080                  # listen for connections (Ctrl+C to stop)
```

### Cron

```bash
ms cron "*/5 * * * *"                # "At every 5 minutes of every hour"
ms cron "0 9 * * 1"                  # "At minute 0, hour 9, on Monday"
```

### Environment Files

```bash
ms env .env                          # display with sensitive values masked
# API_KEY=sk******* (red)
# DATABASE_URL=postgres://... (cyan)
```

### Shell Completions & Man Page

```bash
ms completions zsh >> ~/.zfunc/_ms   # generate completions
ms completions bash                  # bash completions
ms completions fish                  # fish completions
ms manpage > ms.1                    # generate man page
```

## Architecture

**Auto-discovery module system** — create a directory in `src/modules/` with a `mod.rs` implementing `ToolModule` and it's automatically registered at build time. No manual wiring needed.

```
src/
├── main.rs              # subcommand dispatch
├── error.rs             # MsError + MsResult
├── tool_module.rs       # ToolModule trait
├── module_registry.rs   # auto-generated registry
├── util/
│   ├── clipboard.rs     # copy_and_print helpers
│   └── stdin.rs         # stdin detection + reading
└── modules/
    ├── base64/          ├── jwt/
    ├── case_convert/    ├── http/
    ├── checksum/        ├── diff/
    ├── color_convert/   ├── convert/
    ├── convert_to_branch/ ├── epoch/
    ├── cron/            ├── ip/
    ├── date/            ├── inspect/
    ├── db_connect/      ├── port/
    ├── env/             ├── url/
    ├── file_size/       ├── uuid_generate/
    ├── flatten_text/    ├── qr_generate/
    ├── hash/            ├── regex_test/
    ├── json/            ├── run_file/
    └── password_gen/
```

### Adding a Module

```rust
// src/modules/my_tool/mod.rs
use crate::error::MsResult;
use crate::tool_module::ToolModule;
use clap::{ArgMatches, Command};

pub struct MyToolModule;

impl ToolModule for MyToolModule {
    fn name(&self) -> &'static str { "my-tool" }

    fn command(&self) -> Command {
        Command::new("my-tool")
            .about("Does something useful")
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        // implementation
        Ok(())
    }
}
```

Build. Done. It's discovered automatically.

## Development

```bash
cargo test --all-features             # 160 tests (127 unit + 33 integration)
cargo clippy --all-features -- -D warnings
cargo fmt
cargo build --release --all-features
```
