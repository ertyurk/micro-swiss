use crate::error::{MsError, MsResult};
use crate::tool_module::ToolModule;
use clap::{Arg, ArgMatches, Command};
use chrono::Utc;

pub struct EpochModule;

impl ToolModule for EpochModule {
    fn name(&self) -> &'static str {
        "epoch"
    }

    fn command(&self) -> Command {
        Command::new("epoch")
            .about("Convert between epoch timestamps and human-readable dates")
            .arg(
                Arg::new("input")
                    .value_name("TS|now")
                    .help("'now' for current epoch, number for epoch→date, or omit for now")
                    .default_value("now"),
            )
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        let input = matches.get_one::<String>("input").unwrap();

        if input == "now" {
            let now = Utc::now();
            println!("Epoch:   {}", now.timestamp());
            println!("Millis:  {}", now.timestamp_millis());
            println!("UTC:     {}", now.format("%Y-%m-%d %H:%M:%S UTC"));
        } else if let Ok(ts) = input.parse::<i64>() {
            // Detect seconds vs milliseconds
            let (secs, label) = if ts > 1_000_000_000_000 {
                (ts / 1000, "milliseconds")
            } else {
                (ts, "seconds")
            };

            let dt = chrono::DateTime::from_timestamp(secs, 0).ok_or_else(|| {
                MsError::InvalidInput("Invalid epoch timestamp".into())
            })?;

            println!("Input:   {} ({})", ts, label);
            println!("UTC:     {}", dt.format("%Y-%m-%d %H:%M:%S UTC"));
            println!("Local:   {}", dt.with_timezone(&chrono::Local).format("%Y-%m-%d %H:%M:%S %Z"));

            let now = Utc::now();
            let diff = now.signed_duration_since(dt);
            if diff.num_seconds() > 0 {
                println!("Ago:     {}", format_duration(diff));
            } else {
                println!("In:      {}", format_duration(-diff));
            }
        } else {
            // Try parsing as date string
            if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S") {
                let utc = dt.and_utc();
                println!("Epoch:   {}", utc.timestamp());
                println!("Millis:  {}", utc.timestamp_millis());
            } else if let Ok(d) = chrono::NaiveDate::parse_from_str(input, "%Y-%m-%d") {
                let dt = d.and_hms_opt(0, 0, 0).unwrap().and_utc();
                println!("Epoch:   {}", dt.timestamp());
                println!("Millis:  {}", dt.timestamp_millis());
            } else {
                return Err(MsError::InvalidInput(
                    "Could not parse input. Use epoch number, 'now', or YYYY-MM-DD".into(),
                ));
            }
        }

        Ok(())
    }
}

fn format_duration(d: chrono::Duration) -> String {
    let total_secs = d.num_seconds();
    let days = total_secs / 86400;
    let hours = (total_secs % 86400) / 3600;
    let mins = (total_secs % 3600) / 60;

    if days > 0 {
        format!("{}d {}h {}m", days, hours, mins)
    } else if hours > 0 {
        format!("{}h {}m", hours, mins)
    } else {
        format!("{}m", mins)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        assert_eq!(
            format_duration(chrono::Duration::seconds(90)),
            "1m"
        );
        assert_eq!(
            format_duration(chrono::Duration::seconds(3661)),
            "1h 1m"
        );
        assert_eq!(
            format_duration(chrono::Duration::seconds(90061)),
            "1d 1h 1m"
        );
    }
}
