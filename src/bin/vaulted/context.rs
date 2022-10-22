/*
    Appellation: context <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use super::settings::Settings;
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Context {
    pub settings: Settings,
}

impl Context {
    pub fn new(settings: Settings) -> Self {
        Self { settings }
    }
}

#[derive(Clone, Debug, Deserialize, EnumString, EnumVariantNames, Eq, PartialEq, Serialize)]
#[strum(serialize_all = "snake_case")]
pub enum State {
    Request { data: serde_json::Value },
    Idle,
}

impl Default for State {
    fn default() -> Self {
        Self::Idle
    }
}
