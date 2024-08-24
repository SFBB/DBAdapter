use std::env;

use super::error::AdapterError;

pub struct Config {
    pub db_url: String,
}

impl Config {
    pub fn new() -> Result<Self, AdapterError> {
        let mut db_url = String::new();
        match env::var("db_url") {
            Ok(val) => db_url = val.to_string(),
            Err(e) => {
                print!("Error when creating config: {}", e);
                return Err(
                    AdapterError::ConfigLoadFailed
                )
            }
        }

        Ok(Self {
            db_url: db_url,
        })
    }
}
