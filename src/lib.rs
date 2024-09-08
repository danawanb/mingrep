use std::error::Error;
use std::fs;

#[derive(Debug, Clone)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

#[derive(Debug)]
pub enum ErrorParse {
    LenError(String),
    Default,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, ErrorParse> {
        if args.len() < 3 {
            ErrorParse::LenError(String::from("jumlah Argumen kurang dari 3"));
        }

        let mut queryx = String::new();
        let mut file_pathx = String::new();
        let mut case_sensitive = true;

        if let Some(x) = args.get(1) {
            queryx = x.to_string();
        }

        if let Some(y) = args.get(2) {
            file_pathx = y.to_string();
        }

        if let Some(case) = args.get(3) {
            case_sensitive = case.to_lowercase() != "s";
        }

        let config = Config {
            query: queryx,
            file_path: file_pathx,
            ignore_case: case_sensitive,
        };

        Ok(config)
    }
}

pub fn run(c: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&c.file_path)?;

    let res = if c.ignore_case {
        search_case_insensitive(&c.query, &contents)
    } else {
        search(&c.query, &contents)
    };

    for line in res {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res: Vec<&'a str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            res.push(line)
        }
    }
    res
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res: Vec<&'a str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(query) {
            res.push(line)
        }
    }
    println!("test");
    res
}
