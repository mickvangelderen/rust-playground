pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(args: std::env::Args) -> Result<Config, &'static str> {
        // Skip the executable path argument.
        let mut args = args.skip(1);

        Result::Ok(Config {
            query: match args.next() {
                Some(value) => value,
                _ => return Result::Err("Provide the string to search for."),
            },
            file_path: match args.next() {
                Some(value) => value,
                _ => return Result::Err("Provide the path to the file in which to search."),
            }
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<std::error::Error>> {
    let contents = read_file_to_string(&config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{}", &line);
    }

    Ok(())
}

fn read_file_to_string(file_path: &str) -> std::io::Result<String> {
    let mut file = std::fs::File::open(&file_path)?;
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents)?;
    Ok(contents)
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
    .filter(|&line| { line.contains(query) })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_matches() {
        let query = "duck";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            std::vec::Vec::<&str>::new(),
            search(query, contents)
        );
    }

    #[test]
    fn one_match() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn multiple_matches() {
        let query = ".";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec![
                "safe, fast, productive.",
                "Pick three.",
            ],
            search(query, contents)
        );
    }
}
