/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::settings::Settings;
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Context {
    pub settings: Settings,
    pub state: State,
}

impl Context {
    pub fn new(settings: Settings) -> Self {
        let state = State::default();
        Self { settings, state }
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

#[derive(
    Clone, Debug, Deserialize, EnumString, EnumVariantNames, Eq, Hash, PartialEq, Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum State {
    Request { data: Vec<String> },
    Idle,
}

impl Default for State {
    fn default() -> Self {
        Self::Idle
    }
}
