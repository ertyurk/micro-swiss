use crate::error::{MsError, MsResult};
use crate::tool_module::ToolModule;
use clap::{Arg, ArgMatches, Command};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

pub struct PortModule;

impl ToolModule for PortModule {
    fn name(&self) -> &'static str {
        "port"
    }

    fn command(&self) -> Command {
        Command::new("port")
            .about("Check if a port is open or listen on it")
            .subcommand_required(true)
            .subcommand(
                Command::new("check")
                    .about("Check if a port is open")
                    .arg(
                        Arg::new("port")
                            .required(true)
                            .help("Port number to check"),
                    ),
            )
            .subcommand(
                Command::new("listen")
                    .about("Listen on a port (press Ctrl+C to stop)")
                    .arg(
                        Arg::new("port")
                            .required(true)
                            .help("Port number to listen on"),
                    ),
            )
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        match matches.subcommand() {
            Some(("check", sub_m)) => {
                let port: u16 = sub_m
                    .get_one::<String>("port")
                    .unwrap()
                    .parse()
                    .map_err(|_| {
                        MsError::InvalidInput("Invalid port number".into())
                    })?;

                let addr = format!("127.0.0.1:{}", port);
                match TcpStream::connect_timeout(
                    &addr.parse().unwrap(),
                    Duration::from_secs(2),
                ) {
                    Ok(_) => println!("Port {} is OPEN", port),
                    Err(_) => println!("Port {} is CLOSED", port),
                }
            }
            Some(("listen", sub_m)) => {
                let port: u16 = sub_m
                    .get_one::<String>("port")
                    .unwrap()
                    .parse()
                    .map_err(|_| {
                        MsError::InvalidInput("Invalid port number".into())
                    })?;

                let addr = format!("127.0.0.1:{}", port);
                let listener = TcpListener::bind(&addr).map_err(|e| {
                    MsError::Other(format!("Cannot bind to port {}: {}", port, e))
                })?;

                println!("Listening on {}...", addr);
                println!("Press Ctrl+C to stop.");

                for stream in listener.incoming() {
                    match stream {
                        Ok(s) => {
                            println!(
                                "Connection from {}",
                                s.peer_addr()
                                    .map(|a| a.to_string())
                                    .unwrap_or_else(|_| "unknown".into())
                            );
                        }
                        Err(e) => {
                            eprintln!("Connection error: {}", e);
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_closed_port() {
        // Port 19999 is almost certainly not in use
        let addr = "127.0.0.1:19999";
        let result =
            TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_millis(100));
        assert!(result.is_err());
    }
}
