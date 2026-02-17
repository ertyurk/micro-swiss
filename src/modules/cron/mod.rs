use crate::error::{MsError, MsResult};
use crate::tool_module::ToolModule;
use clap::{Arg, ArgMatches, Command};

pub struct CronModule;

impl ToolModule for CronModule {
    fn name(&self) -> &'static str {
        "cron"
    }

    fn command(&self) -> Command {
        Command::new("cron")
            .about("Describe a cron expression in English")
            .arg(
                Arg::new("expr")
                    .required(true)
                    .help("Cron expression (5 fields: min hour dom mon dow)"),
            )
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        let expr = matches.get_one::<String>("expr").unwrap();
        let description = describe_cron(expr)?;
        println!("{}", description);
        Ok(())
    }
}

fn describe_cron(expr: &str) -> MsResult<String> {
    let fields: Vec<&str> = expr.split_whitespace().collect();
    if fields.len() != 5 {
        return Err(MsError::InvalidInput(
            "Cron expression must have 5 fields: minute hour day-of-month month day-of-week"
                .into(),
        ));
    }

    let minute = describe_field(fields[0], "minute", 0, 59)?;
    let hour = describe_field(fields[1], "hour", 0, 23)?;
    let dom = describe_field(fields[2], "day-of-month", 1, 31)?;
    let month = describe_month_field(fields[3])?;
    let dow = describe_dow_field(fields[4])?;

    let mut parts = Vec::new();

    // Time description
    match (fields[0], fields[1]) {
        ("*", "*") => parts.push("Every minute".to_string()),
        (m, "*") if m != "*" => parts.push(format!("At {} {}", minute, "of every hour")),
        ("*", h) if h != "*" => parts.push(format!("Every minute at {}", hour)),
        (_, _) => parts.push(format!("At {}, {}", minute, hour)),
    }

    // Day/month constraints
    if fields[2] != "*" {
        parts.push(format!("on {}", dom));
    }
    if fields[3] != "*" {
        parts.push(format!("in {}", month));
    }
    if fields[4] != "*" {
        parts.push(format!("on {}", dow));
    }

    Ok(parts.join(", "))
}

fn describe_field(field: &str, name: &str, min: u32, max: u32) -> MsResult<String> {
    if field == "*" {
        return Ok(format!("every {}", name));
    }

    if let Some(step) = field.strip_prefix("*/") {
        let n: u32 = step
            .parse()
            .map_err(|_| MsError::InvalidInput(format!("Invalid step in {}", name)))?;
        return Ok(format!("every {} {}s", n, name));
    }

    if field.contains(',') {
        let values: Vec<&str> = field.split(',').collect();
        return Ok(format!("{} {}", name, values.join(", ")));
    }

    if field.contains('-') {
        let parts: Vec<&str> = field.split('-').collect();
        if parts.len() == 2 {
            return Ok(format!("{} {}-{}", name, parts[0], parts[1]));
        }
    }

    let n: u32 = field.parse().map_err(|_| {
        MsError::InvalidInput(format!("Invalid value '{}' for {}", field, name))
    })?;
    if n < min || n > max {
        return Err(MsError::InvalidInput(format!(
            "{} value {} out of range {}-{}",
            name, n, min, max
        )));
    }

    Ok(format!("{} {}", name, n))
}

fn describe_month_field(field: &str) -> MsResult<String> {
    if field == "*" {
        return Ok("every month".to_string());
    }
    let months = [
        "", "January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December",
    ];
    if let Ok(n) = field.parse::<usize>() {
        if (1..=12).contains(&n) {
            return Ok(months[n].to_string());
        }
    }
    Ok(format!("month {}", field))
}

fn describe_dow_field(field: &str) -> MsResult<String> {
    if field == "*" {
        return Ok("every day".to_string());
    }
    let days = [
        "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday",
        "Saturday",
    ];
    if let Ok(n) = field.parse::<usize>() {
        if n <= 6 {
            return Ok(days[n].to_string());
        }
    }
    Ok(format!("day {}", field))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_every_minute() {
        let result = describe_cron("* * * * *").unwrap();
        assert!(result.contains("Every minute"));
    }

    #[test]
    fn test_specific_time() {
        let result = describe_cron("30 9 * * *").unwrap();
        assert!(result.contains("30"));
        assert!(result.contains("9"));
    }

    #[test]
    fn test_step() {
        let result = describe_cron("*/5 * * * *").unwrap();
        assert!(result.contains("every 5"));
    }

    #[test]
    fn test_invalid_fields() {
        assert!(describe_cron("* *").is_err());
        assert!(describe_cron("* * * * * *").is_err());
    }

    #[test]
    fn test_with_dow() {
        let result = describe_cron("0 9 * * 1").unwrap();
        assert!(result.contains("Monday"));
    }

    #[test]
    fn test_with_month() {
        let result = describe_cron("0 0 1 6 *").unwrap();
        assert!(result.contains("June"));
    }
}
