use crate::error::{MsError, MsResult};
use crate::tool_module::ToolModule;
use clap::{Arg, ArgMatches, Command};

#[cfg(feature = "http")]
use colored::*;

pub struct HttpModule;

impl ToolModule for HttpModule {
    fn name(&self) -> &'static str {
        "http"
    }

    fn command(&self) -> Command {
        Command::new("http")
            .about("Make HTTP requests")
            .arg(
                Arg::new("method")
                    .required(true)
                    .help("HTTP method (GET, POST, PUT, DELETE, PATCH, HEAD)"),
            )
            .arg(
                Arg::new("url")
                    .required(true)
                    .help("URL to request"),
            )
            .arg(
                Arg::new("body")
                    .value_name("BODY")
                    .help("Request body (for POST/PUT/PATCH)"),
            )
    }

    #[cfg(feature = "http")]
    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        let method = matches.get_one::<String>("method").unwrap();
        let url = matches.get_one::<String>("url").unwrap();
        let body = matches.get_one::<String>("body");

        let client = reqwest::blocking::Client::new();

        let request = match method.to_uppercase().as_str() {
            "GET" => client.get(url),
            "POST" => {
                let mut req = client.post(url);
                if let Some(b) = body {
                    req = req
                        .header("Content-Type", "application/json")
                        .body(b.clone());
                }
                req
            }
            "PUT" => {
                let mut req = client.put(url);
                if let Some(b) = body {
                    req = req
                        .header("Content-Type", "application/json")
                        .body(b.clone());
                }
                req
            }
            "DELETE" => client.delete(url),
            "PATCH" => {
                let mut req = client.patch(url);
                if let Some(b) = body {
                    req = req
                        .header("Content-Type", "application/json")
                        .body(b.clone());
                }
                req
            }
            "HEAD" => client.head(url),
            _ => {
                return Err(MsError::InvalidInput(format!(
                    "Unsupported method: {}",
                    method
                )))
            }
        };

        let response = request
            .send()
            .map_err(|e| MsError::Http(e.to_string()))?;

        // Print status
        let status = response.status();
        let status_color = if status.is_success() {
            format!("{} {}", status.as_u16(), status.canonical_reason().unwrap_or(""))
                .green()
                .bold()
        } else if status.is_client_error() {
            format!("{} {}", status.as_u16(), status.canonical_reason().unwrap_or(""))
                .yellow()
                .bold()
        } else {
            format!("{} {}", status.as_u16(), status.canonical_reason().unwrap_or(""))
                .red()
                .bold()
        };
        println!("{}", status_color);

        // Print headers
        println!("{}", "--- Headers ---".dimmed());
        for (key, value) in response.headers() {
            println!(
                "  {}: {}",
                key.as_str().cyan(),
                value.to_str().unwrap_or("<binary>")
            );
        }

        // Print body
        let body_text = response
            .text()
            .map_err(|e| MsError::Http(e.to_string()))?;

        if !body_text.is_empty() {
            println!("\n{}", "--- Body ---".dimmed());
            // Try to pretty-print JSON
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body_text) {
                if let Ok(pretty) = serde_json::to_string_pretty(&json) {
                    println!("{}", pretty);
                } else {
                    println!("{}", body_text);
                }
            } else {
                println!("{}", body_text);
            }
        }

        Ok(())
    }

    #[cfg(not(feature = "http"))]
    fn execute(&self, _matches: &ArgMatches) -> MsResult<()> {
        Err(MsError::Other(
            "HTTP feature not enabled. Rebuild with --features http".into(),
        ))
    }
}
