use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Mode {
    Auth,
    Start,
    End,
}

impl FromStr for Mode {
    type Err = ();
    fn from_str(input: &str) -> Result<Mode, Self::Err> {
        match input {
            "auth" => Ok(Mode::Auth),
            "start" => Ok(Mode::Start),
            "end" => Ok(Mode::End),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub mode: Mode,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments.");
        }
        if args.len() >= 3 {
            return Err("Too many arguments.");
        }
        let result = Mode::from_str(&args[1]);
        match result {
            Ok(mode) => Ok(Config { mode }),
            Err(_) => Err("Incorrect argument character."),
        }
    }
}
