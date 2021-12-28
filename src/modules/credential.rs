use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

const JSON_PATH: &str = "src/credential.json";

#[derive(Deserialize, Debug)]
pub struct Credential {
    pub email: String,
    pub password: String,
}

impl Credential {
    pub fn from_json() -> Result<Credential, Box<dyn Error>> {
        let current_dir = env::current_dir()?;
        let path = current_dir.join(JSON_PATH);
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let credential: Credential = serde_json::from_reader(reader)?;

        Ok(credential)
    }
}
