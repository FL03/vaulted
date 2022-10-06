/*
    Appellation: args <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::{EnumString, EnumVariantNames};

#[derive(
    ValueEnum,
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    EnumString,
    EnumVariantNames,
    Hash,
    PartialEq,
    Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum PowerArgs {
    On,
    Off,
    #[default]
    Idle,
}

#[derive(
    ValueEnum,
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    EnumString,
    EnumVariantNames,
    Hash,
    PartialEq,
    Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum CRUDArgs {
    #[default]
    Create,
    Read,
    Update,
    Delete,
}
