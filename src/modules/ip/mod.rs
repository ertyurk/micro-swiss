use crate::error::{MsError, MsResult};
use crate::tool_module::ToolModule;
use clap::{Arg, ArgMatches, Command};
use std::io::{Read, Write};
use std::net::{TcpStream, UdpSocket};
use std::time::Duration;

pub struct IpModule;

impl ToolModule for IpModule {
    fn name(&self) -> &'static str {
        "ip"
    }

    fn command(&self) -> Command {
        Command::new("ip")
            .about("Show local or public IP address")
            .arg(
                Arg::new("type")
                    .value_name("TYPE")
                    .help("'local' or 'public' (default: both)")
                    .default_value("both"),
            )
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        let ip_type = matches
            .get_one::<String>("type")
            .map(|s| s.as_str())
            .unwrap_or("both");

        match ip_type {
            "local" => {
                let ip = get_local_ip()?;
                println!("Local: {}", ip);
            }
            "public" => {
                let ip = get_public_ip()?;
                println!("Public: {}", ip);
            }
            _ => {
                if let Ok(ip) = get_local_ip() {
                    println!("Local:  {}", ip);
                }
                match get_public_ip() {
                    Ok(ip) => println!("Public: {}", ip),
                    Err(e) => eprintln!("Public: unavailable ({})", e),
                }
            }
        }

        Ok(())
    }
}

fn get_local_ip() -> MsResult<String> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("8.8.8.8:80")?;
    let addr = socket.local_addr()?;
    Ok(addr.ip().to_string())
}

fn get_public_ip() -> MsResult<String> {
    let request = "GET / HTTP/1.1\r\nHost: api.ipify.org\r\nConnection: close\r\n\r\n";

    let mut stream = TcpStream::connect_timeout(
        &"api.ipify.org:80".parse().unwrap(),
        Duration::from_secs(5),
    )
    .map_err(|e| MsError::Http(format!("Cannot connect to ipify: {}", e)))?;

    stream.write_all(request.as_bytes())?;
    stream.flush()?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    // Extract body from HTTP response
    if let Some(body_start) = response.find("\r\n\r\n") {
        let body = response[body_start + 4..].trim().to_string();
        if !body.is_empty() {
            return Ok(body);
        }
    }

    Err(MsError::Http("Could not determine public IP".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_local_ip() {
        let result = get_local_ip();
        // May fail in CI without network, but should work locally
        if let Ok(ip) = result {
            assert!(!ip.is_empty());
            assert!(ip.contains('.'));
        }
    }
}
