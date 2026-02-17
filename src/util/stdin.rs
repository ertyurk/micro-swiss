use std::io::{self, IsTerminal, Read};

pub fn stdin_has_data() -> bool {
    !io::stdin().is_terminal()
}

pub fn read_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn get_text_input(arg: Option<&String>) -> Option<String> {
    if let Some(text) = arg {
        Some(text.clone())
    } else if stdin_has_data() {
        read_stdin().ok().map(|s| s.trim_end().to_string())
    } else {
        None
    }
}
