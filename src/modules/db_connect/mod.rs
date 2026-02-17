use crate::error::{MsError, MsResult};
use crate::tool_module::ToolModule;
use clap::{Arg, ArgMatches, Command};

#[cfg(feature = "db")]
use csv::Writer;
#[cfg(feature = "db")]
use std::io::{self, Write};
#[cfg(feature = "db")]
use tokio_postgres::{Client, NoTls};
#[cfg(feature = "db")]
use url::Url;

pub struct DbConnectModule;

impl ToolModule for DbConnectModule {
    fn name(&self) -> &'static str {
        "db"
    }

    fn command(&self) -> Command {
        Command::new("db")
            .about("Connect to a PostgreSQL database")
            .arg(
                Arg::new("connection")
                    .required(true)
                    .help("Connection string (postgres://user:pass@host:port/db)"),
            )
    }

    #[cfg(feature = "db")]
    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        let connection_string = matches.get_one::<String>("connection").unwrap();

        let parsed_url = Url::parse(connection_string)
            .map_err(|e| MsError::Database(format!("Invalid connection string: {}", e)))?;

        if parsed_url.scheme() != "postgres" && parsed_url.scheme() != "postgresql" {
            return Err(MsError::Database(
                "Only PostgreSQL connections are supported".into(),
            ));
        }

        println!("Connecting to PostgreSQL database...");
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async {
            let client = establish_connection(connection_string).await?;
            println!("Connected successfully!");
            println!("Interactive SQL session started. Type 'exit' or 'quit' to end.\n");
            interactive_session(client).await
        })
    }

    #[cfg(not(feature = "db"))]
    fn execute(&self, _matches: &ArgMatches) -> MsResult<()> {
        Err(MsError::Other(
            "Database feature not enabled. Rebuild with --features db".into(),
        ))
    }
}

#[cfg(feature = "db")]
async fn establish_connection(connection_string: &str) -> MsResult<Client> {
    let (client, connection) = tokio_postgres::connect(connection_string, NoTls)
        .await
        .map_err(|e| MsError::Database(e.to_string()))?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    Ok(client)
}

#[cfg(feature = "db")]
async fn interactive_session(client: Client) -> MsResult<()> {
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
            println!("Goodbye!");
            break;
        }

        match execute_query_as_csv(&client, query).await {
            Ok(csv_output) => {
                if csv_output.trim().is_empty() {
                    println!("Query executed successfully. No results returned.");
                } else {
                    println!("{}", csv_output);
                }
            }
            Err(e) => {
                eprintln!("Query error: {}", e);
            }
        }
    }

    Ok(())
}

#[cfg(feature = "db")]
async fn execute_query_as_csv(client: &Client, query: &str) -> MsResult<String> {
    let rows = client
        .query(query, &[])
        .await
        .map_err(|e| MsError::Database(e.to_string()))?;

    if rows.is_empty() {
        return Ok(String::new());
    }

    let mut csv_output = Vec::new();
    {
        let mut writer = Writer::from_writer(&mut csv_output);

        let columns = rows[0].columns();
        let headers: Vec<&str> = columns.iter().map(|col| col.name()).collect();
        writer
            .write_record(headers)
            .map_err(|e| MsError::Other(e.to_string()))?;

        for row in &rows {
            let mut record = Vec::new();
            for (i, column) in columns.iter().enumerate() {
                let value = match column.type_() {
                    &tokio_postgres::types::Type::INT4 => row
                        .try_get::<_, Option<i32>>(i)
                        .map_err(|e| MsError::Database(e.to_string()))?
                        .map_or(String::new(), |v| v.to_string()),
                    &tokio_postgres::types::Type::INT8 => row
                        .try_get::<_, Option<i64>>(i)
                        .map_err(|e| MsError::Database(e.to_string()))?
                        .map_or(String::new(), |v| v.to_string()),
                    &tokio_postgres::types::Type::FLOAT4 => row
                        .try_get::<_, Option<f32>>(i)
                        .map_err(|e| MsError::Database(e.to_string()))?
                        .map_or(String::new(), |v| v.to_string()),
                    &tokio_postgres::types::Type::FLOAT8 => row
                        .try_get::<_, Option<f64>>(i)
                        .map_err(|e| MsError::Database(e.to_string()))?
                        .map_or(String::new(), |v| v.to_string()),
                    &tokio_postgres::types::Type::TEXT
                    | &tokio_postgres::types::Type::VARCHAR => row
                        .try_get::<_, Option<String>>(i)
                        .map_err(|e| MsError::Database(e.to_string()))?
                        .unwrap_or_default(),
                    &tokio_postgres::types::Type::BOOL => row
                        .try_get::<_, Option<bool>>(i)
                        .map_err(|e| MsError::Database(e.to_string()))?
                        .map_or(String::new(), |v| v.to_string()),
                    &tokio_postgres::types::Type::TIMESTAMP
                    | &tokio_postgres::types::Type::TIMESTAMPTZ => row
                        .try_get::<_, Option<chrono::NaiveDateTime>>(i)
                        .map_err(|e| MsError::Database(e.to_string()))?
                        .map_or(String::new(), |v| v.to_string()),
                    _ => row
                        .try_get::<_, Option<String>>(i)
                        .unwrap_or(None)
                        .unwrap_or_default(),
                };
                record.push(value);
            }
            writer
                .write_record(record)
                .map_err(|e| MsError::Other(e.to_string()))?;
        }

        writer.flush()?;
    }

    String::from_utf8(csv_output).map_err(|e| MsError::Parse(e.to_string()))
}
