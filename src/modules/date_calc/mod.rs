use crate::tool_module::ToolModule;
use clap::{Arg, ArgMatches, Command};
use std::error::Error;
use chrono::{NaiveDate, Datelike, Weekday};

pub struct DateCalcModule;

impl ToolModule for DateCalcModule {
    fn name(&self) -> &'static str {
        "date-calc"
    }

    fn configure_args(&self, cmd: Command) -> Command {
        cmd.arg(
            Arg::new("date-add")
                .long("date-add")
                .value_names(["DATE", "DAYS"])
                .num_args(2)
                .help("Add days to a date (format: DDMMYYYY, DD/MM/YYYY, or DD-MM-YYYY)")
                .long_help("Add specified number of days to the given date. Date can be in DDMMYYYY, DD/MM/YYYY, or DD-MM-YYYY format. Returns the new date with day of the week.")
        )
        .arg(
            Arg::new("date-sub")
                .long("date-sub")
                .value_names(["DATE", "DAYS"])
                .num_args(2)
                .help("Subtract days from a date (format: DDMMYYYY, DD/MM/YYYY, or DD-MM-YYYY)")
                .long_help("Subtract specified number of days from the given date. Date can be in DDMMYYYY, DD/MM/YYYY, or DD-MM-YYYY format. Returns the new date with day of the week.")
        )
    }

    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        if let Some(values) = matches.get_many::<String>("date-add") {
            let values: Vec<&String> = values.collect();
            if values.len() == 2 {
                let date_str = values[0];
                let days_str = values[1];
                let days: i64 = days_str.parse()?;
                
                let date = parse_date(date_str)?;
                let new_date = date + chrono::Duration::days(days);
                let weekday = format_weekday(new_date.weekday());
                
                println!("{} ({})", format_date_output(new_date), weekday);
            }
        } else if let Some(values) = matches.get_many::<String>("date-sub") {
            let values: Vec<&String> = values.collect();
            if values.len() == 2 {
                let date_str = values[0];
                let days_str = values[1];
                let days: i64 = days_str.parse()?;
                
                let date = parse_date(date_str)?;
                let new_date = date - chrono::Duration::days(days);
                let weekday = format_weekday(new_date.weekday());
                
                println!("{} ({})", format_date_output(new_date), weekday);
            }
        }
        Ok(())
    }
}

fn parse_date(date_str: &str) -> Result<NaiveDate, Box<dyn Error>> {
    // Try DDMMYYYY format first
    if date_str.len() == 8 && date_str.chars().all(|c| c.is_numeric()) {
        let day: u32 = date_str[0..2].parse()?;
        let month: u32 = date_str[2..4].parse()?;
        let year: i32 = date_str[4..8].parse()?;
        return Ok(NaiveDate::from_ymd_opt(year, month, day).ok_or("Invalid date")?);
    }
    
    // Try DD/MM/YYYY format
    if let Some(date) = try_parse_with_separator(date_str, '/') {
        return Ok(date);
    }
    
    // Try DD-MM-YYYY format
    if let Some(date) = try_parse_with_separator(date_str, '-') {
        return Ok(date);
    }
    
    Err("Invalid date format. Use DDMMYYYY, DD/MM/YYYY, or DD-MM-YYYY".into())
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
    use chrono::NaiveDate;

    #[test]
    fn test_parse_date_ddmmyyyy() {
        assert_eq!(parse_date("01012023").unwrap(), NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
        assert_eq!(parse_date("31122023").unwrap(), NaiveDate::from_ymd_opt(2023, 12, 31).unwrap());
    }

    #[test]
    fn test_parse_date_with_slashes() {
        assert_eq!(parse_date("01/01/2023").unwrap(), NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
        assert_eq!(parse_date("31/12/2023").unwrap(), NaiveDate::from_ymd_opt(2023, 12, 31).unwrap());
    }

    #[test]
    fn test_parse_date_with_dashes() {
        assert_eq!(parse_date("01-01-2023").unwrap(), NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
        assert_eq!(parse_date("31-12-2023").unwrap(), NaiveDate::from_ymd_opt(2023, 12, 31).unwrap());
    }

    #[test]
    fn test_invalid_date_formats() {
        assert!(parse_date("2023-01-01").is_err());
        assert!(parse_date("invalid").is_err());
        assert!(parse_date("32/01/2023").is_err());
        assert!(parse_date("01/13/2023").is_err());
    }

    #[test]
    fn test_format_date_output() {
        let date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        assert_eq!(format_date_output(date), "01/01/2023");
        
        let date = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
        assert_eq!(format_date_output(date), "31/12/2023");
    }

    #[test]
    fn test_format_weekday() {
        assert_eq!(format_weekday(Weekday::Mon), "Monday");
        assert_eq!(format_weekday(Weekday::Sat), "Saturday");
        assert_eq!(format_weekday(Weekday::Sun), "Sunday");
    }
}