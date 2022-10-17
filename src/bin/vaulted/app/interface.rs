/*
    Appellation: interface <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use super::{
    cli::{cmds::Commands, CommandLineInterface},
    states::State,
};
use scsys::BoxResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct App {
    pub state: State,
}

impl App {
    pub fn new(state: State) -> Self {
        Self { state }
    }
    async fn cli(&self) -> &Self {
        CommandLineInterface::new()
            .handler()
            .expect("Failed to run the cli...");
        self
    }
    pub fn with_logging(&self) -> &Self {
        tracing_subscriber::fmt::init();
        self
    }
    pub async fn run(&mut self) -> BoxResult<&Self> {
        self.set_state("complete");
        self.cli().await;

        Ok(self)
    }
    pub fn set_state(&mut self, state: &str) -> &Self {
        let s = match State::try_from(state) {
            Ok(v) => v,
            Err(_) => self.state.clone(),
        };
        self.state = s;
        tracing::info!("{:?}", self.state);
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
