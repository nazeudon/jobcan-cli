use crate::modules::error::HomeDirectoryNotFoundError;
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const JSON_NAME: &str = "credential.json";

#[derive(Deserialize, Serialize, Debug)]
pub struct Credential {
    pub email: String,
    pub password: String,
}

impl Credential {
    pub fn new(email: String, password: String) -> Credential {
        Credential {
            email: email,
            password: password,
        }
    }

    pub fn to_json(&self) -> Result<(), Box<dyn Error>> {
        match home_dir() {
            Some(path) => {
                let path = path.join(JSON_NAME);
                let file = File::create(path)?;
                let credential_text = serde_json::to_string(self)?;
                write!(&file, "{}", credential_text)?;
                Ok(())
            }
            None => return Err(Box::new(HomeDirectoryNotFoundError())),
        }
    }

    pub fn from_json() -> Result<Credential, Box<dyn Error>> {
        match home_dir() {
            Some(path) => {
                let path = path.join(JSON_NAME);
                let file = File::open(path)?;
                let reader = BufReader::new(file);
                let credential: Credential = serde_json::from_reader(reader)?;
                Ok(credential)
            }
            None => return Err(Box::new(HomeDirectoryNotFoundError())),
        }
    }
}
