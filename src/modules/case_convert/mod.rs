use crate::tool_module::ToolModule;
use arboard::Clipboard;
use clap::{Arg, ArgMatches, Command};
use std::error::Error;

pub struct CaseConvertModule;

impl ToolModule for CaseConvertModule {
    fn name(&self) -> &'static str {
        "case-convert"
    }

    fn configure_args(&self, cmd: Command) -> Command {
        cmd.arg(
            Arg::new("case-convert")
                .long("case-convert")
                .value_names(["TEXT", "TYPE"])
                .num_args(2)
                .help("Convert text case (upper/lower/title/camel/snake/kebab)")
                .long_help("Convert text to different case formats:\n- upper: UPPERCASE\n- lower: lowercase\n- title: Title Case\n- camel: camelCase\n- pascal: PascalCase\n- snake: snake_case\n- kebab: kebab-case\n- constant: CONSTANT_CASE\n\nResult is automatically copied to clipboard.")
        )
    }

    fn execute(&self, matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
        if let Some(values) = matches.get_many::<String>("case-convert") {
            let values: Vec<&String> = values.collect();
            if values.len() == 2 {
                let text = values[0];
                let case_type = values[1];
                
                let converted = match case_type.to_lowercase().as_str() {
                    "upper" => text.to_uppercase(),
                    "lower" => text.to_lowercase(),
                    "title" => to_title_case(text),
                    "camel" => to_camel_case(text),
                    "pascal" => to_pascal_case(text),
                    "snake" => to_snake_case(text),
                    "kebab" => to_kebab_case(text),
                    "constant" => to_constant_case(text),
                    _ => return Err("Invalid case type. Use: upper, lower, title, camel, pascal, snake, kebab, constant".into()),
                };
                
                copy_to_clipboard_and_print(&converted);
            }
        }
        Ok(())
    }
}

fn to_title_case(text: &str) -> String {
    text.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
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
    let chars: Vec<char> = text.chars().collect();
    
    for &ch in chars.iter() {
        if ch.is_alphanumeric() {
            let should_split = if !current_word.is_empty() {
                let last_char = current_word.chars().last().unwrap();
                // Split on lowercase to uppercase transition
                (ch.is_uppercase() && last_char.is_lowercase()) ||
                // Split on number to letter transition
                (ch.is_alphabetic() && last_char.is_numeric()) ||
                // Split on letter to number transition  
                (ch.is_numeric() && last_char.is_alphabetic())
            } else {
                false
            };
            
            if should_split {
                words.push(current_word.clone());
                current_word.clear();
            }
            current_word.push(ch);
        } else if ch.is_whitespace() || ch == '_' || ch == '-' || !ch.is_alphanumeric() {
            if !current_word.is_empty() {
                words.push(current_word.clone());
                current_word.clear();
            }
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
        Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
    }
}

fn copy_to_clipboard_and_print(text: &str) {
    match Clipboard::new() {
        Ok(mut clipboard) => {
            if let Err(e) = clipboard.set_text(text) {
                eprintln!("Warning: Failed to copy to clipboard: {}", e);
                println!("{}", text);
            } else {
                println!("{} (copied to clipboard)", text);
            }
        }
        Err(e) => {
            eprintln!("Warning: Failed to access clipboard: {}", e);
            println!("{}", text);
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
        assert_eq!(to_title_case("hello"), "Hello");
        assert_eq!(to_title_case(""), "");
        assert_eq!(to_title_case("the quick brown fox"), "The Quick Brown Fox");
    }

    #[test]
    fn test_to_camel_case() {
        assert_eq!(to_camel_case("hello world"), "helloWorld");
        assert_eq!(to_camel_case("Hello World"), "helloWorld");
        assert_eq!(to_camel_case("hello_world"), "helloWorld");
        assert_eq!(to_camel_case("hello-world"), "helloWorld");
        assert_eq!(to_camel_case("HelloWorld"), "helloWorld");
        assert_eq!(to_camel_case("hello"), "hello");
        assert_eq!(to_camel_case(""), "");
    }

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("hello world"), "HelloWorld");
        assert_eq!(to_pascal_case("hello_world"), "HelloWorld");
        assert_eq!(to_pascal_case("hello-world"), "HelloWorld");
        assert_eq!(to_pascal_case("helloWorld"), "HelloWorld");
        assert_eq!(to_pascal_case("hello"), "Hello");
        assert_eq!(to_pascal_case(""), "");
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("hello world"), "hello_world");
        assert_eq!(to_snake_case("Hello World"), "hello_world");
        assert_eq!(to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(to_snake_case("helloWorld"), "hello_world");
        assert_eq!(to_snake_case("hello-world"), "hello_world");
        assert_eq!(to_snake_case("hello"), "hello");
        assert_eq!(to_snake_case(""), "");
    }

    #[test]
    fn test_to_kebab_case() {
        assert_eq!(to_kebab_case("hello world"), "hello-world");
        assert_eq!(to_kebab_case("Hello World"), "hello-world");
        assert_eq!(to_kebab_case("HelloWorld"), "hello-world");
        assert_eq!(to_kebab_case("helloWorld"), "hello-world");
        assert_eq!(to_kebab_case("hello_world"), "hello-world");
        assert_eq!(to_kebab_case("hello"), "hello");
        assert_eq!(to_kebab_case(""), "");
    }

    #[test]
    fn test_to_constant_case() {
        assert_eq!(to_constant_case("hello world"), "HELLO_WORLD");
        assert_eq!(to_constant_case("Hello World"), "HELLO_WORLD");
        assert_eq!(to_constant_case("HelloWorld"), "HELLO_WORLD");
        assert_eq!(to_constant_case("helloWorld"), "HELLO_WORLD");
        assert_eq!(to_constant_case("hello-world"), "HELLO_WORLD");
        assert_eq!(to_constant_case("hello"), "HELLO");
        assert_eq!(to_constant_case(""), "");
    }

    #[test]
    fn test_split_into_words() {
        assert_eq!(split_into_words("hello world"), vec!["hello", "world"]);
        assert_eq!(split_into_words("HelloWorld"), vec!["Hello", "World"]);
        assert_eq!(split_into_words("helloWorld"), vec!["hello", "World"]);
        assert_eq!(split_into_words("hello_world"), vec!["hello", "world"]);
        assert_eq!(split_into_words("hello-world"), vec!["hello", "world"]);
        assert_eq!(split_into_words("hello123world"), vec!["hello", "123", "world"]);
        assert_eq!(split_into_words("XMLHttpRequest"), vec!["XMLHttp", "Request"]);
    }

    #[test]
    fn test_complex_cases() {
        let input = "XMLHttpRequestFactory";
        assert_eq!(to_snake_case(input), "xmlhttp_request_factory");
        assert_eq!(to_kebab_case(input), "xmlhttp-request-factory");
        assert_eq!(to_camel_case(input), "xmlhttpRequestFactory");
    }

    #[test]
    fn test_with_numbers_and_special_chars() {
        assert_eq!(to_snake_case("hello123World"), "hello_123_world");
        assert_eq!(to_camel_case("hello-world-2023"), "helloWorld2023");
        assert_eq!(to_pascal_case("api_v2_endpoint"), "ApiV2Endpoint");
    }
}