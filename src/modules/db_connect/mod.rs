use crate::tool_module::ToolModule;
use clap::{Arg, ArgMatches, Command};
use std::error::Error;
use std::io::{self, Write};
use tokio_postgres::{Client, NoTls};
use url::Url;
use csv::Writer;
use std::sync::{Arc, Mutex, OnceLock};
use std::collections::HashMap;

pub struct DbConnectModule;

// Global session storage
static DB_SESSIONS: OnceLock<Arc<Mutex<HashMap<String, Arc<Mutex<Client>>>>>> = OnceLock::new();

impl ToolModule for DbConnectModule {
    fn name(&self) -> &'static str {
        "db-connect"
    }

    fn configure_args(&self, cmd: Command) -> Command {
        cmd.arg(
            Arg::new("connect")
                .long("connect")
                .short('c')
                .value_name("CONNECTION_STRING")
                .help("Connect to a PostgreSQL database and start interactive session")
                .long_help("Connect to a PostgreSQL database using a connection string format like:\npostgres://username:password@host:port/database\nExample: postgres://postgres:mysecretpassword@localhost:5432/medusa")
                .num_args(1)
        )
    }

    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        if let Some(connection_string) = matches.get_one::<String>("connect") {
            // Parse and validate the connection string
            let parsed_url = Url::parse(connection_string)
                .map_err(|e| format!("Invalid connection string format: {}", e))?;
            
            // Check if it's a PostgreSQL connection
            if parsed_url.scheme() != "postgres" && parsed_url.scheme() != "postgresql" {
                return Err("Only PostgreSQL connections are supported. Use postgres:// or postgresql:// scheme".into());
            }

            // Connect to database
            println!("Connecting to PostgreSQL database...");
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(async {
                match establish_connection(connection_string).await {
                    Ok(client) => {
                        println!("✅ Connected successfully to PostgreSQL database!");
                        println!("Interactive SQL session started. Type your queries below.");
                        println!("Results will be displayed in CSV format.");
                        println!("Type 'exit' or 'quit' to end the session.\n");

                        // Store the client in global session storage
                        let sessions = get_sessions();
                        let mut sessions_lock = sessions.lock().unwrap();
                        sessions_lock.insert("current".to_string(), Arc::new(Mutex::new(client)));
                        drop(sessions_lock);

                        // Start interactive session
                        interactive_session().await?;
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to connect to database: {}", e);
                        return Err(e);
                    }
                }
                Ok::<(), Box<dyn Error>>(())
            })?;
        }
        Ok(())
    }
}

fn get_sessions() -> &'static Arc<Mutex<HashMap<String, Arc<Mutex<Client>>>>> {
    DB_SESSIONS.get_or_init(|| Arc::new(Mutex::new(HashMap::new())))
}

async fn establish_connection(connection_string: &str) -> Result<Client, Box<dyn Error>> {
    let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await?;
    
    // Spawn the connection task
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });
    
    Ok(client)
}

async fn interactive_session() -> Result<(), Box<dyn Error>> {
    let sessions = get_sessions();
    
    loop {
        print!("sql> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let query = input.trim();
        
        if query.is_empty() {
            continue;
        }
        
        if query.eq_ignore_ascii_case("exit") || query.eq_ignore_ascii_case("quit") {
            println!("Goodbye! 👋");
            break;
        }
        
        // Get the current client
        let sessions_lock = sessions.lock().unwrap();
        if let Some(client_arc) = sessions_lock.get("current") {
            let client = client_arc.clone();
            drop(sessions_lock);
            
            // Execute query and format as CSV
            match execute_query_as_csv(&client, query).await {
                Ok(csv_output) => {
                    if csv_output.trim().is_empty() {
                        println!("Query executed successfully. No results returned.");
                    } else {
                        println!("{}", csv_output);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Query error: {}", e);
                }
            }
        } else {
            eprintln!("❌ No active database connection");
            break;
        }
    }
    
    Ok(())
}

async fn execute_query_as_csv(client_arc: &Arc<Mutex<Client>>, query: &str) -> Result<String, Box<dyn Error>> {
    let client = client_arc.lock().unwrap();
    
    // Execute the query
    let rows = client.query(query, &[]).await?;
    
    if rows.is_empty() {
        return Ok(String::new());
    }
    
    // Prepare CSV writer
    let mut csv_output = Vec::new();
    {
        let mut writer = Writer::from_writer(&mut csv_output);
        
        // Write headers
        let columns = rows[0].columns();
        let headers: Vec<&str> = columns.iter().map(|col| col.name()).collect();
        writer.write_record(headers)?;
        
        // Write data rows
        for row in &rows {
            let mut record = Vec::new();
            for (i, column) in columns.iter().enumerate() {
                let value = match column.type_() {
                    &tokio_postgres::types::Type::INT4 => {
                        row.try_get::<_, Option<i32>>(i)?.map_or("".to_string(), |v| v.to_string())
                    }
                    &tokio_postgres::types::Type::INT8 => {
                        row.try_get::<_, Option<i64>>(i)?.map_or("".to_string(), |v| v.to_string())
                    }
                    &tokio_postgres::types::Type::FLOAT4 => {
                        row.try_get::<_, Option<f32>>(i)?.map_or("".to_string(), |v| v.to_string())
                    }
                    &tokio_postgres::types::Type::FLOAT8 => {
                        row.try_get::<_, Option<f64>>(i)?.map_or("".to_string(), |v| v.to_string())
                    }
                    &tokio_postgres::types::Type::TEXT | &tokio_postgres::types::Type::VARCHAR => {
                        row.try_get::<_, Option<String>>(i)?.unwrap_or_default()
                    }
                    &tokio_postgres::types::Type::BOOL => {
                        row.try_get::<_, Option<bool>>(i)?.map_or("".to_string(), |v| v.to_string())
                    }
                    &tokio_postgres::types::Type::TIMESTAMP | &tokio_postgres::types::Type::TIMESTAMPTZ => {
                        row.try_get::<_, Option<chrono::NaiveDateTime>>(i)?.map_or("".to_string(), |v| v.to_string())
                    }
                    _ => {
                        // For other types, try to get as string or return empty
                        row.try_get::<_, Option<String>>(i).unwrap_or(None).unwrap_or_default()
                    }
                };
                record.push(value);
            }
            writer.write_record(record)?;
        }
        
        writer.flush()?;
    }
    
    Ok(String::from_utf8(csv_output)?)
}