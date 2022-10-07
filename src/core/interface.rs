/*
    Appellation: interface <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use crate::cli::{CommandLineInterface, Commands};
use scsys::core::BoxResult;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::{EnumString, EnumVariantNames};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct App {
    pub state: State,
}

impl App {
    pub fn new(state: State) -> Self {
        Self { state }
    }
    fn cli(&self) -> CommandLineInterface {
        let data = CommandLineInterface::data();
        let cmd = data.clone().command;
        if cmd.is_some() {
            let cmd =cmd.unwrap();
            match cmd {
                Commands::Application { mode } => {
                    println!("{:?}", mode);
                },
            }

        }

        data
    }
    pub async fn run(&mut self) -> BoxResult<&Self> {
        self.set_state("complete");
        println!("{:?}", self.state);

        let cli = self.cli();

        Ok(self)
    }
    pub fn set_state(&mut self, state: &str) -> &Self {
        let s = match State::try_from(state) {
            Ok(v) => v,
            Err(_) => self.state.clone(),
        };
        self.state = s;
        self
    }
    pub fn state(&self) -> &State {
        &self.state
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new(State::default())
    }
}

#[derive(Clone, Debug, Deserialize, EnumString, EnumVariantNames, Eq, PartialEq, Serialize)]
#[strum(serialize_all = "snake_case")]
pub enum State {
    Awaiting,
    Complete,
    Compute,
    Idle,
    Processing,
    Request { header: serde_json::Value },
}

impl Default for State {
    fn default() -> Self {
        Self::Idle
    }
}
