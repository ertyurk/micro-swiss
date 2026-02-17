use crate::error::{MsError, MsResult};
use crate::tool_module::ToolModule;
use chrono::{Datelike, NaiveDate, Weekday};
use clap::{Arg, ArgMatches, Command};

pub struct DateModule;

impl ToolModule for DateModule {
    fn name(&self) -> &'static str {
        "date"
    }

    fn command(&self) -> Command {
        Command::new("date")
            .about("Date arithmetic (add/subtract days)")
            .subcommand_required(true)
            .subcommand(
                Command::new("add")
                    .about("Add days to a date")
                    .arg(
                        Arg::new("date")
                            .required(true)
                            .help("Date (DDMMYYYY, DD/MM/YYYY, or DD-MM-YYYY)"),
                    )
                    .arg(
                        Arg::new("days")
                            .required(true)
                            .help("Number of days to add"),
                    ),
            )
            .subcommand(
                Command::new("sub")
                    .about("Subtract days from a date")
                    .arg(
                        Arg::new("date")
                            .required(true)
                            .help("Date (DDMMYYYY, DD/MM/YYYY, or DD-MM-YYYY)"),
                    )
                    .arg(
                        Arg::new("days")
                            .required(true)
                            .help("Number of days to subtract"),
                    ),
            )
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        match matches.subcommand() {
            Some(("add", sub_m)) => {
                let date_str = sub_m.get_one::<String>("date").unwrap();
                let days: i64 = sub_m.get_one::<String>("days").unwrap().parse()?;
                let date = parse_date(date_str)?;
                let new_date = date + chrono::Duration::days(days);
                println!(
                    "{} ({})",
                    format_date_output(new_date),
                    format_weekday(new_date.weekday())
                );
            }
            Some(("sub", sub_m)) => {
                let date_str = sub_m.get_one::<String>("date").unwrap();
                let days: i64 = sub_m.get_one::<String>("days").unwrap().parse()?;
                let date = parse_date(date_str)?;
                let new_date = date - chrono::Duration::days(days);
                println!(
                    "{} ({})",
                    format_date_output(new_date),
                    format_weekday(new_date.weekday())
                );
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}

fn parse_date(date_str: &str) -> MsResult<NaiveDate> {
    if date_str.len() == 8 && date_str.chars().all(|c| c.is_numeric()) {
        let day: u32 = date_str[0..2].parse()?;
        let month: u32 = date_str[2..4].parse()?;
        let year: i32 = date_str[4..8].parse()?;
        return NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| MsError::InvalidInput("Invalid date".into()));
    }

    if let Some(date) = try_parse_with_separator(date_str, '/') {
        return Ok(date);
    }
    if let Some(date) = try_parse_with_separator(date_str, '-') {
        return Ok(date);
    }

    Err(MsError::InvalidInput(
        "Invalid date format. Use DDMMYYYY, DD/MM/YYYY, or DD-MM-YYYY".into(),
    ))
}

fn try_parse_with_separator(date_str: &str, separator: char) -> Option<NaiveDate> {
    let parts: Vec<&str> = date_str.split(separator).collect();
    if parts.len() == 3 {
        if let (Ok(day), Ok(month), Ok(year)) = (
            parts[0].parse::<u32>(),
            parts[1].parse::<u32>(),
            parts[2].parse::<i32>(),
        ) {
            return NaiveDate::from_ymd_opt(year, month, day);
        }
    }
    None
}

fn format_date_output(date: NaiveDate) -> String {
    format!("{:02}/{:02}/{}", date.day(), date.month(), date.year())
}

fn format_weekday(weekday: Weekday) -> String {
    match weekday {
        Weekday::Mon => "Monday".to_string(),
        Weekday::Tue => "Tuesday".to_string(),
        Weekday::Wed => "Wednesday".to_string(),
        Weekday::Thu => "Thursday".to_string(),
        Weekday::Fri => "Friday".to_string(),
        Weekday::Sat => "Saturday".to_string(),
        Weekday::Sun => "Sunday".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date_ddmmyyyy() {
        assert_eq!(
            parse_date("01012023").unwrap(),
            NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()
        );
    }

    #[test]
    fn test_parse_date_with_slashes() {
        assert_eq!(
            parse_date("01/01/2023").unwrap(),
            NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()
        );
    }

    #[test]
    fn test_parse_date_with_dashes() {
        assert_eq!(
            parse_date("01-01-2023").unwrap(),
            NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()
        );
    }

    #[test]
    fn test_invalid_date_formats() {
        assert!(parse_date("2023-01-01").is_err());
        assert!(parse_date("invalid").is_err());
    }

    #[test]
    fn test_format_date_output() {
        let date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        assert_eq!(format_date_output(date), "01/01/2023");
    }

    #[test]
    fn test_format_weekday() {
        assert_eq!(format_weekday(Weekday::Mon), "Monday");
        assert_eq!(format_weekday(Weekday::Sun), "Sunday");
    }
}
