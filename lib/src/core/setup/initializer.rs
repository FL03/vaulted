/*
    Appellation: initializer <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: Implements the initializer
*/
use scsys::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Initializer {
    pub curdir: String,
    pub temp: String,
    pub timestamp: Timestamp,
}

impl Initializer {
    pub fn new(curdir: String, temp: String, timestamp: Timestamp) -> Self {
        Self {
            curdir,
            temp,
            timestamp,
        }
    }
}
