use crate::error::{MsError, MsResult};
use crate::tool_module::ToolModule;
use clap::{Arg, ArgMatches, Command};

pub struct JwtModule;

impl ToolModule for JwtModule {
    fn name(&self) -> &'static str {
        "jwt"
    }

    fn command(&self) -> Command {
        Command::new("jwt")
            .about("Decode and inspect a JWT token")
            .arg(
                Arg::new("token")
                    .required(true)
                    .help("JWT token to decode"),
            )
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        let token = matches.get_one::<String>("token").unwrap();
        decode_jwt(token)
    }
}

fn decode_jwt(token: &str) -> MsResult<()> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(MsError::InvalidInput(
            "Invalid JWT: expected 3 parts separated by '.'".into(),
        ));
    }

    let header = base64url_decode(parts[0])?;
    let payload = base64url_decode(parts[1])?;

    let header_json: serde_json::Value = serde_json::from_str(&header)?;
    let payload_json: serde_json::Value = serde_json::from_str(&payload)?;

    println!("--- Header ---");
    println!("{}", serde_json::to_string_pretty(&header_json)?);

    println!("\n--- Payload ---");
    println!("{}", serde_json::to_string_pretty(&payload_json)?);

    // Show expiration as human-readable date
    if let Some(exp) = payload_json.get("exp").and_then(|v| v.as_i64()) {
        let dt = chrono::DateTime::from_timestamp(exp, 0);
        if let Some(dt) = dt {
            let now = chrono::Utc::now();
            let status = if dt < now { "EXPIRED" } else { "valid" };
            println!("\n--- Expiration ---");
            println!("{} ({})", dt.format("%Y-%m-%d %H:%M:%S UTC"), status);
        }
    }

    if let Some(iat) = payload_json.get("iat").and_then(|v| v.as_i64()) {
        if let Some(dt) = chrono::DateTime::from_timestamp(iat, 0) {
            println!(
                "Issued at: {}",
                dt.format("%Y-%m-%d %H:%M:%S UTC")
            );
        }
    }

    println!("\n--- Signature ---");
    println!("{}", parts[2]);

    Ok(())
}

fn base64url_decode(input: &str) -> MsResult<String> {
    // Convert base64url to standard base64
    let mut b64 = input.replace('-', "+").replace('_', "/");
    // Add padding
    match b64.len() % 4 {
        2 => b64.push_str("=="),
        3 => b64.push('='),
        _ => {}
    }

    // Decode using our base64 logic
    const CHARS: &str =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut result = Vec::new();
    let input_chars: Vec<char> = b64.chars().collect();

    for chunk in input_chars.chunks(4) {
        let mut buf = [0u8; 4];
        let mut valid_chars = 0;

        for (i, &c) in chunk.iter().enumerate() {
            if c == '=' {
                break;
            }
            if let Some(pos) = CHARS.find(c) {
                buf[i] = pos as u8;
                valid_chars += 1;
            }
        }

        if valid_chars == 0 {
            break;
        }

        let b = ((buf[0] as u32) << 18)
            | ((buf[1] as u32) << 12)
            | ((buf[2] as u32) << 6)
            | (buf[3] as u32);

        result.push(((b >> 16) & 0xFF) as u8);
        if valid_chars > 2 {
            result.push(((b >> 8) & 0xFF) as u8);
        }
        if valid_chars > 3 {
            result.push((b & 0xFF) as u8);
        }
    }

    String::from_utf8(result)
        .map_err(|e| MsError::InvalidInput(format!("Invalid UTF-8 in JWT: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64url_decode() {
        let result = base64url_decode("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9");
        assert!(result.is_ok());
        let decoded = result.unwrap();
        assert!(decoded.contains("alg"));
        assert!(decoded.contains("HS256"));
    }

    #[test]
    fn test_decode_jwt_valid() {
        // Standard test JWT
        let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.\
                      eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.\
                      SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        assert!(decode_jwt(token).is_ok());
    }

    #[test]
    fn test_decode_jwt_invalid_parts() {
        assert!(decode_jwt("not.a.valid.jwt.token").is_err());
        assert!(decode_jwt("only_one_part").is_err());
    }
}
