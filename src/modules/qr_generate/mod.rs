use crate::tool_module::ToolModule;
use clap::{Arg, ArgMatches, Command};
use qrcode::{QrCode, render::unicode};
use std::error::Error;

pub struct QrGenerateModule;

impl ToolModule for QrGenerateModule {
    fn name(&self) -> &'static str {
        "qr-generate"
    }

    fn configure_args(&self, cmd: Command) -> Command {
        cmd.arg(
            Arg::new("qr-generate")
                .long("qr-generate")
                .value_name("TEXT")
                .help("Generate QR code for text or URL")
                .long_help("Generate a QR code as ASCII art for the given text or URL. Perfect for terminal display.")
        )
    }

    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        if let Some(text) = matches.get_one::<String>("qr-generate") {
            let qr_ascii = generate_qr_ascii(text)?;
            println!("{}", qr_ascii);
        }
        Ok(())
    }
}

fn generate_qr_ascii(text: &str) -> Result<String, Box<dyn Error>> {
    let code = QrCode::new(text)?;
    let image = code.render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    Ok(image)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_qr_ascii_simple() {
        let result = generate_qr_ascii("Hello");
        assert!(result.is_ok());
        let qr_code = result.unwrap();
        assert!(!qr_code.is_empty());
        // QR code should contain the typical border patterns
        assert!(qr_code.contains("█"));
    }

    #[test]
    fn test_generate_qr_ascii_url() {
        let result = generate_qr_ascii("https://github.com");
        assert!(result.is_ok());
        let qr_code = result.unwrap();
        assert!(!qr_code.is_empty());
        assert!(qr_code.contains("█"));
    }

    #[test]
    fn test_generate_qr_ascii_empty() {
        let result = generate_qr_ascii("");
        assert!(result.is_ok());
        let qr_code = result.unwrap();
        assert!(!qr_code.is_empty());
    }

    #[test]
    fn test_generate_qr_ascii_long_text() {
        let long_text = "This is a very long text that will test the QR code generation with more data content to ensure it works properly with larger inputs";
        let result = generate_qr_ascii(long_text);
        assert!(result.is_ok());
        let qr_code = result.unwrap();
        assert!(!qr_code.is_empty());
        // Longer text should result in larger QR code
        assert!(qr_code.len() > 100);
    }

    #[test]
    fn test_generate_qr_ascii_special_chars() {
        let special_text = "Hello 🌍! Special chars: @#$%^&*()";
        let result = generate_qr_ascii(special_text);
        assert!(result.is_ok());
        let qr_code = result.unwrap();
        assert!(!qr_code.is_empty());
        assert!(qr_code.contains("█"));
    }

    #[test]
    fn test_generate_qr_ascii_json() {
        let json_text = r#"{"name":"John","age":30,"city":"New York"}"#;
        let result = generate_qr_ascii(json_text);
        assert!(result.is_ok());
        let qr_code = result.unwrap();
        assert!(!qr_code.is_empty());
        assert!(qr_code.contains("█"));
    }

    #[test]
    fn test_qr_code_structure() {
        let result = generate_qr_ascii("test");
        assert!(result.is_ok());
        let qr_code = result.unwrap();
        
        // QR code should be multi-line
        let lines: Vec<&str> = qr_code.lines().collect();
        assert!(lines.len() > 10);
        
        // Each line should have some width
        for line in &lines {
            if !line.is_empty() {
                assert!(line.len() > 5);
            }
        }
    }
}