use crate::error::{MsError, MsResult};
use crate::tool_module::ToolModule;
use crate::util::{clipboard, stdin};
use clap::{Arg, ArgMatches, Command};

pub struct CaseConvertModule;

impl ToolModule for CaseConvertModule {
    fn name(&self) -> &'static str {
        "case"
    }

    fn command(&self) -> Command {
        Command::new("case")
            .about("Convert text case (upper/lower/title/camel/snake/kebab)")
            .arg(
                Arg::new("type")
                    .required(true)
                    .help("Case type: upper, lower, title, camel, pascal, snake, kebab, constant"),
            )
            .arg(Arg::new("text").value_name("TEXT").help("Text to convert"))
    }

    fn execute(&self, matches: &ArgMatches) -> MsResult<()> {
        let case_type = matches.get_one::<String>("type").unwrap();
        let text = stdin::get_text_input(matches.get_one::<String>("text"))
            .ok_or_else(|| MsError::InvalidInput("No text provided".into()))?;

        let converted = match case_type.to_lowercase().as_str() {
            "upper" => text.to_uppercase(),
            "lower" => text.to_lowercase(),
            "title" => to_title_case(&text),
            "camel" => to_camel_case(&text),
            "pascal" => to_pascal_case(&text),
            "snake" => to_snake_case(&text),
            "kebab" => to_kebab_case(&text),
            "constant" => to_constant_case(&text),
            _ => {
                return Err(MsError::InvalidInput(
                    "Invalid case type. Use: upper, lower, title, camel, pascal, snake, kebab, constant".into(),
                ))
            }
        };

        clipboard::copy_and_print(&converted);
        Ok(())
    }
}

fn to_title_case(text: &str) -> String {
    text.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().collect::<String>()
                        + &chars.as_str().to_lowercase()
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn to_camel_case(text: &str) -> String {
    let words = split_into_words(text);
    if words.is_empty() {
        return String::new();
    }
    let mut result = words[0].to_lowercase();
    for word in &words[1..] {
        result.push_str(&capitalize_word(word));
    }
    result
}

fn to_pascal_case(text: &str) -> String {
    split_into_words(text)
        .iter()
        .map(|word| capitalize_word(word))
        .collect()
}

fn to_snake_case(text: &str) -> String {
    split_into_words(text)
        .iter()
        .map(|word| word.to_lowercase())
        .collect::<Vec<_>>()
        .join("_")
}

fn to_kebab_case(text: &str) -> String {
    split_into_words(text)
        .iter()
        .map(|word| word.to_lowercase())
        .collect::<Vec<_>>()
        .join("-")
}

fn to_constant_case(text: &str) -> String {
    split_into_words(text)
        .iter()
        .map(|word| word.to_uppercase())
        .collect::<Vec<_>>()
        .join("_")
}

fn split_into_words(text: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current_word = String::new();

    for ch in text.chars() {
        if ch.is_alphanumeric() {
            let should_split = if !current_word.is_empty() {
                let last_char = current_word.chars().last().unwrap();
                (ch.is_uppercase() && last_char.is_lowercase())
                    || (ch.is_alphabetic() && last_char.is_numeric())
                    || (ch.is_numeric() && last_char.is_alphabetic())
            } else {
                false
            };

            if should_split {
                words.push(current_word.clone());
                current_word.clear();
            }
            current_word.push(ch);
        } else if !current_word.is_empty() {
            words.push(current_word.clone());
            current_word.clear();
        }
    }

    if !current_word.is_empty() {
        words.push(current_word);
    }

    words
}

fn capitalize_word(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            first.to_uppercase().collect::<String>()
                + &chars.as_str().to_lowercase()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_title_case() {
        assert_eq!(to_title_case("hello world"), "Hello World");
        assert_eq!(to_title_case("HELLO WORLD"), "Hello World");
        assert_eq!(to_title_case(""), "");
    }

    #[test]
    fn test_to_camel_case() {
        assert_eq!(to_camel_case("hello world"), "helloWorld");
        assert_eq!(to_camel_case("hello_world"), "helloWorld");
        assert_eq!(to_camel_case("hello-world"), "helloWorld");
        assert_eq!(to_camel_case("HelloWorld"), "helloWorld");
        assert_eq!(to_camel_case(""), "");
    }

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("hello world"), "HelloWorld");
        assert_eq!(to_pascal_case("hello_world"), "HelloWorld");
        assert_eq!(to_pascal_case(""), "");
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("hello world"), "hello_world");
        assert_eq!(to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(to_snake_case("helloWorld"), "hello_world");
        assert_eq!(to_snake_case(""), "");
    }

    #[test]
    fn test_to_kebab_case() {
        assert_eq!(to_kebab_case("hello world"), "hello-world");
        assert_eq!(to_kebab_case("HelloWorld"), "hello-world");
        assert_eq!(to_kebab_case(""), "");
    }

    #[test]
    fn test_to_constant_case() {
        assert_eq!(to_constant_case("hello world"), "HELLO_WORLD");
        assert_eq!(to_constant_case("HelloWorld"), "HELLO_WORLD");
        assert_eq!(to_constant_case(""), "");
    }

    #[test]
    fn test_split_into_words() {
        assert_eq!(split_into_words("hello world"), vec!["hello", "world"]);
        assert_eq!(split_into_words("HelloWorld"), vec!["Hello", "World"]);
        assert_eq!(split_into_words("helloWorld"), vec!["hello", "World"]);
        assert_eq!(split_into_words("hello_world"), vec!["hello", "world"]);
        assert_eq!(
            split_into_words("hello123world"),
            vec!["hello", "123", "world"]
        );
    }

    #[test]
    fn test_with_numbers() {
        assert_eq!(to_snake_case("hello123World"), "hello_123_world");
        assert_eq!(to_camel_case("hello-world-2023"), "helloWorld2023");
        assert_eq!(to_pascal_case("api_v2_endpoint"), "ApiV2Endpoint");
    }
}
