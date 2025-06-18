use std::fmt;
use rocket::http::Status;

#[derive(Debug)]
pub enum ConfigError {
    InvalidDeviceId,
    InvalidConfig,
    DatabaseError(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Config error: {:?}", self)
    }
}

