/*
    Appellation: interface <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use crate::cli::CommandLineInterface;
use scsys::core::BoxResult;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::{EnumString, EnumVariantNames};

#[derive(
    Clone, Copy, Debug, Hash, PartialEq, EnumString, EnumVariantNames, Deserialize, Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum Stage {
    Startup,
    Shutdown,
}

#[derive(
    Clone, Copy, Debug, Hash, PartialEq, EnumString, EnumVariantNames, Deserialize, Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum State {
    Off,
    On,
    Processing,
}

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Interface {
    pub state: State,
}

impl Interface {
    pub fn new() -> Self {
        Self { state: State::On }
    }
    pub fn cli(&mut self) -> BoxResult<&Self> {
        self.state = State::from_str("processing").ok().unwrap();
        let data = CommandLineInterface::data();
        println!("Processing inputs...");

        Ok(self)
    }
}
