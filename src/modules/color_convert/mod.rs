use crate::tool_module::ToolModule;
use arboard::Clipboard;
use clap::{Arg, ArgMatches, Command};
use std::error::Error;

pub struct ColorConvertModule;

impl ToolModule for ColorConvertModule {
    fn name(&self) -> &'static str {
        "color-convert"
    }

    fn configure_args(&self, cmd: Command) -> Command {
        cmd.arg(
            Arg::new("color-convert")
                .long("color-convert")
                .value_names(["COLOR", "FORMAT"])
                .num_args(1..=2)
                .help("Convert colors between hex/rgb/hsl formats")
                .long_help("Convert colors between different formats:\n- hex: #ff0000 or ff0000\n- rgb: rgb(255,0,0) or 255,0,0\n- hsl: hsl(0,100%,50%)\n\nIf no target format is specified, converts to all formats. Result is automatically copied to clipboard.")
        )
    }

    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        if let Some(values) = matches.get_many::<String>("color-convert") {
            let values: Vec<&String> = values.collect();
            let color_str = values[0];
            let target_format = values.get(1).map(|s| s.as_str());
            
            let color = parse_color(color_str)?;
            let result = match target_format {
                Some("hex") => color.to_hex(),
                Some("rgb") => color.to_rgb_string(),
                Some("hsl") => color.to_hsl_string(),
                _ => format!("HEX: {}\nRGB: {}\nHSL: {}", 
                    color.to_hex(), 
                    color.to_rgb_string(), 
                    color.to_hsl_string()
                ),
            };
            
            copy_to_clipboard_and_print(&result);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    
    fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
    
    fn to_rgb_string(&self) -> String {
        format!("rgb({},{},{})", self.r, self.g, self.b)
    }
    
    fn to_hsl(&self) -> (f32, f32, f32) {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;
        
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;
        
        // Lightness
        let l = (max + min) / 2.0;
        
        if delta == 0.0 {
            return (0.0, 0.0, l * 100.0);
        }
        
        // Saturation
        let s = if l < 0.5 {
            delta / (max + min)
        } else {
            delta / (2.0 - max - min)
        };
        
        // Hue
        let h = if max == r {
            ((g - b) / delta + if g < b { 6.0 } else { 0.0 }) * 60.0
        } else if max == g {
            ((b - r) / delta + 2.0) * 60.0
        } else {
            ((r - g) / delta + 4.0) * 60.0
        };
        
        (h, s * 100.0, l * 100.0)
    }
    
    fn to_hsl_string(&self) -> String {
        let (h, s, l) = self.to_hsl();
        format!("hsl({:.0},{:.0}%,{:.0}%)", h, s, l)
    }
    
    fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        let h = h / 360.0;
        let s = s / 100.0;
        let l = l / 100.0;
        
        if s == 0.0 {
            let gray = (l * 255.0).round() as u8;
            return Color::new(gray, gray, gray);
        }
        
        let hue_to_rgb = |p: f32, q: f32, t: f32| -> f32 {
            let mut t = t;
            if t < 0.0 { t += 1.0; }
            if t > 1.0 { t -= 1.0; }
            if t < 1.0/6.0 { return p + (q - p) * 6.0 * t; }
            if t < 1.0/2.0 { return q; }
            if t < 2.0/3.0 { return p + (q - p) * (2.0/3.0 - t) * 6.0; }
            p
        };
        
        let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
        let p = 2.0 * l - q;
        
        let r = (hue_to_rgb(p, q, h + 1.0/3.0) * 255.0).round() as u8;
        let g = (hue_to_rgb(p, q, h) * 255.0).round() as u8;
        let b = (hue_to_rgb(p, q, h - 1.0/3.0) * 255.0).round() as u8;
        
        Color::new(r, g, b)
    }
}

fn parse_color(color_str: &str) -> Result<Color, Box<dyn Error>> {
    let color_str = color_str.trim();
    
    // Try hex format
    if let Ok(color) = parse_hex(color_str) {
        return Ok(color);
    }
    
    // Try RGB format
    if let Ok(color) = parse_rgb(color_str) {
        return Ok(color);
    }
    
    // Try HSL format
    if let Ok(color) = parse_hsl(color_str) {
        return Ok(color);
    }
    
    Err("Invalid color format. Use hex (#ff0000), rgb (255,0,0), or hsl (0,100%,50%)".into())
}

