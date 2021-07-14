use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if config.help {
        println!("Usage: minigrep <flags> [query] [filename]");
        println!("");
        println!("Flags:                Description");
        println!("-i, --ignorecase      Case insensitive search");
        println!("--noignorecase        Case sensitive search");
        println!("-h, --help            Show this menu");
        println!("");
        println!("NOTE: Named flags such as `--noignorecase` take precedence over");
        println!("      short flags like `-i` no matter the position.");
        println!("      Later flags take precedence over earlier flags:");
        println!("      `minigrep --ignorecase --noignorecase ...` will make the");
        println!("      search case sensitive.");
        return Ok(());
    }

    let contents = fs::read_to_string(config.filename)?;

    let results = match config.case_sensitive {
        true => search(&config.query, &contents),
        false => search_case_insensitive(&config.query, &contents),
    };

    for line in results.into_iter() {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[derive(Debug)]
pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
    help: bool,
}

impl Config {
    pub fn try_from(args: std::env::Args) -> Result<Config, String> {
        let arguments = Arguments::try_from(args)?;

        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        let mut help = false;

        for flag in arguments.simple_flags {
            match flag.as_str() {
                "-i" => case_sensitive = false,
                "-h" => help = true,
                f => return Err(format!("Unknown flag: `{}`", f)),
            }
        }

        for flag in arguments.named_flags {
            match flag.as_str() {
                "--ignorecase" => case_sensitive = false,
                "--noignorecase" => case_sensitive = true,
                "--help" => help = true,
                f => return Err(format!("Unknown flag: `{}`", f)),
            }
        }

        if help {
            return Ok(Config {
                query: "".to_string(),
                filename: "".to_string(),
                case_sensitive,
                help,
            });
        }

        if arguments.names.len() != 3 {
            return Err(format!(
                "Incorrect number of arguments, 2 expected, got: {}",
                arguments.names.len() - 1
            ));
        }

        let mut names = arguments.names.into_iter().skip(1);

        let query = names.next().unwrap();
        let filename = names.next().unwrap();

        Ok(Config {
            query,
            filename,
            case_sensitive,
            help,
        })
    }
}

struct Arguments {
    simple_flags: Vec<String>,
    named_flags: Vec<String>,
    names: Vec<String>,
}

impl Arguments {
    /* Converts a list of strings into 3 lists depending on their format:
     * simple_flags contains flags like -i, -e, and -I
     * named_flags contains the long format flags like --ignorecase, or --restrict
     * names contains non-flag arguments like `hello`, or `input.txt` */
    fn try_from(args: std::env::Args) -> Result<Arguments, String> {
        let mut simple_flags = Vec::new();
        let mut named_flags = Vec::new();
        let mut names = Vec::new();

        for argument in args {
            let mut chars = argument.chars();

            if argument.len() < 1 {
                return Err("Empty string can not be an argument".to_string());
            }

            if let Some('-') = chars.next() {
                match chars.next() {
                    Some('-') => {
                        if let Some(_) = chars.next() {
                            named_flags.push(argument);
                        } else {
                            return Err(
                                "Expected flag after `--`. Example: `--ignorecase`".to_string()
                            );
                        }
                    }
                    Some(_) => {
                        if let None = chars.next() {
                            simple_flags.push(argument);
                        } else {
                            return Err(format!(
                                "Expected single character after `-`, got: `{}`.",
                                argument
                            ));
                        }
                    }
                    None => {
                        return Err("Expected simple flag after `-`. Example: `-i`".to_string());
                    }
                }
            } else {
                names.push(argument);
            }
        }

        Ok(Arguments {
            simple_flags,
            named_flags,
            names,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
