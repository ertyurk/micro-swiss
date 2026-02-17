use arboard::Clipboard;

pub fn copy_and_print(text: &str) {
    match Clipboard::new() {
        Ok(mut clipboard) => {
            if let Err(e) = clipboard.set_text(text) {
                eprintln!("Warning: Failed to copy to clipboard: {}", e);
                println!("{}", text);
            } else {
                println!("{} (copied to clipboard)", text);
            }
        }
        Err(e) => {
            eprintln!("Warning: Failed to access clipboard: {}", e);
            println!("{}", text);
        }
    }
}

pub fn copy_and_print_block(text: &str) {
    match Clipboard::new() {
        Ok(mut clipboard) => {
            if let Err(e) = clipboard.set_text(text) {
                eprintln!("Warning: Failed to copy to clipboard: {}", e);
                println!("{}", text);
            } else {
                println!("{}\n(copied to clipboard)", text);
            }
        }
        Err(e) => {
            eprintln!("Warning: Failed to access clipboard: {}", e);
            println!("{}", text);
        }
    }
}