fn parse_hex(hex_str: &str) -> Result<Color, Box<dyn Error>> {
    let hex_str = hex_str.trim_start_matches('#');
    
    if hex_str.len() != 6 {
        return Err("Hex color must be 6 characters long".into());
    }
    
    let r = u8::from_str_radix(&hex_str[0..2], 16)?;
    let g = u8::from_str_radix(&hex_str[2..4], 16)?;
    let b = u8::from_str_radix(&hex_str[4..6], 16)?;
    
    Ok(Color::new(r, g, b))
}

fn parse_rgb(rgb_str: &str) -> Result<Color, Box<dyn Error>> {
    let rgb_str = rgb_str.trim();
    let rgb_str = if rgb_str.starts_with("rgb(") && rgb_str.ends_with(')') {
        &rgb_str[4..rgb_str.len()-1]
    } else {
        rgb_str
    };
    
    let parts: Vec<&str> = rgb_str.split(',').collect();
    if parts.len() != 3 {
        return Err("RGB format requires 3 values".into());
    }
    
    let r = parts[0].trim().parse::<u8>()?;
    let g = parts[1].trim().parse::<u8>()?;
    let b = parts[2].trim().parse::<u8>()?;
    
    Ok(Color::new(r, g, b))
}

fn parse_hsl(hsl_str: &str) -> Result<Color, Box<dyn Error>> {
    let hsl_str = hsl_str.trim();
    let hsl_str = if hsl_str.starts_with("hsl(") && hsl_str.ends_with(')') {
        &hsl_str[4..hsl_str.len()-1]
    } else {
        return Err("HSL format not recognized".into());
    };
    
    let parts: Vec<&str> = hsl_str.split(',').collect();
    if parts.len() != 3 {
        return Err("HSL format requires 3 values".into());
    }
    
    let h = parts[0].trim().parse::<f32>()?;
    let s = parts[1].trim().trim_end_matches('%').parse::<f32>()?;
    let l = parts[2].trim().trim_end_matches('%').parse::<f32>()?;
    
    Ok(Color::from_hsl(h, s, l))
}

fn copy_to_clipboard_and_print(text: &str) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex() {
        let color = parse_hex("#ff0000").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
        
        let color = parse_hex("00ff00").unwrap();
        assert_eq!(color.r, 0);
        assert_eq!(color.g, 255);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_parse_rgb() {
        let color = parse_rgb("rgb(255,0,0)").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
        
        let color = parse_rgb("0,255,0").unwrap();
        assert_eq!(color.r, 0);
        assert_eq!(color.g, 255);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_parse_hsl() {
        let color = parse_hsl("hsl(0,100%,50%)").unwrap();
        assert_eq!(color.to_hex(), "#ff0000");
        
        let color = parse_hsl("hsl(120,100%,50%)").unwrap();
        assert_eq!(color.to_hex(), "#00ff00");
    }

    #[test]
    fn test_color_conversions() {
        let red = Color::new(255, 0, 0);
        assert_eq!(red.to_hex(), "#ff0000");
        assert_eq!(red.to_rgb_string(), "rgb(255,0,0)");
        assert_eq!(red.to_hsl_string(), "hsl(0,100%,50%)");
        
        let green = Color::new(0, 255, 0);
        assert_eq!(green.to_hex(), "#00ff00");
        assert_eq!(green.to_hsl_string(), "hsl(120,100%,50%)");
    }

    #[test]
    fn test_hsl_to_rgb_conversion() {
        let color = Color::from_hsl(240.0, 100.0, 50.0); // Blue
        assert_eq!(color.to_hex(), "#0000ff");
        
        let color = Color::from_hsl(60.0, 100.0, 50.0); // Yellow
        assert_eq!(color.to_hex(), "#ffff00");
    }

    #[test]
    fn test_grayscale() {
        let gray = Color::new(128, 128, 128);
        let (_h, s, l) = gray.to_hsl();
        assert_eq!(s, 0.0); // No saturation for gray
        assert!((l - 50.2).abs() < 1.0); // Approximately 50% lightness
    }

    #[test]
    fn test_invalid_formats() {
        assert!(parse_hex("#ff00").is_err()); // Too short
        assert!(parse_rgb("255,0").is_err()); // Missing blue
        assert!(parse_hsl("hsl(0,100%)").is_err()); // Missing lightness
    }
}