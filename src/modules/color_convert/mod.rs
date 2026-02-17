use crate::error::{MsError, MsResult};
use crate::tool_module::ToolModule;
use crate::util::clipboard;
use clap::{Arg, ArgMatches, Command};

pub struct ColorConvertModule;

impl ToolModule for ColorConvertModule {
    fn name(&self) -> &'static str {
        "color"
    }

    fn command(&self) -> Command {
        Command::new("color")
            .about("Convert colors between hex/rgb/hsl formats")
            .arg(
                Arg::new("value")
                    .required(true)
                    .help("Color value (#ff0000, rgb(255,0,0), hsl(0,100%,50%))"),
            )
            .arg(
                Arg::new("format")
                    .value_name("FORMAT")
                    .help("Target format: hex, rgb, or hsl"),
            )
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        let color_str = matches.get_one::<String>("value").unwrap();
        let target_format = matches.get_one::<String>("format").map(|s| s.as_str());

        let color = parse_color(color_str)?;
        let result = match target_format {
            Some("hex") => color.to_hex(),
            Some("rgb") => color.to_rgb_string(),
            Some("hsl") => color.to_hsl_string(),
            _ => format!(
                "HEX: {}\nRGB: {}\nHSL: {}",
                color.to_hex(),
                color.to_rgb_string(),
                color.to_hsl_string()
            ),
        };

        clipboard::copy_and_print_block(&result);
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

    #[must_use]
    fn to_hex(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    #[must_use]
    fn to_rgb_string(self) -> String {
        format!("rgb({},{},{})", self.r, self.g, self.b)
    }

    fn to_hsl(self) -> (f32, f32, f32) {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;
        let l = (max + min) / 2.0;

        if delta == 0.0 {
            return (0.0, 0.0, l * 100.0);
        }

        let s = if l < 0.5 {
            delta / (max + min)
        } else {
            delta / (2.0 - max - min)
        };

        let h = if max == r {
            ((g - b) / delta + if g < b { 6.0 } else { 0.0 }) * 60.0
        } else if max == g {
            ((b - r) / delta + 2.0) * 60.0
        } else {
            ((r - g) / delta + 4.0) * 60.0
        };

        (h, s * 100.0, l * 100.0)
    }

    #[must_use]
    fn to_hsl_string(self) -> String {
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
            if t < 0.0 {
                t += 1.0;
            }
            if t > 1.0 {
                t -= 1.0;
            }
            if t < 1.0 / 6.0 {
                return p + (q - p) * 6.0 * t;
            }
            if t < 1.0 / 2.0 {
                return q;
            }
            if t < 2.0 / 3.0 {
                return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
            }
            p
        };

        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;

        let r = (hue_to_rgb(p, q, h + 1.0 / 3.0) * 255.0).round() as u8;
        let g = (hue_to_rgb(p, q, h) * 255.0).round() as u8;
        let b = (hue_to_rgb(p, q, h - 1.0 / 3.0) * 255.0).round() as u8;

        Color::new(r, g, b)
    }
}

fn parse_color(color_str: &str) -> MsResult<Color> {
    let color_str = color_str.trim();
    if let Ok(color) = parse_hex(color_str) {
        return Ok(color);
    }
    if let Ok(color) = parse_rgb(color_str) {
        return Ok(color);
    }
    if let Ok(color) = parse_hsl(color_str) {
        return Ok(color);
    }
    Err(MsError::InvalidInput(
        "Invalid color format. Use hex (#ff0000), rgb (255,0,0), or hsl (0,100%,50%)".into(),
    ))
}

fn parse_hex(hex_str: &str) -> MsResult<Color> {
    let hex_str = hex_str.trim_start_matches('#');
    if hex_str.len() != 6 {
        return Err(MsError::InvalidInput(
            "Hex color must be 6 characters long".into(),
        ));
    }
    let r = u8::from_str_radix(&hex_str[0..2], 16)
        .map_err(|_| MsError::InvalidInput("Invalid hex".into()))?;
    let g = u8::from_str_radix(&hex_str[2..4], 16)
        .map_err(|_| MsError::InvalidInput("Invalid hex".into()))?;
    let b = u8::from_str_radix(&hex_str[4..6], 16)
        .map_err(|_| MsError::InvalidInput("Invalid hex".into()))?;
    Ok(Color::new(r, g, b))
}

fn parse_rgb(rgb_str: &str) -> MsResult<Color> {
    let rgb_str = rgb_str.trim();
    let inner = if rgb_str.starts_with("rgb(") && rgb_str.ends_with(')') {
        &rgb_str[4..rgb_str.len() - 1]
    } else {
        rgb_str
    };
    let parts: Vec<&str> = inner.split(',').collect();
    if parts.len() != 3 {
        return Err(MsError::InvalidInput("RGB format requires 3 values".into()));
    }
    let r = parts[0].trim().parse::<u8>()?;
    let g = parts[1].trim().parse::<u8>()?;
    let b = parts[2].trim().parse::<u8>()?;
    Ok(Color::new(r, g, b))
}

fn parse_hsl(hsl_str: &str) -> MsResult<Color> {
    let hsl_str = hsl_str.trim();
    let inner = if hsl_str.starts_with("hsl(") && hsl_str.ends_with(')') {
        &hsl_str[4..hsl_str.len() - 1]
    } else {
        return Err(MsError::InvalidInput("HSL format not recognized".into()));
    };
    let parts: Vec<&str> = inner.split(',').collect();
    if parts.len() != 3 {
        return Err(MsError::InvalidInput("HSL format requires 3 values".into()));
    }
    let h = parts[0].trim().parse::<f32>()?;
    let s = parts[1].trim().trim_end_matches('%').parse::<f32>()?;
    let l = parts[2].trim().trim_end_matches('%').parse::<f32>()?;
    Ok(Color::from_hsl(h, s, l))
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
        assert_eq!(color.g, 255);
    }

    #[test]
    fn test_parse_rgb() {
        let color = parse_rgb("rgb(255,0,0)").unwrap();
        assert_eq!(color.r, 255);

        let color = parse_rgb("0,255,0").unwrap();
        assert_eq!(color.g, 255);
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
    }

    #[test]
    fn test_hsl_to_rgb_conversion() {
        let color = Color::from_hsl(240.0, 100.0, 50.0);
        assert_eq!(color.to_hex(), "#0000ff");
    }

    #[test]
    fn test_invalid_formats() {
        assert!(parse_hex("#ff00").is_err());
        assert!(parse_rgb("255,0").is_err());
        assert!(parse_hsl("hsl(0,100%)").is_err());
    }
}
