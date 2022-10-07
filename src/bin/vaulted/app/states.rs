/*
    Appellation: states <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::{EnumString, EnumVariantNames};

#[derive(Clone, Debug, Deserialize, EnumString, EnumVariantNames, Eq, PartialEq, Serialize)]
#[strum(serialize_all = "snake_case")]
pub enum State {
    Idle,
    
}

impl Default for State {
    fn default() -> Self {
        Self::Idel
    }
}