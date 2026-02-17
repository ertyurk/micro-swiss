use crate::error::{MsError, MsResult};
use crate::tool_module::ToolModule;
use crate::util::stdin;
use clap::{Arg, ArgMatches, Command};
use qrcode::{render::unicode, QrCode};

pub struct QrGenerateModule;

impl ToolModule for QrGenerateModule {
    fn name(&self) -> &'static str {
        "qr"
    }

    fn command(&self) -> Command {
        Command::new("qr")
            .about("Generate QR code as ASCII art")
            .arg(Arg::new("text").value_name("TEXT").help("Text or URL to encode"))
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        let text = stdin::get_text_input(matches.get_one::<String>("text"))
            .ok_or_else(|| MsError::InvalidInput("No text provided".into()))?;
        let qr_ascii = generate_qr_ascii(&text)?;
        println!("{}", qr_ascii);
        Ok(())
    }
}

fn generate_qr_ascii(text: &str) -> MsResult<String> {
    let code = QrCode::new(text)
        .map_err(|e| MsError::Other(format!("QR code error: {}", e)))?;
    let image = code
        .render::<unicode::Dense1x2>()
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
        let qr = result.unwrap();
        assert!(!qr.is_empty());
        assert!(qr.contains('█'));
    }

    #[test]
    fn test_generate_qr_ascii_url() {
        let result = generate_qr_ascii("https://github.com");
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_qr_ascii_empty() {
        let result = generate_qr_ascii("");
        assert!(result.is_ok());
    }

    #[test]
    fn test_qr_code_structure() {
        let result = generate_qr_ascii("test").unwrap();
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() > 10);
    }
}
